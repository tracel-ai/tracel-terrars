use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct S3controlDirectoryBucketAccessPointScopeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<Vec<S3controlDirectoryBucketAccessPointScopeScopeEl>>,
    dynamic: S3controlDirectoryBucketAccessPointScopeDynamic,
}

struct S3controlDirectoryBucketAccessPointScope_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3controlDirectoryBucketAccessPointScopeData>,
}

#[derive(Clone)]
pub struct S3controlDirectoryBucketAccessPointScope(Rc<S3controlDirectoryBucketAccessPointScope_>);

impl S3controlDirectoryBucketAccessPointScope {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(
        self,
        v: impl Into<BlockAssignable<S3controlDirectoryBucketAccessPointScopeScopeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scope = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scope = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> ListRef<S3controlDirectoryBucketAccessPointScopeScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
}

impl Referable for S3controlDirectoryBucketAccessPointScope {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for S3controlDirectoryBucketAccessPointScope {}

impl ToListMappable for S3controlDirectoryBucketAccessPointScope {
    type O = ListRef<S3controlDirectoryBucketAccessPointScopeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3controlDirectoryBucketAccessPointScope_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3control_directory_bucket_access_point_scope".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3controlDirectoryBucketAccessPointScope {
    pub tf_id: String,
    #[doc = ""]
    pub account_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildS3controlDirectoryBucketAccessPointScope {
    pub fn build(self, stack: &mut Stack) -> S3controlDirectoryBucketAccessPointScope {
        let out = S3controlDirectoryBucketAccessPointScope(Rc::new(
            S3controlDirectoryBucketAccessPointScope_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(S3controlDirectoryBucketAccessPointScopeData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    account_id: self.account_id,
                    name: self.name,
                    region: core::default::Default::default(),
                    scope: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3controlDirectoryBucketAccessPointScopeRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlDirectoryBucketAccessPointScopeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl S3controlDirectoryBucketAccessPointScopeRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> ListRef<S3controlDirectoryBucketAccessPointScopeScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct S3controlDirectoryBucketAccessPointScopeScopeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefixes: Option<ListField<PrimField<String>>>,
}

impl S3controlDirectoryBucketAccessPointScopeScopeEl {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.permissions = Some(v.into());
        self
    }

    #[doc = "Set the field `prefixes`.\n"]
    pub fn set_prefixes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.prefixes = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlDirectoryBucketAccessPointScopeScopeEl {
    type O = BlockAssignable<S3controlDirectoryBucketAccessPointScopeScopeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlDirectoryBucketAccessPointScopeScopeEl {}

impl BuildS3controlDirectoryBucketAccessPointScopeScopeEl {
    pub fn build(self) -> S3controlDirectoryBucketAccessPointScopeScopeEl {
        S3controlDirectoryBucketAccessPointScopeScopeEl {
            permissions: core::default::Default::default(),
            prefixes: core::default::Default::default(),
        }
    }
}

pub struct S3controlDirectoryBucketAccessPointScopeScopeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlDirectoryBucketAccessPointScopeScopeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlDirectoryBucketAccessPointScopeScopeElRef {
        S3controlDirectoryBucketAccessPointScopeScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlDirectoryBucketAccessPointScopeScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc = "Get a reference to the value of field `prefixes` after provisioning.\n"]
    pub fn prefixes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.prefixes", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlDirectoryBucketAccessPointScopeDynamic {
    scope: Option<DynamicBlock<S3controlDirectoryBucketAccessPointScopeScopeEl>>,
}
