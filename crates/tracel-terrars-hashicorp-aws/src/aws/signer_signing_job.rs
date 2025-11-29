use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SignerSigningJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_signing_job_failure: Option<PrimField<bool>>,
    profile_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<Vec<SignerSigningJobDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<SignerSigningJobSourceEl>>,
    dynamic: SignerSigningJobDynamic,
}

struct SignerSigningJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SignerSigningJobData>,
}

#[derive(Clone)]
pub struct SignerSigningJob(Rc<SignerSigningJob_>);

impl SignerSigningJob {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_signing_job_failure`.\n"]
    pub fn set_ignore_signing_job_failure(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_signing_job_failure = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(self, v: impl Into<BlockAssignable<SignerSigningJobDestinationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl Into<BlockAssignable<SignerSigningJobSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `completed_at` after provisioning.\n"]
    pub fn completed_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.completed_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignore_signing_job_failure` after provisioning.\n"]
    pub fn ignore_signing_job_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_signing_job_failure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_id` after provisioning.\n"]
    pub fn job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_invoker` after provisioning.\n"]
    pub fn job_invoker(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_invoker", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_owner` after provisioning.\n"]
    pub fn job_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_display_name` after provisioning.\n"]
    pub fn platform_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_id` after provisioning.\n"]
    pub fn platform_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_name` after provisioning.\n"]
    pub fn profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_version` after provisioning.\n"]
    pub fn profile_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requested_by` after provisioning.\n"]
    pub fn requested_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.requested_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_record` after provisioning.\n"]
    pub fn revocation_record(&self) -> ListRef<SignerSigningJobRevocationRecordElRef> {
        ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_expires_at` after provisioning.\n"]
    pub fn signature_expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signature_expires_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signed_object` after provisioning.\n"]
    pub fn signed_object(&self) -> ListRef<SignerSigningJobSignedObjectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.signed_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<SignerSigningJobDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<SignerSigningJobSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl Referable for SignerSigningJob {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SignerSigningJob { }

impl ToListMappable for SignerSigningJob {
    type O = ListRef<SignerSigningJobRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SignerSigningJob_ {
    fn extract_resource_type(&self) -> String {
        "aws_signer_signing_job".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSignerSigningJob {
    pub tf_id: String,
    #[doc = ""]
    pub profile_name: PrimField<String>,
}

impl BuildSignerSigningJob {
    pub fn build(self, stack: &mut Stack) -> SignerSigningJob {
        let out = SignerSigningJob(Rc::new(SignerSigningJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SignerSigningJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                ignore_signing_job_failure: core::default::Default::default(),
                profile_name: self.profile_name,
                region: core::default::Default::default(),
                destination: core::default::Default::default(),
                source: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SignerSigningJobRef {
    shared: StackShared,
    base: String,
}

impl Ref for SignerSigningJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SignerSigningJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `completed_at` after provisioning.\n"]
    pub fn completed_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.completed_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignore_signing_job_failure` after provisioning.\n"]
    pub fn ignore_signing_job_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_signing_job_failure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_id` after provisioning.\n"]
    pub fn job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_invoker` after provisioning.\n"]
    pub fn job_invoker(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_invoker", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_owner` after provisioning.\n"]
    pub fn job_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_display_name` after provisioning.\n"]
    pub fn platform_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_id` after provisioning.\n"]
    pub fn platform_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_name` after provisioning.\n"]
    pub fn profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_version` after provisioning.\n"]
    pub fn profile_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requested_by` after provisioning.\n"]
    pub fn requested_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.requested_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocation_record` after provisioning.\n"]
    pub fn revocation_record(&self) -> ListRef<SignerSigningJobRevocationRecordElRef> {
        ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_expires_at` after provisioning.\n"]
    pub fn signature_expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signature_expires_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signed_object` after provisioning.\n"]
    pub fn signed_object(&self) -> ListRef<SignerSigningJobSignedObjectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.signed_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<SignerSigningJobDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<SignerSigningJobSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SignerSigningJobRevocationRecordEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoked_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoked_by: Option<PrimField<String>>,
}

impl SignerSigningJobRevocationRecordEl {
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }

    #[doc = "Set the field `revoked_at`.\n"]
    pub fn set_revoked_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revoked_at = Some(v.into());
        self
    }

    #[doc = "Set the field `revoked_by`.\n"]
    pub fn set_revoked_by(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revoked_by = Some(v.into());
        self
    }
}

impl ToListMappable for SignerSigningJobRevocationRecordEl {
    type O = BlockAssignable<SignerSigningJobRevocationRecordEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobRevocationRecordEl {}

impl BuildSignerSigningJobRevocationRecordEl {
    pub fn build(self) -> SignerSigningJobRevocationRecordEl {
        SignerSigningJobRevocationRecordEl {
            reason: core::default::Default::default(),
            revoked_at: core::default::Default::default(),
            revoked_by: core::default::Default::default(),
        }
    }
}

pub struct SignerSigningJobRevocationRecordElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SignerSigningJobRevocationRecordElRef {
    fn new(shared: StackShared, base: String) -> SignerSigningJobRevocationRecordElRef {
        SignerSigningJobRevocationRecordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobRevocationRecordElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }

    #[doc = "Get a reference to the value of field `revoked_at` after provisioning.\n"]
    pub fn revoked_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revoked_at", self.base))
    }

    #[doc = "Get a reference to the value of field `revoked_by` after provisioning.\n"]
    pub fn revoked_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revoked_by", self.base))
    }
}

#[derive(Serialize)]
pub struct SignerSigningJobSignedObjectElS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
}

impl SignerSigningJobSignedObjectElS3El {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
}

impl ToListMappable for SignerSigningJobSignedObjectElS3El {
    type O = BlockAssignable<SignerSigningJobSignedObjectElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobSignedObjectElS3El {}

impl BuildSignerSigningJobSignedObjectElS3El {
    pub fn build(self) -> SignerSigningJobSignedObjectElS3El {
        SignerSigningJobSignedObjectElS3El {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct SignerSigningJobSignedObjectElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SignerSigningJobSignedObjectElS3ElRef {
    fn new(shared: StackShared, base: String) -> SignerSigningJobSignedObjectElS3ElRef {
        SignerSigningJobSignedObjectElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobSignedObjectElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct SignerSigningJobSignedObjectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<ListField<SignerSigningJobSignedObjectElS3El>>,
}

impl SignerSigningJobSignedObjectEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl Into<ListField<SignerSigningJobSignedObjectElS3El>>) -> Self {
        self.s3 = Some(v.into());
        self
    }
}

impl ToListMappable for SignerSigningJobSignedObjectEl {
    type O = BlockAssignable<SignerSigningJobSignedObjectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobSignedObjectEl {}

impl BuildSignerSigningJobSignedObjectEl {
    pub fn build(self) -> SignerSigningJobSignedObjectEl {
        SignerSigningJobSignedObjectEl { s3: core::default::Default::default() }
    }
}

pub struct SignerSigningJobSignedObjectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SignerSigningJobSignedObjectElRef {
    fn new(shared: StackShared, base: String) -> SignerSigningJobSignedObjectElRef {
        SignerSigningJobSignedObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobSignedObjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<SignerSigningJobSignedObjectElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct SignerSigningJobDestinationElS3El {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}

impl SignerSigningJobDestinationElS3El {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}

impl ToListMappable for SignerSigningJobDestinationElS3El {
    type O = BlockAssignable<SignerSigningJobDestinationElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobDestinationElS3El {
    #[doc = ""]
    pub bucket: PrimField<String>,
}

impl BuildSignerSigningJobDestinationElS3El {
    pub fn build(self) -> SignerSigningJobDestinationElS3El {
        SignerSigningJobDestinationElS3El {
            bucket: self.bucket,
            prefix: core::default::Default::default(),
        }
    }
}

pub struct SignerSigningJobDestinationElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SignerSigningJobDestinationElS3ElRef {
    fn new(shared: StackShared, base: String) -> SignerSigningJobDestinationElS3ElRef {
        SignerSigningJobDestinationElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobDestinationElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct SignerSigningJobDestinationElDynamic {
    s3: Option<DynamicBlock<SignerSigningJobDestinationElS3El>>,
}

#[derive(Serialize)]
pub struct SignerSigningJobDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<SignerSigningJobDestinationElS3El>>,
    dynamic: SignerSigningJobDestinationElDynamic,
}

impl SignerSigningJobDestinationEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl Into<BlockAssignable<SignerSigningJobDestinationElS3El>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SignerSigningJobDestinationEl {
    type O = BlockAssignable<SignerSigningJobDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobDestinationEl {}

impl BuildSignerSigningJobDestinationEl {
    pub fn build(self) -> SignerSigningJobDestinationEl {
        SignerSigningJobDestinationEl {
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SignerSigningJobDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SignerSigningJobDestinationElRef {
    fn new(shared: StackShared, base: String) -> SignerSigningJobDestinationElRef {
        SignerSigningJobDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<SignerSigningJobDestinationElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct SignerSigningJobSourceElS3El {
    bucket: PrimField<String>,
    key: PrimField<String>,
    version: PrimField<String>,
}

impl SignerSigningJobSourceElS3El { }

impl ToListMappable for SignerSigningJobSourceElS3El {
    type O = BlockAssignable<SignerSigningJobSourceElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobSourceElS3El {
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub version: PrimField<String>,
}

impl BuildSignerSigningJobSourceElS3El {
    pub fn build(self) -> SignerSigningJobSourceElS3El {
        SignerSigningJobSourceElS3El {
            bucket: self.bucket,
            key: self.key,
            version: self.version,
        }
    }
}

pub struct SignerSigningJobSourceElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SignerSigningJobSourceElS3ElRef {
    fn new(shared: StackShared, base: String) -> SignerSigningJobSourceElS3ElRef {
        SignerSigningJobSourceElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobSourceElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct SignerSigningJobSourceElDynamic {
    s3: Option<DynamicBlock<SignerSigningJobSourceElS3El>>,
}

#[derive(Serialize)]
pub struct SignerSigningJobSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<SignerSigningJobSourceElS3El>>,
    dynamic: SignerSigningJobSourceElDynamic,
}

impl SignerSigningJobSourceEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl Into<BlockAssignable<SignerSigningJobSourceElS3El>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SignerSigningJobSourceEl {
    type O = BlockAssignable<SignerSigningJobSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSignerSigningJobSourceEl {}

impl BuildSignerSigningJobSourceEl {
    pub fn build(self) -> SignerSigningJobSourceEl {
        SignerSigningJobSourceEl {
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SignerSigningJobSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SignerSigningJobSourceElRef {
    fn new(shared: StackShared, base: String) -> SignerSigningJobSourceElRef {
        SignerSigningJobSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SignerSigningJobSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<SignerSigningJobSourceElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct SignerSigningJobDynamic {
    destination: Option<DynamicBlock<SignerSigningJobDestinationEl>>,
    source: Option<DynamicBlock<SignerSigningJobSourceEl>>,
}
