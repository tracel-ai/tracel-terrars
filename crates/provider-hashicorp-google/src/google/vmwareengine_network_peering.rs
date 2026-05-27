use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VmwareengineNetworkPeeringData {
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
    export_custom_routes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_custom_routes_with_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_custom_routes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_custom_routes_with_public_ip: Option<PrimField<bool>>,
    name: PrimField<String>,
    peer_network: PrimField<String>,
    peer_network_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    vmware_engine_network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VmwareengineNetworkPeeringTimeoutsEl>,
}
struct VmwareengineNetworkPeering_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VmwareengineNetworkPeeringData>,
}
#[derive(Clone)]
pub struct VmwareengineNetworkPeering(Rc<VmwareengineNetworkPeering_>);
impl VmwareengineNetworkPeering {
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
    #[doc = "Set the field `description`.\nUser-provided description for this network peering."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `export_custom_routes`.\nTrue if custom routes are exported to the peered network; false otherwise."]
    pub fn set_export_custom_routes(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().export_custom_routes = Some(v.into());
        self
    }
    #[doc = "Set the field `export_custom_routes_with_public_ip`.\nTrue if all subnet routes with a public IP address range are exported; false otherwise."]
    pub fn set_export_custom_routes_with_public_ip(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().export_custom_routes_with_public_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `import_custom_routes`.\nTrue if custom routes are imported from the peered network; false otherwise."]
    pub fn set_import_custom_routes(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().import_custom_routes = Some(v.into());
        self
    }
    #[doc = "Set the field `import_custom_routes_with_public_ip`.\nTrue if custom routes are imported from the peered network; false otherwise."]
    pub fn set_import_custom_routes_with_public_ip(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().import_custom_routes_with_public_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VmwareengineNetworkPeeringTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this network peering."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_custom_routes` after provisioning.\nTrue if custom routes are exported to the peered network; false otherwise."]
    pub fn export_custom_routes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_custom_routes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_custom_routes_with_public_ip` after provisioning.\nTrue if all subnet routes with a public IP address range are exported; false otherwise."]
    pub fn export_custom_routes_with_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_custom_routes_with_public_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `import_custom_routes` after provisioning.\nTrue if custom routes are imported from the peered network; false otherwise."]
    pub fn import_custom_routes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_custom_routes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `import_custom_routes_with_public_ip` after provisioning.\nTrue if custom routes are imported from the peered network; false otherwise."]
    pub fn import_custom_routes_with_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_custom_routes_with_public_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the Network Peering."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_network` after provisioning.\nThe relative resource name of the network to peer with a standard VMware Engine network.\nThe provided network can be a consumer VPC network or another standard VMware Engine network."]
    pub fn peer_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_network_type` after provisioning.\nThe type of the network to peer with the VMware Engine network. Possible values: [\"STANDARD\", \"VMWARE_ENGINE_NETWORK\", \"PRIVATE_SERVICES_ACCESS\", \"NETAPP_CLOUD_VOLUMES\", \"THIRD_PARTY_SERVICE\", \"DELL_POWERSCALE\", \"GOOGLE_CLOUD_NETAPP_VOLUMES\"]"]
    pub fn peer_network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_network_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the network peering.\nThis field has a value of 'ACTIVE' when there's a matching configuration in the peer network.\nNew values may be added to this enum when appropriate."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nDetails about the current state of the network peering."]
    pub fn state_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast updated time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vmware_engine_network` after provisioning.\nThe relative resource name of the VMware Engine network. Specify the name in the following form:\nprojects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}\ncan either be a project number or a project ID."]
    pub fn vmware_engine_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vmware_engine_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vmware_engine_network_canonical` after provisioning.\nThe canonical name of the VMware Engine network in the form:\nprojects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}"]
    pub fn vmware_engine_network_canonical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vmware_engine_network_canonical", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineNetworkPeeringTimeoutsElRef {
        VmwareengineNetworkPeeringTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VmwareengineNetworkPeering {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VmwareengineNetworkPeering {}
impl ToListMappable for VmwareengineNetworkPeering {
    type O = ListRef<VmwareengineNetworkPeeringRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VmwareengineNetworkPeering_ {
    fn extract_resource_type(&self) -> String {
        "google_vmwareengine_network_peering".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVmwareengineNetworkPeering {
    pub tf_id: String,
    #[doc = "The ID of the Network Peering."]
    pub name: PrimField<String>,
    #[doc = "The relative resource name of the network to peer with a standard VMware Engine network.\nThe provided network can be a consumer VPC network or another standard VMware Engine network."]
    pub peer_network: PrimField<String>,
    #[doc = "The type of the network to peer with the VMware Engine network. Possible values: [\"STANDARD\", \"VMWARE_ENGINE_NETWORK\", \"PRIVATE_SERVICES_ACCESS\", \"NETAPP_CLOUD_VOLUMES\", \"THIRD_PARTY_SERVICE\", \"DELL_POWERSCALE\", \"GOOGLE_CLOUD_NETAPP_VOLUMES\"]"]
    pub peer_network_type: PrimField<String>,
    #[doc = "The relative resource name of the VMware Engine network. Specify the name in the following form:\nprojects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}\ncan either be a project number or a project ID."]
    pub vmware_engine_network: PrimField<String>,
}
impl BuildVmwareengineNetworkPeering {
    pub fn build(self, stack: &mut Stack) -> VmwareengineNetworkPeering {
        let out = VmwareengineNetworkPeering(Rc::new(VmwareengineNetworkPeering_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VmwareengineNetworkPeeringData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                export_custom_routes: core::default::Default::default(),
                export_custom_routes_with_public_ip: core::default::Default::default(),
                id: core::default::Default::default(),
                import_custom_routes: core::default::Default::default(),
                import_custom_routes_with_public_ip: core::default::Default::default(),
                name: self.name,
                peer_network: self.peer_network,
                peer_network_type: self.peer_network_type,
                project: core::default::Default::default(),
                vmware_engine_network: self.vmware_engine_network,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VmwareengineNetworkPeeringRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineNetworkPeeringRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VmwareengineNetworkPeeringRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this network peering."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_custom_routes` after provisioning.\nTrue if custom routes are exported to the peered network; false otherwise."]
    pub fn export_custom_routes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_custom_routes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_custom_routes_with_public_ip` after provisioning.\nTrue if all subnet routes with a public IP address range are exported; false otherwise."]
    pub fn export_custom_routes_with_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.export_custom_routes_with_public_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `import_custom_routes` after provisioning.\nTrue if custom routes are imported from the peered network; false otherwise."]
    pub fn import_custom_routes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_custom_routes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `import_custom_routes_with_public_ip` after provisioning.\nTrue if custom routes are imported from the peered network; false otherwise."]
    pub fn import_custom_routes_with_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_custom_routes_with_public_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the Network Peering."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_network` after provisioning.\nThe relative resource name of the network to peer with a standard VMware Engine network.\nThe provided network can be a consumer VPC network or another standard VMware Engine network."]
    pub fn peer_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_network_type` after provisioning.\nThe type of the network to peer with the VMware Engine network. Possible values: [\"STANDARD\", \"VMWARE_ENGINE_NETWORK\", \"PRIVATE_SERVICES_ACCESS\", \"NETAPP_CLOUD_VOLUMES\", \"THIRD_PARTY_SERVICE\", \"DELL_POWERSCALE\", \"GOOGLE_CLOUD_NETAPP_VOLUMES\"]"]
    pub fn peer_network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_network_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the network peering.\nThis field has a value of 'ACTIVE' when there's a matching configuration in the peer network.\nNew values may be added to this enum when appropriate."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nDetails about the current state of the network peering."]
    pub fn state_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast updated time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vmware_engine_network` after provisioning.\nThe relative resource name of the VMware Engine network. Specify the name in the following form:\nprojects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}\ncan either be a project number or a project ID."]
    pub fn vmware_engine_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vmware_engine_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vmware_engine_network_canonical` after provisioning.\nThe canonical name of the VMware Engine network in the form:\nprojects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}"]
    pub fn vmware_engine_network_canonical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vmware_engine_network_canonical", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineNetworkPeeringTimeoutsElRef {
        VmwareengineNetworkPeeringTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VmwareengineNetworkPeeringTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VmwareengineNetworkPeeringTimeoutsEl {
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
impl ToListMappable for VmwareengineNetworkPeeringTimeoutsEl {
    type O = BlockAssignable<VmwareengineNetworkPeeringTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineNetworkPeeringTimeoutsEl {}
impl BuildVmwareengineNetworkPeeringTimeoutsEl {
    pub fn build(self) -> VmwareengineNetworkPeeringTimeoutsEl {
        VmwareengineNetworkPeeringTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VmwareengineNetworkPeeringTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineNetworkPeeringTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineNetworkPeeringTimeoutsElRef {
        VmwareengineNetworkPeeringTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineNetworkPeeringTimeoutsElRef {
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
