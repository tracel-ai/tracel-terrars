use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct VpcPeeringConnectionOptionsData {
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
    vpc_peering_connection_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accepter: Option<Vec<VpcPeeringConnectionOptionsAccepterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requester: Option<Vec<VpcPeeringConnectionOptionsRequesterEl>>,
    dynamic: VpcPeeringConnectionOptionsDynamic,
}

struct VpcPeeringConnectionOptions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcPeeringConnectionOptionsData>,
}

#[derive(Clone)]
pub struct VpcPeeringConnectionOptions(Rc<VpcPeeringConnectionOptions_>);

impl VpcPeeringConnectionOptions {
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

    #[doc = "Set the field `accepter`.\n"]
    pub fn set_accepter(
        self,
        v: impl Into<BlockAssignable<VpcPeeringConnectionOptionsAccepterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().accepter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.accepter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `requester`.\n"]
    pub fn set_requester(
        self,
        v: impl Into<BlockAssignable<VpcPeeringConnectionOptionsRequesterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().requester = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.requester = Some(d);
            }
        }
        self
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

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_peering_connection_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `accepter` after provisioning.\n"]
    pub fn accepter(&self) -> ListRef<VpcPeeringConnectionOptionsAccepterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accepter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> ListRef<VpcPeeringConnectionOptionsRequesterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.requester", self.extract_ref()),
        )
    }
}

impl Referable for VpcPeeringConnectionOptions {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for VpcPeeringConnectionOptions {}

impl ToListMappable for VpcPeeringConnectionOptions {
    type O = ListRef<VpcPeeringConnectionOptionsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VpcPeeringConnectionOptions_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpc_peering_connection_options".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcPeeringConnectionOptions {
    pub tf_id: String,
    #[doc = ""]
    pub vpc_peering_connection_id: PrimField<String>,
}

impl BuildVpcPeeringConnectionOptions {
    pub fn build(self, stack: &mut Stack) -> VpcPeeringConnectionOptions {
        let out = VpcPeeringConnectionOptions(Rc::new(VpcPeeringConnectionOptions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpcPeeringConnectionOptionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                vpc_peering_connection_id: self.vpc_peering_connection_id,
                accepter: core::default::Default::default(),
                requester: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcPeeringConnectionOptionsRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcPeeringConnectionOptionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl VpcPeeringConnectionOptionsRef {
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

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_peering_connection_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `accepter` after provisioning.\n"]
    pub fn accepter(&self) -> ListRef<VpcPeeringConnectionOptionsAccepterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accepter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> ListRef<VpcPeeringConnectionOptionsRequesterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.requester", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct VpcPeeringConnectionOptionsAccepterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_remote_vpc_dns_resolution: Option<PrimField<bool>>,
}

impl VpcPeeringConnectionOptionsAccepterEl {
    #[doc = "Set the field `allow_remote_vpc_dns_resolution`.\n"]
    pub fn set_allow_remote_vpc_dns_resolution(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_remote_vpc_dns_resolution = Some(v.into());
        self
    }
}

impl ToListMappable for VpcPeeringConnectionOptionsAccepterEl {
    type O = BlockAssignable<VpcPeeringConnectionOptionsAccepterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcPeeringConnectionOptionsAccepterEl {}

impl BuildVpcPeeringConnectionOptionsAccepterEl {
    pub fn build(self) -> VpcPeeringConnectionOptionsAccepterEl {
        VpcPeeringConnectionOptionsAccepterEl {
            allow_remote_vpc_dns_resolution: core::default::Default::default(),
        }
    }
}

pub struct VpcPeeringConnectionOptionsAccepterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcPeeringConnectionOptionsAccepterElRef {
    fn new(shared: StackShared, base: String) -> VpcPeeringConnectionOptionsAccepterElRef {
        VpcPeeringConnectionOptionsAccepterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcPeeringConnectionOptionsAccepterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_remote_vpc_dns_resolution` after provisioning.\n"]
    pub fn allow_remote_vpc_dns_resolution(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_remote_vpc_dns_resolution", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct VpcPeeringConnectionOptionsRequesterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_remote_vpc_dns_resolution: Option<PrimField<bool>>,
}

impl VpcPeeringConnectionOptionsRequesterEl {
    #[doc = "Set the field `allow_remote_vpc_dns_resolution`.\n"]
    pub fn set_allow_remote_vpc_dns_resolution(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_remote_vpc_dns_resolution = Some(v.into());
        self
    }
}

impl ToListMappable for VpcPeeringConnectionOptionsRequesterEl {
    type O = BlockAssignable<VpcPeeringConnectionOptionsRequesterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcPeeringConnectionOptionsRequesterEl {}

impl BuildVpcPeeringConnectionOptionsRequesterEl {
    pub fn build(self) -> VpcPeeringConnectionOptionsRequesterEl {
        VpcPeeringConnectionOptionsRequesterEl {
            allow_remote_vpc_dns_resolution: core::default::Default::default(),
        }
    }
}

pub struct VpcPeeringConnectionOptionsRequesterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcPeeringConnectionOptionsRequesterElRef {
    fn new(shared: StackShared, base: String) -> VpcPeeringConnectionOptionsRequesterElRef {
        VpcPeeringConnectionOptionsRequesterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcPeeringConnectionOptionsRequesterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_remote_vpc_dns_resolution` after provisioning.\n"]
    pub fn allow_remote_vpc_dns_resolution(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_remote_vpc_dns_resolution", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct VpcPeeringConnectionOptionsDynamic {
    accepter: Option<DynamicBlock<VpcPeeringConnectionOptionsAccepterEl>>,
    requester: Option<DynamicBlock<VpcPeeringConnectionOptionsRequesterEl>>,
}
