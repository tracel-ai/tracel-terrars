use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkmanagerDxGatewayAttachmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    core_network_id: PrimField<String>,
    direct_connect_gateway_arn: PrimField<String>,
    edge_locations: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkmanagerDxGatewayAttachmentTimeoutsEl>,
}

struct NetworkmanagerDxGatewayAttachment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkmanagerDxGatewayAttachmentData>,
}

#[derive(Clone)]
pub struct NetworkmanagerDxGatewayAttachment(Rc<NetworkmanagerDxGatewayAttachment_>);

impl NetworkmanagerDxGatewayAttachment {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkmanagerDxGatewayAttachmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `direct_connect_gateway_arn` after provisioning.\n"]
    pub fn direct_connect_gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direct_connect_gateway_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `segment_name` after provisioning.\n"]
    pub fn segment_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.segment_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerDxGatewayAttachmentTimeoutsElRef {
        NetworkmanagerDxGatewayAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for NetworkmanagerDxGatewayAttachment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NetworkmanagerDxGatewayAttachment { }

impl ToListMappable for NetworkmanagerDxGatewayAttachment {
    type O = ListRef<NetworkmanagerDxGatewayAttachmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkmanagerDxGatewayAttachment_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkmanager_dx_gateway_attachment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkmanagerDxGatewayAttachment {
    pub tf_id: String,
    #[doc = ""]
    pub core_network_id: PrimField<String>,
    #[doc = ""]
    pub direct_connect_gateway_arn: PrimField<String>,
    #[doc = ""]
    pub edge_locations: ListField<PrimField<String>>,
}

impl BuildNetworkmanagerDxGatewayAttachment {
    pub fn build(self, stack: &mut Stack) -> NetworkmanagerDxGatewayAttachment {
        let out = NetworkmanagerDxGatewayAttachment(Rc::new(NetworkmanagerDxGatewayAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkmanagerDxGatewayAttachmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                core_network_id: self.core_network_id,
                direct_connect_gateway_arn: self.direct_connect_gateway_arn,
                edge_locations: self.edge_locations,
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkmanagerDxGatewayAttachmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerDxGatewayAttachmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl NetworkmanagerDxGatewayAttachmentRef {
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

    #[doc = "Get a reference to the value of field `direct_connect_gateway_arn` after provisioning.\n"]
    pub fn direct_connect_gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direct_connect_gateway_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `segment_name` after provisioning.\n"]
    pub fn segment_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.segment_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerDxGatewayAttachmentTimeoutsElRef {
        NetworkmanagerDxGatewayAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerDxGatewayAttachmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl NetworkmanagerDxGatewayAttachmentTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkmanagerDxGatewayAttachmentTimeoutsEl {
    type O = BlockAssignable<NetworkmanagerDxGatewayAttachmentTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerDxGatewayAttachmentTimeoutsEl {}

impl BuildNetworkmanagerDxGatewayAttachmentTimeoutsEl {
    pub fn build(self) -> NetworkmanagerDxGatewayAttachmentTimeoutsEl {
        NetworkmanagerDxGatewayAttachmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct NetworkmanagerDxGatewayAttachmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerDxGatewayAttachmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerDxGatewayAttachmentTimeoutsElRef {
        NetworkmanagerDxGatewayAttachmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerDxGatewayAttachmentTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
