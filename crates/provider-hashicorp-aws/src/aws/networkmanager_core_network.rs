use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkmanagerCoreNetworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_policy_document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_policy_regions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_base_policy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    global_network_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkmanagerCoreNetworkTimeoutsEl>,
}
struct NetworkmanagerCoreNetwork_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkmanagerCoreNetworkData>,
}
#[derive(Clone)]
pub struct NetworkmanagerCoreNetwork(Rc<NetworkmanagerCoreNetwork_>);
impl NetworkmanagerCoreNetwork {
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
    #[doc = "Set the field `base_policy_document`.\n"]
    pub fn set_base_policy_document(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().base_policy_document = Some(v.into());
        self
    }
    #[doc = "Set the field `base_policy_regions`.\n"]
    pub fn set_base_policy_regions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().base_policy_regions = Some(v.into());
        self
    }
    #[doc = "Set the field `create_base_policy`.\n"]
    pub fn set_create_base_policy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_base_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkmanagerCoreNetworkTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `base_policy_document` after provisioning.\n"]
    pub fn base_policy_document(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_policy_document", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `base_policy_regions` after provisioning.\n"]
    pub fn base_policy_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.base_policy_regions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_base_policy` after provisioning.\n"]
    pub fn create_base_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_base_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edges` after provisioning.\n"]
    pub fn edges(&self) -> ListRef<NetworkmanagerCoreNetworkEdgesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.edges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_network_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]
    pub fn segments(&self) -> ListRef<NetworkmanagerCoreNetworkSegmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.segments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerCoreNetworkTimeoutsElRef {
        NetworkmanagerCoreNetworkTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkmanagerCoreNetwork {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkmanagerCoreNetwork {}
impl ToListMappable for NetworkmanagerCoreNetwork {
    type O = ListRef<NetworkmanagerCoreNetworkRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkmanagerCoreNetwork_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkmanager_core_network".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkmanagerCoreNetwork {
    pub tf_id: String,
    #[doc = ""]
    pub global_network_id: PrimField<String>,
}
impl BuildNetworkmanagerCoreNetwork {
    pub fn build(self, stack: &mut Stack) -> NetworkmanagerCoreNetwork {
        let out = NetworkmanagerCoreNetwork(Rc::new(NetworkmanagerCoreNetwork_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkmanagerCoreNetworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                base_policy_document: core::default::Default::default(),
                base_policy_regions: core::default::Default::default(),
                create_base_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                global_network_id: self.global_network_id,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkmanagerCoreNetworkRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkmanagerCoreNetworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkmanagerCoreNetworkRef {
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
    #[doc = "Get a reference to the value of field `base_policy_document` after provisioning.\n"]
    pub fn base_policy_document(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_policy_document", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `base_policy_regions` after provisioning.\n"]
    pub fn base_policy_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.base_policy_regions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_base_policy` after provisioning.\n"]
    pub fn create_base_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_base_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edges` after provisioning.\n"]
    pub fn edges(&self) -> ListRef<NetworkmanagerCoreNetworkEdgesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.edges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_network_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]
    pub fn segments(&self) -> ListRef<NetworkmanagerCoreNetworkSegmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.segments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerCoreNetworkTimeoutsElRef {
        NetworkmanagerCoreNetworkTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkmanagerCoreNetworkEdgesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    asn: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inside_cidr_blocks: Option<ListField<PrimField<String>>>,
}
impl NetworkmanagerCoreNetworkEdgesEl {
    #[doc = "Set the field `asn`.\n"]
    pub fn set_asn(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.asn = Some(v.into());
        self
    }
    #[doc = "Set the field `edge_location`.\n"]
    pub fn set_edge_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.edge_location = Some(v.into());
        self
    }
    #[doc = "Set the field `inside_cidr_blocks`.\n"]
    pub fn set_inside_cidr_blocks(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.inside_cidr_blocks = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkmanagerCoreNetworkEdgesEl {
    type O = BlockAssignable<NetworkmanagerCoreNetworkEdgesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkmanagerCoreNetworkEdgesEl {}
impl BuildNetworkmanagerCoreNetworkEdgesEl {
    pub fn build(self) -> NetworkmanagerCoreNetworkEdgesEl {
        NetworkmanagerCoreNetworkEdgesEl {
            asn: core::default::Default::default(),
            edge_location: core::default::Default::default(),
            inside_cidr_blocks: core::default::Default::default(),
        }
    }
}
pub struct NetworkmanagerCoreNetworkEdgesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkmanagerCoreNetworkEdgesElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerCoreNetworkEdgesElRef {
        NetworkmanagerCoreNetworkEdgesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkmanagerCoreNetworkEdgesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asn` after provisioning.\n"]
    pub fn asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }
    #[doc = "Get a reference to the value of field `edge_location` after provisioning.\n"]
    pub fn edge_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edge_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inside_cidr_blocks", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkmanagerCoreNetworkSegmentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_locations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_segments: Option<ListField<PrimField<String>>>,
}
impl NetworkmanagerCoreNetworkSegmentsEl {
    #[doc = "Set the field `edge_locations`.\n"]
    pub fn set_edge_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.edge_locations = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `shared_segments`.\n"]
    pub fn set_shared_segments(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.shared_segments = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkmanagerCoreNetworkSegmentsEl {
    type O = BlockAssignable<NetworkmanagerCoreNetworkSegmentsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkmanagerCoreNetworkSegmentsEl {}
impl BuildNetworkmanagerCoreNetworkSegmentsEl {
    pub fn build(self) -> NetworkmanagerCoreNetworkSegmentsEl {
        NetworkmanagerCoreNetworkSegmentsEl {
            edge_locations: core::default::Default::default(),
            name: core::default::Default::default(),
            shared_segments: core::default::Default::default(),
        }
    }
}
pub struct NetworkmanagerCoreNetworkSegmentsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkmanagerCoreNetworkSegmentsElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerCoreNetworkSegmentsElRef {
        NetworkmanagerCoreNetworkSegmentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkmanagerCoreNetworkSegmentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `edge_locations` after provisioning.\n"]
    pub fn edge_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.edge_locations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `shared_segments` after provisioning.\n"]
    pub fn shared_segments(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shared_segments", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkmanagerCoreNetworkTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkmanagerCoreNetworkTimeoutsEl {
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
impl ToListMappable for NetworkmanagerCoreNetworkTimeoutsEl {
    type O = BlockAssignable<NetworkmanagerCoreNetworkTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkmanagerCoreNetworkTimeoutsEl {}
impl BuildNetworkmanagerCoreNetworkTimeoutsEl {
    pub fn build(self) -> NetworkmanagerCoreNetworkTimeoutsEl {
        NetworkmanagerCoreNetworkTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkmanagerCoreNetworkTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkmanagerCoreNetworkTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerCoreNetworkTimeoutsElRef {
        NetworkmanagerCoreNetworkTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkmanagerCoreNetworkTimeoutsElRef {
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
