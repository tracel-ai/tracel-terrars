use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketObjectLockConfigurationData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_lock_enabled: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<S3BucketObjectLockConfigurationRuleEl>>,
    dynamic: S3BucketObjectLockConfigurationDynamic,
}

struct S3BucketObjectLockConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketObjectLockConfigurationData>,
}

#[derive(Clone)]
pub struct S3BucketObjectLockConfiguration(Rc<S3BucketObjectLockConfiguration_>);

impl S3BucketObjectLockConfiguration {
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

    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expected_bucket_owner = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `object_lock_enabled`.\n"]
    pub fn set_object_lock_enabled(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().object_lock_enabled = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `token`.\n"]
    pub fn set_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token = Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl Into<BlockAssignable<S3BucketObjectLockConfigurationRuleEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `object_lock_enabled` after provisioning.\n"]
    pub fn object_lock_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_lock_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<S3BucketObjectLockConfigurationRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

impl Referable for S3BucketObjectLockConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for S3BucketObjectLockConfiguration { }

impl ToListMappable for S3BucketObjectLockConfiguration {
    type O = ListRef<S3BucketObjectLockConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3BucketObjectLockConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_object_lock_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketObjectLockConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
}

impl BuildS3BucketObjectLockConfiguration {
    pub fn build(self, stack: &mut Stack) -> S3BucketObjectLockConfiguration {
        let out = S3BucketObjectLockConfiguration(Rc::new(S3BucketObjectLockConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketObjectLockConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                object_lock_enabled: core::default::Default::default(),
                region: core::default::Default::default(),
                token: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketObjectLockConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketObjectLockConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl S3BucketObjectLockConfigurationRef {
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

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `object_lock_enabled` after provisioning.\n"]
    pub fn object_lock_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_lock_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<S3BucketObjectLockConfigurationRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3BucketObjectLockConfigurationRuleElDefaultRetentionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    years: Option<PrimField<f64>>,
}

impl S3BucketObjectLockConfigurationRuleElDefaultRetentionEl {
    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc = "Set the field `years`.\n"]
    pub fn set_years(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.years = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketObjectLockConfigurationRuleElDefaultRetentionEl {
    type O = BlockAssignable<S3BucketObjectLockConfigurationRuleElDefaultRetentionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketObjectLockConfigurationRuleElDefaultRetentionEl {}

impl BuildS3BucketObjectLockConfigurationRuleElDefaultRetentionEl {
    pub fn build(self) -> S3BucketObjectLockConfigurationRuleElDefaultRetentionEl {
        S3BucketObjectLockConfigurationRuleElDefaultRetentionEl {
            days: core::default::Default::default(),
            mode: core::default::Default::default(),
            years: core::default::Default::default(),
        }
    }
}

pub struct S3BucketObjectLockConfigurationRuleElDefaultRetentionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketObjectLockConfigurationRuleElDefaultRetentionElRef {
    fn new(shared: StackShared, base: String) -> S3BucketObjectLockConfigurationRuleElDefaultRetentionElRef {
        S3BucketObjectLockConfigurationRuleElDefaultRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketObjectLockConfigurationRuleElDefaultRetentionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `years` after provisioning.\n"]
    pub fn years(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.years", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketObjectLockConfigurationRuleElDynamic {
    default_retention: Option<DynamicBlock<S3BucketObjectLockConfigurationRuleElDefaultRetentionEl>>,
}

#[derive(Serialize)]
pub struct S3BucketObjectLockConfigurationRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_retention: Option<Vec<S3BucketObjectLockConfigurationRuleElDefaultRetentionEl>>,
    dynamic: S3BucketObjectLockConfigurationRuleElDynamic,
}

impl S3BucketObjectLockConfigurationRuleEl {
    #[doc = "Set the field `default_retention`.\n"]
    pub fn set_default_retention(
        mut self,
        v: impl Into<BlockAssignable<S3BucketObjectLockConfigurationRuleElDefaultRetentionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_retention = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_retention = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketObjectLockConfigurationRuleEl {
    type O = BlockAssignable<S3BucketObjectLockConfigurationRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketObjectLockConfigurationRuleEl {}

impl BuildS3BucketObjectLockConfigurationRuleEl {
    pub fn build(self) -> S3BucketObjectLockConfigurationRuleEl {
        S3BucketObjectLockConfigurationRuleEl {
            default_retention: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketObjectLockConfigurationRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketObjectLockConfigurationRuleElRef {
    fn new(shared: StackShared, base: String) -> S3BucketObjectLockConfigurationRuleElRef {
        S3BucketObjectLockConfigurationRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketObjectLockConfigurationRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_retention` after provisioning.\n"]
    pub fn default_retention(&self) -> ListRef<S3BucketObjectLockConfigurationRuleElDefaultRetentionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_retention", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketObjectLockConfigurationDynamic {
    rule: Option<DynamicBlock<S3BucketObjectLockConfigurationRuleEl>>,
}
