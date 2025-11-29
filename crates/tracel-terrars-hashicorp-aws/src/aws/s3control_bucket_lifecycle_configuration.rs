use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3controlBucketLifecycleConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<S3controlBucketLifecycleConfigurationRuleEl>>,
    dynamic: S3controlBucketLifecycleConfigurationDynamic,
}

struct S3controlBucketLifecycleConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3controlBucketLifecycleConfigurationData>,
}

#[derive(Clone)]
pub struct S3controlBucketLifecycleConfiguration(Rc<S3controlBucketLifecycleConfiguration_>);

impl S3controlBucketLifecycleConfiguration {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl Into<BlockAssignable<S3controlBucketLifecycleConfigurationRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for S3controlBucketLifecycleConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for S3controlBucketLifecycleConfiguration { }

impl ToListMappable for S3controlBucketLifecycleConfiguration {
    type O = ListRef<S3controlBucketLifecycleConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3controlBucketLifecycleConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3control_bucket_lifecycle_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3controlBucketLifecycleConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
}

impl BuildS3controlBucketLifecycleConfiguration {
    pub fn build(self, stack: &mut Stack) -> S3controlBucketLifecycleConfiguration {
        let out = S3controlBucketLifecycleConfiguration(Rc::new(S3controlBucketLifecycleConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3controlBucketLifecycleConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3controlBucketLifecycleConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlBucketLifecycleConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl S3controlBucketLifecycleConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
    days_after_initiation: PrimField<f64>,
}

impl S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl { }

impl ToListMappable for S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
    type O = BlockAssignable<S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
    #[doc = ""]
    pub days_after_initiation: PrimField<f64>,
}

impl BuildS3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
    pub fn build(self) -> S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
        S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
            days_after_initiation: self.days_after_initiation,
        }
    }
}

pub struct S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
        S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days_after_initiation` after provisioning.\n"]
    pub fn days_after_initiation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days_after_initiation", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlBucketLifecycleConfigurationRuleElExpirationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expired_object_delete_marker: Option<PrimField<bool>>,
}

impl S3controlBucketLifecycleConfigurationRuleElExpirationEl {
    #[doc = "Set the field `date`.\n"]
    pub fn set_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.date = Some(v.into());
        self
    }

    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }

    #[doc = "Set the field `expired_object_delete_marker`.\n"]
    pub fn set_expired_object_delete_marker(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.expired_object_delete_marker = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlBucketLifecycleConfigurationRuleElExpirationEl {
    type O = BlockAssignable<S3controlBucketLifecycleConfigurationRuleElExpirationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlBucketLifecycleConfigurationRuleElExpirationEl {}

impl BuildS3controlBucketLifecycleConfigurationRuleElExpirationEl {
    pub fn build(self) -> S3controlBucketLifecycleConfigurationRuleElExpirationEl {
        S3controlBucketLifecycleConfigurationRuleElExpirationEl {
            date: core::default::Default::default(),
            days: core::default::Default::default(),
            expired_object_delete_marker: core::default::Default::default(),
        }
    }
}

pub struct S3controlBucketLifecycleConfigurationRuleElExpirationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlBucketLifecycleConfigurationRuleElExpirationElRef {
    fn new(shared: StackShared, base: String) -> S3controlBucketLifecycleConfigurationRuleElExpirationElRef {
        S3controlBucketLifecycleConfigurationRuleElExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlBucketLifecycleConfigurationRuleElExpirationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `date` after provisioning.\n"]
    pub fn date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date", self.base))
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `expired_object_delete_marker` after provisioning.\n"]
    pub fn expired_object_delete_marker(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.expired_object_delete_marker", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlBucketLifecycleConfigurationRuleElFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

impl S3controlBucketLifecycleConfigurationRuleElFilterEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlBucketLifecycleConfigurationRuleElFilterEl {
    type O = BlockAssignable<S3controlBucketLifecycleConfigurationRuleElFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlBucketLifecycleConfigurationRuleElFilterEl {}

impl BuildS3controlBucketLifecycleConfigurationRuleElFilterEl {
    pub fn build(self) -> S3controlBucketLifecycleConfigurationRuleElFilterEl {
        S3controlBucketLifecycleConfigurationRuleElFilterEl {
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct S3controlBucketLifecycleConfigurationRuleElFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlBucketLifecycleConfigurationRuleElFilterElRef {
    fn new(shared: StackShared, base: String) -> S3controlBucketLifecycleConfigurationRuleElFilterElRef {
        S3controlBucketLifecycleConfigurationRuleElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlBucketLifecycleConfigurationRuleElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlBucketLifecycleConfigurationRuleElDynamic {
    abort_incomplete_multipart_upload: Option<
        DynamicBlock<S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl>,
    >,
    expiration: Option<DynamicBlock<S3controlBucketLifecycleConfigurationRuleElExpirationEl>>,
    filter: Option<DynamicBlock<S3controlBucketLifecycleConfigurationRuleElFilterEl>>,
}

#[derive(Serialize)]
pub struct S3controlBucketLifecycleConfigurationRuleEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    abort_incomplete_multipart_upload: Option<
        Vec<S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration: Option<Vec<S3controlBucketLifecycleConfigurationRuleElExpirationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<S3controlBucketLifecycleConfigurationRuleElFilterEl>>,
    dynamic: S3controlBucketLifecycleConfigurationRuleElDynamic,
}

impl S3controlBucketLifecycleConfigurationRuleEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `abort_incomplete_multipart_upload`.\n"]
    pub fn set_abort_incomplete_multipart_upload(
        mut self,
        v: impl Into<BlockAssignable<S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.abort_incomplete_multipart_upload = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.abort_incomplete_multipart_upload = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `expiration`.\n"]
    pub fn set_expiration(
        mut self,
        v: impl Into<BlockAssignable<S3controlBucketLifecycleConfigurationRuleElExpirationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expiration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expiration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<BlockAssignable<S3controlBucketLifecycleConfigurationRuleElFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3controlBucketLifecycleConfigurationRuleEl {
    type O = BlockAssignable<S3controlBucketLifecycleConfigurationRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlBucketLifecycleConfigurationRuleEl {
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildS3controlBucketLifecycleConfigurationRuleEl {
    pub fn build(self) -> S3controlBucketLifecycleConfigurationRuleEl {
        S3controlBucketLifecycleConfigurationRuleEl {
            id: self.id,
            status: core::default::Default::default(),
            abort_incomplete_multipart_upload: core::default::Default::default(),
            expiration: core::default::Default::default(),
            filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3controlBucketLifecycleConfigurationRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlBucketLifecycleConfigurationRuleElRef {
    fn new(shared: StackShared, base: String) -> S3controlBucketLifecycleConfigurationRuleElRef {
        S3controlBucketLifecycleConfigurationRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlBucketLifecycleConfigurationRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `abort_incomplete_multipart_upload` after provisioning.\n"]
    pub fn abort_incomplete_multipart_upload(
        &self,
    ) -> ListRef<S3controlBucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef> {
        ListRef::new(self.shared().clone(), format!("{}.abort_incomplete_multipart_upload", self.base))
    }

    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> ListRef<S3controlBucketLifecycleConfigurationRuleElExpirationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expiration", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<S3controlBucketLifecycleConfigurationRuleElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlBucketLifecycleConfigurationDynamic {
    rule: Option<DynamicBlock<S3controlBucketLifecycleConfigurationRuleEl>>,
}
