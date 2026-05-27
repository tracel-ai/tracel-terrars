use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeResourcePolicyData {
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
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_consistency_group_policy: Option<Vec<ComputeResourcePolicyDiskConsistencyGroupPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_placement_policy: Option<Vec<ComputeResourcePolicyGroupPlacementPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_schedule_policy: Option<Vec<ComputeResourcePolicyInstanceSchedulePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_schedule_policy: Option<Vec<ComputeResourcePolicySnapshotSchedulePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeResourcePolicyTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_policy: Option<Vec<ComputeResourcePolicyWorkloadPolicyEl>>,
    dynamic: ComputeResourcePolicyDynamic,
}
struct ComputeResourcePolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeResourcePolicyData>,
}
#[derive(Clone)]
pub struct ComputeResourcePolicy(Rc<ComputeResourcePolicy_>);
impl ComputeResourcePolicy {
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
    #[doc = "Set the field `region`.\nRegion where resource policy resides."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_consistency_group_policy`.\n"]
    pub fn set_disk_consistency_group_policy(
        self,
        v: impl Into<BlockAssignable<ComputeResourcePolicyDiskConsistencyGroupPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_consistency_group_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .disk_consistency_group_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `group_placement_policy`.\n"]
    pub fn set_group_placement_policy(
        self,
        v: impl Into<BlockAssignable<ComputeResourcePolicyGroupPlacementPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().group_placement_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.group_placement_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `instance_schedule_policy`.\n"]
    pub fn set_instance_schedule_policy(
        self,
        v: impl Into<BlockAssignable<ComputeResourcePolicyInstanceSchedulePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_schedule_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_schedule_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `snapshot_schedule_policy`.\n"]
    pub fn set_snapshot_schedule_policy(
        self,
        v: impl Into<BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snapshot_schedule_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snapshot_schedule_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeResourcePolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `workload_policy`.\n"]
    pub fn set_workload_policy(
        self,
        v: impl Into<BlockAssignable<ComputeResourcePolicyWorkloadPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workload_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workload_policy = Some(d);
            }
        }
        self
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the resource, provided by the client when initially creating\nthe resource. The resource name must be 1-63 characters long, and comply\nwith RFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])'? which means the\nfirst character must be a lowercase letter, and all following characters\nmust be a dash, lowercase letter, or digit, except the last character,\nwhich cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where resource policy resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_consistency_group_policy` after provisioning.\n"]
    pub fn disk_consistency_group_policy(
        &self,
    ) -> ListRef<ComputeResourcePolicyDiskConsistencyGroupPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_consistency_group_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_placement_policy` after provisioning.\n"]
    pub fn group_placement_policy(
        &self,
    ) -> ListRef<ComputeResourcePolicyGroupPlacementPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_placement_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_schedule_policy` after provisioning.\n"]
    pub fn instance_schedule_policy(
        &self,
    ) -> ListRef<ComputeResourcePolicyInstanceSchedulePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_schedule_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_schedule_policy` after provisioning.\n"]
    pub fn snapshot_schedule_policy(
        &self,
    ) -> ListRef<ComputeResourcePolicySnapshotSchedulePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_schedule_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeResourcePolicyTimeoutsElRef {
        ComputeResourcePolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_policy` after provisioning.\n"]
    pub fn workload_policy(&self) -> ListRef<ComputeResourcePolicyWorkloadPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_policy", self.extract_ref()),
        )
    }
}
impl Referable for ComputeResourcePolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeResourcePolicy {}
impl ToListMappable for ComputeResourcePolicy {
    type O = ListRef<ComputeResourcePolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeResourcePolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_resource_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeResourcePolicy {
    pub tf_id: String,
    #[doc = "The name of the resource, provided by the client when initially creating\nthe resource. The resource name must be 1-63 characters long, and comply\nwith RFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])'? which means the\nfirst character must be a lowercase letter, and all following characters\nmust be a dash, lowercase letter, or digit, except the last character,\nwhich cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeResourcePolicy {
    pub fn build(self, stack: &mut Stack) -> ComputeResourcePolicy {
        let out = ComputeResourcePolicy(Rc::new(ComputeResourcePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeResourcePolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                disk_consistency_group_policy: core::default::Default::default(),
                group_placement_policy: core::default::Default::default(),
                instance_schedule_policy: core::default::Default::default(),
                snapshot_schedule_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                workload_policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeResourcePolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeResourcePolicyRef {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the resource, provided by the client when initially creating\nthe resource. The resource name must be 1-63 characters long, and comply\nwith RFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])'? which means the\nfirst character must be a lowercase letter, and all following characters\nmust be a dash, lowercase letter, or digit, except the last character,\nwhich cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where resource policy resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_consistency_group_policy` after provisioning.\n"]
    pub fn disk_consistency_group_policy(
        &self,
    ) -> ListRef<ComputeResourcePolicyDiskConsistencyGroupPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_consistency_group_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_placement_policy` after provisioning.\n"]
    pub fn group_placement_policy(
        &self,
    ) -> ListRef<ComputeResourcePolicyGroupPlacementPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_placement_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_schedule_policy` after provisioning.\n"]
    pub fn instance_schedule_policy(
        &self,
    ) -> ListRef<ComputeResourcePolicyInstanceSchedulePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_schedule_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_schedule_policy` after provisioning.\n"]
    pub fn snapshot_schedule_policy(
        &self,
    ) -> ListRef<ComputeResourcePolicySnapshotSchedulePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_schedule_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeResourcePolicyTimeoutsElRef {
        ComputeResourcePolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_policy` after provisioning.\n"]
    pub fn workload_policy(&self) -> ListRef<ComputeResourcePolicyWorkloadPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_policy", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    enabled: PrimField<bool>,
}
impl ComputeResourcePolicyDiskConsistencyGroupPolicyEl {}
impl ToListMappable for ComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    type O = BlockAssignable<ComputeResourcePolicyDiskConsistencyGroupPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    #[doc = "Enable disk consistency on the resource policy."]
    pub enabled: PrimField<bool>,
}
impl BuildComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    pub fn build(self) -> ComputeResourcePolicyDiskConsistencyGroupPolicyEl {
        ComputeResourcePolicyDiskConsistencyGroupPolicyEl {
            enabled: self.enabled,
        }
    }
}
pub struct ComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
        ComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnable disk consistency on the resource policy."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResourcePolicyGroupPlacementPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_domain_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collocation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_topology: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_count: Option<PrimField<f64>>,
}
impl ComputeResourcePolicyGroupPlacementPolicyEl {
    #[doc = "Set the field `availability_domain_count`.\nThe number of availability domains instances will be spread across. If two instances are in different\navailability domain, they will not be put in the same low latency network"]
    pub fn set_availability_domain_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.availability_domain_count = Some(v.into());
        self
    }
    #[doc = "Set the field `collocation`.\nCollocation specifies whether to place VMs inside the same availability domain on the same low-latency network.\nSpecify 'COLLOCATED' to enable collocation. Can only be specified with 'vm_count'. If compute instances are created\nwith a COLLOCATED policy, then exactly 'vm_count' instances must be created at the same time with the resource policy\nattached. Possible values: [\"COLLOCATED\"]"]
    pub fn set_collocation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.collocation = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_topology`.\nSpecifies the shape of the GPU slice, in slice based GPU families eg. A4X."]
    pub fn set_gpu_topology(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_topology = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_count`.\nNumber of VMs in this placement group. Google does not recommend that you use this field\nunless you use a compact policy and you want your policy to work only if it contains this\nexact number of VMs."]
    pub fn set_vm_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vm_count = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResourcePolicyGroupPlacementPolicyEl {
    type O = BlockAssignable<ComputeResourcePolicyGroupPlacementPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicyGroupPlacementPolicyEl {}
impl BuildComputeResourcePolicyGroupPlacementPolicyEl {
    pub fn build(self) -> ComputeResourcePolicyGroupPlacementPolicyEl {
        ComputeResourcePolicyGroupPlacementPolicyEl {
            availability_domain_count: core::default::Default::default(),
            collocation: core::default::Default::default(),
            gpu_topology: core::default::Default::default(),
            vm_count: core::default::Default::default(),
        }
    }
}
pub struct ComputeResourcePolicyGroupPlacementPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicyGroupPlacementPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeResourcePolicyGroupPlacementPolicyElRef {
        ComputeResourcePolicyGroupPlacementPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicyGroupPlacementPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `availability_domain_count` after provisioning.\nThe number of availability domains instances will be spread across. If two instances are in different\navailability domain, they will not be put in the same low latency network"]
    pub fn availability_domain_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_domain_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `collocation` after provisioning.\nCollocation specifies whether to place VMs inside the same availability domain on the same low-latency network.\nSpecify 'COLLOCATED' to enable collocation. Can only be specified with 'vm_count'. If compute instances are created\nwith a COLLOCATED policy, then exactly 'vm_count' instances must be created at the same time with the resource policy\nattached. Possible values: [\"COLLOCATED\"]"]
    pub fn collocation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collocation", self.base))
    }
    #[doc = "Get a reference to the value of field `gpu_topology` after provisioning.\nSpecifies the shape of the GPU slice, in slice based GPU families eg. A4X."]
    pub fn gpu_topology(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gpu_topology", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_count` after provisioning.\nNumber of VMs in this placement group. Google does not recommend that you use this field\nunless you use a compact policy and you want your policy to work only if it contains this\nexact number of VMs."]
    pub fn vm_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vm_count", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    schedule: PrimField<String>,
}
impl ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {}
impl ToListMappable for ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    type O = BlockAssignable<ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    #[doc = "Specifies the frequency for the operation, using the unix-cron format."]
    pub schedule: PrimField<String>,
}
impl BuildComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    pub fn build(self) -> ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
        ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
            schedule: self.schedule,
        }
    }
}
pub struct ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
        ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nSpecifies the frequency for the operation, using the unix-cron format."]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    schedule: PrimField<String>,
}
impl ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {}
impl ToListMappable for ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    type O = BlockAssignable<ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    #[doc = "Specifies the frequency for the operation, using the unix-cron format."]
    pub schedule: PrimField<String>,
}
impl BuildComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    pub fn build(self) -> ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
        ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
            schedule: self.schedule,
        }
    }
}
pub struct ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
        ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nSpecifies the frequency for the operation, using the unix-cron format."]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeResourcePolicyInstanceSchedulePolicyElDynamic {
    vm_start_schedule:
        Option<DynamicBlock<ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>>,
    vm_stop_schedule:
        Option<DynamicBlock<ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>>,
}
#[derive(Serialize)]
pub struct ComputeResourcePolicyInstanceSchedulePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    time_zone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_start_schedule: Option<Vec<ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_stop_schedule: Option<Vec<ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>>,
    dynamic: ComputeResourcePolicyInstanceSchedulePolicyElDynamic,
}
impl ComputeResourcePolicyInstanceSchedulePolicyEl {
    #[doc = "Set the field `expiration_time`.\nThe expiration time of the schedule. The timestamp is an RFC3339 string."]
    pub fn set_expiration_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expiration_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\nThe start time of the schedule. The timestamp is an RFC3339 string."]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_start_schedule`.\n"]
    pub fn set_vm_start_schedule(
        mut self,
        v: impl Into<BlockAssignable<ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vm_start_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vm_start_schedule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vm_stop_schedule`.\n"]
    pub fn set_vm_stop_schedule(
        mut self,
        v: impl Into<BlockAssignable<ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vm_stop_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vm_stop_schedule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeResourcePolicyInstanceSchedulePolicyEl {
    type O = BlockAssignable<ComputeResourcePolicyInstanceSchedulePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicyInstanceSchedulePolicyEl {
    #[doc = "Specifies the time zone to be used in interpreting the schedule. The value of this field must be a time zone name\nfrom the tz database: http://en.wikipedia.org/wiki/Tz_database."]
    pub time_zone: PrimField<String>,
}
impl BuildComputeResourcePolicyInstanceSchedulePolicyEl {
    pub fn build(self) -> ComputeResourcePolicyInstanceSchedulePolicyEl {
        ComputeResourcePolicyInstanceSchedulePolicyEl {
            expiration_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            time_zone: self.time_zone,
            vm_start_schedule: core::default::Default::default(),
            vm_stop_schedule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeResourcePolicyInstanceSchedulePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicyInstanceSchedulePolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeResourcePolicyInstanceSchedulePolicyElRef {
        ComputeResourcePolicyInstanceSchedulePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicyInstanceSchedulePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\nThe expiration time of the schedule. The timestamp is an RFC3339 string."]
    pub fn expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThe start time of the schedule. The timestamp is an RFC3339 string."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nSpecifies the time zone to be used in interpreting the schedule. The value of this field must be a time zone name\nfrom the tz database: http://en.wikipedia.org/wiki/Tz_database."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_start_schedule` after provisioning.\n"]
    pub fn vm_start_schedule(
        &self,
    ) -> ListRef<ComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vm_start_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vm_stop_schedule` after provisioning.\n"]
    pub fn vm_stop_schedule(
        &self,
    ) -> ListRef<ComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vm_stop_schedule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    max_retention_days: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_source_disk_delete: Option<PrimField<String>>,
}
impl ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    #[doc = "Set the field `on_source_disk_delete`.\nSpecifies the behavior to apply to scheduled snapshots when\nthe source disk is deleted. Default value: \"KEEP_AUTO_SNAPSHOTS\" Possible values: [\"KEEP_AUTO_SNAPSHOTS\", \"APPLY_RETENTION_POLICY\"]"]
    pub fn set_on_source_disk_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_source_disk_delete = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    type O = BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    #[doc = "Maximum age of the snapshot that is allowed to be kept."]
    pub max_retention_days: PrimField<f64>,
}
impl BuildComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
        ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
            max_retention_days: self.max_retention_days,
            on_source_disk_delete: core::default::Default::default(),
        }
    }
}
pub struct ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_retention_days` after provisioning.\nMaximum age of the snapshot that is allowed to be kept."]
    pub fn max_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_retention_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_source_disk_delete` after provisioning.\nSpecifies the behavior to apply to scheduled snapshots when\nthe source disk is deleted. Default value: \"KEEP_AUTO_SNAPSHOTS\" Possible values: [\"KEEP_AUTO_SNAPSHOTS\", \"APPLY_RETENTION_POLICY\"]"]
    pub fn on_source_disk_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_source_disk_delete", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    days_in_cycle: PrimField<f64>,
    start_time: PrimField<String>,
}
impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {}
impl ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    type O =
        BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    #[doc = "Defines a schedule with units measured in days. The value determines how many days pass between the start of each cycle. Days in cycle for snapshot schedule policy must be 1."]
    pub days_in_cycle: PrimField<f64>,
    #[doc = "This must be in UTC format that resolves to one of\n00:00, 04:00, 08:00, 12:00, 16:00, or 20:00. For example,\nboth 13:00-5 and 08:00 are valid."]
    pub start_time: PrimField<String>,
}
impl BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
            days_in_cycle: self.days_in_cycle,
            start_time: self.start_time,
        }
    }
}
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `days_in_cycle` after provisioning.\nDefines a schedule with units measured in days. The value determines how many days pass between the start of each cycle. Days in cycle for snapshot schedule policy must be 1."]
    pub fn days_in_cycle(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.days_in_cycle", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThis must be in UTC format that resolves to one of\n00:00, 04:00, 08:00, 12:00, 16:00, or 20:00. For example,\nboth 13:00-5 and 08:00 are valid."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    hours_in_cycle: PrimField<f64>,
    start_time: PrimField<String>,
}
impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {}
impl ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    type O =
        BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    #[doc = "The number of hours between snapshots."]
    pub hours_in_cycle: PrimField<f64>,
    #[doc = "Time within the window to start the operations.\nIt must be in an hourly format \"HH:MM\",\nwhere HH : [00-23] and MM : [00] GMT. eg: 21:00"]
    pub start_time: PrimField<String>,
}
impl BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
            hours_in_cycle: self.hours_in_cycle,
            start_time: self.start_time,
        }
    }
}
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours_in_cycle` after provisioning.\nThe number of hours between snapshots."]
    pub fn hours_in_cycle(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hours_in_cycle", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nTime within the window to start the operations.\nIt must be in an hourly format \"HH:MM\",\nwhere HH : [00-23] and MM : [00] GMT. eg: 21:00"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
    day: PrimField<String>,
    start_time: PrimField<String>,
}
impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {}
impl ToListMappable
    for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl
{
    type O = BlockAssignable<
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl
{
    #[doc = "The day of the week to create the snapshot. e.g. MONDAY Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day: PrimField<String>,
    #[doc = "Time within the window to start the operations.\nIt must be in format \"HH:MM\", where HH : [00-23] and MM : [00-00] GMT."]
    pub start_time: PrimField<String>,
}
impl BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
    pub fn build(
        self,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
            day: self.day,
            start_time: self.start_time,
        }
    }
}
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef
    {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nThe day of the week to create the snapshot. e.g. MONDAY Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nTime within the window to start the operations.\nIt must be in format \"HH:MM\", where HH : [00-23] and MM : [00-00] GMT."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDynamic {
    day_of_weeks: Option<
        DynamicBlock<
            ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_weeks: Option<
        Vec<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl>,
    >,
    dynamic: ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDynamic,
}
impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    #[doc = "Set the field `day_of_weeks`.\n"]
    pub fn set_day_of_weeks(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.day_of_weeks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.day_of_weeks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    type O =
        BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {}
impl BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
            day_of_weeks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDynamic {
    daily_schedule: Option<
        DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>,
    >,
    hourly_schedule: Option<
        DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl>,
    >,
    weekly_schedule: Option<
        DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl>,
    >,
}
#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_schedule:
        Option<Vec<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hourly_schedule:
        Option<Vec<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_schedule:
        Option<Vec<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl>>,
    dynamic: ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDynamic,
}
impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    #[doc = "Set the field `daily_schedule`.\n"]
    pub fn set_daily_schedule(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.daily_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.daily_schedule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hourly_schedule`.\n"]
    pub fn set_hourly_schedule(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hourly_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hourly_schedule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `weekly_schedule`.\n"]
    pub fn set_weekly_schedule(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weekly_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_schedule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    type O = BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {}
impl BuildComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
            daily_schedule: core::default::Default::default(),
            hourly_schedule: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `daily_schedule` after provisioning.\n"]
    pub fn daily_schedule(
        &self,
    ) -> ListRef<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.daily_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hourly_schedule` after provisioning.\n"]
    pub fn hourly_schedule(
        &self,
    ) -> ListRef<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hourly_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(
        &self,
    ) -> ListRef<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_schedule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    chain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_flush: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_locations: Option<SetField<PrimField<String>>>,
}
impl ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    #[doc = "Set the field `chain_name`.\nCreates the new snapshot in the snapshot chain labeled with the\nspecified name. The chain name must be 1-63 characters long and comply\nwith RFC1035."]
    pub fn set_chain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.chain_name = Some(v.into());
        self
    }
    #[doc = "Set the field `guest_flush`.\nWhether to perform a 'guest aware' snapshot."]
    pub fn set_guest_flush(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.guest_flush = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nA set of key-value pairs."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_locations`.\nCloud Storage bucket location to store the auto snapshot\n(regional or multi-regional)"]
    pub fn set_storage_locations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.storage_locations = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    type O = BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {}
impl BuildComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
        ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
            chain_name: core::default::Default::default(),
            guest_flush: core::default::Default::default(),
            labels: core::default::Default::default(),
            storage_locations: core::default::Default::default(),
        }
    }
}
pub struct ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `chain_name` after provisioning.\nCreates the new snapshot in the snapshot chain labeled with the\nspecified name. The chain name must be 1-63 characters long and comply\nwith RFC1035."]
    pub fn chain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.chain_name", self.base))
    }
    #[doc = "Get a reference to the value of field `guest_flush` after provisioning.\nWhether to perform a 'guest aware' snapshot."]
    pub fn guest_flush(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.guest_flush", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key-value pairs."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_locations` after provisioning.\nCloud Storage bucket location to store the auto snapshot\n(regional or multi-regional)"]
    pub fn storage_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.storage_locations", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeResourcePolicySnapshotSchedulePolicyElDynamic {
    retention_policy:
        Option<DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>>,
    schedule: Option<DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>>,
    snapshot_properties:
        Option<DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>>,
}
#[derive(Serialize)]
pub struct ComputeResourcePolicySnapshotSchedulePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_policy: Option<Vec<ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<Vec<ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_properties:
        Option<Vec<ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>>,
    dynamic: ComputeResourcePolicySnapshotSchedulePolicyElDynamic,
}
impl ComputeResourcePolicySnapshotSchedulePolicyEl {
    #[doc = "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retention_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retention_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        mut self,
        v: impl Into<BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `snapshot_properties`.\n"]
    pub fn set_snapshot_properties(
        mut self,
        v: impl Into<BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.snapshot_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.snapshot_properties = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeResourcePolicySnapshotSchedulePolicyEl {
    type O = BlockAssignable<ComputeResourcePolicySnapshotSchedulePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicySnapshotSchedulePolicyEl {}
impl BuildComputeResourcePolicySnapshotSchedulePolicyEl {
    pub fn build(self) -> ComputeResourcePolicySnapshotSchedulePolicyEl {
        ComputeResourcePolicySnapshotSchedulePolicyEl {
            retention_policy: core::default::Default::default(),
            schedule: core::default::Default::default(),
            snapshot_properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeResourcePolicySnapshotSchedulePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicySnapshotSchedulePolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeResourcePolicySnapshotSchedulePolicyElRef {
        ComputeResourcePolicySnapshotSchedulePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicySnapshotSchedulePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(
        &self,
    ) -> ListRef<ComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<ComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
    #[doc = "Get a reference to the value of field `snapshot_properties` after provisioning.\n"]
    pub fn snapshot_properties(
        &self,
    ) -> ListRef<ComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_properties", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeResourcePolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeResourcePolicyTimeoutsEl {
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
impl ToListMappable for ComputeResourcePolicyTimeoutsEl {
    type O = BlockAssignable<ComputeResourcePolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicyTimeoutsEl {}
impl BuildComputeResourcePolicyTimeoutsEl {
    pub fn build(self) -> ComputeResourcePolicyTimeoutsEl {
        ComputeResourcePolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeResourcePolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeResourcePolicyTimeoutsElRef {
        ComputeResourcePolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicyTimeoutsElRef {
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
pub struct ComputeResourcePolicyWorkloadPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_topology: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_topology_distance: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ComputeResourcePolicyWorkloadPolicyEl {
    #[doc = "Set the field `accelerator_topology`.\nThe accelerator topology. This field can be set only when the workload policy type is HIGH_THROUGHPUT\nand cannot be set if max topology distance is set."]
    pub fn set_accelerator_topology(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_topology = Some(v.into());
        self
    }
    #[doc = "Set the field `max_topology_distance`.\nThe maximum topology distance. This field can be set only when the workload policy type is HIGH_THROUGHPUT\nand cannot be set if accelerator topology or accelerator topology mode is set. Possible values: [\"BLOCK\", \"CLUSTER\", \"SUBBLOCK\"]"]
    pub fn set_max_topology_distance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_topology_distance = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResourcePolicyWorkloadPolicyEl {
    type O = BlockAssignable<ComputeResourcePolicyWorkloadPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResourcePolicyWorkloadPolicyEl {
    #[doc = "The type of workload policy. Possible values: [\"HIGH_AVAILABILITY\", \"HIGH_THROUGHPUT\"]"]
    pub type_: PrimField<String>,
}
impl BuildComputeResourcePolicyWorkloadPolicyEl {
    pub fn build(self) -> ComputeResourcePolicyWorkloadPolicyEl {
        ComputeResourcePolicyWorkloadPolicyEl {
            accelerator_topology: core::default::Default::default(),
            max_topology_distance: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct ComputeResourcePolicyWorkloadPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResourcePolicyWorkloadPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeResourcePolicyWorkloadPolicyElRef {
        ComputeResourcePolicyWorkloadPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResourcePolicyWorkloadPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_topology` after provisioning.\nThe accelerator topology. This field can be set only when the workload policy type is HIGH_THROUGHPUT\nand cannot be set if max topology distance is set."]
    pub fn accelerator_topology(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_topology", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_topology_distance` after provisioning.\nThe maximum topology distance. This field can be set only when the workload policy type is HIGH_THROUGHPUT\nand cannot be set if accelerator topology or accelerator topology mode is set. Possible values: [\"BLOCK\", \"CLUSTER\", \"SUBBLOCK\"]"]
    pub fn max_topology_distance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_topology_distance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of workload policy. Possible values: [\"HIGH_AVAILABILITY\", \"HIGH_THROUGHPUT\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeResourcePolicyDynamic {
    disk_consistency_group_policy:
        Option<DynamicBlock<ComputeResourcePolicyDiskConsistencyGroupPolicyEl>>,
    group_placement_policy: Option<DynamicBlock<ComputeResourcePolicyGroupPlacementPolicyEl>>,
    instance_schedule_policy: Option<DynamicBlock<ComputeResourcePolicyInstanceSchedulePolicyEl>>,
    snapshot_schedule_policy: Option<DynamicBlock<ComputeResourcePolicySnapshotSchedulePolicyEl>>,
    workload_policy: Option<DynamicBlock<ComputeResourcePolicyWorkloadPolicyEl>>,
}
