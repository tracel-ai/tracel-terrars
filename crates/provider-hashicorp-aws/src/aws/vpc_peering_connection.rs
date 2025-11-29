use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpcPeeringConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_accept: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_owner_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_region: Option<PrimField<String>>,
    peer_vpc_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    vpc_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accepter: Option<Vec<VpcPeeringConnectionAccepterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requester: Option<Vec<VpcPeeringConnectionRequesterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpcPeeringConnectionTimeoutsEl>,
    dynamic: VpcPeeringConnectionDynamic,
}

struct VpcPeeringConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcPeeringConnectionData>,
}

#[derive(Clone)]
pub struct VpcPeeringConnection(Rc<VpcPeeringConnection_>);

impl VpcPeeringConnection {
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

    #[doc = "Set the field `auto_accept`.\n"]
    pub fn set_auto_accept(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_accept = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `peer_owner_id`.\n"]
    pub fn set_peer_owner_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peer_owner_id = Some(v.into());
        self
    }

    #[doc = "Set the field `peer_region`.\n"]
    pub fn set_peer_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peer_region = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `accepter`.\n"]
    pub fn set_accepter(self, v: impl Into<BlockAssignable<VpcPeeringConnectionAccepterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().accepter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.accepter = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `requester`.\n"]
    pub fn set_requester(self, v: impl Into<BlockAssignable<VpcPeeringConnectionRequesterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().requester = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.requester = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpcPeeringConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accept_status` after provisioning.\n"]
    pub fn accept_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accept_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_accept` after provisioning.\n"]
    pub fn auto_accept(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_accept", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_owner_id` after provisioning.\n"]
    pub fn peer_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_region` after provisioning.\n"]
    pub fn peer_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_vpc_id` after provisioning.\n"]
    pub fn peer_vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_vpc_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `accepter` after provisioning.\n"]
    pub fn accepter(&self) -> ListRef<VpcPeeringConnectionAccepterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.accepter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> ListRef<VpcPeeringConnectionRequesterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.requester", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcPeeringConnectionTimeoutsElRef {
        VpcPeeringConnectionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for VpcPeeringConnection {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VpcPeeringConnection { }

impl ToListMappable for VpcPeeringConnection {
    type O = ListRef<VpcPeeringConnectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VpcPeeringConnection_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpc_peering_connection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcPeeringConnection {
    pub tf_id: String,
    #[doc = ""]
    pub peer_vpc_id: PrimField<String>,
    #[doc = ""]
    pub vpc_id: PrimField<String>,
}

impl BuildVpcPeeringConnection {
    pub fn build(self, stack: &mut Stack) -> VpcPeeringConnection {
        let out = VpcPeeringConnection(Rc::new(VpcPeeringConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpcPeeringConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_accept: core::default::Default::default(),
                id: core::default::Default::default(),
                peer_owner_id: core::default::Default::default(),
                peer_region: core::default::Default::default(),
                peer_vpc_id: self.peer_vpc_id,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_id: self.vpc_id,
                accepter: core::default::Default::default(),
                requester: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcPeeringConnectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcPeeringConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl VpcPeeringConnectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accept_status` after provisioning.\n"]
    pub fn accept_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accept_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_accept` after provisioning.\n"]
    pub fn auto_accept(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_accept", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_owner_id` after provisioning.\n"]
    pub fn peer_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_region` after provisioning.\n"]
    pub fn peer_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_vpc_id` after provisioning.\n"]
    pub fn peer_vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_vpc_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `accepter` after provisioning.\n"]
    pub fn accepter(&self) -> ListRef<VpcPeeringConnectionAccepterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.accepter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> ListRef<VpcPeeringConnectionRequesterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.requester", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcPeeringConnectionTimeoutsElRef {
        VpcPeeringConnectionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VpcPeeringConnectionAccepterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_remote_vpc_dns_resolution: Option<PrimField<bool>>,
}

impl VpcPeeringConnectionAccepterEl {
    #[doc = "Set the field `allow_remote_vpc_dns_resolution`.\n"]
    pub fn set_allow_remote_vpc_dns_resolution(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_remote_vpc_dns_resolution = Some(v.into());
        self
    }
}

impl ToListMappable for VpcPeeringConnectionAccepterEl {
    type O = BlockAssignable<VpcPeeringConnectionAccepterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcPeeringConnectionAccepterEl {}

impl BuildVpcPeeringConnectionAccepterEl {
    pub fn build(self) -> VpcPeeringConnectionAccepterEl {
        VpcPeeringConnectionAccepterEl { allow_remote_vpc_dns_resolution: core::default::Default::default() }
    }
}

pub struct VpcPeeringConnectionAccepterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcPeeringConnectionAccepterElRef {
    fn new(shared: StackShared, base: String) -> VpcPeeringConnectionAccepterElRef {
        VpcPeeringConnectionAccepterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcPeeringConnectionAccepterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_remote_vpc_dns_resolution` after provisioning.\n"]
    pub fn allow_remote_vpc_dns_resolution(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_remote_vpc_dns_resolution", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcPeeringConnectionRequesterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_remote_vpc_dns_resolution: Option<PrimField<bool>>,
}

impl VpcPeeringConnectionRequesterEl {
    #[doc = "Set the field `allow_remote_vpc_dns_resolution`.\n"]
    pub fn set_allow_remote_vpc_dns_resolution(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_remote_vpc_dns_resolution = Some(v.into());
        self
    }
}

impl ToListMappable for VpcPeeringConnectionRequesterEl {
    type O = BlockAssignable<VpcPeeringConnectionRequesterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcPeeringConnectionRequesterEl {}

impl BuildVpcPeeringConnectionRequesterEl {
    pub fn build(self) -> VpcPeeringConnectionRequesterEl {
        VpcPeeringConnectionRequesterEl { allow_remote_vpc_dns_resolution: core::default::Default::default() }
    }
}

pub struct VpcPeeringConnectionRequesterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcPeeringConnectionRequesterElRef {
    fn new(shared: StackShared, base: String) -> VpcPeeringConnectionRequesterElRef {
        VpcPeeringConnectionRequesterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcPeeringConnectionRequesterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_remote_vpc_dns_resolution` after provisioning.\n"]
    pub fn allow_remote_vpc_dns_resolution(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_remote_vpc_dns_resolution", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcPeeringConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl VpcPeeringConnectionTimeoutsEl {
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

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for VpcPeeringConnectionTimeoutsEl {
    type O = BlockAssignable<VpcPeeringConnectionTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcPeeringConnectionTimeoutsEl {}

impl BuildVpcPeeringConnectionTimeoutsEl {
    pub fn build(self) -> VpcPeeringConnectionTimeoutsEl {
        VpcPeeringConnectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct VpcPeeringConnectionTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcPeeringConnectionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpcPeeringConnectionTimeoutsElRef {
        VpcPeeringConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcPeeringConnectionTimeoutsElRef {
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

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpcPeeringConnectionDynamic {
    accepter: Option<DynamicBlock<VpcPeeringConnectionAccepterEl>>,
    requester: Option<DynamicBlock<VpcPeeringConnectionRequesterEl>>,
}
