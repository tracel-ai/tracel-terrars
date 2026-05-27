use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SpannerInstancePartitionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    config: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processing_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_config: Option<Vec<SpannerInstancePartitionAutoscalingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SpannerInstancePartitionTimeoutsEl>,
    dynamic: SpannerInstancePartitionDynamic,
}
struct SpannerInstancePartition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SpannerInstancePartitionData>,
}
#[derive(Clone)]
pub struct SpannerInstancePartition(Rc<SpannerInstancePartition_>);
impl SpannerInstancePartition {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `node_count`.\nThe number of nodes allocated to this instance partition. One node equals\n1000 processing units. Exactly one of either node_count, processing_units,\nor autoscaling_config must be present."]
    pub fn set_node_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `processing_units`.\nThe number of processing units allocated to this instance partition.\nExactly one of either node_count, processing_units, or autoscaling_config\nmust be present."]
    pub fn set_processing_units(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().processing_units = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_config`.\n"]
    pub fn set_autoscaling_config(
        self,
        v: impl Into<BlockAssignable<SpannerInstancePartitionAutoscalingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SpannerInstancePartitionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\nThe name of the instance partition's configuration (similar to a region) which\ndefines the geographic placement and replication of data in this instance partition."]
    pub fn config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe descriptive name for this instance partition as it appears in UIs.\nMust be unique per project and between 4 and 30 characters in length."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe instance to create the instance partition in."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the instance partition, which cannot be changed after\nthe instance partition is created. The name must be between 2 and 64 characters\nand match the regular expression [a-z][a-z0-9\\\\-]{0,61}[a-z0-9]."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes allocated to this instance partition. One node equals\n1000 processing units. Exactly one of either node_count, processing_units,\nor autoscaling_config must be present."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `processing_units` after provisioning.\nThe number of processing units allocated to this instance partition.\nExactly one of either node_count, processing_units, or autoscaling_config\nmust be present."]
    pub fn processing_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.processing_units", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current instance partition state. Possible values are:\nCREATING: The instance partition is being created. Resources are being\nallocated for the instance partition.\nREADY: The instance partition has been allocated resources and is ready for use."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(&self) -> ListRef<SpannerInstancePartitionAutoscalingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerInstancePartitionTimeoutsElRef {
        SpannerInstancePartitionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SpannerInstancePartition {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SpannerInstancePartition {}
impl ToListMappable for SpannerInstancePartition {
    type O = ListRef<SpannerInstancePartitionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SpannerInstancePartition_ {
    fn extract_resource_type(&self) -> String {
        "google_spanner_instance_partition".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSpannerInstancePartition {
    pub tf_id: String,
    #[doc = "The name of the instance partition's configuration (similar to a region) which\ndefines the geographic placement and replication of data in this instance partition."]
    pub config: PrimField<String>,
    #[doc = "The descriptive name for this instance partition as it appears in UIs.\nMust be unique per project and between 4 and 30 characters in length."]
    pub display_name: PrimField<String>,
    #[doc = "The instance to create the instance partition in."]
    pub instance: PrimField<String>,
    #[doc = "A unique identifier for the instance partition, which cannot be changed after\nthe instance partition is created. The name must be between 2 and 64 characters\nand match the regular expression [a-z][a-z0-9\\\\-]{0,61}[a-z0-9]."]
    pub name: PrimField<String>,
}
impl BuildSpannerInstancePartition {
    pub fn build(self, stack: &mut Stack) -> SpannerInstancePartition {
        let out = SpannerInstancePartition(Rc::new(SpannerInstancePartition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SpannerInstancePartitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                config: self.config,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                instance: self.instance,
                name: self.name,
                node_count: core::default::Default::default(),
                processing_units: core::default::Default::default(),
                project: core::default::Default::default(),
                autoscaling_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SpannerInstancePartitionRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstancePartitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SpannerInstancePartitionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\nThe name of the instance partition's configuration (similar to a region) which\ndefines the geographic placement and replication of data in this instance partition."]
    pub fn config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe descriptive name for this instance partition as it appears in UIs.\nMust be unique per project and between 4 and 30 characters in length."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe instance to create the instance partition in."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the instance partition, which cannot be changed after\nthe instance partition is created. The name must be between 2 and 64 characters\nand match the regular expression [a-z][a-z0-9\\\\-]{0,61}[a-z0-9]."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes allocated to this instance partition. One node equals\n1000 processing units. Exactly one of either node_count, processing_units,\nor autoscaling_config must be present."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `processing_units` after provisioning.\nThe number of processing units allocated to this instance partition.\nExactly one of either node_count, processing_units, or autoscaling_config\nmust be present."]
    pub fn processing_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.processing_units", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current instance partition state. Possible values are:\nCREATING: The instance partition is being created. Resources are being\nallocated for the instance partition.\nREADY: The instance partition has been allocated resources and is ready for use."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(&self) -> ListRef<SpannerInstancePartitionAutoscalingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerInstancePartitionTimeoutsElRef {
        SpannerInstancePartitionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_processing_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_processing_units: Option<PrimField<f64>>,
}
impl SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsEl {
    #[doc = "Set the field `max_nodes`.\nSpecifies maximum number of nodes allocated to the instance partition. If set, this number\nshould be greater than or equal to min_nodes."]
    pub fn set_max_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `max_processing_units`.\nSpecifies maximum number of processing units allocated to the instance partition.\nIf set, this number should be multiples of 1000 and be greater than or equal to\nmin_processing_units."]
    pub fn set_max_processing_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_processing_units = Some(v.into());
        self
    }
    #[doc = "Set the field `min_nodes`.\nSpecifies number of nodes allocated to the instance partition. If set, this number\nshould be greater than or equal to 1."]
    pub fn set_min_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `min_processing_units`.\nSpecifies minimum number of processing units allocated to the instance partition.\nIf set, this number should be multiples of 1000."]
    pub fn set_min_processing_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_processing_units = Some(v.into());
        self
    }
}
impl ToListMappable for SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsEl {
    type O = BlockAssignable<SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsEl {}
impl BuildSpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsEl {
    pub fn build(self) -> SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsEl {
        SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsEl {
            max_nodes: core::default::Default::default(),
            max_processing_units: core::default::Default::default(),
            min_nodes: core::default::Default::default(),
            min_processing_units: core::default::Default::default(),
        }
    }
}
pub struct SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsElRef {
        SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_nodes` after provisioning.\nSpecifies maximum number of nodes allocated to the instance partition. If set, this number\nshould be greater than or equal to min_nodes."]
    pub fn max_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_nodes", self.base))
    }
    #[doc = "Get a reference to the value of field `max_processing_units` after provisioning.\nSpecifies maximum number of processing units allocated to the instance partition.\nIf set, this number should be multiples of 1000 and be greater than or equal to\nmin_processing_units."]
    pub fn max_processing_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_processing_units", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_nodes` after provisioning.\nSpecifies number of nodes allocated to the instance partition. If set, this number\nshould be greater than or equal to 1."]
    pub fn min_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_nodes", self.base))
    }
    #[doc = "Get a reference to the value of field `min_processing_units` after provisioning.\nSpecifies minimum number of processing units allocated to the instance partition.\nIf set, this number should be multiples of 1000."]
    pub fn min_processing_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_processing_units", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    high_priority_cpu_utilization_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_utilization_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_cpu_utilization_percent: Option<PrimField<f64>>,
}
impl SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsEl {
    #[doc = "Set the field `high_priority_cpu_utilization_percent`.\nSpecifies the target high priority cpu utilization percentage that the autoscaler\nshould be trying to achieve for the instance partition.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn set_high_priority_cpu_utilization_percent(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.high_priority_cpu_utilization_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_utilization_percent`.\nSpecifies the target storage utilization percentage that the autoscaler\nshould be trying to achieve for the instance partition.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn set_storage_utilization_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_utilization_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `total_cpu_utilization_percent`.\nSpecifies the target total cpu utilization percentage that the autoscaler\nshould be trying to achieve for the instance partition.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization). The valid range is [10, 90] inclusive.\nIf not specified or set to 0, the autoscaler will skip scaling based on total cpu utilization.\nThe value should be higher than high_priority_cpu_utilization_percent if present."]
    pub fn set_total_cpu_utilization_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_cpu_utilization_percent = Some(v.into());
        self
    }
}
impl ToListMappable for SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsEl {
    type O = BlockAssignable<SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsEl {}
impl BuildSpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsEl {
    pub fn build(self) -> SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsEl {
        SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsEl {
            high_priority_cpu_utilization_percent: core::default::Default::default(),
            storage_utilization_percent: core::default::Default::default(),
            total_cpu_utilization_percent: core::default::Default::default(),
        }
    }
}
pub struct SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsElRef {
        SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `high_priority_cpu_utilization_percent` after provisioning.\nSpecifies the target high priority cpu utilization percentage that the autoscaler\nshould be trying to achieve for the instance partition.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn high_priority_cpu_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.high_priority_cpu_utilization_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_utilization_percent` after provisioning.\nSpecifies the target storage utilization percentage that the autoscaler\nshould be trying to achieve for the instance partition.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn storage_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_utilization_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_cpu_utilization_percent` after provisioning.\nSpecifies the target total cpu utilization percentage that the autoscaler\nshould be trying to achieve for the instance partition.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization). The valid range is [10, 90] inclusive.\nIf not specified or set to 0, the autoscaler will skip scaling based on total cpu utilization.\nThe value should be higher than high_priority_cpu_utilization_percent if present."]
    pub fn total_cpu_utilization_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_cpu_utilization_percent", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SpannerInstancePartitionAutoscalingConfigElDynamic {
    autoscaling_limits:
        Option<DynamicBlock<SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsEl>>,
    autoscaling_targets:
        Option<DynamicBlock<SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsEl>>,
}
#[derive(Serialize)]
pub struct SpannerInstancePartitionAutoscalingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_limits: Option<Vec<SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_targets:
        Option<Vec<SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsEl>>,
    dynamic: SpannerInstancePartitionAutoscalingConfigElDynamic,
}
impl SpannerInstancePartitionAutoscalingConfigEl {
    #[doc = "Set the field `autoscaling_limits`.\n"]
    pub fn set_autoscaling_limits(
        mut self,
        v: impl Into<BlockAssignable<SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoscaling_limits = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_limits = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `autoscaling_targets`.\n"]
    pub fn set_autoscaling_targets(
        mut self,
        v: impl Into<BlockAssignable<SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoscaling_targets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_targets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SpannerInstancePartitionAutoscalingConfigEl {
    type O = BlockAssignable<SpannerInstancePartitionAutoscalingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerInstancePartitionAutoscalingConfigEl {}
impl BuildSpannerInstancePartitionAutoscalingConfigEl {
    pub fn build(self) -> SpannerInstancePartitionAutoscalingConfigEl {
        SpannerInstancePartitionAutoscalingConfigEl {
            autoscaling_limits: core::default::Default::default(),
            autoscaling_targets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SpannerInstancePartitionAutoscalingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstancePartitionAutoscalingConfigElRef {
    fn new(shared: StackShared, base: String) -> SpannerInstancePartitionAutoscalingConfigElRef {
        SpannerInstancePartitionAutoscalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerInstancePartitionAutoscalingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `autoscaling_limits` after provisioning.\n"]
    pub fn autoscaling_limits(
        &self,
    ) -> ListRef<SpannerInstancePartitionAutoscalingConfigElAutoscalingLimitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_limits", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_targets` after provisioning.\n"]
    pub fn autoscaling_targets(
        &self,
    ) -> ListRef<SpannerInstancePartitionAutoscalingConfigElAutoscalingTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_targets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SpannerInstancePartitionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SpannerInstancePartitionTimeoutsEl {
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
impl ToListMappable for SpannerInstancePartitionTimeoutsEl {
    type O = BlockAssignable<SpannerInstancePartitionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerInstancePartitionTimeoutsEl {}
impl BuildSpannerInstancePartitionTimeoutsEl {
    pub fn build(self) -> SpannerInstancePartitionTimeoutsEl {
        SpannerInstancePartitionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SpannerInstancePartitionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerInstancePartitionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SpannerInstancePartitionTimeoutsElRef {
        SpannerInstancePartitionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerInstancePartitionTimeoutsElRef {
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
struct SpannerInstancePartitionDynamic {
    autoscaling_config: Option<DynamicBlock<SpannerInstancePartitionAutoscalingConfigEl>>,
}
