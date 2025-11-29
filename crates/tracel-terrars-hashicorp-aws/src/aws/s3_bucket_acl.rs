use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketAclData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acl: Option<PrimField<String>>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_bucket_owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_policy: Option<Vec<S3BucketAclAccessControlPolicyEl>>,
    dynamic: S3BucketAclDynamic,
}

struct S3BucketAcl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketAclData>,
}

#[derive(Clone)]
pub struct S3BucketAcl(Rc<S3BucketAcl_>);

impl S3BucketAcl {
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

    #[doc = "Set the field `acl`.\n"]
    pub fn set_acl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().acl = Some(v.into());
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

    #[doc = "Set the field `access_control_policy`.\n"]
    pub fn set_access_control_policy(self, v: impl Into<BlockAssignable<S3BucketAclAccessControlPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_control_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_control_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `access_control_policy` after provisioning.\n"]
    pub fn access_control_policy(&self) -> ListRef<S3BucketAclAccessControlPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_control_policy", self.extract_ref()))
    }
}

impl Referable for S3BucketAcl {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for S3BucketAcl { }

impl ToListMappable for S3BucketAcl {
    type O = ListRef<S3BucketAclRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3BucketAcl_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_acl".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketAcl {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
}

impl BuildS3BucketAcl {
    pub fn build(self, stack: &mut Stack) -> S3BucketAcl {
        let out = S3BucketAcl(Rc::new(S3BucketAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketAclData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                acl: core::default::Default::default(),
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                access_control_policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketAclRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketAclRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl S3BucketAclRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `access_control_policy` after provisioning.\n"]
    pub fn access_control_policy(&self) -> ListRef<S3BucketAclAccessControlPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_control_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3BucketAclAccessControlPolicyElGrantElGranteeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}

impl S3BucketAclAccessControlPolicyElGrantElGranteeEl {
    #[doc = "Set the field `email_address`.\n"]
    pub fn set_email_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_address = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketAclAccessControlPolicyElGrantElGranteeEl {
    type O = BlockAssignable<S3BucketAclAccessControlPolicyElGrantElGranteeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAclAccessControlPolicyElGrantElGranteeEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildS3BucketAclAccessControlPolicyElGrantElGranteeEl {
    pub fn build(self) -> S3BucketAclAccessControlPolicyElGrantElGranteeEl {
        S3BucketAclAccessControlPolicyElGrantElGranteeEl {
            email_address: core::default::Default::default(),
            id: core::default::Default::default(),
            type_: self.type_,
            uri: core::default::Default::default(),
        }
    }
}

pub struct S3BucketAclAccessControlPolicyElGrantElGranteeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketAclAccessControlPolicyElGrantElGranteeElRef {
    fn new(shared: StackShared, base: String) -> S3BucketAclAccessControlPolicyElGrantElGranteeElRef {
        S3BucketAclAccessControlPolicyElGrantElGranteeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAclAccessControlPolicyElGrantElGranteeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_address", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAclAccessControlPolicyElGrantElDynamic {
    grantee: Option<DynamicBlock<S3BucketAclAccessControlPolicyElGrantElGranteeEl>>,
}

#[derive(Serialize)]
pub struct S3BucketAclAccessControlPolicyElGrantEl {
    permission: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grantee: Option<Vec<S3BucketAclAccessControlPolicyElGrantElGranteeEl>>,
    dynamic: S3BucketAclAccessControlPolicyElGrantElDynamic,
}

impl S3BucketAclAccessControlPolicyElGrantEl {
    #[doc = "Set the field `grantee`.\n"]
    pub fn set_grantee(
        mut self,
        v: impl Into<BlockAssignable<S3BucketAclAccessControlPolicyElGrantElGranteeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grantee = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grantee = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketAclAccessControlPolicyElGrantEl {
    type O = BlockAssignable<S3BucketAclAccessControlPolicyElGrantEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAclAccessControlPolicyElGrantEl {
    #[doc = ""]
    pub permission: PrimField<String>,
}

impl BuildS3BucketAclAccessControlPolicyElGrantEl {
    pub fn build(self) -> S3BucketAclAccessControlPolicyElGrantEl {
        S3BucketAclAccessControlPolicyElGrantEl {
            permission: self.permission,
            grantee: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketAclAccessControlPolicyElGrantElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketAclAccessControlPolicyElGrantElRef {
    fn new(shared: StackShared, base: String) -> S3BucketAclAccessControlPolicyElGrantElRef {
        S3BucketAclAccessControlPolicyElGrantElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAclAccessControlPolicyElGrantElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }

    #[doc = "Get a reference to the value of field `grantee` after provisioning.\n"]
    pub fn grantee(&self) -> ListRef<S3BucketAclAccessControlPolicyElGrantElGranteeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grantee", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketAclAccessControlPolicyElOwnerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    id: PrimField<String>,
}

impl S3BucketAclAccessControlPolicyElOwnerEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketAclAccessControlPolicyElOwnerEl {
    type O = BlockAssignable<S3BucketAclAccessControlPolicyElOwnerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAclAccessControlPolicyElOwnerEl {
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildS3BucketAclAccessControlPolicyElOwnerEl {
    pub fn build(self) -> S3BucketAclAccessControlPolicyElOwnerEl {
        S3BucketAclAccessControlPolicyElOwnerEl {
            display_name: core::default::Default::default(),
            id: self.id,
        }
    }
}

pub struct S3BucketAclAccessControlPolicyElOwnerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketAclAccessControlPolicyElOwnerElRef {
    fn new(shared: StackShared, base: String) -> S3BucketAclAccessControlPolicyElOwnerElRef {
        S3BucketAclAccessControlPolicyElOwnerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAclAccessControlPolicyElOwnerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAclAccessControlPolicyElDynamic {
    grant: Option<DynamicBlock<S3BucketAclAccessControlPolicyElGrantEl>>,
    owner: Option<DynamicBlock<S3BucketAclAccessControlPolicyElOwnerEl>>,
}

#[derive(Serialize)]
pub struct S3BucketAclAccessControlPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grant: Option<Vec<S3BucketAclAccessControlPolicyElGrantEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<Vec<S3BucketAclAccessControlPolicyElOwnerEl>>,
    dynamic: S3BucketAclAccessControlPolicyElDynamic,
}

impl S3BucketAclAccessControlPolicyEl {
    #[doc = "Set the field `grant`.\n"]
    pub fn set_grant(mut self, v: impl Into<BlockAssignable<S3BucketAclAccessControlPolicyElGrantEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grant = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grant = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl Into<BlockAssignable<S3BucketAclAccessControlPolicyElOwnerEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.owner = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.owner = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketAclAccessControlPolicyEl {
    type O = BlockAssignable<S3BucketAclAccessControlPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketAclAccessControlPolicyEl {}

impl BuildS3BucketAclAccessControlPolicyEl {
    pub fn build(self) -> S3BucketAclAccessControlPolicyEl {
        S3BucketAclAccessControlPolicyEl {
            grant: core::default::Default::default(),
            owner: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketAclAccessControlPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketAclAccessControlPolicyElRef {
    fn new(shared: StackShared, base: String) -> S3BucketAclAccessControlPolicyElRef {
        S3BucketAclAccessControlPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketAclAccessControlPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> ListRef<S3BucketAclAccessControlPolicyElOwnerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.owner", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketAclDynamic {
    access_control_policy: Option<DynamicBlock<S3BucketAclAccessControlPolicyEl>>,
}
