use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3controlAccessGrantData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    access_grants_location_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    permission: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_prefix_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_grants_location_configuration: Option<Vec<S3controlAccessGrantAccessGrantsLocationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grantee: Option<Vec<S3controlAccessGrantGranteeEl>>,
    dynamic: S3controlAccessGrantDynamic,
}

struct S3controlAccessGrant_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3controlAccessGrantData>,
}

#[derive(Clone)]
pub struct S3controlAccessGrant(Rc<S3controlAccessGrant_>);

impl S3controlAccessGrant {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_prefix_type`.\n"]
    pub fn set_s3_prefix_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_prefix_type = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `access_grants_location_configuration`.\n"]
    pub fn set_access_grants_location_configuration(
        self,
        v: impl Into<BlockAssignable<S3controlAccessGrantAccessGrantsLocationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_grants_location_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_grants_location_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `grantee`.\n"]
    pub fn set_grantee(self, v: impl Into<BlockAssignable<S3controlAccessGrantGranteeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grantee = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grantee = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `access_grant_arn` after provisioning.\n"]
    pub fn access_grant_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_grant_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_grant_id` after provisioning.\n"]
    pub fn access_grant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_grant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_grants_location_id` after provisioning.\n"]
    pub fn access_grants_location_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_grants_location_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_scope` after provisioning.\n"]
    pub fn grant_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grant_scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_prefix_type` after provisioning.\n"]
    pub fn s3_prefix_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_grants_location_configuration` after provisioning.\n"]
    pub fn access_grants_location_configuration(
        &self,
    ) -> ListRef<S3controlAccessGrantAccessGrantsLocationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_grants_location_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grantee` after provisioning.\n"]
    pub fn grantee(&self) -> ListRef<S3controlAccessGrantGranteeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grantee", self.extract_ref()))
    }
}

impl Referable for S3controlAccessGrant {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for S3controlAccessGrant { }

impl ToListMappable for S3controlAccessGrant {
    type O = ListRef<S3controlAccessGrantRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3controlAccessGrant_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3control_access_grant".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3controlAccessGrant {
    pub tf_id: String,
    #[doc = ""]
    pub access_grants_location_id: PrimField<String>,
    #[doc = ""]
    pub permission: PrimField<String>,
}

impl BuildS3controlAccessGrant {
    pub fn build(self, stack: &mut Stack) -> S3controlAccessGrant {
        let out = S3controlAccessGrant(Rc::new(S3controlAccessGrant_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3controlAccessGrantData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_grants_location_id: self.access_grants_location_id,
                account_id: core::default::Default::default(),
                permission: self.permission,
                region: core::default::Default::default(),
                s3_prefix_type: core::default::Default::default(),
                tags: core::default::Default::default(),
                access_grants_location_configuration: core::default::Default::default(),
                grantee: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3controlAccessGrantRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlAccessGrantRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl S3controlAccessGrantRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_grant_arn` after provisioning.\n"]
    pub fn access_grant_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_grant_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_grant_id` after provisioning.\n"]
    pub fn access_grant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_grant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_grants_location_id` after provisioning.\n"]
    pub fn access_grants_location_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_grants_location_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_scope` after provisioning.\n"]
    pub fn grant_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grant_scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_prefix_type` after provisioning.\n"]
    pub fn s3_prefix_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_grants_location_configuration` after provisioning.\n"]
    pub fn access_grants_location_configuration(
        &self,
    ) -> ListRef<S3controlAccessGrantAccessGrantsLocationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_grants_location_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grantee` after provisioning.\n"]
    pub fn grantee(&self) -> ListRef<S3controlAccessGrantGranteeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grantee", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3controlAccessGrantAccessGrantsLocationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_sub_prefix: Option<PrimField<String>>,
}

impl S3controlAccessGrantAccessGrantsLocationConfigurationEl {
    #[doc = "Set the field `s3_sub_prefix`.\n"]
    pub fn set_s3_sub_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_sub_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlAccessGrantAccessGrantsLocationConfigurationEl {
    type O = BlockAssignable<S3controlAccessGrantAccessGrantsLocationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlAccessGrantAccessGrantsLocationConfigurationEl {}

impl BuildS3controlAccessGrantAccessGrantsLocationConfigurationEl {
    pub fn build(self) -> S3controlAccessGrantAccessGrantsLocationConfigurationEl {
        S3controlAccessGrantAccessGrantsLocationConfigurationEl { s3_sub_prefix: core::default::Default::default() }
    }
}

pub struct S3controlAccessGrantAccessGrantsLocationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlAccessGrantAccessGrantsLocationConfigurationElRef {
    fn new(shared: StackShared, base: String) -> S3controlAccessGrantAccessGrantsLocationConfigurationElRef {
        S3controlAccessGrantAccessGrantsLocationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlAccessGrantAccessGrantsLocationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_sub_prefix` after provisioning.\n"]
    pub fn s3_sub_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_sub_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlAccessGrantGranteeEl {
    grantee_identifier: PrimField<String>,
    grantee_type: PrimField<String>,
}

impl S3controlAccessGrantGranteeEl { }

impl ToListMappable for S3controlAccessGrantGranteeEl {
    type O = BlockAssignable<S3controlAccessGrantGranteeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlAccessGrantGranteeEl {
    #[doc = ""]
    pub grantee_identifier: PrimField<String>,
    #[doc = ""]
    pub grantee_type: PrimField<String>,
}

impl BuildS3controlAccessGrantGranteeEl {
    pub fn build(self) -> S3controlAccessGrantGranteeEl {
        S3controlAccessGrantGranteeEl {
            grantee_identifier: self.grantee_identifier,
            grantee_type: self.grantee_type,
        }
    }
}

pub struct S3controlAccessGrantGranteeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlAccessGrantGranteeElRef {
    fn new(shared: StackShared, base: String) -> S3controlAccessGrantGranteeElRef {
        S3controlAccessGrantGranteeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlAccessGrantGranteeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grantee_identifier` after provisioning.\n"]
    pub fn grantee_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grantee_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `grantee_type` after provisioning.\n"]
    pub fn grantee_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grantee_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlAccessGrantDynamic {
    access_grants_location_configuration: Option<
        DynamicBlock<S3controlAccessGrantAccessGrantsLocationConfigurationEl>,
    >,
    grantee: Option<DynamicBlock<S3controlAccessGrantGranteeEl>>,
}
