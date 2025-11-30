use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct S3controlMultiRegionAccessPointPolicyData {
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
    details: Option<Vec<S3controlMultiRegionAccessPointPolicyDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<S3controlMultiRegionAccessPointPolicyTimeoutsEl>,
    dynamic: S3controlMultiRegionAccessPointPolicyDynamic,
}

struct S3controlMultiRegionAccessPointPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3controlMultiRegionAccessPointPolicyData>,
}

#[derive(Clone)]
pub struct S3controlMultiRegionAccessPointPolicy(Rc<S3controlMultiRegionAccessPointPolicy_>);

impl S3controlMultiRegionAccessPointPolicy {
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
        v: impl Into<BlockAssignable<S3controlMultiRegionAccessPointPolicyDetailsEl>>,
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
    pub fn set_timeouts(
        self,
        v: impl Into<S3controlMultiRegionAccessPointPolicyTimeoutsEl>,
    ) -> Self {
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

    #[doc = "Get a reference to the value of field `established` after provisioning.\n"]
    pub fn established(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.established", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proposed` after provisioning.\n"]
    pub fn proposed(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.proposed", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<S3controlMultiRegionAccessPointPolicyDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.details", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
        S3controlMultiRegionAccessPointPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for S3controlMultiRegionAccessPointPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for S3controlMultiRegionAccessPointPolicy {}

impl ToListMappable for S3controlMultiRegionAccessPointPolicy {
    type O = ListRef<S3controlMultiRegionAccessPointPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3controlMultiRegionAccessPointPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3control_multi_region_access_point_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3controlMultiRegionAccessPointPolicy {
    pub tf_id: String,
}

impl BuildS3controlMultiRegionAccessPointPolicy {
    pub fn build(self, stack: &mut Stack) -> S3controlMultiRegionAccessPointPolicy {
        let out = S3controlMultiRegionAccessPointPolicy(Rc::new(
            S3controlMultiRegionAccessPointPolicy_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(S3controlMultiRegionAccessPointPolicyData {
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
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3controlMultiRegionAccessPointPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlMultiRegionAccessPointPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl S3controlMultiRegionAccessPointPolicyRef {
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

    #[doc = "Get a reference to the value of field `established` after provisioning.\n"]
    pub fn established(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.established", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proposed` after provisioning.\n"]
    pub fn proposed(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.proposed", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<S3controlMultiRegionAccessPointPolicyDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.details", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
        S3controlMultiRegionAccessPointPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct S3controlMultiRegionAccessPointPolicyDetailsEl {
    name: PrimField<String>,
    policy: PrimField<String>,
}

impl S3controlMultiRegionAccessPointPolicyDetailsEl {}

impl ToListMappable for S3controlMultiRegionAccessPointPolicyDetailsEl {
    type O = BlockAssignable<S3controlMultiRegionAccessPointPolicyDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlMultiRegionAccessPointPolicyDetailsEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub policy: PrimField<String>,
}

impl BuildS3controlMultiRegionAccessPointPolicyDetailsEl {
    pub fn build(self) -> S3controlMultiRegionAccessPointPolicyDetailsEl {
        S3controlMultiRegionAccessPointPolicyDetailsEl {
            name: self.name,
            policy: self.policy,
        }
    }
}

pub struct S3controlMultiRegionAccessPointPolicyDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlMultiRegionAccessPointPolicyDetailsElRef {
    fn new(shared: StackShared, base: String) -> S3controlMultiRegionAccessPointPolicyDetailsElRef {
        S3controlMultiRegionAccessPointPolicyDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlMultiRegionAccessPointPolicyDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlMultiRegionAccessPointPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl S3controlMultiRegionAccessPointPolicyTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for S3controlMultiRegionAccessPointPolicyTimeoutsEl {
    type O = BlockAssignable<S3controlMultiRegionAccessPointPolicyTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlMultiRegionAccessPointPolicyTimeoutsEl {}

impl BuildS3controlMultiRegionAccessPointPolicyTimeoutsEl {
    pub fn build(self) -> S3controlMultiRegionAccessPointPolicyTimeoutsEl {
        S3controlMultiRegionAccessPointPolicyTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
        S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlMultiRegionAccessPointPolicyDynamic {
    details: Option<DynamicBlock<S3controlMultiRegionAccessPointPolicyDetailsEl>>,
}
