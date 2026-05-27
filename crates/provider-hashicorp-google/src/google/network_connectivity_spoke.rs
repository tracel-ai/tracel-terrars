use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkConnectivitySpokeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<PrimField<String>>,
    hub: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linked_interconnect_attachments:
        Option<Vec<NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linked_producer_vpc_network: Option<Vec<NetworkConnectivitySpokeLinkedProducerVpcNetworkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linked_router_appliance_instances:
        Option<Vec<NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linked_vpc_network: Option<Vec<NetworkConnectivitySpokeLinkedVpcNetworkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linked_vpn_tunnels: Option<Vec<NetworkConnectivitySpokeLinkedVpnTunnelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkConnectivitySpokeTimeoutsEl>,
    dynamic: NetworkConnectivitySpokeDynamic,
}
struct NetworkConnectivitySpoke_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkConnectivitySpokeData>,
}
#[derive(Clone)]
pub struct NetworkConnectivitySpoke(Rc<NetworkConnectivitySpoke_>);
impl NetworkConnectivitySpoke {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of the spoke."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `group`.\nThe name of the group that this spoke is associated with."]
    pub fn set_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().group = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional labels in key:value format. For more information about labels, see [Requirements for labels](https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `linked_interconnect_attachments`.\n"]
    pub fn set_linked_interconnect_attachments(
        self,
        v: impl Into<BlockAssignable<NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().linked_interconnect_attachments = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .linked_interconnect_attachments = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `linked_producer_vpc_network`.\n"]
    pub fn set_linked_producer_vpc_network(
        self,
        v: impl Into<BlockAssignable<NetworkConnectivitySpokeLinkedProducerVpcNetworkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().linked_producer_vpc_network = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.linked_producer_vpc_network = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `linked_router_appliance_instances`.\n"]
    pub fn set_linked_router_appliance_instances(
        self,
        v: impl Into<BlockAssignable<NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().linked_router_appliance_instances = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .linked_router_appliance_instances = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `linked_vpc_network`.\n"]
    pub fn set_linked_vpc_network(
        self,
        v: impl Into<BlockAssignable<NetworkConnectivitySpokeLinkedVpcNetworkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().linked_vpc_network = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.linked_vpc_network = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `linked_vpn_tunnels`.\n"]
    pub fn set_linked_vpn_tunnels(
        self,
        v: impl Into<BlockAssignable<NetworkConnectivitySpokeLinkedVpnTunnelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().linked_vpn_tunnels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.linked_vpn_tunnels = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkConnectivitySpokeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time the spoke was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of the spoke."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group` after provisioning.\nThe name of the group that this spoke is associated with."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hub` after provisioning.\nImmutable. The URI of the hub that this spoke is attached to."]
    pub fn hub(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hub", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional labels in key:value format. For more information about labels, see [Requirements for labels](https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. The name of the spoke. Spoke names must be unique."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reasons` after provisioning.\nThe reasons for the current state in the lifecycle"]
    pub fn reasons(&self) -> ListRef<NetworkConnectivitySpokeReasonsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reasons", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current lifecycle state of this spoke."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unique_id` after provisioning.\nOutput only. The Google-generated UUID for the spoke. This value is unique across all spoke resources. If a spoke is deleted and another with the same name is created, the new spoke is assigned a different unique_id."]
    pub fn unique_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unique_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time the spoke was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_interconnect_attachments` after provisioning.\n"]
    pub fn linked_interconnect_attachments(
        &self,
    ) -> ListRef<NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_interconnect_attachments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_producer_vpc_network` after provisioning.\n"]
    pub fn linked_producer_vpc_network(
        &self,
    ) -> ListRef<NetworkConnectivitySpokeLinkedProducerVpcNetworkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_producer_vpc_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_router_appliance_instances` after provisioning.\n"]
    pub fn linked_router_appliance_instances(
        &self,
    ) -> ListRef<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_router_appliance_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_vpc_network` after provisioning.\n"]
    pub fn linked_vpc_network(&self) -> ListRef<NetworkConnectivitySpokeLinkedVpcNetworkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_vpc_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_vpn_tunnels` after provisioning.\n"]
    pub fn linked_vpn_tunnels(&self) -> ListRef<NetworkConnectivitySpokeLinkedVpnTunnelsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_vpn_tunnels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivitySpokeTimeoutsElRef {
        NetworkConnectivitySpokeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkConnectivitySpoke {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkConnectivitySpoke {}
impl ToListMappable for NetworkConnectivitySpoke {
    type O = ListRef<NetworkConnectivitySpokeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkConnectivitySpoke_ {
    fn extract_resource_type(&self) -> String {
        "google_network_connectivity_spoke".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkConnectivitySpoke {
    pub tf_id: String,
    #[doc = "Immutable. The URI of the hub that this spoke is attached to."]
    pub hub: PrimField<String>,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "Immutable. The name of the spoke. Spoke names must be unique."]
    pub name: PrimField<String>,
}
impl BuildNetworkConnectivitySpoke {
    pub fn build(self, stack: &mut Stack) -> NetworkConnectivitySpoke {
        let out = NetworkConnectivitySpoke(Rc::new(NetworkConnectivitySpoke_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkConnectivitySpokeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                group: core::default::Default::default(),
                hub: self.hub,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                linked_interconnect_attachments: core::default::Default::default(),
                linked_producer_vpc_network: core::default::Default::default(),
                linked_router_appliance_instances: core::default::Default::default(),
                linked_vpc_network: core::default::Default::default(),
                linked_vpn_tunnels: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkConnectivitySpokeRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivitySpokeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkConnectivitySpokeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time the spoke was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of the spoke."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group` after provisioning.\nThe name of the group that this spoke is associated with."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hub` after provisioning.\nImmutable. The URI of the hub that this spoke is attached to."]
    pub fn hub(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hub", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional labels in key:value format. For more information about labels, see [Requirements for labels](https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. The name of the spoke. Spoke names must be unique."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reasons` after provisioning.\nThe reasons for the current state in the lifecycle"]
    pub fn reasons(&self) -> ListRef<NetworkConnectivitySpokeReasonsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reasons", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current lifecycle state of this spoke."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unique_id` after provisioning.\nOutput only. The Google-generated UUID for the spoke. This value is unique across all spoke resources. If a spoke is deleted and another with the same name is created, the new spoke is assigned a different unique_id."]
    pub fn unique_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unique_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time the spoke was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_interconnect_attachments` after provisioning.\n"]
    pub fn linked_interconnect_attachments(
        &self,
    ) -> ListRef<NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_interconnect_attachments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_producer_vpc_network` after provisioning.\n"]
    pub fn linked_producer_vpc_network(
        &self,
    ) -> ListRef<NetworkConnectivitySpokeLinkedProducerVpcNetworkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_producer_vpc_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_router_appliance_instances` after provisioning.\n"]
    pub fn linked_router_appliance_instances(
        &self,
    ) -> ListRef<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_router_appliance_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_vpc_network` after provisioning.\n"]
    pub fn linked_vpc_network(&self) -> ListRef<NetworkConnectivitySpokeLinkedVpcNetworkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_vpc_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_vpn_tunnels` after provisioning.\n"]
    pub fn linked_vpn_tunnels(&self) -> ListRef<NetworkConnectivitySpokeLinkedVpnTunnelsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_vpn_tunnels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivitySpokeTimeoutsElRef {
        NetworkConnectivitySpokeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivitySpokeReasonsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_details: Option<PrimField<String>>,
}
impl NetworkConnectivitySpokeReasonsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `user_details`.\n"]
    pub fn set_user_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_details = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivitySpokeReasonsEl {
    type O = BlockAssignable<NetworkConnectivitySpokeReasonsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivitySpokeReasonsEl {}
impl BuildNetworkConnectivitySpokeReasonsEl {
    pub fn build(self) -> NetworkConnectivitySpokeReasonsEl {
        NetworkConnectivitySpokeReasonsEl {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
            user_details: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivitySpokeReasonsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivitySpokeReasonsElRef {
    fn new(shared: StackShared, base: String) -> NetworkConnectivitySpokeReasonsElRef {
        NetworkConnectivitySpokeReasonsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivitySpokeReasonsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `user_details` after provisioning.\n"]
    pub fn user_details(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_details", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_export_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_import_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_export_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_import_ranges: Option<ListField<PrimField<String>>>,
    site_to_site_data_transfer: PrimField<bool>,
    uris: ListField<PrimField<String>>,
}
impl NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
    #[doc = "Set the field `exclude_export_ranges`.\nDynamic routes overlapped/encompassed by exclude export ranges are excluded during export to hub."]
    pub fn set_exclude_export_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_export_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_import_ranges`.\nHub routes overlapped/encompassed by exclude import ranges are excluded during import from hub."]
    pub fn set_exclude_import_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_import_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `include_export_ranges`.\nDynamic routes fully encompassed by include export ranges are included during export to hub."]
    pub fn set_include_export_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_export_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `include_import_ranges`.\nHub routes fully encompassed by include import ranges are included during import from hub.\n\"ALL_IPV4_RANGES\" or IPv4 CIDR ranges are allowed."]
    pub fn set_include_import_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_import_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
    type O = BlockAssignable<NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
    #[doc = "A value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub site_to_site_data_transfer: PrimField<bool>,
    #[doc = "The URIs of linked interconnect attachment resources"]
    pub uris: ListField<PrimField<String>>,
}
impl BuildNetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
    pub fn build(self) -> NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
        NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl {
            exclude_export_ranges: core::default::Default::default(),
            exclude_import_ranges: core::default::Default::default(),
            include_export_ranges: core::default::Default::default(),
            include_import_ranges: core::default::Default::default(),
            site_to_site_data_transfer: self.site_to_site_data_transfer,
            uris: self.uris,
        }
    }
}
pub struct NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef {
        NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivitySpokeLinkedInterconnectAttachmentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclude_export_ranges` after provisioning.\nDynamic routes overlapped/encompassed by exclude export ranges are excluded during export to hub."]
    pub fn exclude_export_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_export_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_import_ranges` after provisioning.\nHub routes overlapped/encompassed by exclude import ranges are excluded during import from hub."]
    pub fn exclude_import_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_import_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_export_ranges` after provisioning.\nDynamic routes fully encompassed by include export ranges are included during export to hub."]
    pub fn include_export_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_export_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_import_ranges` after provisioning.\nHub routes fully encompassed by include import ranges are included during import from hub.\n\"ALL_IPV4_RANGES\" or IPv4 CIDR ranges are allowed."]
    pub fn include_import_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_import_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `site_to_site_data_transfer` after provisioning.\nA value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub fn site_to_site_data_transfer(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.site_to_site_data_transfer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uris` after provisioning.\nThe URIs of linked interconnect attachment resources"]
    pub fn uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivitySpokeLinkedProducerVpcNetworkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_export_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_export_ranges: Option<ListField<PrimField<String>>>,
    network: PrimField<String>,
    peering: PrimField<String>,
}
impl NetworkConnectivitySpokeLinkedProducerVpcNetworkEl {
    #[doc = "Set the field `exclude_export_ranges`.\nIP ranges encompassing the subnets to be excluded from peering."]
    pub fn set_exclude_export_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_export_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `include_export_ranges`.\nIP ranges allowed to be included from peering."]
    pub fn set_include_export_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_export_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivitySpokeLinkedProducerVpcNetworkEl {
    type O = BlockAssignable<NetworkConnectivitySpokeLinkedProducerVpcNetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivitySpokeLinkedProducerVpcNetworkEl {
    #[doc = "The URI of the Service Consumer VPC that the Producer VPC is peered with."]
    pub network: PrimField<String>,
    #[doc = "The name of the VPC peering between the Service Consumer VPC and the Producer VPC (defined in the Tenant project) which is added to the NCC hub. This peering must be in ACTIVE state."]
    pub peering: PrimField<String>,
}
impl BuildNetworkConnectivitySpokeLinkedProducerVpcNetworkEl {
    pub fn build(self) -> NetworkConnectivitySpokeLinkedProducerVpcNetworkEl {
        NetworkConnectivitySpokeLinkedProducerVpcNetworkEl {
            exclude_export_ranges: core::default::Default::default(),
            include_export_ranges: core::default::Default::default(),
            network: self.network,
            peering: self.peering,
        }
    }
}
pub struct NetworkConnectivitySpokeLinkedProducerVpcNetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivitySpokeLinkedProducerVpcNetworkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivitySpokeLinkedProducerVpcNetworkElRef {
        NetworkConnectivitySpokeLinkedProducerVpcNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivitySpokeLinkedProducerVpcNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclude_export_ranges` after provisioning.\nIP ranges encompassing the subnets to be excluded from peering."]
    pub fn exclude_export_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_export_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_export_ranges` after provisioning.\nIP ranges allowed to be included from peering."]
    pub fn include_export_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_export_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe URI of the Service Consumer VPC that the Producer VPC is peered with."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `peering` after provisioning.\nThe name of the VPC peering between the Service Consumer VPC and the Producer VPC (defined in the Tenant project) which is added to the NCC hub. This peering must be in ACTIVE state."]
    pub fn peering(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peering", self.base))
    }
    #[doc = "Get a reference to the value of field `producer_network` after provisioning.\nThe URI of the Producer VPC."]
    pub fn producer_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.producer_network", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {
    ip_address: PrimField<String>,
    virtual_machine: PrimField<String>,
}
impl NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {}
impl ToListMappable for NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {
    type O = BlockAssignable<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {
    #[doc = "The IP address on the VM to use for peering."]
    pub ip_address: PrimField<String>,
    #[doc = "The URI of the virtual machine resource"]
    pub virtual_machine: PrimField<String>,
}
impl BuildNetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {
    pub fn build(self) -> NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {
        NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl {
            ip_address: self.ip_address,
            virtual_machine: self.virtual_machine,
        }
    }
}
pub struct NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesElRef {
        NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address on the VM to use for peering."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `virtual_machine` after provisioning.\nThe URI of the virtual machine resource"]
    pub fn virtual_machine(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_machine", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkConnectivitySpokeLinkedRouterApplianceInstancesElDynamic {
    instances:
        Option<DynamicBlock<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl>>,
}
#[derive(Serialize)]
pub struct NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_export_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_import_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_export_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_import_ranges: Option<ListField<PrimField<String>>>,
    site_to_site_data_transfer: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instances: Option<Vec<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl>>,
    dynamic: NetworkConnectivitySpokeLinkedRouterApplianceInstancesElDynamic,
}
impl NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
    #[doc = "Set the field `exclude_export_ranges`.\nDynamic routes overlapped/encompassed by exclude export ranges are excluded during export to hub."]
    pub fn set_exclude_export_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_export_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_import_ranges`.\nHub routes overlapped/encompassed by exclude import ranges are excluded during import from hub."]
    pub fn set_exclude_import_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_import_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `include_export_ranges`.\nDynamic routes fully encompassed by include export ranges are included during export to hub."]
    pub fn set_include_export_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_export_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `include_import_ranges`.\nHub routes fully encompassed by include import ranges are included during import from hub.\n\"ALL_IPV4_RANGES\" or IPv4 CIDR ranges are allowed."]
    pub fn set_include_import_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_import_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `instances`.\n"]
    pub fn set_instances(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instances = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instances = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
    type O = BlockAssignable<NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
    #[doc = "A value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub site_to_site_data_transfer: PrimField<bool>,
}
impl BuildNetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
    pub fn build(self) -> NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
        NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl {
            exclude_export_ranges: core::default::Default::default(),
            exclude_import_ranges: core::default::Default::default(),
            include_export_ranges: core::default::Default::default(),
            include_import_ranges: core::default::Default::default(),
            site_to_site_data_transfer: self.site_to_site_data_transfer,
            instances: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef {
        NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivitySpokeLinkedRouterApplianceInstancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclude_export_ranges` after provisioning.\nDynamic routes overlapped/encompassed by exclude export ranges are excluded during export to hub."]
    pub fn exclude_export_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_export_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_import_ranges` after provisioning.\nHub routes overlapped/encompassed by exclude import ranges are excluded during import from hub."]
    pub fn exclude_import_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_import_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_export_ranges` after provisioning.\nDynamic routes fully encompassed by include export ranges are included during export to hub."]
    pub fn include_export_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_export_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_import_ranges` after provisioning.\nHub routes fully encompassed by include import ranges are included during import from hub.\n\"ALL_IPV4_RANGES\" or IPv4 CIDR ranges are allowed."]
    pub fn include_import_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_import_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `site_to_site_data_transfer` after provisioning.\nA value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub fn site_to_site_data_transfer(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.site_to_site_data_transfer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(
        &self,
    ) -> ListRef<NetworkConnectivitySpokeLinkedRouterApplianceInstancesElInstancesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivitySpokeLinkedVpcNetworkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_export_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_export_ranges: Option<ListField<PrimField<String>>>,
    uri: PrimField<String>,
}
impl NetworkConnectivitySpokeLinkedVpcNetworkEl {
    #[doc = "Set the field `exclude_export_ranges`.\nIP ranges encompassing the subnets to be excluded from peering."]
    pub fn set_exclude_export_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_export_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `include_export_ranges`.\nIP ranges allowed to be included from peering."]
    pub fn set_include_export_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_export_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivitySpokeLinkedVpcNetworkEl {
    type O = BlockAssignable<NetworkConnectivitySpokeLinkedVpcNetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivitySpokeLinkedVpcNetworkEl {
    #[doc = "The URI of the VPC network resource."]
    pub uri: PrimField<String>,
}
impl BuildNetworkConnectivitySpokeLinkedVpcNetworkEl {
    pub fn build(self) -> NetworkConnectivitySpokeLinkedVpcNetworkEl {
        NetworkConnectivitySpokeLinkedVpcNetworkEl {
            exclude_export_ranges: core::default::Default::default(),
            include_export_ranges: core::default::Default::default(),
            uri: self.uri,
        }
    }
}
pub struct NetworkConnectivitySpokeLinkedVpcNetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivitySpokeLinkedVpcNetworkElRef {
    fn new(shared: StackShared, base: String) -> NetworkConnectivitySpokeLinkedVpcNetworkElRef {
        NetworkConnectivitySpokeLinkedVpcNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivitySpokeLinkedVpcNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclude_export_ranges` after provisioning.\nIP ranges encompassing the subnets to be excluded from peering."]
    pub fn exclude_export_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_export_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_export_ranges` after provisioning.\nIP ranges allowed to be included from peering."]
    pub fn include_export_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_export_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe URI of the VPC network resource."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivitySpokeLinkedVpnTunnelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_export_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_import_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_export_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_import_ranges: Option<ListField<PrimField<String>>>,
    site_to_site_data_transfer: PrimField<bool>,
    uris: ListField<PrimField<String>>,
}
impl NetworkConnectivitySpokeLinkedVpnTunnelsEl {
    #[doc = "Set the field `exclude_export_ranges`.\nDynamic routes overlapped/encompassed by exclude export ranges are excluded during export to hub."]
    pub fn set_exclude_export_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_export_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_import_ranges`.\nHub routes overlapped/encompassed by exclude import ranges are excluded during import from hub."]
    pub fn set_exclude_import_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_import_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `include_export_ranges`.\nDynamic routes fully encompassed by include export ranges are included during export to hub."]
    pub fn set_include_export_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_export_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `include_import_ranges`.\nHub routes fully encompassed by include import ranges are included during import from hub.\n\"ALL_IPV4_RANGES\" or IPv4 CIDR ranges are allowed."]
    pub fn set_include_import_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_import_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivitySpokeLinkedVpnTunnelsEl {
    type O = BlockAssignable<NetworkConnectivitySpokeLinkedVpnTunnelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivitySpokeLinkedVpnTunnelsEl {
    #[doc = "A value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub site_to_site_data_transfer: PrimField<bool>,
    #[doc = "The URIs of linked VPN tunnel resources."]
    pub uris: ListField<PrimField<String>>,
}
impl BuildNetworkConnectivitySpokeLinkedVpnTunnelsEl {
    pub fn build(self) -> NetworkConnectivitySpokeLinkedVpnTunnelsEl {
        NetworkConnectivitySpokeLinkedVpnTunnelsEl {
            exclude_export_ranges: core::default::Default::default(),
            exclude_import_ranges: core::default::Default::default(),
            include_export_ranges: core::default::Default::default(),
            include_import_ranges: core::default::Default::default(),
            site_to_site_data_transfer: self.site_to_site_data_transfer,
            uris: self.uris,
        }
    }
}
pub struct NetworkConnectivitySpokeLinkedVpnTunnelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivitySpokeLinkedVpnTunnelsElRef {
    fn new(shared: StackShared, base: String) -> NetworkConnectivitySpokeLinkedVpnTunnelsElRef {
        NetworkConnectivitySpokeLinkedVpnTunnelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivitySpokeLinkedVpnTunnelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclude_export_ranges` after provisioning.\nDynamic routes overlapped/encompassed by exclude export ranges are excluded during export to hub."]
    pub fn exclude_export_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_export_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_import_ranges` after provisioning.\nHub routes overlapped/encompassed by exclude import ranges are excluded during import from hub."]
    pub fn exclude_import_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_import_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_export_ranges` after provisioning.\nDynamic routes fully encompassed by include export ranges are included during export to hub."]
    pub fn include_export_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_export_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_import_ranges` after provisioning.\nHub routes fully encompassed by include import ranges are included during import from hub.\n\"ALL_IPV4_RANGES\" or IPv4 CIDR ranges are allowed."]
    pub fn include_import_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_import_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `site_to_site_data_transfer` after provisioning.\nA value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations."]
    pub fn site_to_site_data_transfer(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.site_to_site_data_transfer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uris` after provisioning.\nThe URIs of linked VPN tunnel resources."]
    pub fn uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivitySpokeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkConnectivitySpokeTimeoutsEl {
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
impl ToListMappable for NetworkConnectivitySpokeTimeoutsEl {
    type O = BlockAssignable<NetworkConnectivitySpokeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivitySpokeTimeoutsEl {}
impl BuildNetworkConnectivitySpokeTimeoutsEl {
    pub fn build(self) -> NetworkConnectivitySpokeTimeoutsEl {
        NetworkConnectivitySpokeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivitySpokeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivitySpokeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkConnectivitySpokeTimeoutsElRef {
        NetworkConnectivitySpokeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivitySpokeTimeoutsElRef {
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
struct NetworkConnectivitySpokeDynamic {
    linked_interconnect_attachments:
        Option<DynamicBlock<NetworkConnectivitySpokeLinkedInterconnectAttachmentsEl>>,
    linked_producer_vpc_network:
        Option<DynamicBlock<NetworkConnectivitySpokeLinkedProducerVpcNetworkEl>>,
    linked_router_appliance_instances:
        Option<DynamicBlock<NetworkConnectivitySpokeLinkedRouterApplianceInstancesEl>>,
    linked_vpc_network: Option<DynamicBlock<NetworkConnectivitySpokeLinkedVpcNetworkEl>>,
    linked_vpn_tunnels: Option<DynamicBlock<NetworkConnectivitySpokeLinkedVpnTunnelsEl>>,
}
