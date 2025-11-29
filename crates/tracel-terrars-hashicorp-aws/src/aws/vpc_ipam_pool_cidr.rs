use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpcIpamPoolCidrData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    ipam_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    netmask_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_authorization_context: Option<Vec<VpcIpamPoolCidrCidrAuthorizationContextEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpcIpamPoolCidrTimeoutsEl>,
    dynamic: VpcIpamPoolCidrDynamic,
}

struct VpcIpamPoolCidr_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcIpamPoolCidrData>,
}

#[derive(Clone)]
pub struct VpcIpamPoolCidr(Rc<VpcIpamPoolCidr_>);

impl VpcIpamPoolCidr {
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

    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `netmask_length`.\n"]
    pub fn set_netmask_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().netmask_length = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `cidr_authorization_context`.\n"]
    pub fn set_cidr_authorization_context(
        self,
        v: impl Into<BlockAssignable<VpcIpamPoolCidrCidrAuthorizationContextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cidr_authorization_context = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cidr_authorization_context = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpcIpamPoolCidrTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_cidr_id` after provisioning.\n"]
    pub fn ipam_pool_cidr_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_cidr_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `netmask_length` after provisioning.\n"]
    pub fn netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.netmask_length", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_authorization_context` after provisioning.\n"]
    pub fn cidr_authorization_context(&self) -> ListRef<VpcIpamPoolCidrCidrAuthorizationContextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cidr_authorization_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcIpamPoolCidrTimeoutsElRef {
        VpcIpamPoolCidrTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for VpcIpamPoolCidr {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VpcIpamPoolCidr { }

impl ToListMappable for VpcIpamPoolCidr {
    type O = ListRef<VpcIpamPoolCidrRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VpcIpamPoolCidr_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpc_ipam_pool_cidr".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcIpamPoolCidr {
    pub tf_id: String,
    #[doc = ""]
    pub ipam_pool_id: PrimField<String>,
}

impl BuildVpcIpamPoolCidr {
    pub fn build(self, stack: &mut Stack) -> VpcIpamPoolCidr {
        let out = VpcIpamPoolCidr(Rc::new(VpcIpamPoolCidr_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpcIpamPoolCidrData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cidr: core::default::Default::default(),
                id: core::default::Default::default(),
                ipam_pool_id: self.ipam_pool_id,
                netmask_length: core::default::Default::default(),
                region: core::default::Default::default(),
                cidr_authorization_context: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcIpamPoolCidrRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcIpamPoolCidrRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl VpcIpamPoolCidrRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_cidr_id` after provisioning.\n"]
    pub fn ipam_pool_cidr_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_cidr_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `netmask_length` after provisioning.\n"]
    pub fn netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.netmask_length", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_authorization_context` after provisioning.\n"]
    pub fn cidr_authorization_context(&self) -> ListRef<VpcIpamPoolCidrCidrAuthorizationContextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cidr_authorization_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcIpamPoolCidrTimeoutsElRef {
        VpcIpamPoolCidrTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VpcIpamPoolCidrCidrAuthorizationContextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<PrimField<String>>,
}

impl VpcIpamPoolCidrCidrAuthorizationContextEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }

    #[doc = "Set the field `signature`.\n"]
    pub fn set_signature(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.signature = Some(v.into());
        self
    }
}

impl ToListMappable for VpcIpamPoolCidrCidrAuthorizationContextEl {
    type O = BlockAssignable<VpcIpamPoolCidrCidrAuthorizationContextEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcIpamPoolCidrCidrAuthorizationContextEl {}

impl BuildVpcIpamPoolCidrCidrAuthorizationContextEl {
    pub fn build(self) -> VpcIpamPoolCidrCidrAuthorizationContextEl {
        VpcIpamPoolCidrCidrAuthorizationContextEl {
            message: core::default::Default::default(),
            signature: core::default::Default::default(),
        }
    }
}

pub struct VpcIpamPoolCidrCidrAuthorizationContextElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcIpamPoolCidrCidrAuthorizationContextElRef {
    fn new(shared: StackShared, base: String) -> VpcIpamPoolCidrCidrAuthorizationContextElRef {
        VpcIpamPoolCidrCidrAuthorizationContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcIpamPoolCidrCidrAuthorizationContextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `signature` after provisioning.\n"]
    pub fn signature(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signature", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcIpamPoolCidrTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl VpcIpamPoolCidrTimeoutsEl {
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

impl ToListMappable for VpcIpamPoolCidrTimeoutsEl {
    type O = BlockAssignable<VpcIpamPoolCidrTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcIpamPoolCidrTimeoutsEl {}

impl BuildVpcIpamPoolCidrTimeoutsEl {
    pub fn build(self) -> VpcIpamPoolCidrTimeoutsEl {
        VpcIpamPoolCidrTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct VpcIpamPoolCidrTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcIpamPoolCidrTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpcIpamPoolCidrTimeoutsElRef {
        VpcIpamPoolCidrTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcIpamPoolCidrTimeoutsElRef {
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
struct VpcIpamPoolCidrDynamic {
    cidr_authorization_context: Option<DynamicBlock<VpcIpamPoolCidrCidrAuthorizationContextEl>>,
}
