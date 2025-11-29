use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkmanagerConnectPeerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    connect_attachment_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_network_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inside_cidr_blocks: Option<ListField<PrimField<String>>>,
    peer_address: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bgp_options: Option<Vec<NetworkmanagerConnectPeerBgpOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkmanagerConnectPeerTimeoutsEl>,
    dynamic: NetworkmanagerConnectPeerDynamic,
}

struct NetworkmanagerConnectPeer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkmanagerConnectPeerData>,
}

#[derive(Clone)]
pub struct NetworkmanagerConnectPeer(Rc<NetworkmanagerConnectPeer_>);

impl NetworkmanagerConnectPeer {
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

    #[doc = "Set the field `core_network_address`.\n"]
    pub fn set_core_network_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().core_network_address = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `inside_cidr_blocks`.\n"]
    pub fn set_inside_cidr_blocks(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().inside_cidr_blocks = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_arn`.\n"]
    pub fn set_subnet_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnet_arn = Some(v.into());
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

    #[doc = "Set the field `bgp_options`.\n"]
    pub fn set_bgp_options(self, v: impl Into<BlockAssignable<NetworkmanagerConnectPeerBgpOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bgp_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bgp_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkmanagerConnectPeerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<NetworkmanagerConnectPeerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_attachment_id` after provisioning.\n"]
    pub fn connect_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connect_attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_peer_id` after provisioning.\n"]
    pub fn connect_peer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connect_peer_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_address` after provisioning.\n"]
    pub fn core_network_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_id` after provisioning.\n"]
    pub fn core_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edge_location` after provisioning.\n"]
    pub fn edge_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edge_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.inside_cidr_blocks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_arn` after provisioning.\n"]
    pub fn subnet_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_options` after provisioning.\n"]
    pub fn bgp_options(&self) -> ListRef<NetworkmanagerConnectPeerBgpOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bgp_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerConnectPeerTimeoutsElRef {
        NetworkmanagerConnectPeerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for NetworkmanagerConnectPeer {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NetworkmanagerConnectPeer { }

impl ToListMappable for NetworkmanagerConnectPeer {
    type O = ListRef<NetworkmanagerConnectPeerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkmanagerConnectPeer_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkmanager_connect_peer".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkmanagerConnectPeer {
    pub tf_id: String,
    #[doc = ""]
    pub connect_attachment_id: PrimField<String>,
    #[doc = ""]
    pub peer_address: PrimField<String>,
}

impl BuildNetworkmanagerConnectPeer {
    pub fn build(self, stack: &mut Stack) -> NetworkmanagerConnectPeer {
        let out = NetworkmanagerConnectPeer(Rc::new(NetworkmanagerConnectPeer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkmanagerConnectPeerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                connect_attachment_id: self.connect_attachment_id,
                core_network_address: core::default::Default::default(),
                id: core::default::Default::default(),
                inside_cidr_blocks: core::default::Default::default(),
                peer_address: self.peer_address,
                subnet_arn: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                bgp_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkmanagerConnectPeerRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerConnectPeerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl NetworkmanagerConnectPeerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<NetworkmanagerConnectPeerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_attachment_id` after provisioning.\n"]
    pub fn connect_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connect_attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_peer_id` after provisioning.\n"]
    pub fn connect_peer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connect_peer_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_address` after provisioning.\n"]
    pub fn core_network_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_id` after provisioning.\n"]
    pub fn core_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edge_location` after provisioning.\n"]
    pub fn edge_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edge_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.inside_cidr_blocks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_arn` after provisioning.\n"]
    pub fn subnet_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_options` after provisioning.\n"]
    pub fn bgp_options(&self) -> ListRef<NetworkmanagerConnectPeerBgpOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bgp_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerConnectPeerTimeoutsElRef {
        NetworkmanagerConnectPeerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerConnectPeerConfigurationElBgpConfigurationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    core_network_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_network_asn: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_asn: Option<PrimField<f64>>,
}

impl NetworkmanagerConnectPeerConfigurationElBgpConfigurationsEl {
    #[doc = "Set the field `core_network_address`.\n"]
    pub fn set_core_network_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.core_network_address = Some(v.into());
        self
    }

    #[doc = "Set the field `core_network_asn`.\n"]
    pub fn set_core_network_asn(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.core_network_asn = Some(v.into());
        self
    }

    #[doc = "Set the field `peer_address`.\n"]
    pub fn set_peer_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_address = Some(v.into());
        self
    }

    #[doc = "Set the field `peer_asn`.\n"]
    pub fn set_peer_asn(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.peer_asn = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkmanagerConnectPeerConfigurationElBgpConfigurationsEl {
    type O = BlockAssignable<NetworkmanagerConnectPeerConfigurationElBgpConfigurationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerConnectPeerConfigurationElBgpConfigurationsEl {}

impl BuildNetworkmanagerConnectPeerConfigurationElBgpConfigurationsEl {
    pub fn build(self) -> NetworkmanagerConnectPeerConfigurationElBgpConfigurationsEl {
        NetworkmanagerConnectPeerConfigurationElBgpConfigurationsEl {
            core_network_address: core::default::Default::default(),
            core_network_asn: core::default::Default::default(),
            peer_address: core::default::Default::default(),
            peer_asn: core::default::Default::default(),
        }
    }
}

pub struct NetworkmanagerConnectPeerConfigurationElBgpConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerConnectPeerConfigurationElBgpConfigurationsElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerConnectPeerConfigurationElBgpConfigurationsElRef {
        NetworkmanagerConnectPeerConfigurationElBgpConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerConnectPeerConfigurationElBgpConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `core_network_address` after provisioning.\n"]
    pub fn core_network_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_address", self.base))
    }

    #[doc = "Get a reference to the value of field `core_network_asn` after provisioning.\n"]
    pub fn core_network_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_asn", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_asn` after provisioning.\n"]
    pub fn peer_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_asn", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerConnectPeerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bgp_configurations: Option<ListField<NetworkmanagerConnectPeerConfigurationElBgpConfigurationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_network_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inside_cidr_blocks: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
}

impl NetworkmanagerConnectPeerConfigurationEl {
    #[doc = "Set the field `bgp_configurations`.\n"]
    pub fn set_bgp_configurations(
        mut self,
        v: impl Into<ListField<NetworkmanagerConnectPeerConfigurationElBgpConfigurationsEl>>,
    ) -> Self {
        self.bgp_configurations = Some(v.into());
        self
    }

    #[doc = "Set the field `core_network_address`.\n"]
    pub fn set_core_network_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.core_network_address = Some(v.into());
        self
    }

    #[doc = "Set the field `inside_cidr_blocks`.\n"]
    pub fn set_inside_cidr_blocks(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.inside_cidr_blocks = Some(v.into());
        self
    }

    #[doc = "Set the field `peer_address`.\n"]
    pub fn set_peer_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_address = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkmanagerConnectPeerConfigurationEl {
    type O = BlockAssignable<NetworkmanagerConnectPeerConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerConnectPeerConfigurationEl {}

impl BuildNetworkmanagerConnectPeerConfigurationEl {
    pub fn build(self) -> NetworkmanagerConnectPeerConfigurationEl {
        NetworkmanagerConnectPeerConfigurationEl {
            bgp_configurations: core::default::Default::default(),
            core_network_address: core::default::Default::default(),
            inside_cidr_blocks: core::default::Default::default(),
            peer_address: core::default::Default::default(),
            protocol: core::default::Default::default(),
        }
    }
}

pub struct NetworkmanagerConnectPeerConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerConnectPeerConfigurationElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerConnectPeerConfigurationElRef {
        NetworkmanagerConnectPeerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerConnectPeerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bgp_configurations` after provisioning.\n"]
    pub fn bgp_configurations(&self) -> ListRef<NetworkmanagerConnectPeerConfigurationElBgpConfigurationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bgp_configurations", self.base))
    }

    #[doc = "Get a reference to the value of field `core_network_address` after provisioning.\n"]
    pub fn core_network_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_address", self.base))
    }

    #[doc = "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.inside_cidr_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerConnectPeerBgpOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_asn: Option<PrimField<f64>>,
}

impl NetworkmanagerConnectPeerBgpOptionsEl {
    #[doc = "Set the field `peer_asn`.\n"]
    pub fn set_peer_asn(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.peer_asn = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkmanagerConnectPeerBgpOptionsEl {
    type O = BlockAssignable<NetworkmanagerConnectPeerBgpOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerConnectPeerBgpOptionsEl {}

impl BuildNetworkmanagerConnectPeerBgpOptionsEl {
    pub fn build(self) -> NetworkmanagerConnectPeerBgpOptionsEl {
        NetworkmanagerConnectPeerBgpOptionsEl { peer_asn: core::default::Default::default() }
    }
}

pub struct NetworkmanagerConnectPeerBgpOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerConnectPeerBgpOptionsElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerConnectPeerBgpOptionsElRef {
        NetworkmanagerConnectPeerBgpOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerConnectPeerBgpOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `peer_asn` after provisioning.\n"]
    pub fn peer_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_asn", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerConnectPeerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl NetworkmanagerConnectPeerTimeoutsEl {
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

impl ToListMappable for NetworkmanagerConnectPeerTimeoutsEl {
    type O = BlockAssignable<NetworkmanagerConnectPeerTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerConnectPeerTimeoutsEl {}

impl BuildNetworkmanagerConnectPeerTimeoutsEl {
    pub fn build(self) -> NetworkmanagerConnectPeerTimeoutsEl {
        NetworkmanagerConnectPeerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct NetworkmanagerConnectPeerTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerConnectPeerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerConnectPeerTimeoutsElRef {
        NetworkmanagerConnectPeerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerConnectPeerTimeoutsElRef {
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
struct NetworkmanagerConnectPeerDynamic {
    bgp_options: Option<DynamicBlock<NetworkmanagerConnectPeerBgpOptionsEl>>,
}
