use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeResourcePolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataComputeResourcePolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeResourcePolicyData>,
}
#[derive(Clone)]
pub struct DataComputeResourcePolicy(Rc<DataComputeResourcePolicy_>);
impl DataComputeResourcePolicy {
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
    #[doc = "Get a reference to the value of field `disk_consistency_group_policy` after provisioning.\nReplication consistency group for asynchronous disk replication."]
    pub fn disk_consistency_group_policy(
        &self,
    ) -> ListRef<DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_consistency_group_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_placement_policy` after provisioning.\nResource policy for instances used for placement configuration."]
    pub fn group_placement_policy(
        &self,
    ) -> ListRef<DataComputeResourcePolicyGroupPlacementPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_placement_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_schedule_policy` after provisioning.\nResource policy for scheduling instance operations."]
    pub fn instance_schedule_policy(
        &self,
    ) -> ListRef<DataComputeResourcePolicyInstanceSchedulePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_schedule_policy", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `snapshot_schedule_policy` after provisioning.\nPolicy for creating snapshots of persistent disks."]
    pub fn snapshot_schedule_policy(
        &self,
    ) -> ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_schedule_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_policy` after provisioning.\nRepresents the workload policy."]
    pub fn workload_policy(&self) -> ListRef<DataComputeResourcePolicyWorkloadPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_policy", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeResourcePolicy {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeResourcePolicy {}
impl ToListMappable for DataComputeResourcePolicy {
    type O = ListRef<DataComputeResourcePolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeResourcePolicy_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_resource_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeResourcePolicy {
    pub tf_id: String,
    #[doc = "The name of the resource, provided by the client when initially creating\nthe resource. The resource name must be 1-63 characters long, and comply\nwith RFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])'? which means the\nfirst character must be a lowercase letter, and all following characters\nmust be a dash, lowercase letter, or digit, except the last character,\nwhich cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildDataComputeResourcePolicy {
    pub fn build(self, stack: &mut Stack) -> DataComputeResourcePolicy {
        let out = DataComputeResourcePolicy(Rc::new(DataComputeResourcePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeResourcePolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeResourcePolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeResourcePolicyRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_consistency_group_policy` after provisioning.\nReplication consistency group for asynchronous disk replication."]
    pub fn disk_consistency_group_policy(
        &self,
    ) -> ListRef<DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_consistency_group_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `group_placement_policy` after provisioning.\nResource policy for instances used for placement configuration."]
    pub fn group_placement_policy(
        &self,
    ) -> ListRef<DataComputeResourcePolicyGroupPlacementPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_placement_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_schedule_policy` after provisioning.\nResource policy for scheduling instance operations."]
    pub fn instance_schedule_policy(
        &self,
    ) -> ListRef<DataComputeResourcePolicyInstanceSchedulePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_schedule_policy", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `snapshot_schedule_policy` after provisioning.\nPolicy for creating snapshots of persistent disks."]
    pub fn snapshot_schedule_policy(
        &self,
    ) -> ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_schedule_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_policy` after provisioning.\nRepresents the workload policy."]
    pub fn workload_policy(&self) -> ListRef<DataComputeResourcePolicyWorkloadPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_policy", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    type O = BlockAssignable<DataComputeResourcePolicyDiskConsistencyGroupPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicyDiskConsistencyGroupPolicyEl {}
impl BuildDataComputeResourcePolicyDiskConsistencyGroupPolicyEl {
    pub fn build(self) -> DataComputeResourcePolicyDiskConsistencyGroupPolicyEl {
        DataComputeResourcePolicyDiskConsistencyGroupPolicyEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
        DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicyDiskConsistencyGroupPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicyGroupPlacementPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_domain_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collocation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_topology: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_count: Option<PrimField<f64>>,
}
impl DataComputeResourcePolicyGroupPlacementPolicyEl {
    #[doc = "Set the field `availability_domain_count`.\n"]
    pub fn set_availability_domain_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.availability_domain_count = Some(v.into());
        self
    }
    #[doc = "Set the field `collocation`.\n"]
    pub fn set_collocation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.collocation = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_topology`.\n"]
    pub fn set_gpu_topology(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_topology = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_count`.\n"]
    pub fn set_vm_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vm_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeResourcePolicyGroupPlacementPolicyEl {
    type O = BlockAssignable<DataComputeResourcePolicyGroupPlacementPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicyGroupPlacementPolicyEl {}
impl BuildDataComputeResourcePolicyGroupPlacementPolicyEl {
    pub fn build(self) -> DataComputeResourcePolicyGroupPlacementPolicyEl {
        DataComputeResourcePolicyGroupPlacementPolicyEl {
            availability_domain_count: core::default::Default::default(),
            collocation: core::default::Default::default(),
            gpu_topology: core::default::Default::default(),
            vm_count: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicyGroupPlacementPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicyGroupPlacementPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicyGroupPlacementPolicyElRef {
        DataComputeResourcePolicyGroupPlacementPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicyGroupPlacementPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `availability_domain_count` after provisioning.\n"]
    pub fn availability_domain_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_domain_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `collocation` after provisioning.\n"]
    pub fn collocation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collocation", self.base))
    }
    #[doc = "Get a reference to the value of field `gpu_topology` after provisioning.\n"]
    pub fn gpu_topology(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gpu_topology", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_count` after provisioning.\n"]
    pub fn vm_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vm_count", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
}
impl DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    type O = BlockAssignable<DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {}
impl BuildDataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
    pub fn build(self) -> DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
        DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl {
            schedule: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
        DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
}
impl DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    type O = BlockAssignable<DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {}
impl BuildDataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
    pub fn build(self) -> DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
        DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl {
            schedule: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
        DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicyInstanceSchedulePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_start_schedule:
        Option<ListField<DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_stop_schedule:
        Option<ListField<DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>>,
}
impl DataComputeResourcePolicyInstanceSchedulePolicyEl {
    #[doc = "Set the field `expiration_time`.\n"]
    pub fn set_expiration_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expiration_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_start_schedule`.\n"]
    pub fn set_vm_start_schedule(
        mut self,
        v: impl Into<ListField<DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleEl>>,
    ) -> Self {
        self.vm_start_schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_stop_schedule`.\n"]
    pub fn set_vm_stop_schedule(
        mut self,
        v: impl Into<ListField<DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleEl>>,
    ) -> Self {
        self.vm_stop_schedule = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeResourcePolicyInstanceSchedulePolicyEl {
    type O = BlockAssignable<DataComputeResourcePolicyInstanceSchedulePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicyInstanceSchedulePolicyEl {}
impl BuildDataComputeResourcePolicyInstanceSchedulePolicyEl {
    pub fn build(self) -> DataComputeResourcePolicyInstanceSchedulePolicyEl {
        DataComputeResourcePolicyInstanceSchedulePolicyEl {
            expiration_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            vm_start_schedule: core::default::Default::default(),
            vm_stop_schedule: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicyInstanceSchedulePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicyInstanceSchedulePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicyInstanceSchedulePolicyElRef {
        DataComputeResourcePolicyInstanceSchedulePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicyInstanceSchedulePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\n"]
    pub fn expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_start_schedule` after provisioning.\n"]
    pub fn vm_start_schedule(
        &self,
    ) -> ListRef<DataComputeResourcePolicyInstanceSchedulePolicyElVmStartScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vm_start_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vm_stop_schedule` after provisioning.\n"]
    pub fn vm_stop_schedule(
        &self,
    ) -> ListRef<DataComputeResourcePolicyInstanceSchedulePolicyElVmStopScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vm_stop_schedule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retention_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_source_disk_delete: Option<PrimField<String>>,
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    #[doc = "Set the field `max_retention_days`.\n"]
    pub fn set_max_retention_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_retention_days = Some(v.into());
        self
    }
    #[doc = "Set the field `on_source_disk_delete`.\n"]
    pub fn set_on_source_disk_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_source_disk_delete = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    type O = BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {}
impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
    pub fn build(self) -> DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl {
            max_retention_days: core::default::Default::default(),
            on_source_disk_delete: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_retention_days` after provisioning.\n"]
    pub fn max_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_retention_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_source_disk_delete` after provisioning.\n"]
    pub fn on_source_disk_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_source_disk_delete", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days_in_cycle: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    #[doc = "Set the field `days_in_cycle`.\n"]
    pub fn set_days_in_cycle(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days_in_cycle = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    type O =
        BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {}
impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
    pub fn build(
        self,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl {
            days_in_cycle: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `days_in_cycle` after provisioning.\n"]
    pub fn days_in_cycle(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.days_in_cycle", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours_in_cycle: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    #[doc = "Set the field `hours_in_cycle`.\n"]
    pub fn set_hours_in_cycle(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours_in_cycle = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl
{
    type O = BlockAssignable<
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {}
impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
    pub fn build(
        self,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl {
            hours_in_cycle: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours_in_cycle` after provisioning.\n"]
    pub fn hours_in_cycle(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hours_in_cycle", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl
{
    type O = BlockAssignable<
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl
{}
impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
    pub fn build(
        self,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl
    {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl {
            day: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef
    {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_weeks: Option<
        SetField<
            DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl,
        >,
    >,
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    #[doc = "Set the field `day_of_weeks`.\n"]
    pub fn set_day_of_weeks(
        mut self,
        v : impl Into < SetField < DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksEl > >,
    ) -> Self {
        self.day_of_weeks = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl
{
    type O = BlockAssignable<
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {}
impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
    pub fn build(
        self,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl {
            day_of_weeks: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day_of_weeks` after provisioning.\n"]
    pub fn day_of_weeks(
        &self,
    ) -> SetRef<
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElDayOfWeeksElRef,
    > {
        SetRef::new(self.shared().clone(), format!("{}.day_of_weeks", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_schedule: Option<
        ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    hourly_schedule: Option<
        ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_schedule: Option<
        ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl>,
    >,
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    #[doc = "Set the field `daily_schedule`.\n"]
    pub fn set_daily_schedule(
        mut self,
        v: impl Into<
            ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleEl>,
        >,
    ) -> Self {
        self.daily_schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `hourly_schedule`.\n"]
    pub fn set_hourly_schedule(
        mut self,
        v: impl Into<
            ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleEl>,
        >,
    ) -> Self {
        self.hourly_schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `weekly_schedule`.\n"]
    pub fn set_weekly_schedule(
        mut self,
        v: impl Into<
            ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleEl>,
        >,
    ) -> Self {
        self.weekly_schedule = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    type O = BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {}
impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
    pub fn build(self) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl {
            daily_schedule: core::default::Default::default(),
            hourly_schedule: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `daily_schedule` after provisioning.\n"]
    pub fn daily_schedule(
        &self,
    ) -> ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElDailyScheduleElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.daily_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hourly_schedule` after provisioning.\n"]
    pub fn hourly_schedule(
        &self,
    ) -> ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElHourlyScheduleElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hourly_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(
        &self,
    ) -> ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElWeeklyScheduleElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_schedule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    chain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_flush: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_locations: Option<SetField<PrimField<String>>>,
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    #[doc = "Set the field `chain_name`.\n"]
    pub fn set_chain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.chain_name = Some(v.into());
        self
    }
    #[doc = "Set the field `guest_flush`.\n"]
    pub fn set_guest_flush(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.guest_flush = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_locations`.\n"]
    pub fn set_storage_locations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.storage_locations = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    type O = BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {}
impl BuildDataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
    pub fn build(self) -> DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
        DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl {
            chain_name: core::default::Default::default(),
            guest_flush: core::default::Default::default(),
            labels: core::default::Default::default(),
            storage_locations: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `chain_name` after provisioning.\n"]
    pub fn chain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.chain_name", self.base))
    }
    #[doc = "Get a reference to the value of field `guest_flush` after provisioning.\n"]
    pub fn guest_flush(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.guest_flush", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_locations` after provisioning.\n"]
    pub fn storage_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.storage_locations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicySnapshotSchedulePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_policy:
        Option<ListField<DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_properties:
        Option<ListField<DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>>,
}
impl DataComputeResourcePolicySnapshotSchedulePolicyEl {
    #[doc = "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(
        mut self,
        v: impl Into<ListField<DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyEl>>,
    ) -> Self {
        self.retention_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        mut self,
        v: impl Into<ListField<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleEl>>,
    ) -> Self {
        self.schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_properties`.\n"]
    pub fn set_snapshot_properties(
        mut self,
        v: impl Into<ListField<DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesEl>>,
    ) -> Self {
        self.snapshot_properties = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeResourcePolicySnapshotSchedulePolicyEl {
    type O = BlockAssignable<DataComputeResourcePolicySnapshotSchedulePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicySnapshotSchedulePolicyEl {}
impl BuildDataComputeResourcePolicySnapshotSchedulePolicyEl {
    pub fn build(self) -> DataComputeResourcePolicySnapshotSchedulePolicyEl {
        DataComputeResourcePolicySnapshotSchedulePolicyEl {
            retention_policy: core::default::Default::default(),
            schedule: core::default::Default::default(),
            snapshot_properties: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicySnapshotSchedulePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicySnapshotSchedulePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeResourcePolicySnapshotSchedulePolicyElRef {
        DataComputeResourcePolicySnapshotSchedulePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicySnapshotSchedulePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(
        &self,
    ) -> ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElRetentionPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(
        &self,
    ) -> ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
    #[doc = "Get a reference to the value of field `snapshot_properties` after provisioning.\n"]
    pub fn snapshot_properties(
        &self,
    ) -> ListRef<DataComputeResourcePolicySnapshotSchedulePolicyElSnapshotPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_properties", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeResourcePolicyWorkloadPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_topology: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_topology_distance: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataComputeResourcePolicyWorkloadPolicyEl {
    #[doc = "Set the field `accelerator_topology`.\n"]
    pub fn set_accelerator_topology(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_topology = Some(v.into());
        self
    }
    #[doc = "Set the field `max_topology_distance`.\n"]
    pub fn set_max_topology_distance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_topology_distance = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeResourcePolicyWorkloadPolicyEl {
    type O = BlockAssignable<DataComputeResourcePolicyWorkloadPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeResourcePolicyWorkloadPolicyEl {}
impl BuildDataComputeResourcePolicyWorkloadPolicyEl {
    pub fn build(self) -> DataComputeResourcePolicyWorkloadPolicyEl {
        DataComputeResourcePolicyWorkloadPolicyEl {
            accelerator_topology: core::default::Default::default(),
            max_topology_distance: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataComputeResourcePolicyWorkloadPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeResourcePolicyWorkloadPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataComputeResourcePolicyWorkloadPolicyElRef {
        DataComputeResourcePolicyWorkloadPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeResourcePolicyWorkloadPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_topology` after provisioning.\n"]
    pub fn accelerator_topology(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_topology", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_topology_distance` after provisioning.\n"]
    pub fn max_topology_distance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_topology_distance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
