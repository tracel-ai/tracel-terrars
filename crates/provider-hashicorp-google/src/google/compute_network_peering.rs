use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeNetworkPeeringData {
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
    export_custom_routes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_subnet_routes_with_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_custom_routes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_subnet_routes_with_public_ip: Option<PrimField<bool>>,
    name: PrimField<String>,
    network: PrimField<String>,
    peer_network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeNetworkPeeringTimeoutsEl>,
}
struct ComputeNetworkPeering_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeNetworkPeeringData>,
}
#[derive(Clone)]
pub struct ComputeNetworkPeering(Rc<ComputeNetworkPeering_>);
impl ComputeNetworkPeering {
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
    #[doc = "Set the field `export_custom_routes`.\nWhether to export the custom routes to the peer network. Defaults to false."]
    pub fn set_export_custom_routes(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().export_custom_routes = Some(v.into());
        self
    }
    #[doc = "Set the field `export_subnet_routes_with_public_ip`.\n"]
    pub fn set_export_subnet_routes_with_public_ip(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().export_subnet_routes_with_public_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `import_custom_routes`.\nWhether to export the custom routes from the peer network. Defaults to false."]
    pub fn set_import_custom_routes(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().import_custom_routes = Some(v.into());
        self
    }
    #[doc = "Set the field `import_subnet_routes_with_public_ip`.\n"]
    pub fn set_import_subnet_routes_with_public_ip(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().import_subnet_routes_with_public_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `stack_type`.\nWhich IP version(s) of traffic and routes are allowed to be imported or exported between peer networks. The default value is IPV4_ONLY. Possible values: [\"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn set_stack_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stack_type = Some(v.into());
        self
    }
    #[doc = "Set the field `update_strategy`.\nThe update strategy determines the semantics for updates and deletes to the peering connection configuration. The default value is INDEPENDENT. Possible values: [\"INDEPENDENT\", \"CONSENSUS\"]"]
    pub fn set_update_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().update_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeNetworkPeeringTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_custom_routes` after provisioning.\nWhether to export the custom routes to the peer network. Defaults to false."]
    pub fn export_custom_routes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_custom_routes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_subnet_routes_with_public_ip` after provisioning.\n"]
    pub fn export_subnet_routes_with_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_subnet_routes_with_public_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `import_custom_routes` after provisioning.\nWhether to export the custom routes from the peer network. Defaults to false."]
    pub fn import_custom_routes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_custom_routes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `import_subnet_routes_with_public_ip` after provisioning.\n"]
    pub fn import_subnet_routes_with_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_subnet_routes_with_public_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the peering."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe primary network of the peering."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_network` after provisioning.\nThe peer network in the peering. The peer network may belong to a different project."]
    pub fn peer_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stack_type` after provisioning.\nWhich IP version(s) of traffic and routes are allowed to be imported or exported between peer networks. The default value is IPV4_ONLY. Possible values: [\"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stack_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState for the peering, either ACTIVE or INACTIVE. The peering is ACTIVE when there's a matching configuration in the peer network."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nDetails about the current state of the peering."]
    pub fn state_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_strategy` after provisioning.\nThe update strategy determines the semantics for updates and deletes to the peering connection configuration. The default value is INDEPENDENT. Possible values: [\"INDEPENDENT\", \"CONSENSUS\"]"]
    pub fn update_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNetworkPeeringTimeoutsElRef {
        ComputeNetworkPeeringTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeNetworkPeering {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeNetworkPeering {}
impl ToListMappable for ComputeNetworkPeering {
    type O = ListRef<ComputeNetworkPeeringRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeNetworkPeering_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_network_peering".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeNetworkPeering {
    pub tf_id: String,
    #[doc = "Name of the peering."]
    pub name: PrimField<String>,
    #[doc = "The primary network of the peering."]
    pub network: PrimField<String>,
    #[doc = "The peer network in the peering. The peer network may belong to a different project."]
    pub peer_network: PrimField<String>,
}
impl BuildComputeNetworkPeering {
    pub fn build(self, stack: &mut Stack) -> ComputeNetworkPeering {
        let out = ComputeNetworkPeering(Rc::new(ComputeNetworkPeering_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeNetworkPeeringData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                export_custom_routes: core::default::Default::default(),
                export_subnet_routes_with_public_ip: core::default::Default::default(),
                id: core::default::Default::default(),
                import_custom_routes: core::default::Default::default(),
                import_subnet_routes_with_public_ip: core::default::Default::default(),
                name: self.name,
                network: self.network,
                peer_network: self.peer_network,
                stack_type: core::default::Default::default(),
                update_strategy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeNetworkPeeringRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNetworkPeeringRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeNetworkPeeringRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_custom_routes` after provisioning.\nWhether to export the custom routes to the peer network. Defaults to false."]
    pub fn export_custom_routes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_custom_routes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_subnet_routes_with_public_ip` after provisioning.\n"]
    pub fn export_subnet_routes_with_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_subnet_routes_with_public_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `import_custom_routes` after provisioning.\nWhether to export the custom routes from the peer network. Defaults to false."]
    pub fn import_custom_routes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_custom_routes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `import_subnet_routes_with_public_ip` after provisioning.\n"]
    pub fn import_subnet_routes_with_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_subnet_routes_with_public_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the peering."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe primary network of the peering."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_network` after provisioning.\nThe peer network in the peering. The peer network may belong to a different project."]
    pub fn peer_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stack_type` after provisioning.\nWhich IP version(s) of traffic and routes are allowed to be imported or exported between peer networks. The default value is IPV4_ONLY. Possible values: [\"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stack_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState for the peering, either ACTIVE or INACTIVE. The peering is ACTIVE when there's a matching configuration in the peer network."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nDetails about the current state of the peering."]
    pub fn state_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_strategy` after provisioning.\nThe update strategy determines the semantics for updates and deletes to the peering connection configuration. The default value is INDEPENDENT. Possible values: [\"INDEPENDENT\", \"CONSENSUS\"]"]
    pub fn update_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNetworkPeeringTimeoutsElRef {
        ComputeNetworkPeeringTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeNetworkPeeringTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeNetworkPeeringTimeoutsEl {
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
impl ToListMappable for ComputeNetworkPeeringTimeoutsEl {
    type O = BlockAssignable<ComputeNetworkPeeringTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeNetworkPeeringTimeoutsEl {}
impl BuildComputeNetworkPeeringTimeoutsEl {
    pub fn build(self) -> ComputeNetworkPeeringTimeoutsEl {
        ComputeNetworkPeeringTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeNetworkPeeringTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNetworkPeeringTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeNetworkPeeringTimeoutsElRef {
        ComputeNetworkPeeringTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeNetworkPeeringTimeoutsElRef {
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
