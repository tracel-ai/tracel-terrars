use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VpcPeeringConnectionAccepterData {
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
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    vpc_peering_connection_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accepter: Option<Vec<VpcPeeringConnectionAccepterAccepterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requester: Option<Vec<VpcPeeringConnectionAccepterRequesterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpcPeeringConnectionAccepterTimeoutsEl>,
    dynamic: VpcPeeringConnectionAccepterDynamic,
}
struct VpcPeeringConnectionAccepter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcPeeringConnectionAccepterData>,
}
#[derive(Clone)]
pub struct VpcPeeringConnectionAccepter(Rc<VpcPeeringConnectionAccepter_>);
impl VpcPeeringConnectionAccepter {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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
    pub fn set_accepter(
        self,
        v: impl Into<BlockAssignable<VpcPeeringConnectionAccepterAccepterEl>>,
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
        v: impl Into<BlockAssignable<VpcPeeringConnectionAccepterRequesterEl>>,
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpcPeeringConnectionAccepterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `accept_status` after provisioning.\n"]
    pub fn accept_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accept_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_accept` after provisioning.\n"]
    pub fn auto_accept(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_accept", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
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
    pub fn accepter(&self) -> ListRef<VpcPeeringConnectionAccepterAccepterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accepter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> ListRef<VpcPeeringConnectionAccepterRequesterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.requester", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcPeeringConnectionAccepterTimeoutsElRef {
        VpcPeeringConnectionAccepterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VpcPeeringConnectionAccepter {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VpcPeeringConnectionAccepter {}
impl ToListMappable for VpcPeeringConnectionAccepter {
    type O = ListRef<VpcPeeringConnectionAccepterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VpcPeeringConnectionAccepter_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpc_peering_connection_accepter".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVpcPeeringConnectionAccepter {
    pub tf_id: String,
    #[doc = ""]
    pub vpc_peering_connection_id: PrimField<String>,
}
impl BuildVpcPeeringConnectionAccepter {
    pub fn build(self, stack: &mut Stack) -> VpcPeeringConnectionAccepter {
        let out = VpcPeeringConnectionAccepter(Rc::new(VpcPeeringConnectionAccepter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpcPeeringConnectionAccepterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_accept: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_peering_connection_id: self.vpc_peering_connection_id,
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
pub struct VpcPeeringConnectionAccepterRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcPeeringConnectionAccepterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VpcPeeringConnectionAccepterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accept_status` after provisioning.\n"]
    pub fn accept_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accept_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_accept` after provisioning.\n"]
    pub fn auto_accept(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_accept", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
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
    pub fn accepter(&self) -> ListRef<VpcPeeringConnectionAccepterAccepterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accepter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> ListRef<VpcPeeringConnectionAccepterRequesterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.requester", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcPeeringConnectionAccepterTimeoutsElRef {
        VpcPeeringConnectionAccepterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VpcPeeringConnectionAccepterAccepterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_remote_vpc_dns_resolution: Option<PrimField<bool>>,
}
impl VpcPeeringConnectionAccepterAccepterEl {
    #[doc = "Set the field `allow_remote_vpc_dns_resolution`.\n"]
    pub fn set_allow_remote_vpc_dns_resolution(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_remote_vpc_dns_resolution = Some(v.into());
        self
    }
}
impl ToListMappable for VpcPeeringConnectionAccepterAccepterEl {
    type O = BlockAssignable<VpcPeeringConnectionAccepterAccepterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpcPeeringConnectionAccepterAccepterEl {}
impl BuildVpcPeeringConnectionAccepterAccepterEl {
    pub fn build(self) -> VpcPeeringConnectionAccepterAccepterEl {
        VpcPeeringConnectionAccepterAccepterEl {
            allow_remote_vpc_dns_resolution: core::default::Default::default(),
        }
    }
}
pub struct VpcPeeringConnectionAccepterAccepterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcPeeringConnectionAccepterAccepterElRef {
    fn new(shared: StackShared, base: String) -> VpcPeeringConnectionAccepterAccepterElRef {
        VpcPeeringConnectionAccepterAccepterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpcPeeringConnectionAccepterAccepterElRef {
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
pub struct VpcPeeringConnectionAccepterRequesterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_remote_vpc_dns_resolution: Option<PrimField<bool>>,
}
impl VpcPeeringConnectionAccepterRequesterEl {
    #[doc = "Set the field `allow_remote_vpc_dns_resolution`.\n"]
    pub fn set_allow_remote_vpc_dns_resolution(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_remote_vpc_dns_resolution = Some(v.into());
        self
    }
}
impl ToListMappable for VpcPeeringConnectionAccepterRequesterEl {
    type O = BlockAssignable<VpcPeeringConnectionAccepterRequesterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpcPeeringConnectionAccepterRequesterEl {}
impl BuildVpcPeeringConnectionAccepterRequesterEl {
    pub fn build(self) -> VpcPeeringConnectionAccepterRequesterEl {
        VpcPeeringConnectionAccepterRequesterEl {
            allow_remote_vpc_dns_resolution: core::default::Default::default(),
        }
    }
}
pub struct VpcPeeringConnectionAccepterRequesterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcPeeringConnectionAccepterRequesterElRef {
    fn new(shared: StackShared, base: String) -> VpcPeeringConnectionAccepterRequesterElRef {
        VpcPeeringConnectionAccepterRequesterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpcPeeringConnectionAccepterRequesterElRef {
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
pub struct VpcPeeringConnectionAccepterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VpcPeeringConnectionAccepterTimeoutsEl {
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
impl ToListMappable for VpcPeeringConnectionAccepterTimeoutsEl {
    type O = BlockAssignable<VpcPeeringConnectionAccepterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpcPeeringConnectionAccepterTimeoutsEl {}
impl BuildVpcPeeringConnectionAccepterTimeoutsEl {
    pub fn build(self) -> VpcPeeringConnectionAccepterTimeoutsEl {
        VpcPeeringConnectionAccepterTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VpcPeeringConnectionAccepterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcPeeringConnectionAccepterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpcPeeringConnectionAccepterTimeoutsElRef {
        VpcPeeringConnectionAccepterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpcPeeringConnectionAccepterTimeoutsElRef {
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
struct VpcPeeringConnectionAccepterDynamic {
    accepter: Option<DynamicBlock<VpcPeeringConnectionAccepterAccepterEl>>,
    requester: Option<DynamicBlock<VpcPeeringConnectionAccepterRequesterEl>>,
}
