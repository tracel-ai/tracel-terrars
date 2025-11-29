use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkmanagerAttachmentAccepterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    attachment_id: PrimField<String>,
    attachment_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkmanagerAttachmentAccepterTimeoutsEl>,
}

struct NetworkmanagerAttachmentAccepter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkmanagerAttachmentAccepterData>,
}

#[derive(Clone)]
pub struct NetworkmanagerAttachmentAccepter(Rc<NetworkmanagerAttachmentAccepter_>);

impl NetworkmanagerAttachmentAccepter {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkmanagerAttachmentAccepterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_policy_rule_number` after provisioning.\n"]
    pub fn attachment_policy_rule_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachment_policy_rule_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_type` after provisioning.\n"]
    pub fn attachment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_id` after provisioning.\n"]
    pub fn core_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edge_location` after provisioning.\n"]
    pub fn edge_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edge_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edge_locations` after provisioning.\n"]
    pub fn edge_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.edge_locations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `segment_name` after provisioning.\n"]
    pub fn segment_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.segment_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerAttachmentAccepterTimeoutsElRef {
        NetworkmanagerAttachmentAccepterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for NetworkmanagerAttachmentAccepter {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NetworkmanagerAttachmentAccepter { }

impl ToListMappable for NetworkmanagerAttachmentAccepter {
    type O = ListRef<NetworkmanagerAttachmentAccepterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkmanagerAttachmentAccepter_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkmanager_attachment_accepter".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkmanagerAttachmentAccepter {
    pub tf_id: String,
    #[doc = ""]
    pub attachment_id: PrimField<String>,
    #[doc = ""]
    pub attachment_type: PrimField<String>,
}

impl BuildNetworkmanagerAttachmentAccepter {
    pub fn build(self, stack: &mut Stack) -> NetworkmanagerAttachmentAccepter {
        let out = NetworkmanagerAttachmentAccepter(Rc::new(NetworkmanagerAttachmentAccepter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkmanagerAttachmentAccepterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                attachment_id: self.attachment_id,
                attachment_type: self.attachment_type,
                id: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkmanagerAttachmentAccepterRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerAttachmentAccepterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl NetworkmanagerAttachmentAccepterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_policy_rule_number` after provisioning.\n"]
    pub fn attachment_policy_rule_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachment_policy_rule_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_type` after provisioning.\n"]
    pub fn attachment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_id` after provisioning.\n"]
    pub fn core_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edge_location` after provisioning.\n"]
    pub fn edge_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edge_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edge_locations` after provisioning.\n"]
    pub fn edge_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.edge_locations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `segment_name` after provisioning.\n"]
    pub fn segment_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.segment_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerAttachmentAccepterTimeoutsElRef {
        NetworkmanagerAttachmentAccepterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerAttachmentAccepterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl NetworkmanagerAttachmentAccepterTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkmanagerAttachmentAccepterTimeoutsEl {
    type O = BlockAssignable<NetworkmanagerAttachmentAccepterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerAttachmentAccepterTimeoutsEl {}

impl BuildNetworkmanagerAttachmentAccepterTimeoutsEl {
    pub fn build(self) -> NetworkmanagerAttachmentAccepterTimeoutsEl {
        NetworkmanagerAttachmentAccepterTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct NetworkmanagerAttachmentAccepterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerAttachmentAccepterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerAttachmentAccepterTimeoutsElRef {
        NetworkmanagerAttachmentAccepterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerAttachmentAccepterTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
