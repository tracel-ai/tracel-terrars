use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputePacketMirroringData {
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
    enable: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collector_ilb: Option<Vec<ComputePacketMirroringCollectorIlbEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<ComputePacketMirroringFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirrored_resources: Option<Vec<ComputePacketMirroringMirroredResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<Vec<ComputePacketMirroringNetworkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputePacketMirroringTimeoutsEl>,
    dynamic: ComputePacketMirroringDynamic,
}
struct ComputePacketMirroring_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputePacketMirroringData>,
}
#[derive(Clone)]
pub struct ComputePacketMirroring(Rc<ComputePacketMirroring_>);
impl ComputePacketMirroring {
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
    #[doc = "Set the field `description`.\nA human-readable description of the rule."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `enable`.\nIndicates whether or not this packet mirroring takes effect. If set to FALSE, this packet mirroring\npolicy will not be enforced on the network. The default is TRUE. Possible values: [\"TRUE\", \"FALSE\"]"]
    pub fn set_enable(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().enable = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\nSince only one rule can be active at a time, priority is\nused to break ties in the case of two rules that apply to\nthe same instances."]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe Region in which the created address should reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `collector_ilb`.\n"]
    pub fn set_collector_ilb(
        self,
        v: impl Into<BlockAssignable<ComputePacketMirroringCollectorIlbEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().collector_ilb = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.collector_ilb = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<ComputePacketMirroringFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mirrored_resources`.\n"]
    pub fn set_mirrored_resources(
        self,
        v: impl Into<BlockAssignable<ComputePacketMirroringMirroredResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mirrored_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mirrored_resources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(
        self,
        v: impl Into<BlockAssignable<ComputePacketMirroringNetworkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputePacketMirroringTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\nIndicates whether or not this packet mirroring takes effect. If set to FALSE, this packet mirroring\npolicy will not be enforced on the network. The default is TRUE. Possible values: [\"TRUE\", \"FALSE\"]"]
    pub fn enable(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the packet mirroring rule"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nSince only one rule can be active at a time, priority is\nused to break ties in the case of two rules that apply to\nthe same instances."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created address should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collector_ilb` after provisioning.\n"]
    pub fn collector_ilb(&self) -> ListRef<ComputePacketMirroringCollectorIlbElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.collector_ilb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<ComputePacketMirroringFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mirrored_resources` after provisioning.\n"]
    pub fn mirrored_resources(&self) -> ListRef<ComputePacketMirroringMirroredResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mirrored_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> ListRef<ComputePacketMirroringNetworkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePacketMirroringTimeoutsElRef {
        ComputePacketMirroringTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputePacketMirroring {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputePacketMirroring {}
impl ToListMappable for ComputePacketMirroring {
    type O = ListRef<ComputePacketMirroringRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputePacketMirroring_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_packet_mirroring".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputePacketMirroring {
    pub tf_id: String,
    #[doc = "The name of the packet mirroring rule"]
    pub name: PrimField<String>,
}
impl BuildComputePacketMirroring {
    pub fn build(self, stack: &mut Stack) -> ComputePacketMirroring {
        let out = ComputePacketMirroring(Rc::new(ComputePacketMirroring_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputePacketMirroringData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                enable: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                priority: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                collector_ilb: core::default::Default::default(),
                filter: core::default::Default::default(),
                mirrored_resources: core::default::Default::default(),
                network: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputePacketMirroringRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePacketMirroringRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputePacketMirroringRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\nIndicates whether or not this packet mirroring takes effect. If set to FALSE, this packet mirroring\npolicy will not be enforced on the network. The default is TRUE. Possible values: [\"TRUE\", \"FALSE\"]"]
    pub fn enable(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the packet mirroring rule"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nSince only one rule can be active at a time, priority is\nused to break ties in the case of two rules that apply to\nthe same instances."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created address should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collector_ilb` after provisioning.\n"]
    pub fn collector_ilb(&self) -> ListRef<ComputePacketMirroringCollectorIlbElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.collector_ilb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<ComputePacketMirroringFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mirrored_resources` after provisioning.\n"]
    pub fn mirrored_resources(&self) -> ListRef<ComputePacketMirroringMirroredResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mirrored_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> ListRef<ComputePacketMirroringNetworkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePacketMirroringTimeoutsElRef {
        ComputePacketMirroringTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputePacketMirroringCollectorIlbEl {
    url: PrimField<String>,
}
impl ComputePacketMirroringCollectorIlbEl {}
impl ToListMappable for ComputePacketMirroringCollectorIlbEl {
    type O = BlockAssignable<ComputePacketMirroringCollectorIlbEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputePacketMirroringCollectorIlbEl {
    #[doc = "The URL of the forwarding rule."]
    pub url: PrimField<String>,
}
impl BuildComputePacketMirroringCollectorIlbEl {
    pub fn build(self) -> ComputePacketMirroringCollectorIlbEl {
        ComputePacketMirroringCollectorIlbEl { url: self.url }
    }
}
pub struct ComputePacketMirroringCollectorIlbElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePacketMirroringCollectorIlbElRef {
    fn new(shared: StackShared, base: String) -> ComputePacketMirroringCollectorIlbElRef {
        ComputePacketMirroringCollectorIlbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputePacketMirroringCollectorIlbElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL of the forwarding rule."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputePacketMirroringFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_protocols: Option<ListField<PrimField<String>>>,
}
impl ComputePacketMirroringFilterEl {
    #[doc = "Set the field `cidr_ranges`.\nIP CIDR ranges that apply as a filter on the source (ingress) or\ndestination (egress) IP in the IP header. Only IPv4 is supported."]
    pub fn set_cidr_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.cidr_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `direction`.\nDirection of traffic to mirror. Default value: \"BOTH\" Possible values: [\"INGRESS\", \"EGRESS\", \"BOTH\"]"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_protocols`.\nPossible IP protocols including tcp, udp, icmp and esp"]
    pub fn set_ip_protocols(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_protocols = Some(v.into());
        self
    }
}
impl ToListMappable for ComputePacketMirroringFilterEl {
    type O = BlockAssignable<ComputePacketMirroringFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputePacketMirroringFilterEl {}
impl BuildComputePacketMirroringFilterEl {
    pub fn build(self) -> ComputePacketMirroringFilterEl {
        ComputePacketMirroringFilterEl {
            cidr_ranges: core::default::Default::default(),
            direction: core::default::Default::default(),
            ip_protocols: core::default::Default::default(),
        }
    }
}
pub struct ComputePacketMirroringFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePacketMirroringFilterElRef {
    fn new(shared: StackShared, base: String) -> ComputePacketMirroringFilterElRef {
        ComputePacketMirroringFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputePacketMirroringFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr_ranges` after provisioning.\nIP CIDR ranges that apply as a filter on the source (ingress) or\ndestination (egress) IP in the IP header. Only IPv4 is supported."]
    pub fn cidr_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.cidr_ranges", self.base))
    }
    #[doc = "Get a reference to the value of field `direction` after provisioning.\nDirection of traffic to mirror. Default value: \"BOTH\" Possible values: [\"INGRESS\", \"EGRESS\", \"BOTH\"]"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_protocols` after provisioning.\nPossible IP protocols including tcp, udp, icmp and esp"]
    pub fn ip_protocols(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_protocols", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputePacketMirroringMirroredResourcesElInstancesEl {
    url: PrimField<String>,
}
impl ComputePacketMirroringMirroredResourcesElInstancesEl {}
impl ToListMappable for ComputePacketMirroringMirroredResourcesElInstancesEl {
    type O = BlockAssignable<ComputePacketMirroringMirroredResourcesElInstancesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputePacketMirroringMirroredResourcesElInstancesEl {
    #[doc = "The URL of the instances where this rule should be active."]
    pub url: PrimField<String>,
}
impl BuildComputePacketMirroringMirroredResourcesElInstancesEl {
    pub fn build(self) -> ComputePacketMirroringMirroredResourcesElInstancesEl {
        ComputePacketMirroringMirroredResourcesElInstancesEl { url: self.url }
    }
}
pub struct ComputePacketMirroringMirroredResourcesElInstancesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePacketMirroringMirroredResourcesElInstancesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputePacketMirroringMirroredResourcesElInstancesElRef {
        ComputePacketMirroringMirroredResourcesElInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputePacketMirroringMirroredResourcesElInstancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL of the instances where this rule should be active."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputePacketMirroringMirroredResourcesElSubnetworksEl {
    url: PrimField<String>,
}
impl ComputePacketMirroringMirroredResourcesElSubnetworksEl {}
impl ToListMappable for ComputePacketMirroringMirroredResourcesElSubnetworksEl {
    type O = BlockAssignable<ComputePacketMirroringMirroredResourcesElSubnetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputePacketMirroringMirroredResourcesElSubnetworksEl {
    #[doc = "The URL of the subnetwork where this rule should be active."]
    pub url: PrimField<String>,
}
impl BuildComputePacketMirroringMirroredResourcesElSubnetworksEl {
    pub fn build(self) -> ComputePacketMirroringMirroredResourcesElSubnetworksEl {
        ComputePacketMirroringMirroredResourcesElSubnetworksEl { url: self.url }
    }
}
pub struct ComputePacketMirroringMirroredResourcesElSubnetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePacketMirroringMirroredResourcesElSubnetworksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputePacketMirroringMirroredResourcesElSubnetworksElRef {
        ComputePacketMirroringMirroredResourcesElSubnetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputePacketMirroringMirroredResourcesElSubnetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL of the subnetwork where this rule should be active."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputePacketMirroringMirroredResourcesElDynamic {
    instances: Option<DynamicBlock<ComputePacketMirroringMirroredResourcesElInstancesEl>>,
    subnetworks: Option<DynamicBlock<ComputePacketMirroringMirroredResourcesElSubnetworksEl>>,
}
#[derive(Serialize)]
pub struct ComputePacketMirroringMirroredResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instances: Option<Vec<ComputePacketMirroringMirroredResourcesElInstancesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetworks: Option<Vec<ComputePacketMirroringMirroredResourcesElSubnetworksEl>>,
    dynamic: ComputePacketMirroringMirroredResourcesElDynamic,
}
impl ComputePacketMirroringMirroredResourcesEl {
    #[doc = "Set the field `tags`.\nAll instances with these tags will be mirrored."]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `instances`.\n"]
    pub fn set_instances(
        mut self,
        v: impl Into<BlockAssignable<ComputePacketMirroringMirroredResourcesElInstancesEl>>,
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
    #[doc = "Set the field `subnetworks`.\n"]
    pub fn set_subnetworks(
        mut self,
        v: impl Into<BlockAssignable<ComputePacketMirroringMirroredResourcesElSubnetworksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.subnetworks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.subnetworks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputePacketMirroringMirroredResourcesEl {
    type O = BlockAssignable<ComputePacketMirroringMirroredResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputePacketMirroringMirroredResourcesEl {}
impl BuildComputePacketMirroringMirroredResourcesEl {
    pub fn build(self) -> ComputePacketMirroringMirroredResourcesEl {
        ComputePacketMirroringMirroredResourcesEl {
            tags: core::default::Default::default(),
            instances: core::default::Default::default(),
            subnetworks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputePacketMirroringMirroredResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePacketMirroringMirroredResourcesElRef {
    fn new(shared: StackShared, base: String) -> ComputePacketMirroringMirroredResourcesElRef {
        ComputePacketMirroringMirroredResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputePacketMirroringMirroredResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nAll instances with these tags will be mirrored."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputePacketMirroringNetworkEl {
    url: PrimField<String>,
}
impl ComputePacketMirroringNetworkEl {}
impl ToListMappable for ComputePacketMirroringNetworkEl {
    type O = BlockAssignable<ComputePacketMirroringNetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputePacketMirroringNetworkEl {
    #[doc = "The full self_link URL of the network where this rule is active."]
    pub url: PrimField<String>,
}
impl BuildComputePacketMirroringNetworkEl {
    pub fn build(self) -> ComputePacketMirroringNetworkEl {
        ComputePacketMirroringNetworkEl { url: self.url }
    }
}
pub struct ComputePacketMirroringNetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePacketMirroringNetworkElRef {
    fn new(shared: StackShared, base: String) -> ComputePacketMirroringNetworkElRef {
        ComputePacketMirroringNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputePacketMirroringNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe full self_link URL of the network where this rule is active."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputePacketMirroringTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputePacketMirroringTimeoutsEl {
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
impl ToListMappable for ComputePacketMirroringTimeoutsEl {
    type O = BlockAssignable<ComputePacketMirroringTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputePacketMirroringTimeoutsEl {}
impl BuildComputePacketMirroringTimeoutsEl {
    pub fn build(self) -> ComputePacketMirroringTimeoutsEl {
        ComputePacketMirroringTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputePacketMirroringTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePacketMirroringTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputePacketMirroringTimeoutsElRef {
        ComputePacketMirroringTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputePacketMirroringTimeoutsElRef {
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
struct ComputePacketMirroringDynamic {
    collector_ilb: Option<DynamicBlock<ComputePacketMirroringCollectorIlbEl>>,
    filter: Option<DynamicBlock<ComputePacketMirroringFilterEl>>,
    mirrored_resources: Option<DynamicBlock<ComputePacketMirroringMirroredResourcesEl>>,
    network: Option<DynamicBlock<ComputePacketMirroringNetworkEl>>,
}
