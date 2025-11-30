use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct S3BucketInventoryData {
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
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    included_object_versions: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_fields: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<Vec<S3BucketInventoryDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<S3BucketInventoryFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<Vec<S3BucketInventoryScheduleEl>>,
    dynamic: S3BucketInventoryDynamic,
}

struct S3BucketInventory_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketInventoryData>,
}

#[derive(Clone)]
pub struct S3BucketInventory(Rc<S3BucketInventory_>);

impl S3BucketInventory {
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

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `optional_fields`.\n"]
    pub fn set_optional_fields(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().optional_fields = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        self,
        v: impl Into<BlockAssignable<S3BucketInventoryDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<S3BucketInventoryFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl Into<BlockAssignable<S3BucketInventoryScheduleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `included_object_versions` after provisioning.\n"]
    pub fn included_object_versions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.included_object_versions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `optional_fields` after provisioning.\n"]
    pub fn optional_fields(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.optional_fields", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<S3BucketInventoryDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<S3BucketInventoryFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<S3BucketInventoryScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
}

impl Referable for S3BucketInventory {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for S3BucketInventory {}

impl ToListMappable for S3BucketInventory {
    type O = ListRef<S3BucketInventoryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3BucketInventory_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_inventory".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketInventory {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub included_object_versions: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildS3BucketInventory {
    pub fn build(self, stack: &mut Stack) -> S3BucketInventory {
        let out = S3BucketInventory(Rc::new(S3BucketInventory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketInventoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                included_object_versions: self.included_object_versions,
                name: self.name,
                optional_fields: core::default::Default::default(),
                region: core::default::Default::default(),
                destination: core::default::Default::default(),
                filter: core::default::Default::default(),
                schedule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketInventoryRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketInventoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl S3BucketInventoryRef {
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

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `included_object_versions` after provisioning.\n"]
    pub fn included_object_versions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.included_object_versions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `optional_fields` after provisioning.\n"]
    pub fn optional_fields(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.optional_fields", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<S3BucketInventoryDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<S3BucketInventoryFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<S3BucketInventoryScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct S3BucketInventoryDestinationElBucketElEncryptionElSseKmsEl {
    key_id: PrimField<String>,
}

impl S3BucketInventoryDestinationElBucketElEncryptionElSseKmsEl {}

impl ToListMappable for S3BucketInventoryDestinationElBucketElEncryptionElSseKmsEl {
    type O = BlockAssignable<S3BucketInventoryDestinationElBucketElEncryptionElSseKmsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventoryDestinationElBucketElEncryptionElSseKmsEl {
    #[doc = ""]
    pub key_id: PrimField<String>,
}

impl BuildS3BucketInventoryDestinationElBucketElEncryptionElSseKmsEl {
    pub fn build(self) -> S3BucketInventoryDestinationElBucketElEncryptionElSseKmsEl {
        S3BucketInventoryDestinationElBucketElEncryptionElSseKmsEl {
            key_id: self.key_id,
        }
    }
}

pub struct S3BucketInventoryDestinationElBucketElEncryptionElSseKmsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketInventoryDestinationElBucketElEncryptionElSseKmsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketInventoryDestinationElBucketElEncryptionElSseKmsElRef {
        S3BucketInventoryDestinationElBucketElEncryptionElSseKmsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventoryDestinationElBucketElEncryptionElSseKmsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketInventoryDestinationElBucketElEncryptionElSseS3El {}

impl S3BucketInventoryDestinationElBucketElEncryptionElSseS3El {}

impl ToListMappable for S3BucketInventoryDestinationElBucketElEncryptionElSseS3El {
    type O = BlockAssignable<S3BucketInventoryDestinationElBucketElEncryptionElSseS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventoryDestinationElBucketElEncryptionElSseS3El {}

impl BuildS3BucketInventoryDestinationElBucketElEncryptionElSseS3El {
    pub fn build(self) -> S3BucketInventoryDestinationElBucketElEncryptionElSseS3El {
        S3BucketInventoryDestinationElBucketElEncryptionElSseS3El {}
    }
}

pub struct S3BucketInventoryDestinationElBucketElEncryptionElSseS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketInventoryDestinationElBucketElEncryptionElSseS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketInventoryDestinationElBucketElEncryptionElSseS3ElRef {
        S3BucketInventoryDestinationElBucketElEncryptionElSseS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventoryDestinationElBucketElEncryptionElSseS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct S3BucketInventoryDestinationElBucketElEncryptionElDynamic {
    sse_kms: Option<DynamicBlock<S3BucketInventoryDestinationElBucketElEncryptionElSseKmsEl>>,
    sse_s3: Option<DynamicBlock<S3BucketInventoryDestinationElBucketElEncryptionElSseS3El>>,
}

#[derive(Serialize)]
pub struct S3BucketInventoryDestinationElBucketElEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sse_kms: Option<Vec<S3BucketInventoryDestinationElBucketElEncryptionElSseKmsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sse_s3: Option<Vec<S3BucketInventoryDestinationElBucketElEncryptionElSseS3El>>,
    dynamic: S3BucketInventoryDestinationElBucketElEncryptionElDynamic,
}

impl S3BucketInventoryDestinationElBucketElEncryptionEl {
    #[doc = "Set the field `sse_kms`.\n"]
    pub fn set_sse_kms(
        mut self,
        v: impl Into<BlockAssignable<S3BucketInventoryDestinationElBucketElEncryptionElSseKmsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sse_kms = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sse_kms = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `sse_s3`.\n"]
    pub fn set_sse_s3(
        mut self,
        v: impl Into<BlockAssignable<S3BucketInventoryDestinationElBucketElEncryptionElSseS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sse_s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sse_s3 = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for S3BucketInventoryDestinationElBucketElEncryptionEl {
    type O = BlockAssignable<S3BucketInventoryDestinationElBucketElEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventoryDestinationElBucketElEncryptionEl {}

impl BuildS3BucketInventoryDestinationElBucketElEncryptionEl {
    pub fn build(self) -> S3BucketInventoryDestinationElBucketElEncryptionEl {
        S3BucketInventoryDestinationElBucketElEncryptionEl {
            sse_kms: core::default::Default::default(),
            sse_s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketInventoryDestinationElBucketElEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketInventoryDestinationElBucketElEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketInventoryDestinationElBucketElEncryptionElRef {
        S3BucketInventoryDestinationElBucketElEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventoryDestinationElBucketElEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sse_kms` after provisioning.\n"]
    pub fn sse_kms(
        &self,
    ) -> ListRef<S3BucketInventoryDestinationElBucketElEncryptionElSseKmsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sse_kms", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_s3` after provisioning.\n"]
    pub fn sse_s3(&self) -> ListRef<S3BucketInventoryDestinationElBucketElEncryptionElSseS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sse_s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketInventoryDestinationElBucketElDynamic {
    encryption: Option<DynamicBlock<S3BucketInventoryDestinationElBucketElEncryptionEl>>,
}

#[derive(Serialize)]
pub struct S3BucketInventoryDestinationElBucketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    bucket_arn: PrimField<String>,
    format: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption: Option<Vec<S3BucketInventoryDestinationElBucketElEncryptionEl>>,
    dynamic: S3BucketInventoryDestinationElBucketElDynamic,
}

impl S3BucketInventoryDestinationElBucketEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption`.\n"]
    pub fn set_encryption(
        mut self,
        v: impl Into<BlockAssignable<S3BucketInventoryDestinationElBucketElEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for S3BucketInventoryDestinationElBucketEl {
    type O = BlockAssignable<S3BucketInventoryDestinationElBucketEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventoryDestinationElBucketEl {
    #[doc = ""]
    pub bucket_arn: PrimField<String>,
    #[doc = ""]
    pub format: PrimField<String>,
}

impl BuildS3BucketInventoryDestinationElBucketEl {
    pub fn build(self) -> S3BucketInventoryDestinationElBucketEl {
        S3BucketInventoryDestinationElBucketEl {
            account_id: core::default::Default::default(),
            bucket_arn: self.bucket_arn,
            format: self.format,
            prefix: core::default::Default::default(),
            encryption: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketInventoryDestinationElBucketElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketInventoryDestinationElBucketElRef {
    fn new(shared: StackShared, base: String) -> S3BucketInventoryDestinationElBucketElRef {
        S3BucketInventoryDestinationElBucketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventoryDestinationElBucketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(&self) -> ListRef<S3BucketInventoryDestinationElBucketElEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketInventoryDestinationElDynamic {
    bucket: Option<DynamicBlock<S3BucketInventoryDestinationElBucketEl>>,
}

#[derive(Serialize)]
pub struct S3BucketInventoryDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<Vec<S3BucketInventoryDestinationElBucketEl>>,
    dynamic: S3BucketInventoryDestinationElDynamic,
}

impl S3BucketInventoryDestinationEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(
        mut self,
        v: impl Into<BlockAssignable<S3BucketInventoryDestinationElBucketEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bucket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bucket = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for S3BucketInventoryDestinationEl {
    type O = BlockAssignable<S3BucketInventoryDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventoryDestinationEl {}

impl BuildS3BucketInventoryDestinationEl {
    pub fn build(self) -> S3BucketInventoryDestinationEl {
        S3BucketInventoryDestinationEl {
            bucket: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketInventoryDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketInventoryDestinationElRef {
    fn new(shared: StackShared, base: String) -> S3BucketInventoryDestinationElRef {
        S3BucketInventoryDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventoryDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> ListRef<S3BucketInventoryDestinationElBucketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketInventoryFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}

impl S3BucketInventoryFilterEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketInventoryFilterEl {
    type O = BlockAssignable<S3BucketInventoryFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventoryFilterEl {}

impl BuildS3BucketInventoryFilterEl {
    pub fn build(self) -> S3BucketInventoryFilterEl {
        S3BucketInventoryFilterEl {
            prefix: core::default::Default::default(),
        }
    }
}

pub struct S3BucketInventoryFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketInventoryFilterElRef {
    fn new(shared: StackShared, base: String) -> S3BucketInventoryFilterElRef {
        S3BucketInventoryFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventoryFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketInventoryScheduleEl {
    frequency: PrimField<String>,
}

impl S3BucketInventoryScheduleEl {}

impl ToListMappable for S3BucketInventoryScheduleEl {
    type O = BlockAssignable<S3BucketInventoryScheduleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventoryScheduleEl {
    #[doc = ""]
    pub frequency: PrimField<String>,
}

impl BuildS3BucketInventoryScheduleEl {
    pub fn build(self) -> S3BucketInventoryScheduleEl {
        S3BucketInventoryScheduleEl {
            frequency: self.frequency,
        }
    }
}

pub struct S3BucketInventoryScheduleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketInventoryScheduleElRef {
    fn new(shared: StackShared, base: String) -> S3BucketInventoryScheduleElRef {
        S3BucketInventoryScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventoryScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `frequency` after provisioning.\n"]
    pub fn frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketInventoryDynamic {
    destination: Option<DynamicBlock<S3BucketInventoryDestinationEl>>,
    filter: Option<DynamicBlock<S3BucketInventoryFilterEl>>,
    schedule: Option<DynamicBlock<S3BucketInventoryScheduleEl>>,
}
