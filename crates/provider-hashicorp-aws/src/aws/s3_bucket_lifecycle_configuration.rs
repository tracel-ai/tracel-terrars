use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct S3BucketLifecycleConfigurationData {
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
    expected_bucket_owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transition_default_minimum_object_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<S3BucketLifecycleConfigurationRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<S3BucketLifecycleConfigurationTimeoutsEl>,
    dynamic: S3BucketLifecycleConfigurationDynamic,
}
struct S3BucketLifecycleConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketLifecycleConfigurationData>,
}
#[derive(Clone)]
pub struct S3BucketLifecycleConfiguration(Rc<S3BucketLifecycleConfiguration_>);
impl S3BucketLifecycleConfiguration {
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
    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expected_bucket_owner = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `transition_default_minimum_object_size`.\n"]
    pub fn set_transition_default_minimum_object_size(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .transition_default_minimum_object_size = Some(v.into());
        self
    }
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl Into<BlockAssignable<S3BucketLifecycleConfigurationRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<S3BucketLifecycleConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expected_bucket_owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transition_default_minimum_object_size` after provisioning.\n"]
    pub fn transition_default_minimum_object_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.transition_default_minimum_object_size",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<S3BucketLifecycleConfigurationRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3BucketLifecycleConfigurationTimeoutsElRef {
        S3BucketLifecycleConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for S3BucketLifecycleConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for S3BucketLifecycleConfiguration {}
impl ToListMappable for S3BucketLifecycleConfiguration {
    type O = ListRef<S3BucketLifecycleConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for S3BucketLifecycleConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_lifecycle_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildS3BucketLifecycleConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
}
impl BuildS3BucketLifecycleConfiguration {
    pub fn build(self, stack: &mut Stack) -> S3BucketLifecycleConfiguration {
        let out = S3BucketLifecycleConfiguration(Rc::new(S3BucketLifecycleConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketLifecycleConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                region: core::default::Default::default(),
                transition_default_minimum_object_size: core::default::Default::default(),
                rule: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct S3BucketLifecycleConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketLifecycleConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl S3BucketLifecycleConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expected_bucket_owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transition_default_minimum_object_size` after provisioning.\n"]
    pub fn transition_default_minimum_object_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.transition_default_minimum_object_size",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<S3BucketLifecycleConfigurationRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3BucketLifecycleConfigurationTimeoutsElRef {
        S3BucketLifecycleConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days_after_initiation: Option<PrimField<f64>>,
}
impl S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
    #[doc = "Set the field `days_after_initiation`.\n"]
    pub fn set_days_after_initiation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days_after_initiation = Some(v.into());
        self
    }
}
impl ToListMappable for S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
    type O = BlockAssignable<S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {}
impl BuildS3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
        S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl {
            days_after_initiation: core::default::Default::default(),
        }
    }
}
pub struct S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
        S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `days_after_initiation` after provisioning.\n"]
    pub fn days_after_initiation(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.days_after_initiation", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElExpirationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expired_object_delete_marker: Option<PrimField<bool>>,
}
impl S3BucketLifecycleConfigurationRuleElExpirationEl {
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
impl ToListMappable for S3BucketLifecycleConfigurationRuleElExpirationEl {
    type O = BlockAssignable<S3BucketLifecycleConfigurationRuleElExpirationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketLifecycleConfigurationRuleElExpirationEl {}
impl BuildS3BucketLifecycleConfigurationRuleElExpirationEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElExpirationEl {
        S3BucketLifecycleConfigurationRuleElExpirationEl {
            date: core::default::Default::default(),
            days: core::default::Default::default(),
            expired_object_delete_marker: core::default::Default::default(),
        }
    }
}
pub struct S3BucketLifecycleConfigurationRuleElExpirationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketLifecycleConfigurationRuleElExpirationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketLifecycleConfigurationRuleElExpirationElRef {
        S3BucketLifecycleConfigurationRuleElExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketLifecycleConfigurationRuleElExpirationElRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expired_object_delete_marker", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElFilterElAndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    object_size_greater_than: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_size_less_than: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
impl S3BucketLifecycleConfigurationRuleElFilterElAndEl {
    #[doc = "Set the field `object_size_greater_than`.\n"]
    pub fn set_object_size_greater_than(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.object_size_greater_than = Some(v.into());
        self
    }
    #[doc = "Set the field `object_size_less_than`.\n"]
    pub fn set_object_size_less_than(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.object_size_less_than = Some(v.into());
        self
    }
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
impl ToListMappable for S3BucketLifecycleConfigurationRuleElFilterElAndEl {
    type O = BlockAssignable<S3BucketLifecycleConfigurationRuleElFilterElAndEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketLifecycleConfigurationRuleElFilterElAndEl {}
impl BuildS3BucketLifecycleConfigurationRuleElFilterElAndEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElFilterElAndEl {
        S3BucketLifecycleConfigurationRuleElFilterElAndEl {
            object_size_greater_than: core::default::Default::default(),
            object_size_less_than: core::default::Default::default(),
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct S3BucketLifecycleConfigurationRuleElFilterElAndElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketLifecycleConfigurationRuleElFilterElAndElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketLifecycleConfigurationRuleElFilterElAndElRef {
        S3BucketLifecycleConfigurationRuleElFilterElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketLifecycleConfigurationRuleElFilterElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_size_greater_than` after provisioning.\n"]
    pub fn object_size_greater_than(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_size_greater_than", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `object_size_less_than` after provisioning.\n"]
    pub fn object_size_less_than(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_size_less_than", self.base),
        )
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
#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElFilterElTagEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl S3BucketLifecycleConfigurationRuleElFilterElTagEl {}
impl ToListMappable for S3BucketLifecycleConfigurationRuleElFilterElTagEl {
    type O = BlockAssignable<S3BucketLifecycleConfigurationRuleElFilterElTagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketLifecycleConfigurationRuleElFilterElTagEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildS3BucketLifecycleConfigurationRuleElFilterElTagEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElFilterElTagEl {
        S3BucketLifecycleConfigurationRuleElFilterElTagEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct S3BucketLifecycleConfigurationRuleElFilterElTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketLifecycleConfigurationRuleElFilterElTagElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketLifecycleConfigurationRuleElFilterElTagElRef {
        S3BucketLifecycleConfigurationRuleElFilterElTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketLifecycleConfigurationRuleElFilterElTagElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3BucketLifecycleConfigurationRuleElFilterElDynamic {
    and: Option<DynamicBlock<S3BucketLifecycleConfigurationRuleElFilterElAndEl>>,
    tag: Option<DynamicBlock<S3BucketLifecycleConfigurationRuleElFilterElTagEl>>,
}
#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    object_size_greater_than: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_size_less_than: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<S3BucketLifecycleConfigurationRuleElFilterElAndEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<Vec<S3BucketLifecycleConfigurationRuleElFilterElTagEl>>,
    dynamic: S3BucketLifecycleConfigurationRuleElFilterElDynamic,
}
impl S3BucketLifecycleConfigurationRuleElFilterEl {
    #[doc = "Set the field `object_size_greater_than`.\n"]
    pub fn set_object_size_greater_than(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.object_size_greater_than = Some(v.into());
        self
    }
    #[doc = "Set the field `object_size_less_than`.\n"]
    pub fn set_object_size_less_than(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.object_size_less_than = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl Into<BlockAssignable<S3BucketLifecycleConfigurationRuleElFilterElAndEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.and = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.and = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(
        mut self,
        v: impl Into<BlockAssignable<S3BucketLifecycleConfigurationRuleElFilterElTagEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3BucketLifecycleConfigurationRuleElFilterEl {
    type O = BlockAssignable<S3BucketLifecycleConfigurationRuleElFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketLifecycleConfigurationRuleElFilterEl {}
impl BuildS3BucketLifecycleConfigurationRuleElFilterEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElFilterEl {
        S3BucketLifecycleConfigurationRuleElFilterEl {
            object_size_greater_than: core::default::Default::default(),
            object_size_less_than: core::default::Default::default(),
            prefix: core::default::Default::default(),
            and: core::default::Default::default(),
            tag: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3BucketLifecycleConfigurationRuleElFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketLifecycleConfigurationRuleElFilterElRef {
    fn new(shared: StackShared, base: String) -> S3BucketLifecycleConfigurationRuleElFilterElRef {
        S3BucketLifecycleConfigurationRuleElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketLifecycleConfigurationRuleElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_size_greater_than` after provisioning.\n"]
    pub fn object_size_greater_than(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_size_greater_than", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `object_size_less_than` after provisioning.\n"]
    pub fn object_size_less_than(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_size_less_than", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> ListRef<S3BucketLifecycleConfigurationRuleElFilterElAndElRef> {
        ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> ListRef<S3BucketLifecycleConfigurationRuleElFilterElTagElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    newer_noncurrent_versions: Option<PrimField<f64>>,
    noncurrent_days: PrimField<f64>,
}
impl S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
    #[doc = "Set the field `newer_noncurrent_versions`.\n"]
    pub fn set_newer_noncurrent_versions(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.newer_noncurrent_versions = Some(v.into());
        self
    }
}
impl ToListMappable for S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
    type O = BlockAssignable<S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
    #[doc = ""]
    pub noncurrent_days: PrimField<f64>,
}
impl BuildS3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
        S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl {
            newer_noncurrent_versions: core::default::Default::default(),
            noncurrent_days: self.noncurrent_days,
        }
    }
}
pub struct S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationElRef {
        S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `newer_noncurrent_versions` after provisioning.\n"]
    pub fn newer_noncurrent_versions(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.newer_noncurrent_versions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `noncurrent_days` after provisioning.\n"]
    pub fn noncurrent_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.noncurrent_days", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    newer_noncurrent_versions: Option<PrimField<f64>>,
    noncurrent_days: PrimField<f64>,
    storage_class: PrimField<String>,
}
impl S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
    #[doc = "Set the field `newer_noncurrent_versions`.\n"]
    pub fn set_newer_noncurrent_versions(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.newer_noncurrent_versions = Some(v.into());
        self
    }
}
impl ToListMappable for S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
    type O = BlockAssignable<S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
    #[doc = ""]
    pub noncurrent_days: PrimField<f64>,
    #[doc = ""]
    pub storage_class: PrimField<String>,
}
impl BuildS3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
        S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl {
            newer_noncurrent_versions: core::default::Default::default(),
            noncurrent_days: self.noncurrent_days,
            storage_class: self.storage_class,
        }
    }
}
pub struct S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionElRef {
        S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `newer_noncurrent_versions` after provisioning.\n"]
    pub fn newer_noncurrent_versions(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.newer_noncurrent_versions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `noncurrent_days` after provisioning.\n"]
    pub fn noncurrent_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.noncurrent_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleElTransitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
    storage_class: PrimField<String>,
}
impl S3BucketLifecycleConfigurationRuleElTransitionEl {
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
}
impl ToListMappable for S3BucketLifecycleConfigurationRuleElTransitionEl {
    type O = BlockAssignable<S3BucketLifecycleConfigurationRuleElTransitionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketLifecycleConfigurationRuleElTransitionEl {
    #[doc = ""]
    pub storage_class: PrimField<String>,
}
impl BuildS3BucketLifecycleConfigurationRuleElTransitionEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleElTransitionEl {
        S3BucketLifecycleConfigurationRuleElTransitionEl {
            date: core::default::Default::default(),
            days: core::default::Default::default(),
            storage_class: self.storage_class,
        }
    }
}
pub struct S3BucketLifecycleConfigurationRuleElTransitionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketLifecycleConfigurationRuleElTransitionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketLifecycleConfigurationRuleElTransitionElRef {
        S3BucketLifecycleConfigurationRuleElTransitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketLifecycleConfigurationRuleElTransitionElRef {
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
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct S3BucketLifecycleConfigurationRuleElDynamic {
    abort_incomplete_multipart_upload:
        Option<DynamicBlock<S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl>>,
    expiration: Option<DynamicBlock<S3BucketLifecycleConfigurationRuleElExpirationEl>>,
    filter: Option<DynamicBlock<S3BucketLifecycleConfigurationRuleElFilterEl>>,
    noncurrent_version_expiration:
        Option<DynamicBlock<S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl>>,
    noncurrent_version_transition:
        Option<DynamicBlock<S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl>>,
    transition: Option<DynamicBlock<S3BucketLifecycleConfigurationRuleElTransitionEl>>,
}
#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationRuleEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    status: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    abort_incomplete_multipart_upload:
        Option<Vec<S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration: Option<Vec<S3BucketLifecycleConfigurationRuleElExpirationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<S3BucketLifecycleConfigurationRuleElFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    noncurrent_version_expiration:
        Option<Vec<S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    noncurrent_version_transition:
        Option<Vec<S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transition: Option<Vec<S3BucketLifecycleConfigurationRuleElTransitionEl>>,
    dynamic: S3BucketLifecycleConfigurationRuleElDynamic,
}
impl S3BucketLifecycleConfigurationRuleEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `abort_incomplete_multipart_upload`.\n"]
    pub fn set_abort_incomplete_multipart_upload(
        mut self,
        v: impl Into<
            BlockAssignable<S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.abort_incomplete_multipart_upload = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.abort_incomplete_multipart_upload = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `expiration`.\n"]
    pub fn set_expiration(
        mut self,
        v: impl Into<BlockAssignable<S3BucketLifecycleConfigurationRuleElExpirationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expiration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expiration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<BlockAssignable<S3BucketLifecycleConfigurationRuleElFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `noncurrent_version_expiration`.\n"]
    pub fn set_noncurrent_version_expiration(
        mut self,
        v: impl Into<BlockAssignable<S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.noncurrent_version_expiration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.noncurrent_version_expiration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `noncurrent_version_transition`.\n"]
    pub fn set_noncurrent_version_transition(
        mut self,
        v: impl Into<BlockAssignable<S3BucketLifecycleConfigurationRuleElNoncurrentVersionTransitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.noncurrent_version_transition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.noncurrent_version_transition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `transition`.\n"]
    pub fn set_transition(
        mut self,
        v: impl Into<BlockAssignable<S3BucketLifecycleConfigurationRuleElTransitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3BucketLifecycleConfigurationRuleEl {
    type O = BlockAssignable<S3BucketLifecycleConfigurationRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketLifecycleConfigurationRuleEl {
    #[doc = ""]
    pub id: PrimField<String>,
    #[doc = ""]
    pub status: PrimField<String>,
}
impl BuildS3BucketLifecycleConfigurationRuleEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationRuleEl {
        S3BucketLifecycleConfigurationRuleEl {
            id: self.id,
            prefix: core::default::Default::default(),
            status: self.status,
            abort_incomplete_multipart_upload: core::default::Default::default(),
            expiration: core::default::Default::default(),
            filter: core::default::Default::default(),
            noncurrent_version_expiration: core::default::Default::default(),
            noncurrent_version_transition: core::default::Default::default(),
            transition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3BucketLifecycleConfigurationRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketLifecycleConfigurationRuleElRef {
    fn new(shared: StackShared, base: String) -> S3BucketLifecycleConfigurationRuleElRef {
        S3BucketLifecycleConfigurationRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketLifecycleConfigurationRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `abort_incomplete_multipart_upload` after provisioning.\n"]
    pub fn abort_incomplete_multipart_upload(
        &self,
    ) -> ListRef<S3BucketLifecycleConfigurationRuleElAbortIncompleteMultipartUploadElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.abort_incomplete_multipart_upload", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> ListRef<S3BucketLifecycleConfigurationRuleElExpirationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expiration", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<S3BucketLifecycleConfigurationRuleElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `noncurrent_version_expiration` after provisioning.\n"]
    pub fn noncurrent_version_expiration(
        &self,
    ) -> ListRef<S3BucketLifecycleConfigurationRuleElNoncurrentVersionExpirationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.noncurrent_version_expiration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3BucketLifecycleConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl S3BucketLifecycleConfigurationTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for S3BucketLifecycleConfigurationTimeoutsEl {
    type O = BlockAssignable<S3BucketLifecycleConfigurationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3BucketLifecycleConfigurationTimeoutsEl {}
impl BuildS3BucketLifecycleConfigurationTimeoutsEl {
    pub fn build(self) -> S3BucketLifecycleConfigurationTimeoutsEl {
        S3BucketLifecycleConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct S3BucketLifecycleConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3BucketLifecycleConfigurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> S3BucketLifecycleConfigurationTimeoutsElRef {
        S3BucketLifecycleConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3BucketLifecycleConfigurationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3BucketLifecycleConfigurationDynamic {
    rule: Option<DynamicBlock<S3BucketLifecycleConfigurationRuleEl>>,
}
