use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeWireGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_enabled: Option<PrimField<bool>>,
    cross_site_network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<Vec<ComputeWireGroupEndpointsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeWireGroupTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wire_properties: Option<Vec<ComputeWireGroupWirePropertiesEl>>,
    dynamic: ComputeWireGroupDynamic,
}
struct ComputeWireGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeWireGroupData>,
}
#[derive(Clone)]
pub struct ComputeWireGroup(Rc<ComputeWireGroup_>);
impl ComputeWireGroup {
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
    #[doc = "Set the field `admin_enabled`.\nIndicates whether the wire group is administratively enabled."]
    pub fn set_admin_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().admin_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(self, v: impl Into<BlockAssignable<ComputeWireGroupEndpointsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoints = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeWireGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `wire_properties`.\n"]
    pub fn set_wire_properties(
        self,
        v: impl Into<BlockAssignable<ComputeWireGroupWirePropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().wire_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.wire_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `admin_enabled` after provisioning.\nIndicates whether the wire group is administratively enabled."]
    pub fn admin_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cross_site_network` after provisioning.\nRequired cross site network to which wire group belongs."]
    pub fn cross_site_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cross_site_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `topology` after provisioning.\nTopology details for the wire group configuration."]
    pub fn topology(&self) -> ListRef<ComputeWireGroupTopologyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topology", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wires` after provisioning.\nThe single/redundant wire(s) managed by the wire group."]
    pub fn wires(&self) -> ListRef<ComputeWireGroupWiresElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.wires", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeWireGroupTimeoutsElRef {
        ComputeWireGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wire_properties` after provisioning.\n"]
    pub fn wire_properties(&self) -> ListRef<ComputeWireGroupWirePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.wire_properties", self.extract_ref()),
        )
    }
}
impl Referable for ComputeWireGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeWireGroup {}
impl ToListMappable for ComputeWireGroup {
    type O = ListRef<ComputeWireGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeWireGroup_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_wire_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeWireGroup {
    pub tf_id: String,
    #[doc = "Required cross site network to which wire group belongs."]
    pub cross_site_network: PrimField<String>,
    #[doc = "Name of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeWireGroup {
    pub fn build(self, stack: &mut Stack) -> ComputeWireGroup {
        let out = ComputeWireGroup(Rc::new(ComputeWireGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeWireGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admin_enabled: core::default::Default::default(),
                cross_site_network: self.cross_site_network,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                endpoints: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                wire_properties: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeWireGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeWireGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeWireGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_enabled` after provisioning.\nIndicates whether the wire group is administratively enabled."]
    pub fn admin_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cross_site_network` after provisioning.\nRequired cross site network to which wire group belongs."]
    pub fn cross_site_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cross_site_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `topology` after provisioning.\nTopology details for the wire group configuration."]
    pub fn topology(&self) -> ListRef<ComputeWireGroupTopologyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topology", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wires` after provisioning.\nThe single/redundant wire(s) managed by the wire group."]
    pub fn wires(&self) -> ListRef<ComputeWireGroupWiresElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.wires", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeWireGroupTimeoutsElRef {
        ComputeWireGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `wire_properties` after provisioning.\n"]
    pub fn wire_properties(&self) -> ListRef<ComputeWireGroupWirePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.wire_properties", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeWireGroupTopologyElEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
}
impl ComputeWireGroupTopologyElEndpointsEl {
    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.city = Some(v.into());
        self
    }
    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.label = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeWireGroupTopologyElEndpointsEl {
    type O = BlockAssignable<ComputeWireGroupTopologyElEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeWireGroupTopologyElEndpointsEl {}
impl BuildComputeWireGroupTopologyElEndpointsEl {
    pub fn build(self) -> ComputeWireGroupTopologyElEndpointsEl {
        ComputeWireGroupTopologyElEndpointsEl {
            city: core::default::Default::default(),
            label: core::default::Default::default(),
        }
    }
}
pub struct ComputeWireGroupTopologyElEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeWireGroupTopologyElEndpointsElRef {
    fn new(shared: StackShared, base: String) -> ComputeWireGroupTopologyElEndpointsElRef {
        ComputeWireGroupTopologyElEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeWireGroupTopologyElEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }
    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeWireGroupTopologyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<ListField<ComputeWireGroupTopologyElEndpointsEl>>,
}
impl ComputeWireGroupTopologyEl {
    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        mut self,
        v: impl Into<ListField<ComputeWireGroupTopologyElEndpointsEl>>,
    ) -> Self {
        self.endpoints = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeWireGroupTopologyEl {
    type O = BlockAssignable<ComputeWireGroupTopologyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeWireGroupTopologyEl {}
impl BuildComputeWireGroupTopologyEl {
    pub fn build(self) -> ComputeWireGroupTopologyEl {
        ComputeWireGroupTopologyEl {
            endpoints: core::default::Default::default(),
        }
    }
}
pub struct ComputeWireGroupTopologyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeWireGroupTopologyElRef {
    fn new(shared: StackShared, base: String) -> ComputeWireGroupTopologyElRef {
        ComputeWireGroupTopologyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeWireGroupTopologyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> ListRef<ComputeWireGroupTopologyElEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeWireGroupWiresElEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vlan_tag: Option<PrimField<f64>>,
}
impl ComputeWireGroupWiresElEndpointsEl {
    #[doc = "Set the field `interconnect`.\n"]
    pub fn set_interconnect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interconnect = Some(v.into());
        self
    }
    #[doc = "Set the field `vlan_tag`.\n"]
    pub fn set_vlan_tag(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vlan_tag = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeWireGroupWiresElEndpointsEl {
    type O = BlockAssignable<ComputeWireGroupWiresElEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeWireGroupWiresElEndpointsEl {}
impl BuildComputeWireGroupWiresElEndpointsEl {
    pub fn build(self) -> ComputeWireGroupWiresElEndpointsEl {
        ComputeWireGroupWiresElEndpointsEl {
            interconnect: core::default::Default::default(),
            vlan_tag: core::default::Default::default(),
        }
    }
}
pub struct ComputeWireGroupWiresElEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeWireGroupWiresElEndpointsElRef {
    fn new(shared: StackShared, base: String) -> ComputeWireGroupWiresElEndpointsElRef {
        ComputeWireGroupWiresElEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeWireGroupWiresElEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `interconnect` after provisioning.\n"]
    pub fn interconnect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interconnect", self.base))
    }
    #[doc = "Get a reference to the value of field `vlan_tag` after provisioning.\n"]
    pub fn vlan_tag(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeWireGroupWiresElWirePropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bandwidth_unmetered: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault_response: Option<PrimField<String>>,
}
impl ComputeWireGroupWiresElWirePropertiesEl {
    #[doc = "Set the field `bandwidth_unmetered`.\n"]
    pub fn set_bandwidth_unmetered(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bandwidth_unmetered = Some(v.into());
        self
    }
    #[doc = "Set the field `fault_response`.\n"]
    pub fn set_fault_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fault_response = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeWireGroupWiresElWirePropertiesEl {
    type O = BlockAssignable<ComputeWireGroupWiresElWirePropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeWireGroupWiresElWirePropertiesEl {}
impl BuildComputeWireGroupWiresElWirePropertiesEl {
    pub fn build(self) -> ComputeWireGroupWiresElWirePropertiesEl {
        ComputeWireGroupWiresElWirePropertiesEl {
            bandwidth_unmetered: core::default::Default::default(),
            fault_response: core::default::Default::default(),
        }
    }
}
pub struct ComputeWireGroupWiresElWirePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeWireGroupWiresElWirePropertiesElRef {
    fn new(shared: StackShared, base: String) -> ComputeWireGroupWiresElWirePropertiesElRef {
        ComputeWireGroupWiresElWirePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeWireGroupWiresElWirePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bandwidth_unmetered` after provisioning.\n"]
    pub fn bandwidth_unmetered(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bandwidth_unmetered", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fault_response` after provisioning.\n"]
    pub fn fault_response(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fault_response", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeWireGroupWiresEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<ListField<ComputeWireGroupWiresElEndpointsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wire_properties: Option<ListField<ComputeWireGroupWiresElWirePropertiesEl>>,
}
impl ComputeWireGroupWiresEl {
    #[doc = "Set the field `admin_enabled`.\n"]
    pub fn set_admin_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.admin_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        mut self,
        v: impl Into<ListField<ComputeWireGroupWiresElEndpointsEl>>,
    ) -> Self {
        self.endpoints = Some(v.into());
        self
    }
    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.label = Some(v.into());
        self
    }
    #[doc = "Set the field `wire_properties`.\n"]
    pub fn set_wire_properties(
        mut self,
        v: impl Into<ListField<ComputeWireGroupWiresElWirePropertiesEl>>,
    ) -> Self {
        self.wire_properties = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeWireGroupWiresEl {
    type O = BlockAssignable<ComputeWireGroupWiresEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeWireGroupWiresEl {}
impl BuildComputeWireGroupWiresEl {
    pub fn build(self) -> ComputeWireGroupWiresEl {
        ComputeWireGroupWiresEl {
            admin_enabled: core::default::Default::default(),
            endpoints: core::default::Default::default(),
            label: core::default::Default::default(),
            wire_properties: core::default::Default::default(),
        }
    }
}
pub struct ComputeWireGroupWiresElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeWireGroupWiresElRef {
    fn new(shared: StackShared, base: String) -> ComputeWireGroupWiresElRef {
        ComputeWireGroupWiresElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeWireGroupWiresElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_enabled` after provisioning.\n"]
    pub fn admin_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> ListRef<ComputeWireGroupWiresElEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }
    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
    #[doc = "Get a reference to the value of field `wire_properties` after provisioning.\n"]
    pub fn wire_properties(&self) -> ListRef<ComputeWireGroupWiresElWirePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.wire_properties", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeWireGroupEndpointsElInterconnectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnect: Option<PrimField<String>>,
    interconnect_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vlan_tags: Option<ListField<PrimField<f64>>>,
}
impl ComputeWireGroupEndpointsElInterconnectsEl {
    #[doc = "Set the field `interconnect`.\n"]
    pub fn set_interconnect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interconnect = Some(v.into());
        self
    }
    #[doc = "Set the field `vlan_tags`.\nVLAN tags for the interconnect."]
    pub fn set_vlan_tags(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.vlan_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeWireGroupEndpointsElInterconnectsEl {
    type O = BlockAssignable<ComputeWireGroupEndpointsElInterconnectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeWireGroupEndpointsElInterconnectsEl {
    #[doc = ""]
    pub interconnect_name: PrimField<String>,
}
impl BuildComputeWireGroupEndpointsElInterconnectsEl {
    pub fn build(self) -> ComputeWireGroupEndpointsElInterconnectsEl {
        ComputeWireGroupEndpointsElInterconnectsEl {
            interconnect: core::default::Default::default(),
            interconnect_name: self.interconnect_name,
            vlan_tags: core::default::Default::default(),
        }
    }
}
pub struct ComputeWireGroupEndpointsElInterconnectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeWireGroupEndpointsElInterconnectsElRef {
    fn new(shared: StackShared, base: String) -> ComputeWireGroupEndpointsElInterconnectsElRef {
        ComputeWireGroupEndpointsElInterconnectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeWireGroupEndpointsElInterconnectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `interconnect` after provisioning.\n"]
    pub fn interconnect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interconnect", self.base))
    }
    #[doc = "Get a reference to the value of field `interconnect_name` after provisioning.\n"]
    pub fn interconnect_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interconnect_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vlan_tags` after provisioning.\nVLAN tags for the interconnect."]
    pub fn vlan_tags(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.vlan_tags", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeWireGroupEndpointsElDynamic {
    interconnects: Option<DynamicBlock<ComputeWireGroupEndpointsElInterconnectsEl>>,
}
#[derive(Serialize)]
pub struct ComputeWireGroupEndpointsEl {
    endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnects: Option<Vec<ComputeWireGroupEndpointsElInterconnectsEl>>,
    dynamic: ComputeWireGroupEndpointsElDynamic,
}
impl ComputeWireGroupEndpointsEl {
    #[doc = "Set the field `interconnects`.\n"]
    pub fn set_interconnects(
        mut self,
        v: impl Into<BlockAssignable<ComputeWireGroupEndpointsElInterconnectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.interconnects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.interconnects = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeWireGroupEndpointsEl {
    type O = BlockAssignable<ComputeWireGroupEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeWireGroupEndpointsEl {
    #[doc = ""]
    pub endpoint: PrimField<String>,
}
impl BuildComputeWireGroupEndpointsEl {
    pub fn build(self) -> ComputeWireGroupEndpointsEl {
        ComputeWireGroupEndpointsEl {
            endpoint: self.endpoint,
            interconnects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeWireGroupEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeWireGroupEndpointsElRef {
    fn new(shared: StackShared, base: String) -> ComputeWireGroupEndpointsElRef {
        ComputeWireGroupEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeWireGroupEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeWireGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeWireGroupTimeoutsEl {
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
impl ToListMappable for ComputeWireGroupTimeoutsEl {
    type O = BlockAssignable<ComputeWireGroupTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeWireGroupTimeoutsEl {}
impl BuildComputeWireGroupTimeoutsEl {
    pub fn build(self) -> ComputeWireGroupTimeoutsEl {
        ComputeWireGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeWireGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeWireGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeWireGroupTimeoutsElRef {
        ComputeWireGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeWireGroupTimeoutsElRef {
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
#[derive(Serialize)]
pub struct ComputeWireGroupWirePropertiesEl {
    bandwidth_allocation: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bandwidth_unmetered: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault_response: Option<PrimField<String>>,
}
impl ComputeWireGroupWirePropertiesEl {
    #[doc = "Set the field `bandwidth_unmetered`.\nThe unmetered bandwidth setting."]
    pub fn set_bandwidth_unmetered(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bandwidth_unmetered = Some(v.into());
        self
    }
    #[doc = "Set the field `fault_response`.\nResponse when a fault is detected in a pseudowire:\nNONE: default.\nDISABLE_PORT: set the port line protocol down when inline probes detect a fault. This setting is only permitted on port mode pseudowires."]
    pub fn set_fault_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fault_response = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeWireGroupWirePropertiesEl {
    type O = BlockAssignable<ComputeWireGroupWirePropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeWireGroupWirePropertiesEl {
    #[doc = "The configuration of a wire's bandwidth allocation.\nALLOCATE_PER_WIRE: configures a separate unmetered bandwidth allocation (and associated charges) for each wire in the group.\nSHARED_WITH_WIRE_GROUP: this is the default behavior, which configures one unmetered bandwidth allocation for the wire group. The unmetered bandwidth is divided equally across each wire in the group, but dynamic\nthrottling reallocates unused unmetered bandwidth from unused or underused wires to other wires in the group."]
    pub bandwidth_allocation: PrimField<String>,
}
impl BuildComputeWireGroupWirePropertiesEl {
    pub fn build(self) -> ComputeWireGroupWirePropertiesEl {
        ComputeWireGroupWirePropertiesEl {
            bandwidth_allocation: self.bandwidth_allocation,
            bandwidth_unmetered: core::default::Default::default(),
            fault_response: core::default::Default::default(),
        }
    }
}
pub struct ComputeWireGroupWirePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeWireGroupWirePropertiesElRef {
    fn new(shared: StackShared, base: String) -> ComputeWireGroupWirePropertiesElRef {
        ComputeWireGroupWirePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeWireGroupWirePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bandwidth_allocation` after provisioning.\nThe configuration of a wire's bandwidth allocation.\nALLOCATE_PER_WIRE: configures a separate unmetered bandwidth allocation (and associated charges) for each wire in the group.\nSHARED_WITH_WIRE_GROUP: this is the default behavior, which configures one unmetered bandwidth allocation for the wire group. The unmetered bandwidth is divided equally across each wire in the group, but dynamic\nthrottling reallocates unused unmetered bandwidth from unused or underused wires to other wires in the group."]
    pub fn bandwidth_allocation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bandwidth_allocation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bandwidth_unmetered` after provisioning.\nThe unmetered bandwidth setting."]
    pub fn bandwidth_unmetered(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bandwidth_unmetered", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fault_response` after provisioning.\nResponse when a fault is detected in a pseudowire:\nNONE: default.\nDISABLE_PORT: set the port line protocol down when inline probes detect a fault. This setting is only permitted on port mode pseudowires."]
    pub fn fault_response(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fault_response", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeWireGroupDynamic {
    endpoints: Option<DynamicBlock<ComputeWireGroupEndpointsEl>>,
    wire_properties: Option<DynamicBlock<ComputeWireGroupWirePropertiesEl>>,
}
