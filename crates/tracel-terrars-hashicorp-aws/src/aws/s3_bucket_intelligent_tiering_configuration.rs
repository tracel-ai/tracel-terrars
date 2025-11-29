use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketIntelligentTieringConfigurationData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<S3BucketIntelligentTieringConfigurationFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiering: Option<Vec<S3BucketIntelligentTieringConfigurationTieringEl>>,
    dynamic: S3BucketIntelligentTieringConfigurationDynamic,
}

struct S3BucketIntelligentTieringConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketIntelligentTieringConfigurationData>,
}

#[derive(Clone)]
pub struct S3BucketIntelligentTieringConfiguration(Rc<S3BucketIntelligentTieringConfiguration_>);

impl S3BucketIntelligentTieringConfiguration {
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

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<S3BucketIntelligentTieringConfigurationFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tiering`.\n"]
    pub fn set_tiering(self, v: impl Into<BlockAssignable<S3BucketIntelligentTieringConfigurationTieringEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tiering = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tiering = Some(d);
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<S3BucketIntelligentTieringConfigurationFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

impl Referable for S3BucketIntelligentTieringConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for S3BucketIntelligentTieringConfiguration { }

impl ToListMappable for S3BucketIntelligentTieringConfiguration {
    type O = ListRef<S3BucketIntelligentTieringConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3BucketIntelligentTieringConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_intelligent_tiering_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketIntelligentTieringConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildS3BucketIntelligentTieringConfiguration {
    pub fn build(self, stack: &mut Stack) -> S3BucketIntelligentTieringConfiguration {
        let out = S3BucketIntelligentTieringConfiguration(Rc::new(S3BucketIntelligentTieringConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketIntelligentTieringConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                status: core::default::Default::default(),
                filter: core::default::Default::default(),
                tiering: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketIntelligentTieringConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketIntelligentTieringConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl S3BucketIntelligentTieringConfigurationRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<S3BucketIntelligentTieringConfigurationFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3BucketIntelligentTieringConfigurationFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

impl S3BucketIntelligentTieringConfigurationFilterEl {
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

impl ToListMappable for S3BucketIntelligentTieringConfigurationFilterEl {
    type O = BlockAssignable<S3BucketIntelligentTieringConfigurationFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketIntelligentTieringConfigurationFilterEl {}

impl BuildS3BucketIntelligentTieringConfigurationFilterEl {
    pub fn build(self) -> S3BucketIntelligentTieringConfigurationFilterEl {
        S3BucketIntelligentTieringConfigurationFilterEl {
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct S3BucketIntelligentTieringConfigurationFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketIntelligentTieringConfigurationFilterElRef {
    fn new(shared: StackShared, base: String) -> S3BucketIntelligentTieringConfigurationFilterElRef {
        S3BucketIntelligentTieringConfigurationFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketIntelligentTieringConfigurationFilterElRef {
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

#[derive(Serialize)]
pub struct S3BucketIntelligentTieringConfigurationTieringEl {
    access_tier: PrimField<String>,
    days: PrimField<f64>,
}

impl S3BucketIntelligentTieringConfigurationTieringEl { }

impl ToListMappable for S3BucketIntelligentTieringConfigurationTieringEl {
    type O = BlockAssignable<S3BucketIntelligentTieringConfigurationTieringEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketIntelligentTieringConfigurationTieringEl {
    #[doc = ""]
    pub access_tier: PrimField<String>,
    #[doc = ""]
    pub days: PrimField<f64>,
}

impl BuildS3BucketIntelligentTieringConfigurationTieringEl {
    pub fn build(self) -> S3BucketIntelligentTieringConfigurationTieringEl {
        S3BucketIntelligentTieringConfigurationTieringEl {
            access_tier: self.access_tier,
            days: self.days,
        }
    }
}

pub struct S3BucketIntelligentTieringConfigurationTieringElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketIntelligentTieringConfigurationTieringElRef {
    fn new(shared: StackShared, base: String) -> S3BucketIntelligentTieringConfigurationTieringElRef {
        S3BucketIntelligentTieringConfigurationTieringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketIntelligentTieringConfigurationTieringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_tier` after provisioning.\n"]
    pub fn access_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_tier", self.base))
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketIntelligentTieringConfigurationDynamic {
    filter: Option<DynamicBlock<S3BucketIntelligentTieringConfigurationFilterEl>>,
    tiering: Option<DynamicBlock<S3BucketIntelligentTieringConfigurationTieringEl>>,
}
