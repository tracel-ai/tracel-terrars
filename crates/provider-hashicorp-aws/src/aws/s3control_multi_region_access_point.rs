use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct S3controlMultiRegionAccessPointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Vec<S3controlMultiRegionAccessPointDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<S3controlMultiRegionAccessPointTimeoutsEl>,
    dynamic: S3controlMultiRegionAccessPointDynamic,
}
struct S3controlMultiRegionAccessPoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3controlMultiRegionAccessPointData>,
}
#[derive(Clone)]
pub struct S3controlMultiRegionAccessPoint(Rc<S3controlMultiRegionAccessPoint_>);
impl S3controlMultiRegionAccessPoint {
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
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        self,
        v: impl Into<BlockAssignable<S3controlMultiRegionAccessPointDetailsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().details = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.details = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<S3controlMultiRegionAccessPointTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<S3controlMultiRegionAccessPointDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3controlMultiRegionAccessPointTimeoutsElRef {
        S3controlMultiRegionAccessPointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for S3controlMultiRegionAccessPoint {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for S3controlMultiRegionAccessPoint {}
impl ToListMappable for S3controlMultiRegionAccessPoint {
    type O = ListRef<S3controlMultiRegionAccessPointRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for S3controlMultiRegionAccessPoint_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3control_multi_region_access_point".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildS3controlMultiRegionAccessPoint {
    pub tf_id: String,
}
impl BuildS3controlMultiRegionAccessPoint {
    pub fn build(self, stack: &mut Stack) -> S3controlMultiRegionAccessPoint {
        let out = S3controlMultiRegionAccessPoint(Rc::new(S3controlMultiRegionAccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3controlMultiRegionAccessPointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                details: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct S3controlMultiRegionAccessPointRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3controlMultiRegionAccessPointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl S3controlMultiRegionAccessPointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<S3controlMultiRegionAccessPointDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3controlMultiRegionAccessPointTimeoutsElRef {
        S3controlMultiRegionAccessPointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    block_public_acls: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_public_policy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_public_acls: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrict_public_buckets: Option<PrimField<bool>>,
}
impl S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {
    #[doc = "Set the field `block_public_acls`.\n"]
    pub fn set_block_public_acls(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.block_public_acls = Some(v.into());
        self
    }
    #[doc = "Set the field `block_public_policy`.\n"]
    pub fn set_block_public_policy(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.block_public_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_public_acls`.\n"]
    pub fn set_ignore_public_acls(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_public_acls = Some(v.into());
        self
    }
    #[doc = "Set the field `restrict_public_buckets`.\n"]
    pub fn set_restrict_public_buckets(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.restrict_public_buckets = Some(v.into());
        self
    }
}
impl ToListMappable for S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {
    type O = BlockAssignable<S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {}
impl BuildS3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {
    pub fn build(self) -> S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {
        S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl {
            block_public_acls: core::default::Default::default(),
            block_public_policy: core::default::Default::default(),
            ignore_public_acls: core::default::Default::default(),
            restrict_public_buckets: core::default::Default::default(),
        }
    }
}
pub struct S3controlMultiRegionAccessPointDetailsElPublicAccessBlockElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3controlMultiRegionAccessPointDetailsElPublicAccessBlockElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlMultiRegionAccessPointDetailsElPublicAccessBlockElRef {
        S3controlMultiRegionAccessPointDetailsElPublicAccessBlockElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3controlMultiRegionAccessPointDetailsElPublicAccessBlockElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `block_public_acls` after provisioning.\n"]
    pub fn block_public_acls(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_public_acls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `block_public_policy` after provisioning.\n"]
    pub fn block_public_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_public_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_public_acls` after provisioning.\n"]
    pub fn ignore_public_acls(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_public_acls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `restrict_public_buckets` after provisioning.\n"]
    pub fn restrict_public_buckets(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restrict_public_buckets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3controlMultiRegionAccessPointDetailsElRegionEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_account_id: Option<PrimField<String>>,
}
impl S3controlMultiRegionAccessPointDetailsElRegionEl {
    #[doc = "Set the field `bucket_account_id`.\n"]
    pub fn set_bucket_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_account_id = Some(v.into());
        self
    }
}
impl ToListMappable for S3controlMultiRegionAccessPointDetailsElRegionEl {
    type O = BlockAssignable<S3controlMultiRegionAccessPointDetailsElRegionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3controlMultiRegionAccessPointDetailsElRegionEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
}
impl BuildS3controlMultiRegionAccessPointDetailsElRegionEl {
    pub fn build(self) -> S3controlMultiRegionAccessPointDetailsElRegionEl {
        S3controlMultiRegionAccessPointDetailsElRegionEl {
            bucket: self.bucket,
            bucket_account_id: core::default::Default::default(),
        }
    }
}
pub struct S3controlMultiRegionAccessPointDetailsElRegionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3controlMultiRegionAccessPointDetailsElRegionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlMultiRegionAccessPointDetailsElRegionElRef {
        S3controlMultiRegionAccessPointDetailsElRegionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3controlMultiRegionAccessPointDetailsElRegionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_account_id` after provisioning.\n"]
    pub fn bucket_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_account_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
#[derive(Serialize, Default)]
struct S3controlMultiRegionAccessPointDetailsElDynamic {
    public_access_block:
        Option<DynamicBlock<S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl>>,
    region: Option<DynamicBlock<S3controlMultiRegionAccessPointDetailsElRegionEl>>,
}
#[derive(Serialize)]
pub struct S3controlMultiRegionAccessPointDetailsEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_access_block: Option<Vec<S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<Vec<S3controlMultiRegionAccessPointDetailsElRegionEl>>,
    dynamic: S3controlMultiRegionAccessPointDetailsElDynamic,
}
impl S3controlMultiRegionAccessPointDetailsEl {
    #[doc = "Set the field `public_access_block`.\n"]
    pub fn set_public_access_block(
        mut self,
        v: impl Into<BlockAssignable<S3controlMultiRegionAccessPointDetailsElPublicAccessBlockEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.public_access_block = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.public_access_block = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(
        mut self,
        v: impl Into<BlockAssignable<S3controlMultiRegionAccessPointDetailsElRegionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.region = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.region = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for S3controlMultiRegionAccessPointDetailsEl {
    type O = BlockAssignable<S3controlMultiRegionAccessPointDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3controlMultiRegionAccessPointDetailsEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildS3controlMultiRegionAccessPointDetailsEl {
    pub fn build(self) -> S3controlMultiRegionAccessPointDetailsEl {
        S3controlMultiRegionAccessPointDetailsEl {
            name: self.name,
            public_access_block: core::default::Default::default(),
            region: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct S3controlMultiRegionAccessPointDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3controlMultiRegionAccessPointDetailsElRef {
    fn new(shared: StackShared, base: String) -> S3controlMultiRegionAccessPointDetailsElRef {
        S3controlMultiRegionAccessPointDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3controlMultiRegionAccessPointDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `public_access_block` after provisioning.\n"]
    pub fn public_access_block(
        &self,
    ) -> ListRef<S3controlMultiRegionAccessPointDetailsElPublicAccessBlockElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_access_block", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct S3controlMultiRegionAccessPointTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl S3controlMultiRegionAccessPointTimeoutsEl {
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
}
impl ToListMappable for S3controlMultiRegionAccessPointTimeoutsEl {
    type O = BlockAssignable<S3controlMultiRegionAccessPointTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildS3controlMultiRegionAccessPointTimeoutsEl {}
impl BuildS3controlMultiRegionAccessPointTimeoutsEl {
    pub fn build(self) -> S3controlMultiRegionAccessPointTimeoutsEl {
        S3controlMultiRegionAccessPointTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct S3controlMultiRegionAccessPointTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for S3controlMultiRegionAccessPointTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> S3controlMultiRegionAccessPointTimeoutsElRef {
        S3controlMultiRegionAccessPointTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl S3controlMultiRegionAccessPointTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct S3controlMultiRegionAccessPointDynamic {
    details: Option<DynamicBlock<S3controlMultiRegionAccessPointDetailsEl>>,
}
