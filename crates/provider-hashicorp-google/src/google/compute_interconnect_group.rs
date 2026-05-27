use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeInterconnectGroupData {
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
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intent: Option<Vec<ComputeInterconnectGroupIntentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnects: Option<Vec<ComputeInterconnectGroupInterconnectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeInterconnectGroupTimeoutsEl>,
    dynamic: ComputeInterconnectGroupDynamic,
}
struct ComputeInterconnectGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeInterconnectGroupData>,
}
#[derive(Clone)]
pub struct ComputeInterconnectGroup(Rc<ComputeInterconnectGroup_>);
impl ComputeInterconnectGroup {
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
    #[doc = "Set the field `intent`.\n"]
    pub fn set_intent(
        self,
        v: impl Into<BlockAssignable<ComputeInterconnectGroupIntentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().intent = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.intent = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `interconnects`.\n"]
    pub fn set_interconnects(
        self,
        v: impl Into<BlockAssignable<ComputeInterconnectGroupInterconnectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().interconnects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.interconnects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeInterconnectGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `configured` after provisioning.\nThe status of the group as configured. This has the same\nstructure as the operational field reported by the OperationalStatus\nmethod, but does not take into account the operational status of each\nresource."]
    pub fn configured(&self) -> ListRef<ComputeInterconnectGroupConfiguredElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configured", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `physical_structure` after provisioning.\nAn analysis of the physical layout of Interconnects in this\ngroup. Every Interconnect in the group is shown once in this structure."]
    pub fn physical_structure(&self) -> ListRef<ComputeInterconnectGroupPhysicalStructureElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.physical_structure", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> ListRef<ComputeInterconnectGroupIntentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.intent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectGroupTimeoutsElRef {
        ComputeInterconnectGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeInterconnectGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeInterconnectGroup {}
impl ToListMappable for ComputeInterconnectGroup {
    type O = ListRef<ComputeInterconnectGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeInterconnectGroup_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_interconnect_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeInterconnectGroup {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeInterconnectGroup {
    pub fn build(self, stack: &mut Stack) -> ComputeInterconnectGroup {
        let out = ComputeInterconnectGroup(Rc::new(ComputeInterconnectGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeInterconnectGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                intent: core::default::Default::default(),
                interconnects: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeInterconnectGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeInterconnectGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `configured` after provisioning.\nThe status of the group as configured. This has the same\nstructure as the operational field reported by the OperationalStatus\nmethod, but does not take into account the operational status of each\nresource."]
    pub fn configured(&self) -> ListRef<ComputeInterconnectGroupConfiguredElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configured", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `physical_structure` after provisioning.\nAn analysis of the physical layout of Interconnects in this\ngroup. Every Interconnect in the group is shown once in this structure."]
    pub fn physical_structure(&self) -> ListRef<ComputeInterconnectGroupPhysicalStructureElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.physical_structure", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> ListRef<ComputeInterconnectGroupIntentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.intent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectGroupTimeoutsElRef {
        ComputeInterconnectGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    blocker_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentation_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facilities: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnects: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metros: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zones: Option<ListField<PrimField<String>>>,
}
impl ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersEl {
    #[doc = "Set the field `blocker_type`.\n"]
    pub fn set_blocker_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.blocker_type = Some(v.into());
        self
    }
    #[doc = "Set the field `documentation_link`.\n"]
    pub fn set_documentation_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.documentation_link = Some(v.into());
        self
    }
    #[doc = "Set the field `explanation`.\n"]
    pub fn set_explanation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.explanation = Some(v.into());
        self
    }
    #[doc = "Set the field `facilities`.\n"]
    pub fn set_facilities(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.facilities = Some(v.into());
        self
    }
    #[doc = "Set the field `interconnects`.\n"]
    pub fn set_interconnects(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.interconnects = Some(v.into());
        self
    }
    #[doc = "Set the field `metros`.\n"]
    pub fn set_metros(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.metros = Some(v.into());
        self
    }
    #[doc = "Set the field `zones`.\n"]
    pub fn set_zones(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.zones = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersEl
{
    type O = BlockAssignable<
        ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersEl
{}
impl BuildComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersEl {
    pub fn build(
        self,
    ) -> ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersEl {
        ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersEl {
            blocker_type: core::default::Default::default(),
            documentation_link: core::default::Default::default(),
            explanation: core::default::Default::default(),
            facilities: core::default::Default::default(),
            interconnects: core::default::Default::default(),
            metros: core::default::Default::default(),
            zones: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersElRef
    {
        ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `blocker_type` after provisioning.\n"]
    pub fn blocker_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blocker_type", self.base))
    }
    #[doc = "Get a reference to the value of field `documentation_link` after provisioning.\n"]
    pub fn documentation_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.documentation_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `explanation` after provisioning.\n"]
    pub fn explanation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.explanation", self.base))
    }
    #[doc = "Get a reference to the value of field `facilities` after provisioning.\n"]
    pub fn facilities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.facilities", self.base))
    }
    #[doc = "Get a reference to the value of field `interconnects` after provisioning.\n"]
    pub fn interconnects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.interconnects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metros` after provisioning.\n"]
    pub fn metros(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.metros", self.base))
    }
    #[doc = "Get a reference to the value of field `zones` after provisioning.\n"]
    pub fn zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectGroupConfiguredElTopologyCapabilityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    intended_capability_blockers: Option<
        ListField<
            ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_sla: Option<PrimField<String>>,
}
impl ComputeInterconnectGroupConfiguredElTopologyCapabilityEl {
    #[doc = "Set the field `intended_capability_blockers`.\n"]
    pub fn set_intended_capability_blockers(
        mut self,
        v : impl Into < ListField < ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersEl > >,
    ) -> Self {
        self.intended_capability_blockers = Some(v.into());
        self
    }
    #[doc = "Set the field `supported_sla`.\n"]
    pub fn set_supported_sla(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.supported_sla = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectGroupConfiguredElTopologyCapabilityEl {
    type O = BlockAssignable<ComputeInterconnectGroupConfiguredElTopologyCapabilityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectGroupConfiguredElTopologyCapabilityEl {}
impl BuildComputeInterconnectGroupConfiguredElTopologyCapabilityEl {
    pub fn build(self) -> ComputeInterconnectGroupConfiguredElTopologyCapabilityEl {
        ComputeInterconnectGroupConfiguredElTopologyCapabilityEl {
            intended_capability_blockers: core::default::Default::default(),
            supported_sla: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectGroupConfiguredElTopologyCapabilityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectGroupConfiguredElTopologyCapabilityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectGroupConfiguredElTopologyCapabilityElRef {
        ComputeInterconnectGroupConfiguredElTopologyCapabilityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectGroupConfiguredElTopologyCapabilityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `intended_capability_blockers` after provisioning.\n"]
    pub fn intended_capability_blockers(
        &self,
    ) -> ListRef<
        ComputeInterconnectGroupConfiguredElTopologyCapabilityElIntendedCapabilityBlockersElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.intended_capability_blockers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `supported_sla` after provisioning.\n"]
    pub fn supported_sla(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supported_sla", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectGroupConfiguredEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    topology_capability:
        Option<ListField<ComputeInterconnectGroupConfiguredElTopologyCapabilityEl>>,
}
impl ComputeInterconnectGroupConfiguredEl {
    #[doc = "Set the field `topology_capability`.\n"]
    pub fn set_topology_capability(
        mut self,
        v: impl Into<ListField<ComputeInterconnectGroupConfiguredElTopologyCapabilityEl>>,
    ) -> Self {
        self.topology_capability = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectGroupConfiguredEl {
    type O = BlockAssignable<ComputeInterconnectGroupConfiguredEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectGroupConfiguredEl {}
impl BuildComputeInterconnectGroupConfiguredEl {
    pub fn build(self) -> ComputeInterconnectGroupConfiguredEl {
        ComputeInterconnectGroupConfiguredEl {
            topology_capability: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectGroupConfiguredElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectGroupConfiguredElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectGroupConfiguredElRef {
        ComputeInterconnectGroupConfiguredElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectGroupConfiguredElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `topology_capability` after provisioning.\n"]
    pub fn topology_capability(
        &self,
    ) -> ListRef<ComputeInterconnectGroupConfiguredElTopologyCapabilityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topology_capability", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnects: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesEl {
    #[doc = "Set the field `interconnects`.\n"]
    pub fn set_interconnects(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.interconnects = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesEl {
    type O =
        BlockAssignable<ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesEl {}
impl BuildComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesEl {
    pub fn build(self) -> ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesEl {
        ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesEl {
            interconnects: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesElRef {
        ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `interconnects` after provisioning.\n"]
    pub fn interconnects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.interconnects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    facility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zones:
        Option<ListField<ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesEl>>,
}
impl ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesEl {
    #[doc = "Set the field `facility`.\n"]
    pub fn set_facility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.facility = Some(v.into());
        self
    }
    #[doc = "Set the field `zones`.\n"]
    pub fn set_zones(
        mut self,
        v: impl Into<ListField<ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesEl>>,
    ) -> Self {
        self.zones = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesEl {
    type O = BlockAssignable<ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesEl {}
impl BuildComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesEl {
    pub fn build(self) -> ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesEl {
        ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesEl {
            facility: core::default::Default::default(),
            zones: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElRef {
        ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `facility` after provisioning.\n"]
    pub fn facility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.facility", self.base))
    }
    #[doc = "Get a reference to the value of field `zones` after provisioning.\n"]
    pub fn zones(
        &self,
    ) -> ListRef<ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElZonesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectGroupPhysicalStructureElMetrosEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    facilities: Option<ListField<ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metro: Option<PrimField<String>>,
}
impl ComputeInterconnectGroupPhysicalStructureElMetrosEl {
    #[doc = "Set the field `facilities`.\n"]
    pub fn set_facilities(
        mut self,
        v: impl Into<ListField<ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesEl>>,
    ) -> Self {
        self.facilities = Some(v.into());
        self
    }
    #[doc = "Set the field `metro`.\n"]
    pub fn set_metro(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metro = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectGroupPhysicalStructureElMetrosEl {
    type O = BlockAssignable<ComputeInterconnectGroupPhysicalStructureElMetrosEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectGroupPhysicalStructureElMetrosEl {}
impl BuildComputeInterconnectGroupPhysicalStructureElMetrosEl {
    pub fn build(self) -> ComputeInterconnectGroupPhysicalStructureElMetrosEl {
        ComputeInterconnectGroupPhysicalStructureElMetrosEl {
            facilities: core::default::Default::default(),
            metro: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectGroupPhysicalStructureElMetrosElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectGroupPhysicalStructureElMetrosElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeInterconnectGroupPhysicalStructureElMetrosElRef {
        ComputeInterconnectGroupPhysicalStructureElMetrosElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectGroupPhysicalStructureElMetrosElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `facilities` after provisioning.\n"]
    pub fn facilities(
        &self,
    ) -> ListRef<ComputeInterconnectGroupPhysicalStructureElMetrosElFacilitiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.facilities", self.base))
    }
    #[doc = "Get a reference to the value of field `metro` after provisioning.\n"]
    pub fn metro(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metro", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectGroupPhysicalStructureEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metros: Option<ListField<ComputeInterconnectGroupPhysicalStructureElMetrosEl>>,
}
impl ComputeInterconnectGroupPhysicalStructureEl {
    #[doc = "Set the field `metros`.\n"]
    pub fn set_metros(
        mut self,
        v: impl Into<ListField<ComputeInterconnectGroupPhysicalStructureElMetrosEl>>,
    ) -> Self {
        self.metros = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectGroupPhysicalStructureEl {
    type O = BlockAssignable<ComputeInterconnectGroupPhysicalStructureEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectGroupPhysicalStructureEl {}
impl BuildComputeInterconnectGroupPhysicalStructureEl {
    pub fn build(self) -> ComputeInterconnectGroupPhysicalStructureEl {
        ComputeInterconnectGroupPhysicalStructureEl {
            metros: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectGroupPhysicalStructureElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectGroupPhysicalStructureElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectGroupPhysicalStructureElRef {
        ComputeInterconnectGroupPhysicalStructureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectGroupPhysicalStructureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metros` after provisioning.\n"]
    pub fn metros(&self) -> ListRef<ComputeInterconnectGroupPhysicalStructureElMetrosElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metros", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectGroupIntentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    topology_capability: Option<PrimField<String>>,
}
impl ComputeInterconnectGroupIntentEl {
    #[doc = "Set the field `topology_capability`.\nThe reliability the user intends this group to be capable of, in terms\nof the Interconnect product SLAs. Possible values: [\"PRODUCTION_NON_CRITICAL\", \"PRODUCTION_CRITICAL\", \"NO_SLA\", \"AVAILABILITY_SLA_UNSPECIFIED\"]"]
    pub fn set_topology_capability(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topology_capability = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectGroupIntentEl {
    type O = BlockAssignable<ComputeInterconnectGroupIntentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectGroupIntentEl {}
impl BuildComputeInterconnectGroupIntentEl {
    pub fn build(self) -> ComputeInterconnectGroupIntentEl {
        ComputeInterconnectGroupIntentEl {
            topology_capability: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectGroupIntentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectGroupIntentElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectGroupIntentElRef {
        ComputeInterconnectGroupIntentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectGroupIntentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `topology_capability` after provisioning.\nThe reliability the user intends this group to be capable of, in terms\nof the Interconnect product SLAs. Possible values: [\"PRODUCTION_NON_CRITICAL\", \"PRODUCTION_CRITICAL\", \"NO_SLA\", \"AVAILABILITY_SLA_UNSPECIFIED\"]"]
    pub fn topology_capability(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.topology_capability", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectGroupInterconnectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnect: Option<PrimField<String>>,
    name: PrimField<String>,
}
impl ComputeInterconnectGroupInterconnectsEl {
    #[doc = "Set the field `interconnect`.\nThe URL of an Interconnect in this group. All Interconnects in the group are unique."]
    pub fn set_interconnect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interconnect = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectGroupInterconnectsEl {
    type O = BlockAssignable<ComputeInterconnectGroupInterconnectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectGroupInterconnectsEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildComputeInterconnectGroupInterconnectsEl {
    pub fn build(self) -> ComputeInterconnectGroupInterconnectsEl {
        ComputeInterconnectGroupInterconnectsEl {
            interconnect: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct ComputeInterconnectGroupInterconnectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectGroupInterconnectsElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectGroupInterconnectsElRef {
        ComputeInterconnectGroupInterconnectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectGroupInterconnectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `interconnect` after provisioning.\nThe URL of an Interconnect in this group. All Interconnects in the group are unique."]
    pub fn interconnect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interconnect", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeInterconnectGroupTimeoutsEl {
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
impl ToListMappable for ComputeInterconnectGroupTimeoutsEl {
    type O = BlockAssignable<ComputeInterconnectGroupTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectGroupTimeoutsEl {}
impl BuildComputeInterconnectGroupTimeoutsEl {
    pub fn build(self) -> ComputeInterconnectGroupTimeoutsEl {
        ComputeInterconnectGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectGroupTimeoutsElRef {
        ComputeInterconnectGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectGroupTimeoutsElRef {
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
struct ComputeInterconnectGroupDynamic {
    intent: Option<DynamicBlock<ComputeInterconnectGroupIntentEl>>,
    interconnects: Option<DynamicBlock<ComputeInterconnectGroupInterconnectsEl>>,
}
