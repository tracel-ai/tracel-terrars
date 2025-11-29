use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LbTrustStoreRevocationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    revocations_s3_bucket: PrimField<String>,
    revocations_s3_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revocations_s3_object_version: Option<PrimField<String>>,
    trust_store_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LbTrustStoreRevocationTimeoutsEl>,
}

struct LbTrustStoreRevocation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LbTrustStoreRevocationData>,
}

#[derive(Clone)]
pub struct LbTrustStoreRevocation(Rc<LbTrustStoreRevocation_>);

impl LbTrustStoreRevocation {
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

    #[doc = "Set the field `revocations_s3_object_version`.\n"]
    pub fn set_revocations_s3_object_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().revocations_s3_object_version = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LbTrustStoreRevocationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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

    #[doc = "Get a reference to the value of field `revocation_id` after provisioning.\n"]
    pub fn revocation_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revocation_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocations_s3_bucket` after provisioning.\n"]
    pub fn revocations_s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revocations_s3_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocations_s3_key` after provisioning.\n"]
    pub fn revocations_s3_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revocations_s3_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocations_s3_object_version` after provisioning.\n"]
    pub fn revocations_s3_object_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revocations_s3_object_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_store_arn` after provisioning.\n"]
    pub fn trust_store_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_store_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LbTrustStoreRevocationTimeoutsElRef {
        LbTrustStoreRevocationTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for LbTrustStoreRevocation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LbTrustStoreRevocation { }

impl ToListMappable for LbTrustStoreRevocation {
    type O = ListRef<LbTrustStoreRevocationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LbTrustStoreRevocation_ {
    fn extract_resource_type(&self) -> String {
        "aws_lb_trust_store_revocation".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLbTrustStoreRevocation {
    pub tf_id: String,
    #[doc = ""]
    pub revocations_s3_bucket: PrimField<String>,
    #[doc = ""]
    pub revocations_s3_key: PrimField<String>,
    #[doc = ""]
    pub trust_store_arn: PrimField<String>,
}

impl BuildLbTrustStoreRevocation {
    pub fn build(self, stack: &mut Stack) -> LbTrustStoreRevocation {
        let out = LbTrustStoreRevocation(Rc::new(LbTrustStoreRevocation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LbTrustStoreRevocationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                revocations_s3_bucket: self.revocations_s3_bucket,
                revocations_s3_key: self.revocations_s3_key,
                revocations_s3_object_version: core::default::Default::default(),
                trust_store_arn: self.trust_store_arn,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LbTrustStoreRevocationRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbTrustStoreRevocationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl LbTrustStoreRevocationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `revocation_id` after provisioning.\n"]
    pub fn revocation_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revocation_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocations_s3_bucket` after provisioning.\n"]
    pub fn revocations_s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revocations_s3_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocations_s3_key` after provisioning.\n"]
    pub fn revocations_s3_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revocations_s3_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revocations_s3_object_version` after provisioning.\n"]
    pub fn revocations_s3_object_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revocations_s3_object_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_store_arn` after provisioning.\n"]
    pub fn trust_store_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_store_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LbTrustStoreRevocationTimeoutsElRef {
        LbTrustStoreRevocationTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LbTrustStoreRevocationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl LbTrustStoreRevocationTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for LbTrustStoreRevocationTimeoutsEl {
    type O = BlockAssignable<LbTrustStoreRevocationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbTrustStoreRevocationTimeoutsEl {}

impl BuildLbTrustStoreRevocationTimeoutsEl {
    pub fn build(self) -> LbTrustStoreRevocationTimeoutsEl {
        LbTrustStoreRevocationTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct LbTrustStoreRevocationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LbTrustStoreRevocationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LbTrustStoreRevocationTimeoutsElRef {
        LbTrustStoreRevocationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbTrustStoreRevocationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
