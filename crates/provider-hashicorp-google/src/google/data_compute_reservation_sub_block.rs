use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeReservationSubBlockData {
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
    reservation: PrimField<String>,
    reservation_block: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
struct DataComputeReservationSubBlock_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeReservationSubBlockData>,
}
#[derive(Clone)]
pub struct DataComputeReservationSubBlock(Rc<DataComputeReservationSubBlock_>);
impl DataComputeReservationSubBlock {
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
    #[doc = "Set the field `project`.\nThe project in which the resource belongs."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe zone where the reservation sub-block resides."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_info` after provisioning.\nHealth information for the reservation sub-block."]
    pub fn health_info(&self) -> ListRef<DataComputeReservationSubBlockHealthInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `in_use_count` after provisioning.\nThe number of instances that are currently in use on this reservation sub-block."]
    pub fn in_use_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.in_use_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#reservationSubBlock for reservation sub-blocks."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the reservation sub-block."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `physical_topology` after provisioning.\nThe physical topology of the reservation sub-block."]
    pub fn physical_topology(
        &self,
    ) -> ListRef<DataComputeReservationSubBlockPhysicalTopologyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.physical_topology", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation` after provisioning.\nThe name of the parent reservation."]
    pub fn reservation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reservation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_block` after provisioning.\nThe name of the parent reservation block."]
    pub fn reservation_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reservation_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_sub_block_maintenance` after provisioning.\nMaintenance information for this reservation sub-block."]
    pub fn reservation_sub_block_maintenance(
        &self,
    ) -> ListRef<DataComputeReservationSubBlockReservationSubBlockMaintenanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_sub_block_maintenance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined fully-qualified URL for this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_with_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the reservation sub-block."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sub_block_count` after provisioning.\nThe number of hosts that are allocated in this reservation sub-block."]
    pub fn sub_block_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sub_block_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone where the reservation sub-block resides."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeReservationSubBlock {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeReservationSubBlock {}
impl ToListMappable for DataComputeReservationSubBlock {
    type O = ListRef<DataComputeReservationSubBlockRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeReservationSubBlock_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_reservation_sub_block".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeReservationSubBlock {
    pub tf_id: String,
    #[doc = "The name of the reservation sub-block."]
    pub name: PrimField<String>,
    #[doc = "The name of the parent reservation."]
    pub reservation: PrimField<String>,
    #[doc = "The name of the parent reservation block."]
    pub reservation_block: PrimField<String>,
}
impl BuildDataComputeReservationSubBlock {
    pub fn build(self, stack: &mut Stack) -> DataComputeReservationSubBlock {
        let out = DataComputeReservationSubBlock(Rc::new(DataComputeReservationSubBlock_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeReservationSubBlockData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                reservation: self.reservation,
                reservation_block: self.reservation_block,
                zone: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeReservationSubBlockRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationSubBlockRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeReservationSubBlockRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_info` after provisioning.\nHealth information for the reservation sub-block."]
    pub fn health_info(&self) -> ListRef<DataComputeReservationSubBlockHealthInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `in_use_count` after provisioning.\nThe number of instances that are currently in use on this reservation sub-block."]
    pub fn in_use_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.in_use_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#reservationSubBlock for reservation sub-blocks."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the reservation sub-block."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `physical_topology` after provisioning.\nThe physical topology of the reservation sub-block."]
    pub fn physical_topology(
        &self,
    ) -> ListRef<DataComputeReservationSubBlockPhysicalTopologyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.physical_topology", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation` after provisioning.\nThe name of the parent reservation."]
    pub fn reservation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reservation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_block` after provisioning.\nThe name of the parent reservation block."]
    pub fn reservation_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reservation_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_sub_block_maintenance` after provisioning.\nMaintenance information for this reservation sub-block."]
    pub fn reservation_sub_block_maintenance(
        &self,
    ) -> ListRef<DataComputeReservationSubBlockReservationSubBlockMaintenanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_sub_block_maintenance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined fully-qualified URL for this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_with_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the reservation sub-block."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sub_block_count` after provisioning.\nThe number of hosts that are allocated in this reservation sub-block."]
    pub fn sub_block_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sub_block_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone where the reservation sub-block resides."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeReservationSubBlockHealthInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    degraded_host_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    degraded_infra_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthy_host_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthy_infra_count: Option<PrimField<f64>>,
}
impl DataComputeReservationSubBlockHealthInfoEl {
    #[doc = "Set the field `degraded_host_count`.\n"]
    pub fn set_degraded_host_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.degraded_host_count = Some(v.into());
        self
    }
    #[doc = "Set the field `degraded_infra_count`.\n"]
    pub fn set_degraded_infra_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.degraded_infra_count = Some(v.into());
        self
    }
    #[doc = "Set the field `health_status`.\n"]
    pub fn set_health_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.health_status = Some(v.into());
        self
    }
    #[doc = "Set the field `healthy_host_count`.\n"]
    pub fn set_healthy_host_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.healthy_host_count = Some(v.into());
        self
    }
    #[doc = "Set the field `healthy_infra_count`.\n"]
    pub fn set_healthy_infra_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.healthy_infra_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeReservationSubBlockHealthInfoEl {
    type O = BlockAssignable<DataComputeReservationSubBlockHealthInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationSubBlockHealthInfoEl {}
impl BuildDataComputeReservationSubBlockHealthInfoEl {
    pub fn build(self) -> DataComputeReservationSubBlockHealthInfoEl {
        DataComputeReservationSubBlockHealthInfoEl {
            degraded_host_count: core::default::Default::default(),
            degraded_infra_count: core::default::Default::default(),
            health_status: core::default::Default::default(),
            healthy_host_count: core::default::Default::default(),
            healthy_infra_count: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationSubBlockHealthInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationSubBlockHealthInfoElRef {
    fn new(shared: StackShared, base: String) -> DataComputeReservationSubBlockHealthInfoElRef {
        DataComputeReservationSubBlockHealthInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationSubBlockHealthInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `degraded_host_count` after provisioning.\n"]
    pub fn degraded_host_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.degraded_host_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `degraded_infra_count` after provisioning.\n"]
    pub fn degraded_infra_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.degraded_infra_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_status` after provisioning.\n"]
    pub fn health_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_status", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `healthy_host_count` after provisioning.\n"]
    pub fn healthy_host_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy_host_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `healthy_infra_count` after provisioning.\n"]
    pub fn healthy_infra_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy_infra_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeReservationSubBlockPhysicalTopologyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_block: Option<PrimField<String>>,
}
impl DataComputeReservationSubBlockPhysicalTopologyEl {
    #[doc = "Set the field `block`.\n"]
    pub fn set_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.block = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `sub_block`.\n"]
    pub fn set_sub_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sub_block = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeReservationSubBlockPhysicalTopologyEl {
    type O = BlockAssignable<DataComputeReservationSubBlockPhysicalTopologyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationSubBlockPhysicalTopologyEl {}
impl BuildDataComputeReservationSubBlockPhysicalTopologyEl {
    pub fn build(self) -> DataComputeReservationSubBlockPhysicalTopologyEl {
        DataComputeReservationSubBlockPhysicalTopologyEl {
            block: core::default::Default::default(),
            cluster: core::default::Default::default(),
            sub_block: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationSubBlockPhysicalTopologyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationSubBlockPhysicalTopologyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeReservationSubBlockPhysicalTopologyElRef {
        DataComputeReservationSubBlockPhysicalTopologyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationSubBlockPhysicalTopologyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `block` after provisioning.\n"]
    pub fn block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.block", self.base))
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `sub_block` after provisioning.\n"]
    pub fn sub_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sub_block", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeReservationSubBlockReservationSubBlockMaintenanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_maintenance_ongoing_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_maintenance_pending_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_ongoing_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_pending_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduling_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subblock_infra_maintenance_ongoing_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subblock_infra_maintenance_pending_count: Option<PrimField<f64>>,
}
impl DataComputeReservationSubBlockReservationSubBlockMaintenanceEl {
    #[doc = "Set the field `instance_maintenance_ongoing_count`.\n"]
    pub fn set_instance_maintenance_ongoing_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_maintenance_ongoing_count = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_maintenance_pending_count`.\n"]
    pub fn set_instance_maintenance_pending_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_maintenance_pending_count = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_ongoing_count`.\n"]
    pub fn set_maintenance_ongoing_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maintenance_ongoing_count = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_pending_count`.\n"]
    pub fn set_maintenance_pending_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maintenance_pending_count = Some(v.into());
        self
    }
    #[doc = "Set the field `scheduling_type`.\n"]
    pub fn set_scheduling_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scheduling_type = Some(v.into());
        self
    }
    #[doc = "Set the field `subblock_infra_maintenance_ongoing_count`.\n"]
    pub fn set_subblock_infra_maintenance_ongoing_count(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.subblock_infra_maintenance_ongoing_count = Some(v.into());
        self
    }
    #[doc = "Set the field `subblock_infra_maintenance_pending_count`.\n"]
    pub fn set_subblock_infra_maintenance_pending_count(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.subblock_infra_maintenance_pending_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeReservationSubBlockReservationSubBlockMaintenanceEl {
    type O = BlockAssignable<DataComputeReservationSubBlockReservationSubBlockMaintenanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationSubBlockReservationSubBlockMaintenanceEl {}
impl BuildDataComputeReservationSubBlockReservationSubBlockMaintenanceEl {
    pub fn build(self) -> DataComputeReservationSubBlockReservationSubBlockMaintenanceEl {
        DataComputeReservationSubBlockReservationSubBlockMaintenanceEl {
            instance_maintenance_ongoing_count: core::default::Default::default(),
            instance_maintenance_pending_count: core::default::Default::default(),
            maintenance_ongoing_count: core::default::Default::default(),
            maintenance_pending_count: core::default::Default::default(),
            scheduling_type: core::default::Default::default(),
            subblock_infra_maintenance_ongoing_count: core::default::Default::default(),
            subblock_infra_maintenance_pending_count: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationSubBlockReservationSubBlockMaintenanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationSubBlockReservationSubBlockMaintenanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeReservationSubBlockReservationSubBlockMaintenanceElRef {
        DataComputeReservationSubBlockReservationSubBlockMaintenanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationSubBlockReservationSubBlockMaintenanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_maintenance_ongoing_count` after provisioning.\n"]
    pub fn instance_maintenance_ongoing_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_maintenance_ongoing_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_maintenance_pending_count` after provisioning.\n"]
    pub fn instance_maintenance_pending_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_maintenance_pending_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_ongoing_count` after provisioning.\n"]
    pub fn maintenance_ongoing_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_ongoing_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_pending_count` after provisioning.\n"]
    pub fn maintenance_pending_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_pending_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scheduling_type` after provisioning.\n"]
    pub fn scheduling_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduling_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subblock_infra_maintenance_ongoing_count` after provisioning.\n"]
    pub fn subblock_infra_maintenance_ongoing_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subblock_infra_maintenance_ongoing_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subblock_infra_maintenance_pending_count` after provisioning.\n"]
    pub fn subblock_infra_maintenance_pending_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subblock_infra_maintenance_pending_count", self.base),
        )
    }
}
