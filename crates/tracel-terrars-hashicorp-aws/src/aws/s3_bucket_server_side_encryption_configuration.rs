use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketServerSideEncryptionConfigurationData {
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
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<S3BucketServerSideEncryptionConfigurationRuleEl>>,
    dynamic: S3BucketServerSideEncryptionConfigurationDynamic,
}

struct S3BucketServerSideEncryptionConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketServerSideEncryptionConfigurationData>,
}

#[derive(Clone)]
pub struct S3BucketServerSideEncryptionConfiguration(Rc<S3BucketServerSideEncryptionConfiguration_>);

impl S3BucketServerSideEncryptionConfiguration {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl Into<BlockAssignable<S3BucketServerSideEncryptionConfigurationRuleEl>>) -> Self {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for S3BucketServerSideEncryptionConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for S3BucketServerSideEncryptionConfiguration { }

impl ToListMappable for S3BucketServerSideEncryptionConfiguration {
    type O = ListRef<S3BucketServerSideEncryptionConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3BucketServerSideEncryptionConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_server_side_encryption_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketServerSideEncryptionConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
}

impl BuildS3BucketServerSideEncryptionConfiguration {
    pub fn build(self, stack: &mut Stack) -> S3BucketServerSideEncryptionConfiguration {
        let out = S3BucketServerSideEncryptionConfiguration(Rc::new(S3BucketServerSideEncryptionConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketServerSideEncryptionConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
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

pub struct S3BucketServerSideEncryptionConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketServerSideEncryptionConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl S3BucketServerSideEncryptionConfigurationRef {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_master_key_id: Option<PrimField<String>>,
    sse_algorithm: PrimField<String>,
}

impl S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
    #[doc = "Set the field `kms_master_key_id`.\n"]
    pub fn set_kms_master_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_master_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
    type O = BlockAssignable<S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
    #[doc = ""]
    pub sse_algorithm: PrimField<String>,
}

impl BuildS3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
    pub fn build(self) -> S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
        S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl {
            kms_master_key_id: core::default::Default::default(),
            sse_algorithm: self.sse_algorithm,
        }
    }
}

pub struct S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultElRef {
        S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_master_key_id` after provisioning.\n"]
    pub fn kms_master_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_master_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_algorithm` after provisioning.\n"]
    pub fn sse_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sse_algorithm", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketServerSideEncryptionConfigurationRuleElDynamic {
    apply_server_side_encryption_by_default: Option<
        DynamicBlock<S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl>,
    >,
}

#[derive(Serialize)]
pub struct S3BucketServerSideEncryptionConfigurationRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    blocked_encryption_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_key_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apply_server_side_encryption_by_default: Option<
        Vec<S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl>,
    >,
    dynamic: S3BucketServerSideEncryptionConfigurationRuleElDynamic,
}

impl S3BucketServerSideEncryptionConfigurationRuleEl {
    #[doc = "Set the field `blocked_encryption_types`.\n"]
    pub fn set_blocked_encryption_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.blocked_encryption_types = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_key_enabled`.\n"]
    pub fn set_bucket_key_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bucket_key_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `apply_server_side_encryption_by_default`.\n"]
    pub fn set_apply_server_side_encryption_by_default(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.apply_server_side_encryption_by_default = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.apply_server_side_encryption_by_default = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketServerSideEncryptionConfigurationRuleEl {
    type O = BlockAssignable<S3BucketServerSideEncryptionConfigurationRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketServerSideEncryptionConfigurationRuleEl {}

impl BuildS3BucketServerSideEncryptionConfigurationRuleEl {
    pub fn build(self) -> S3BucketServerSideEncryptionConfigurationRuleEl {
        S3BucketServerSideEncryptionConfigurationRuleEl {
            blocked_encryption_types: core::default::Default::default(),
            bucket_key_enabled: core::default::Default::default(),
            apply_server_side_encryption_by_default: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketServerSideEncryptionConfigurationRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketServerSideEncryptionConfigurationRuleElRef {
    fn new(shared: StackShared, base: String) -> S3BucketServerSideEncryptionConfigurationRuleElRef {
        S3BucketServerSideEncryptionConfigurationRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketServerSideEncryptionConfigurationRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `blocked_encryption_types` after provisioning.\n"]
    pub fn blocked_encryption_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.blocked_encryption_types", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_key_enabled` after provisioning.\n"]
    pub fn bucket_key_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_key_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `apply_server_side_encryption_by_default` after provisioning.\n"]
    pub fn apply_server_side_encryption_by_default(
        &self,
    ) -> ListRef<S3BucketServerSideEncryptionConfigurationRuleElApplyServerSideEncryptionByDefaultElRef> {
        ListRef::new(self.shared().clone(), format!("{}.apply_server_side_encryption_by_default", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketServerSideEncryptionConfigurationDynamic {
    rule: Option<DynamicBlock<S3BucketServerSideEncryptionConfigurationRuleEl>>,
}
