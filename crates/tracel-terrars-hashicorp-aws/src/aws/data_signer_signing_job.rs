use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSignerSigningJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    job_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataSignerSigningJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSignerSigningJobData>,
}

#[derive(Clone)]
pub struct DataSignerSigningJob(Rc<DataSignerSigningJob_>);

impl DataSignerSigningJob {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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
    pub fn revocation_record(&self) -> ListRef<DataSignerSigningJobRevocationRecordElRef> {
        ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_expires_at` after provisioning.\n"]
    pub fn signature_expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signature_expires_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signed_object` after provisioning.\n"]
    pub fn signed_object(&self) -> ListRef<DataSignerSigningJobSignedObjectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.signed_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<DataSignerSigningJobSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }
}

impl Referable for DataSignerSigningJob {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSignerSigningJob { }

impl ToListMappable for DataSignerSigningJob {
    type O = ListRef<DataSignerSigningJobRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSignerSigningJob_ {
    fn extract_datasource_type(&self) -> String {
        "aws_signer_signing_job".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSignerSigningJob {
    pub tf_id: String,
    #[doc = ""]
    pub job_id: PrimField<String>,
}

impl BuildDataSignerSigningJob {
    pub fn build(self, stack: &mut Stack) -> DataSignerSigningJob {
        let out = DataSignerSigningJob(Rc::new(DataSignerSigningJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSignerSigningJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                job_id: self.job_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSignerSigningJobRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSignerSigningJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataSignerSigningJobRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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
    pub fn revocation_record(&self) -> ListRef<DataSignerSigningJobRevocationRecordElRef> {
        ListRef::new(self.shared().clone(), format!("{}.revocation_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signature_expires_at` after provisioning.\n"]
    pub fn signature_expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signature_expires_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signed_object` after provisioning.\n"]
    pub fn signed_object(&self) -> ListRef<DataSignerSigningJobSignedObjectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.signed_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<DataSignerSigningJobSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSignerSigningJobRevocationRecordEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoked_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoked_by: Option<PrimField<String>>,
}

impl DataSignerSigningJobRevocationRecordEl {
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

impl ToListMappable for DataSignerSigningJobRevocationRecordEl {
    type O = BlockAssignable<DataSignerSigningJobRevocationRecordEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningJobRevocationRecordEl {}

impl BuildDataSignerSigningJobRevocationRecordEl {
    pub fn build(self) -> DataSignerSigningJobRevocationRecordEl {
        DataSignerSigningJobRevocationRecordEl {
            reason: core::default::Default::default(),
            revoked_at: core::default::Default::default(),
            revoked_by: core::default::Default::default(),
        }
    }
}

pub struct DataSignerSigningJobRevocationRecordElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSignerSigningJobRevocationRecordElRef {
    fn new(shared: StackShared, base: String) -> DataSignerSigningJobRevocationRecordElRef {
        DataSignerSigningJobRevocationRecordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningJobRevocationRecordElRef {
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
pub struct DataSignerSigningJobSignedObjectElS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
}

impl DataSignerSigningJobSignedObjectElS3El {
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

impl ToListMappable for DataSignerSigningJobSignedObjectElS3El {
    type O = BlockAssignable<DataSignerSigningJobSignedObjectElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningJobSignedObjectElS3El {}

impl BuildDataSignerSigningJobSignedObjectElS3El {
    pub fn build(self) -> DataSignerSigningJobSignedObjectElS3El {
        DataSignerSigningJobSignedObjectElS3El {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct DataSignerSigningJobSignedObjectElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSignerSigningJobSignedObjectElS3ElRef {
    fn new(shared: StackShared, base: String) -> DataSignerSigningJobSignedObjectElS3ElRef {
        DataSignerSigningJobSignedObjectElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningJobSignedObjectElS3ElRef {
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
pub struct DataSignerSigningJobSignedObjectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<ListField<DataSignerSigningJobSignedObjectElS3El>>,
}

impl DataSignerSigningJobSignedObjectEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl Into<ListField<DataSignerSigningJobSignedObjectElS3El>>) -> Self {
        self.s3 = Some(v.into());
        self
    }
}

impl ToListMappable for DataSignerSigningJobSignedObjectEl {
    type O = BlockAssignable<DataSignerSigningJobSignedObjectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningJobSignedObjectEl {}

impl BuildDataSignerSigningJobSignedObjectEl {
    pub fn build(self) -> DataSignerSigningJobSignedObjectEl {
        DataSignerSigningJobSignedObjectEl { s3: core::default::Default::default() }
    }
}

pub struct DataSignerSigningJobSignedObjectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSignerSigningJobSignedObjectElRef {
    fn new(shared: StackShared, base: String) -> DataSignerSigningJobSignedObjectElRef {
        DataSignerSigningJobSignedObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningJobSignedObjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<DataSignerSigningJobSignedObjectElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSignerSigningJobSourceElS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataSignerSigningJobSourceElS3El {
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

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataSignerSigningJobSourceElS3El {
    type O = BlockAssignable<DataSignerSigningJobSourceElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningJobSourceElS3El {}

impl BuildDataSignerSigningJobSourceElS3El {
    pub fn build(self) -> DataSignerSigningJobSourceElS3El {
        DataSignerSigningJobSourceElS3El {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataSignerSigningJobSourceElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSignerSigningJobSourceElS3ElRef {
    fn new(shared: StackShared, base: String) -> DataSignerSigningJobSourceElS3ElRef {
        DataSignerSigningJobSourceElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningJobSourceElS3ElRef {
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

#[derive(Serialize)]
pub struct DataSignerSigningJobSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<ListField<DataSignerSigningJobSourceElS3El>>,
}

impl DataSignerSigningJobSourceEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl Into<ListField<DataSignerSigningJobSourceElS3El>>) -> Self {
        self.s3 = Some(v.into());
        self
    }
}

impl ToListMappable for DataSignerSigningJobSourceEl {
    type O = BlockAssignable<DataSignerSigningJobSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSignerSigningJobSourceEl {}

impl BuildDataSignerSigningJobSourceEl {
    pub fn build(self) -> DataSignerSigningJobSourceEl {
        DataSignerSigningJobSourceEl { s3: core::default::Default::default() }
    }
}

pub struct DataSignerSigningJobSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSignerSigningJobSourceElRef {
    fn new(shared: StackShared, base: String) -> DataSignerSigningJobSourceElRef {
        DataSignerSigningJobSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSignerSigningJobSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<DataSignerSigningJobSourceElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}
