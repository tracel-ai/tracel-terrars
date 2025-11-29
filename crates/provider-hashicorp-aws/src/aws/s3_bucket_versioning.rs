use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketVersioningData {
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
    mfa: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versioning_configuration: Option<Vec<S3BucketVersioningVersioningConfigurationEl>>,
    dynamic: S3BucketVersioningDynamic,
}

struct S3BucketVersioning_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketVersioningData>,
}

#[derive(Clone)]
pub struct S3BucketVersioning(Rc<S3BucketVersioning_>);

impl S3BucketVersioning {
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

    #[doc = "Set the field `mfa`.\n"]
    pub fn set_mfa(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mfa = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `versioning_configuration`.\n"]
    pub fn set_versioning_configuration(
        self,
        v: impl Into<BlockAssignable<S3BucketVersioningVersioningConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().versioning_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.versioning_configuration = Some(d);
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

    #[doc = "Get a reference to the value of field `mfa` after provisioning.\n"]
    pub fn mfa(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mfa", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `versioning_configuration` after provisioning.\n"]
    pub fn versioning_configuration(&self) -> ListRef<S3BucketVersioningVersioningConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.versioning_configuration", self.extract_ref()))
    }
}

impl Referable for S3BucketVersioning {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for S3BucketVersioning { }

impl ToListMappable for S3BucketVersioning {
    type O = ListRef<S3BucketVersioningRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3BucketVersioning_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_versioning".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketVersioning {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
}

impl BuildS3BucketVersioning {
    pub fn build(self, stack: &mut Stack) -> S3BucketVersioning {
        let out = S3BucketVersioning(Rc::new(S3BucketVersioning_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketVersioningData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                mfa: core::default::Default::default(),
                region: core::default::Default::default(),
                versioning_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketVersioningRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketVersioningRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl S3BucketVersioningRef {
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

    #[doc = "Get a reference to the value of field `mfa` after provisioning.\n"]
    pub fn mfa(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mfa", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `versioning_configuration` after provisioning.\n"]
    pub fn versioning_configuration(&self) -> ListRef<S3BucketVersioningVersioningConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.versioning_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3BucketVersioningVersioningConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mfa_delete: Option<PrimField<String>>,
    status: PrimField<String>,
}

impl S3BucketVersioningVersioningConfigurationEl {
    #[doc = "Set the field `mfa_delete`.\n"]
    pub fn set_mfa_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mfa_delete = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketVersioningVersioningConfigurationEl {
    type O = BlockAssignable<S3BucketVersioningVersioningConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketVersioningVersioningConfigurationEl {
    #[doc = ""]
    pub status: PrimField<String>,
}

impl BuildS3BucketVersioningVersioningConfigurationEl {
    pub fn build(self) -> S3BucketVersioningVersioningConfigurationEl {
        S3BucketVersioningVersioningConfigurationEl {
            mfa_delete: core::default::Default::default(),
            status: self.status,
        }
    }
}

pub struct S3BucketVersioningVersioningConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketVersioningVersioningConfigurationElRef {
    fn new(shared: StackShared, base: String) -> S3BucketVersioningVersioningConfigurationElRef {
        S3BucketVersioningVersioningConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketVersioningVersioningConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mfa_delete` after provisioning.\n"]
    pub fn mfa_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mfa_delete", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketVersioningDynamic {
    versioning_configuration: Option<DynamicBlock<S3BucketVersioningVersioningConfigurationEl>>,
}
