use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeNetworkPeeringData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataComputeNetworkPeeringTimeoutsEl>,
}
struct DataComputeNetworkPeering_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeNetworkPeeringData>,
}
#[derive(Clone)]
pub struct DataComputeNetworkPeering(Rc<DataComputeNetworkPeering_>);
impl DataComputeNetworkPeering {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataComputeNetworkPeeringTimeoutsEl>) -> Self {
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
    pub fn timeouts(&self) -> DataComputeNetworkPeeringTimeoutsElRef {
        DataComputeNetworkPeeringTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeNetworkPeering {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeNetworkPeering {}
impl ToListMappable for DataComputeNetworkPeering {
    type O = ListRef<DataComputeNetworkPeeringRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeNetworkPeering_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_network_peering".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeNetworkPeering {
    pub tf_id: String,
    #[doc = "Name of the peering."]
    pub name: PrimField<String>,
    #[doc = "The primary network of the peering."]
    pub network: PrimField<String>,
}
impl BuildDataComputeNetworkPeering {
    pub fn build(self, stack: &mut Stack) -> DataComputeNetworkPeering {
        let out = DataComputeNetworkPeering(Rc::new(DataComputeNetworkPeering_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeNetworkPeeringData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                network: self.network,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeNetworkPeeringRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeNetworkPeeringRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeNetworkPeeringRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    pub fn timeouts(&self) -> DataComputeNetworkPeeringTimeoutsElRef {
        DataComputeNetworkPeeringTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeNetworkPeeringTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}
impl DataComputeNetworkPeeringTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeNetworkPeeringTimeoutsEl {
    type O = BlockAssignable<DataComputeNetworkPeeringTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeNetworkPeeringTimeoutsEl {}
impl BuildDataComputeNetworkPeeringTimeoutsEl {
    pub fn build(self) -> DataComputeNetworkPeeringTimeoutsEl {
        DataComputeNetworkPeeringTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}
pub struct DataComputeNetworkPeeringTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeNetworkPeeringTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeNetworkPeeringTimeoutsElRef {
        DataComputeNetworkPeeringTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeNetworkPeeringTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
