use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeNodeGroupData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_policy: Option<PrimField<String>>,
    name: PrimField<String>,
    node_template: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_policy: Option<Vec<ComputeNodeGroupAutoscalingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window: Option<Vec<ComputeNodeGroupMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_settings: Option<Vec<ComputeNodeGroupShareSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeNodeGroupTimeoutsEl>,
    dynamic: ComputeNodeGroupDynamic,
}
struct ComputeNodeGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeNodeGroupData>,
}
#[derive(Clone)]
pub struct ComputeNodeGroup(Rc<ComputeNodeGroup_>);
impl ComputeNodeGroup {
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
    #[doc = "Set the field `description`.\nAn optional textual description of the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_size`.\nThe initial number of nodes in the node group. One of 'initial_size' or 'autoscaling_policy' must be configured on resource creation."]
    pub fn set_initial_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().initial_size = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_policy`.\nSpecifies how to handle instances when a node in the group undergoes maintenance. Set to one of: DEFAULT, RESTART_IN_PLACE, or MIGRATE_WITHIN_NODE_GROUP. The default value is DEFAULT."]
    pub fn set_maintenance_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maintenance_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nZone where this node group is located"]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_policy`.\n"]
    pub fn set_autoscaling_policy(
        self,
        v: impl Into<BlockAssignable<ComputeNodeGroupAutoscalingPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        self,
        v: impl Into<BlockAssignable<ComputeNodeGroupMaintenanceWindowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_window = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `share_settings`.\n"]
    pub fn set_share_settings(
        self,
        v: impl Into<BlockAssignable<ComputeNodeGroupShareSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().share_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.share_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeNodeGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the resource."]
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
    #[doc = "Get a reference to the value of field `initial_size` after provisioning.\nThe initial number of nodes in the node group. One of 'initial_size' or 'autoscaling_policy' must be configured on resource creation."]
    pub fn initial_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nSpecifies how to handle instances when a node in the group undergoes maintenance. Set to one of: DEFAULT, RESTART_IN_PLACE, or MIGRATE_WITHIN_NODE_GROUP. The default value is DEFAULT."]
    pub fn maintenance_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_template` after provisioning.\nThe URL of the node template to which this node group belongs."]
    pub fn node_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\nThe total number of nodes in the node group."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nZone where this node group is located"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_policy` after provisioning.\n"]
    pub fn autoscaling_policy(&self) -> ListRef<ComputeNodeGroupAutoscalingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> ListRef<ComputeNodeGroupMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `share_settings` after provisioning.\n"]
    pub fn share_settings(&self) -> ListRef<ComputeNodeGroupShareSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.share_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNodeGroupTimeoutsElRef {
        ComputeNodeGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeNodeGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeNodeGroup {}
impl ToListMappable for ComputeNodeGroup {
    type O = ListRef<ComputeNodeGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeNodeGroup_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_node_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeNodeGroup {
    pub tf_id: String,
    #[doc = "Name of the resource."]
    pub name: PrimField<String>,
    #[doc = "The URL of the node template to which this node group belongs."]
    pub node_template: PrimField<String>,
}
impl BuildComputeNodeGroup {
    pub fn build(self, stack: &mut Stack) -> ComputeNodeGroup {
        let out = ComputeNodeGroup(Rc::new(ComputeNodeGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeNodeGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                initial_size: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                name: self.name,
                node_template: self.node_template,
                project: core::default::Default::default(),
                zone: core::default::Default::default(),
                autoscaling_policy: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                share_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeNodeGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNodeGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeNodeGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the resource."]
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
    #[doc = "Get a reference to the value of field `initial_size` after provisioning.\nThe initial number of nodes in the node group. One of 'initial_size' or 'autoscaling_policy' must be configured on resource creation."]
    pub fn initial_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nSpecifies how to handle instances when a node in the group undergoes maintenance. Set to one of: DEFAULT, RESTART_IN_PLACE, or MIGRATE_WITHIN_NODE_GROUP. The default value is DEFAULT."]
    pub fn maintenance_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_template` after provisioning.\nThe URL of the node template to which this node group belongs."]
    pub fn node_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\nThe total number of nodes in the node group."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nZone where this node group is located"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_policy` after provisioning.\n"]
    pub fn autoscaling_policy(&self) -> ListRef<ComputeNodeGroupAutoscalingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> ListRef<ComputeNodeGroupMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `share_settings` after provisioning.\n"]
    pub fn share_settings(&self) -> ListRef<ComputeNodeGroupShareSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.share_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNodeGroupTimeoutsElRef {
        ComputeNodeGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeNodeGroupAutoscalingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl ComputeNodeGroupAutoscalingPolicyEl {
    #[doc = "Set the field `max_nodes`.\nMaximum size of the node group. Set to a value less than or equal\nto 100 and greater than or equal to min-nodes."]
    pub fn set_max_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `min_nodes`.\nMinimum size of the node group. Must be less\nthan or equal to max-nodes. The default value is 0."]
    pub fn set_min_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nThe autoscaling mode. Set to one of the following:\n  - OFF: Disables the autoscaler.\n  - ON: Enables scaling in and scaling out.\n  - ONLY_SCALE_OUT: Enables only scaling out.\n  You must use this mode if your node groups are configured to\n  restart their hosted VMs on minimal servers. Possible values: [\"OFF\", \"ON\", \"ONLY_SCALE_OUT\"]"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeNodeGroupAutoscalingPolicyEl {
    type O = BlockAssignable<ComputeNodeGroupAutoscalingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeNodeGroupAutoscalingPolicyEl {}
impl BuildComputeNodeGroupAutoscalingPolicyEl {
    pub fn build(self) -> ComputeNodeGroupAutoscalingPolicyEl {
        ComputeNodeGroupAutoscalingPolicyEl {
            max_nodes: core::default::Default::default(),
            min_nodes: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}
pub struct ComputeNodeGroupAutoscalingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNodeGroupAutoscalingPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeNodeGroupAutoscalingPolicyElRef {
        ComputeNodeGroupAutoscalingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeNodeGroupAutoscalingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_nodes` after provisioning.\nMaximum size of the node group. Set to a value less than or equal\nto 100 and greater than or equal to min-nodes."]
    pub fn max_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_nodes", self.base))
    }
    #[doc = "Get a reference to the value of field `min_nodes` after provisioning.\nMinimum size of the node group. Must be less\nthan or equal to max-nodes. The default value is 0."]
    pub fn min_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_nodes", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe autoscaling mode. Set to one of the following:\n  - OFF: Disables the autoscaler.\n  - ON: Enables scaling in and scaling out.\n  - ONLY_SCALE_OUT: Enables only scaling out.\n  You must use this mode if your node groups are configured to\n  restart their hosted VMs on minimal servers. Possible values: [\"OFF\", \"ON\", \"ONLY_SCALE_OUT\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeNodeGroupMaintenanceWindowEl {
    start_time: PrimField<String>,
}
impl ComputeNodeGroupMaintenanceWindowEl {}
impl ToListMappable for ComputeNodeGroupMaintenanceWindowEl {
    type O = BlockAssignable<ComputeNodeGroupMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeNodeGroupMaintenanceWindowEl {
    #[doc = "instances.start time of the window. This must be in UTC format that resolves to one of 00:00, 04:00, 08:00, 12:00, 16:00, or 20:00. For example, both 13:00-5 and 08:00 are valid."]
    pub start_time: PrimField<String>,
}
impl BuildComputeNodeGroupMaintenanceWindowEl {
    pub fn build(self) -> ComputeNodeGroupMaintenanceWindowEl {
        ComputeNodeGroupMaintenanceWindowEl {
            start_time: self.start_time,
        }
    }
}
pub struct ComputeNodeGroupMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNodeGroupMaintenanceWindowElRef {
    fn new(shared: StackShared, base: String) -> ComputeNodeGroupMaintenanceWindowElRef {
        ComputeNodeGroupMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeNodeGroupMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\ninstances.start time of the window. This must be in UTC format that resolves to one of 00:00, 04:00, 08:00, 12:00, 16:00, or 20:00. For example, both 13:00-5 and 08:00 are valid."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeNodeGroupShareSettingsElProjectMapEl {
    id: PrimField<String>,
    project_id: PrimField<String>,
}
impl ComputeNodeGroupShareSettingsElProjectMapEl {}
impl ToListMappable for ComputeNodeGroupShareSettingsElProjectMapEl {
    type O = BlockAssignable<ComputeNodeGroupShareSettingsElProjectMapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeNodeGroupShareSettingsElProjectMapEl {
    #[doc = ""]
    pub id: PrimField<String>,
    #[doc = "The project id/number should be the same as the key of this project config in the project map."]
    pub project_id: PrimField<String>,
}
impl BuildComputeNodeGroupShareSettingsElProjectMapEl {
    pub fn build(self) -> ComputeNodeGroupShareSettingsElProjectMapEl {
        ComputeNodeGroupShareSettingsElProjectMapEl {
            id: self.id,
            project_id: self.project_id,
        }
    }
}
pub struct ComputeNodeGroupShareSettingsElProjectMapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNodeGroupShareSettingsElProjectMapElRef {
    fn new(shared: StackShared, base: String) -> ComputeNodeGroupShareSettingsElProjectMapElRef {
        ComputeNodeGroupShareSettingsElProjectMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeNodeGroupShareSettingsElProjectMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe project id/number should be the same as the key of this project config in the project map."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeNodeGroupShareSettingsElDynamic {
    project_map: Option<DynamicBlock<ComputeNodeGroupShareSettingsElProjectMapEl>>,
}
#[derive(Serialize)]
pub struct ComputeNodeGroupShareSettingsEl {
    share_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_map: Option<Vec<ComputeNodeGroupShareSettingsElProjectMapEl>>,
    dynamic: ComputeNodeGroupShareSettingsElDynamic,
}
impl ComputeNodeGroupShareSettingsEl {
    #[doc = "Set the field `project_map`.\n"]
    pub fn set_project_map(
        mut self,
        v: impl Into<BlockAssignable<ComputeNodeGroupShareSettingsElProjectMapEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.project_map = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.project_map = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeNodeGroupShareSettingsEl {
    type O = BlockAssignable<ComputeNodeGroupShareSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeNodeGroupShareSettingsEl {
    #[doc = "Node group sharing type. Possible values: [\"ORGANIZATION\", \"SPECIFIC_PROJECTS\", \"LOCAL\"]"]
    pub share_type: PrimField<String>,
}
impl BuildComputeNodeGroupShareSettingsEl {
    pub fn build(self) -> ComputeNodeGroupShareSettingsEl {
        ComputeNodeGroupShareSettingsEl {
            share_type: self.share_type,
            project_map: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeNodeGroupShareSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNodeGroupShareSettingsElRef {
    fn new(shared: StackShared, base: String) -> ComputeNodeGroupShareSettingsElRef {
        ComputeNodeGroupShareSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeNodeGroupShareSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `share_type` after provisioning.\nNode group sharing type. Possible values: [\"ORGANIZATION\", \"SPECIFIC_PROJECTS\", \"LOCAL\"]"]
    pub fn share_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeNodeGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeNodeGroupTimeoutsEl {
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
impl ToListMappable for ComputeNodeGroupTimeoutsEl {
    type O = BlockAssignable<ComputeNodeGroupTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeNodeGroupTimeoutsEl {}
impl BuildComputeNodeGroupTimeoutsEl {
    pub fn build(self) -> ComputeNodeGroupTimeoutsEl {
        ComputeNodeGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeNodeGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNodeGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeNodeGroupTimeoutsElRef {
        ComputeNodeGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeNodeGroupTimeoutsElRef {
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
struct ComputeNodeGroupDynamic {
    autoscaling_policy: Option<DynamicBlock<ComputeNodeGroupAutoscalingPolicyEl>>,
    maintenance_window: Option<DynamicBlock<ComputeNodeGroupMaintenanceWindowEl>>,
    share_settings: Option<DynamicBlock<ComputeNodeGroupShareSettingsEl>>,
}
