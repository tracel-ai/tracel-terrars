use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataVpcPeeringConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_owner_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_vpc_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataVpcPeeringConnectionFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataVpcPeeringConnectionTimeoutsEl>,
    dynamic: DataVpcPeeringConnectionDynamic,
}

struct DataVpcPeeringConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVpcPeeringConnectionData>,
}

#[derive(Clone)]
pub struct DataVpcPeeringConnection(Rc<DataVpcPeeringConnection_>);

impl DataVpcPeeringConnection {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `owner_id`.\n"]
    pub fn set_owner_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().owner_id = Some(v.into());
        self
    }

    #[doc = "Set the field `peer_cidr_block`.\n"]
    pub fn set_peer_cidr_block(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peer_cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `peer_owner_id`.\n"]
    pub fn set_peer_owner_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peer_owner_id = Some(v.into());
        self
    }

    #[doc = "Set the field `peer_vpc_id`.\n"]
    pub fn set_peer_vpc_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peer_vpc_id = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl Into<BlockAssignable<DataVpcPeeringConnectionFilterEl>>,
    ) -> Self {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataVpcPeeringConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accepter` after provisioning.\n"]
    pub fn accepter(&self) -> RecRef<PrimExpr<bool>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.accepter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cidr_block_set` after provisioning.\n"]
    pub fn cidr_block_set(&self) -> ListRef<DataVpcPeeringConnectionCidrBlockSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cidr_block_set", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block_set` after provisioning.\n"]
    pub fn ipv6_cidr_block_set(&self) -> ListRef<DataVpcPeeringConnectionIpv6CidrBlockSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block_set", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_cidr_block` after provisioning.\n"]
    pub fn peer_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_cidr_block_set` after provisioning.\n"]
    pub fn peer_cidr_block_set(&self) -> ListRef<DataVpcPeeringConnectionPeerCidrBlockSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.peer_cidr_block_set", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_ipv6_cidr_block_set` after provisioning.\n"]
    pub fn peer_ipv6_cidr_block_set(
        &self,
    ) -> ListRef<DataVpcPeeringConnectionPeerIpv6CidrBlockSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.peer_ipv6_cidr_block_set", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_owner_id` after provisioning.\n"]
    pub fn peer_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_owner_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_region` after provisioning.\n"]
    pub fn peer_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_vpc_id` after provisioning.\n"]
    pub fn peer_vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_vpc_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> RecRef<PrimExpr<bool>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.requester", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `requester_region` after provisioning.\n"]
    pub fn requester_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requester_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcPeeringConnectionTimeoutsElRef {
        DataVpcPeeringConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DataVpcPeeringConnection {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataVpcPeeringConnection {}

impl ToListMappable for DataVpcPeeringConnection {
    type O = ListRef<DataVpcPeeringConnectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataVpcPeeringConnection_ {
    fn extract_datasource_type(&self) -> String {
        "aws_vpc_peering_connection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcPeeringConnection {
    pub tf_id: String,
}

impl BuildDataVpcPeeringConnection {
    pub fn build(self, stack: &mut Stack) -> DataVpcPeeringConnection {
        let out = DataVpcPeeringConnection(Rc::new(DataVpcPeeringConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVpcPeeringConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cidr_block: core::default::Default::default(),
                id: core::default::Default::default(),
                owner_id: core::default::Default::default(),
                peer_cidr_block: core::default::Default::default(),
                peer_owner_id: core::default::Default::default(),
                peer_vpc_id: core::default::Default::default(),
                status: core::default::Default::default(),
                tags: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpcPeeringConnectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcPeeringConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataVpcPeeringConnectionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `accepter` after provisioning.\n"]
    pub fn accepter(&self) -> RecRef<PrimExpr<bool>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.accepter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cidr_block_set` after provisioning.\n"]
    pub fn cidr_block_set(&self) -> ListRef<DataVpcPeeringConnectionCidrBlockSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cidr_block_set", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block_set` after provisioning.\n"]
    pub fn ipv6_cidr_block_set(&self) -> ListRef<DataVpcPeeringConnectionIpv6CidrBlockSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block_set", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_cidr_block` after provisioning.\n"]
    pub fn peer_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_cidr_block_set` after provisioning.\n"]
    pub fn peer_cidr_block_set(&self) -> ListRef<DataVpcPeeringConnectionPeerCidrBlockSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.peer_cidr_block_set", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_ipv6_cidr_block_set` after provisioning.\n"]
    pub fn peer_ipv6_cidr_block_set(
        &self,
    ) -> ListRef<DataVpcPeeringConnectionPeerIpv6CidrBlockSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.peer_ipv6_cidr_block_set", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_owner_id` after provisioning.\n"]
    pub fn peer_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_owner_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_region` after provisioning.\n"]
    pub fn peer_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_vpc_id` after provisioning.\n"]
    pub fn peer_vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_vpc_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> RecRef<PrimExpr<bool>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.requester", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `requester_region` after provisioning.\n"]
    pub fn requester_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requester_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcPeeringConnectionTimeoutsElRef {
        DataVpcPeeringConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnectionCidrBlockSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
}

impl DataVpcPeeringConnectionCidrBlockSetEl {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpcPeeringConnectionCidrBlockSetEl {
    type O = BlockAssignable<DataVpcPeeringConnectionCidrBlockSetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnectionCidrBlockSetEl {}

impl BuildDataVpcPeeringConnectionCidrBlockSetEl {
    pub fn build(self) -> DataVpcPeeringConnectionCidrBlockSetEl {
        DataVpcPeeringConnectionCidrBlockSetEl {
            cidr_block: core::default::Default::default(),
        }
    }
}

pub struct DataVpcPeeringConnectionCidrBlockSetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcPeeringConnectionCidrBlockSetElRef {
    fn new(shared: StackShared, base: String) -> DataVpcPeeringConnectionCidrBlockSetElRef {
        DataVpcPeeringConnectionCidrBlockSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnectionCidrBlockSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnectionIpv6CidrBlockSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
}

impl DataVpcPeeringConnectionIpv6CidrBlockSetEl {
    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_cidr_block = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpcPeeringConnectionIpv6CidrBlockSetEl {
    type O = BlockAssignable<DataVpcPeeringConnectionIpv6CidrBlockSetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnectionIpv6CidrBlockSetEl {}

impl BuildDataVpcPeeringConnectionIpv6CidrBlockSetEl {
    pub fn build(self) -> DataVpcPeeringConnectionIpv6CidrBlockSetEl {
        DataVpcPeeringConnectionIpv6CidrBlockSetEl {
            ipv6_cidr_block: core::default::Default::default(),
        }
    }
}

pub struct DataVpcPeeringConnectionIpv6CidrBlockSetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcPeeringConnectionIpv6CidrBlockSetElRef {
    fn new(shared: StackShared, base: String) -> DataVpcPeeringConnectionIpv6CidrBlockSetElRef {
        DataVpcPeeringConnectionIpv6CidrBlockSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnectionIpv6CidrBlockSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnectionPeerCidrBlockSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
}

impl DataVpcPeeringConnectionPeerCidrBlockSetEl {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpcPeeringConnectionPeerCidrBlockSetEl {
    type O = BlockAssignable<DataVpcPeeringConnectionPeerCidrBlockSetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnectionPeerCidrBlockSetEl {}

impl BuildDataVpcPeeringConnectionPeerCidrBlockSetEl {
    pub fn build(self) -> DataVpcPeeringConnectionPeerCidrBlockSetEl {
        DataVpcPeeringConnectionPeerCidrBlockSetEl {
            cidr_block: core::default::Default::default(),
        }
    }
}

pub struct DataVpcPeeringConnectionPeerCidrBlockSetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcPeeringConnectionPeerCidrBlockSetElRef {
    fn new(shared: StackShared, base: String) -> DataVpcPeeringConnectionPeerCidrBlockSetElRef {
        DataVpcPeeringConnectionPeerCidrBlockSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnectionPeerCidrBlockSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnectionPeerIpv6CidrBlockSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
}

impl DataVpcPeeringConnectionPeerIpv6CidrBlockSetEl {
    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_cidr_block = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpcPeeringConnectionPeerIpv6CidrBlockSetEl {
    type O = BlockAssignable<DataVpcPeeringConnectionPeerIpv6CidrBlockSetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnectionPeerIpv6CidrBlockSetEl {}

impl BuildDataVpcPeeringConnectionPeerIpv6CidrBlockSetEl {
    pub fn build(self) -> DataVpcPeeringConnectionPeerIpv6CidrBlockSetEl {
        DataVpcPeeringConnectionPeerIpv6CidrBlockSetEl {
            ipv6_cidr_block: core::default::Default::default(),
        }
    }
}

pub struct DataVpcPeeringConnectionPeerIpv6CidrBlockSetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcPeeringConnectionPeerIpv6CidrBlockSetElRef {
    fn new(shared: StackShared, base: String) -> DataVpcPeeringConnectionPeerIpv6CidrBlockSetElRef {
        DataVpcPeeringConnectionPeerIpv6CidrBlockSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnectionPeerIpv6CidrBlockSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnectionFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataVpcPeeringConnectionFilterEl {}

impl ToListMappable for DataVpcPeeringConnectionFilterEl {
    type O = BlockAssignable<DataVpcPeeringConnectionFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnectionFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataVpcPeeringConnectionFilterEl {
    pub fn build(self) -> DataVpcPeeringConnectionFilterEl {
        DataVpcPeeringConnectionFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataVpcPeeringConnectionFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcPeeringConnectionFilterElRef {
    fn new(shared: StackShared, base: String) -> DataVpcPeeringConnectionFilterElRef {
        DataVpcPeeringConnectionFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnectionFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataVpcPeeringConnectionTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpcPeeringConnectionTimeoutsEl {
    type O = BlockAssignable<DataVpcPeeringConnectionTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnectionTimeoutsEl {}

impl BuildDataVpcPeeringConnectionTimeoutsEl {
    pub fn build(self) -> DataVpcPeeringConnectionTimeoutsEl {
        DataVpcPeeringConnectionTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}

pub struct DataVpcPeeringConnectionTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcPeeringConnectionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataVpcPeeringConnectionTimeoutsElRef {
        DataVpcPeeringConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnectionTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataVpcPeeringConnectionDynamic {
    filter: Option<DynamicBlock<DataVpcPeeringConnectionFilterEl>>,
}
