use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeAutoscalerData {
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
    target: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_policy: Option<Vec<ComputeAutoscalerAutoscalingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeAutoscalerTimeoutsEl>,
    dynamic: ComputeAutoscalerDynamic,
}
struct ComputeAutoscaler_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeAutoscalerData>,
}
#[derive(Clone)]
pub struct ComputeAutoscaler(Rc<ComputeAutoscaler_>);
impl ComputeAutoscaler {
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
    #[doc = "Set the field `description`.\nAn optional description of this resource."]
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
    #[doc = "Set the field `zone`.\nURL of the zone where the instance group resides."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_policy`.\n"]
    pub fn set_autoscaling_policy(
        self,
        v: impl Into<BlockAssignable<ComputeAutoscalerAutoscalingPolicyEl>>,
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeAutoscalerTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\nURL of the managed instance group that this autoscaler will scale."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nURL of the zone where the instance group resides."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_policy` after provisioning.\n"]
    pub fn autoscaling_policy(&self) -> ListRef<ComputeAutoscalerAutoscalingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeAutoscalerTimeoutsElRef {
        ComputeAutoscalerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeAutoscaler {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeAutoscaler {}
impl ToListMappable for ComputeAutoscaler {
    type O = ListRef<ComputeAutoscalerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeAutoscaler_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_autoscaler".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeAutoscaler {
    pub tf_id: String,
    #[doc = "Name of the resource. The name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "URL of the managed instance group that this autoscaler will scale."]
    pub target: PrimField<String>,
}
impl BuildComputeAutoscaler {
    pub fn build(self, stack: &mut Stack) -> ComputeAutoscaler {
        let out = ComputeAutoscaler(Rc::new(ComputeAutoscaler_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeAutoscalerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                target: self.target,
                zone: core::default::Default::default(),
                autoscaling_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeAutoscalerRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeAutoscalerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeAutoscalerRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\nURL of the managed instance group that this autoscaler will scale."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nURL of the zone where the instance group resides."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_policy` after provisioning.\n"]
    pub fn autoscaling_policy(&self) -> ListRef<ComputeAutoscalerAutoscalingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeAutoscalerTimeoutsElRef {
        ComputeAutoscalerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeAutoscalerAutoscalingPolicyElCpuUtilizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    predictive_method: Option<PrimField<String>>,
    target: PrimField<f64>,
}
impl ComputeAutoscalerAutoscalingPolicyElCpuUtilizationEl {
    #[doc = "Set the field `predictive_method`.\nIndicates whether predictive autoscaling based on CPU metric is enabled. Valid values are:\n\n- NONE (default). No predictive method is used. The autoscaler scales the group to meet current demand based on real-time metrics.\n\n- OPTIMIZE_AVAILABILITY. Predictive autoscaling improves availability by monitoring daily and weekly load patterns and scaling out ahead of anticipated demand."]
    pub fn set_predictive_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.predictive_method = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeAutoscalerAutoscalingPolicyElCpuUtilizationEl {
    type O = BlockAssignable<ComputeAutoscalerAutoscalingPolicyElCpuUtilizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeAutoscalerAutoscalingPolicyElCpuUtilizationEl {
    #[doc = "The target CPU utilization that the autoscaler should maintain.\nMust be a float value in the range (0, 1]. If not specified, the\ndefault is 0.6.\n\nIf the CPU level is below the target utilization, the autoscaler\nscales down the number of instances until it reaches the minimum\nnumber of instances you specified or until the average CPU of\nyour instances reaches the target utilization.\n\nIf the average CPU is above the target utilization, the autoscaler\nscales up until it reaches the maximum number of instances you\nspecified or until the average utilization reaches the target\nutilization."]
    pub target: PrimField<f64>,
}
impl BuildComputeAutoscalerAutoscalingPolicyElCpuUtilizationEl {
    pub fn build(self) -> ComputeAutoscalerAutoscalingPolicyElCpuUtilizationEl {
        ComputeAutoscalerAutoscalingPolicyElCpuUtilizationEl {
            predictive_method: core::default::Default::default(),
            target: self.target,
        }
    }
}
pub struct ComputeAutoscalerAutoscalingPolicyElCpuUtilizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeAutoscalerAutoscalingPolicyElCpuUtilizationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeAutoscalerAutoscalingPolicyElCpuUtilizationElRef {
        ComputeAutoscalerAutoscalingPolicyElCpuUtilizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeAutoscalerAutoscalingPolicyElCpuUtilizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `predictive_method` after provisioning.\nIndicates whether predictive autoscaling based on CPU metric is enabled. Valid values are:\n\n- NONE (default). No predictive method is used. The autoscaler scales the group to meet current demand based on real-time metrics.\n\n- OPTIMIZE_AVAILABILITY. Predictive autoscaling improves availability by monitoring daily and weekly load patterns and scaling out ahead of anticipated demand."]
    pub fn predictive_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.predictive_method", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\nThe target CPU utilization that the autoscaler should maintain.\nMust be a float value in the range (0, 1]. If not specified, the\ndefault is 0.6.\n\nIf the CPU level is below the target utilization, the autoscaler\nscales down the number of instances until it reaches the minimum\nnumber of instances you specified or until the average CPU of\nyour instances reaches the target utilization.\n\nIf the average CPU is above the target utilization, the autoscaler\nscales up until it reaches the maximum number of instances you\nspecified or until the average utilization reaches the target\nutilization."]
    pub fn target(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl {
    target: PrimField<f64>,
}
impl ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl {}
impl ToListMappable for ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl {
    type O = BlockAssignable<ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl {
    #[doc = "Fraction of backend capacity utilization (set in HTTP(s) load\nbalancing configuration) that autoscaler should maintain. Must\nbe a positive float value. If not defined, the default is 0.8."]
    pub target: PrimField<f64>,
}
impl BuildComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl {
    pub fn build(self) -> ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl {
        ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl {
            target: self.target,
        }
    }
}
pub struct ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationElRef {
        ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\nFraction of backend capacity utilization (set in HTTP(s) load\nbalancing configuration) that autoscaler should maintain. Must\nbe a positive float value. If not defined, the default is 0.8."]
    pub fn target(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeAutoscalerAutoscalingPolicyElMetricEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_instance_assignment: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl ComputeAutoscalerAutoscalingPolicyElMetricEl {
    #[doc = "Set the field `filter`.\nA filter string to be used as the filter string for\na Stackdriver Monitoring TimeSeries.list API call.\nThis filter is used to select a specific TimeSeries for\nthe purpose of autoscaling and to determine whether the metric\nis exporting per-instance or per-group data.\n\nYou can only use the AND operator for joining selectors.\nYou can only use direct equality comparison operator (=) without\nany functions for each selector.\nYou can specify the metric in both the filter string and in the\nmetric field. However, if specified in both places, the metric must\nbe identical.\n\nThe monitored resource type determines what kind of values are\nexpected for the metric. If it is a gce_instance, the autoscaler\nexpects the metric to include a separate TimeSeries for each\ninstance in a group. In such a case, you cannot filter on resource\nlabels.\n\nIf the resource type is any other value, the autoscaler expects\nthis metric to contain values that apply to the entire autoscaled\ninstance group and resource label filtering can be performed to\npoint autoscaler at the correct TimeSeries to scale upon.\nThis is called a per-group metric for the purpose of autoscaling.\n\nIf not specified, the type defaults to gce_instance.\n\nYou should provide a filter that is selective enough to pick just\none TimeSeries for the autoscaled group or for each of the instances\n(if you are using gce_instance resource type). If multiple\nTimeSeries are returned upon the query execution, the autoscaler\nwill sum their respective values to obtain its scaling value."]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
    #[doc = "Set the field `single_instance_assignment`.\nIf scaling is based on a per-group metric value that represents the\ntotal amount of work to be done or resource usage, set this value to\nan amount assigned for a single instance of the scaled group.\nThe autoscaler will keep the number of instances proportional to the\nvalue of this metric, the metric itself should not change value due\nto group resizing.\n\nFor example, a good metric to use with the target is\n'pubsub.googleapis.com/subscription/num_undelivered_messages'\nor a custom metric exporting the total number of requests coming to\nyour instances.\n\nA bad example would be a metric exporting an average or median\nlatency, since this value can't include a chunk assignable to a\nsingle instance, it could be better used with utilization_target\ninstead."]
    pub fn set_single_instance_assignment(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.single_instance_assignment = Some(v.into());
        self
    }
    #[doc = "Set the field `target`.\nThe target value of the metric that autoscaler should\nmaintain. This must be a positive value. A utilization\nmetric scales number of virtual machines handling requests\nto increase or decrease proportionally to the metric.\n\nFor example, a good metric to use as a utilizationTarget is\nwww.googleapis.com/compute/instance/network/received_bytes_count.\nThe autoscaler will work to keep this value constant for each\nof the instances."]
    pub fn set_target(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nDefines how target utilization value is expressed for a\nStackdriver Monitoring metric. Possible values: [\"GAUGE\", \"DELTA_PER_SECOND\", \"DELTA_PER_MINUTE\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeAutoscalerAutoscalingPolicyElMetricEl {
    type O = BlockAssignable<ComputeAutoscalerAutoscalingPolicyElMetricEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeAutoscalerAutoscalingPolicyElMetricEl {
    #[doc = "The identifier (type) of the Stackdriver Monitoring metric.\nThe metric cannot have negative values.\n\nThe metric must have a value type of INT64 or DOUBLE."]
    pub name: PrimField<String>,
}
impl BuildComputeAutoscalerAutoscalingPolicyElMetricEl {
    pub fn build(self) -> ComputeAutoscalerAutoscalingPolicyElMetricEl {
        ComputeAutoscalerAutoscalingPolicyElMetricEl {
            filter: core::default::Default::default(),
            name: self.name,
            single_instance_assignment: core::default::Default::default(),
            target: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct ComputeAutoscalerAutoscalingPolicyElMetricElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeAutoscalerAutoscalingPolicyElMetricElRef {
    fn new(shared: StackShared, base: String) -> ComputeAutoscalerAutoscalingPolicyElMetricElRef {
        ComputeAutoscalerAutoscalingPolicyElMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeAutoscalerAutoscalingPolicyElMetricElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nA filter string to be used as the filter string for\na Stackdriver Monitoring TimeSeries.list API call.\nThis filter is used to select a specific TimeSeries for\nthe purpose of autoscaling and to determine whether the metric\nis exporting per-instance or per-group data.\n\nYou can only use the AND operator for joining selectors.\nYou can only use direct equality comparison operator (=) without\nany functions for each selector.\nYou can specify the metric in both the filter string and in the\nmetric field. However, if specified in both places, the metric must\nbe identical.\n\nThe monitored resource type determines what kind of values are\nexpected for the metric. If it is a gce_instance, the autoscaler\nexpects the metric to include a separate TimeSeries for each\ninstance in a group. In such a case, you cannot filter on resource\nlabels.\n\nIf the resource type is any other value, the autoscaler expects\nthis metric to contain values that apply to the entire autoscaled\ninstance group and resource label filtering can be performed to\npoint autoscaler at the correct TimeSeries to scale upon.\nThis is called a per-group metric for the purpose of autoscaling.\n\nIf not specified, the type defaults to gce_instance.\n\nYou should provide a filter that is selective enough to pick just\none TimeSeries for the autoscaled group or for each of the instances\n(if you are using gce_instance resource type). If multiple\nTimeSeries are returned upon the query execution, the autoscaler\nwill sum their respective values to obtain its scaling value."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe identifier (type) of the Stackdriver Monitoring metric.\nThe metric cannot have negative values.\n\nThe metric must have a value type of INT64 or DOUBLE."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `single_instance_assignment` after provisioning.\nIf scaling is based on a per-group metric value that represents the\ntotal amount of work to be done or resource usage, set this value to\nan amount assigned for a single instance of the scaled group.\nThe autoscaler will keep the number of instances proportional to the\nvalue of this metric, the metric itself should not change value due\nto group resizing.\n\nFor example, a good metric to use with the target is\n'pubsub.googleapis.com/subscription/num_undelivered_messages'\nor a custom metric exporting the total number of requests coming to\nyour instances.\n\nA bad example would be a metric exporting an average or median\nlatency, since this value can't include a chunk assignable to a\nsingle instance, it could be better used with utilization_target\ninstead."]
    pub fn single_instance_assignment(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_instance_assignment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\nThe target value of the metric that autoscaler should\nmaintain. This must be a positive value. A utilization\nmetric scales number of virtual machines handling requests\nto increase or decrease proportionally to the metric.\n\nFor example, a good metric to use as a utilizationTarget is\nwww.googleapis.com/compute/instance/network/received_bytes_count.\nThe autoscaler will work to keep this value constant for each\nof the instances."]
    pub fn target(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nDefines how target utilization value is expressed for a\nStackdriver Monitoring metric. Possible values: [\"GAUGE\", \"DELTA_PER_SECOND\", \"DELTA_PER_MINUTE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
}
impl ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {
    #[doc = "Set the field `fixed`.\nSpecifies a fixed number of VM instances. This must be a positive\ninteger."]
    pub fn set_fixed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.fixed = Some(v.into());
        self
    }
    #[doc = "Set the field `percent`.\nSpecifies a percentage of instances between 0 to 100%, inclusive.\nFor example, specify 80 for 80%."]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {
    type O =
        BlockAssignable<ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {}
impl BuildComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {
    pub fn build(
        self,
    ) -> ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {
        ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}
pub struct ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasElRef {
        ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fixed` after provisioning.\nSpecifies a fixed number of VM instances. This must be a positive\ninteger."]
    pub fn fixed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed", self.base))
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\nSpecifies a percentage of instances between 0 to 100%, inclusive.\nFor example, specify 80 for 80%."]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeAutoscalerAutoscalingPolicyElScaleInControlElDynamic {
    max_scaled_in_replicas: Option<
        DynamicBlock<ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl>,
    >,
}
#[derive(Serialize)]
pub struct ComputeAutoscalerAutoscalingPolicyElScaleInControlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    time_window_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_scaled_in_replicas:
        Option<Vec<ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl>>,
    dynamic: ComputeAutoscalerAutoscalingPolicyElScaleInControlElDynamic,
}
impl ComputeAutoscalerAutoscalingPolicyElScaleInControlEl {
    #[doc = "Set the field `time_window_sec`.\nHow long back autoscaling should look when computing recommendations\nto include directives regarding slower scale down, as described above."]
    pub fn set_time_window_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.time_window_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `max_scaled_in_replicas`.\n"]
    pub fn set_max_scaled_in_replicas(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.max_scaled_in_replicas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.max_scaled_in_replicas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeAutoscalerAutoscalingPolicyElScaleInControlEl {
    type O = BlockAssignable<ComputeAutoscalerAutoscalingPolicyElScaleInControlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeAutoscalerAutoscalingPolicyElScaleInControlEl {}
impl BuildComputeAutoscalerAutoscalingPolicyElScaleInControlEl {
    pub fn build(self) -> ComputeAutoscalerAutoscalingPolicyElScaleInControlEl {
        ComputeAutoscalerAutoscalingPolicyElScaleInControlEl {
            time_window_sec: core::default::Default::default(),
            max_scaled_in_replicas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeAutoscalerAutoscalingPolicyElScaleInControlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeAutoscalerAutoscalingPolicyElScaleInControlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeAutoscalerAutoscalingPolicyElScaleInControlElRef {
        ComputeAutoscalerAutoscalingPolicyElScaleInControlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeAutoscalerAutoscalingPolicyElScaleInControlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `time_window_sec` after provisioning.\nHow long back autoscaling should look when computing recommendations\nto include directives regarding slower scale down, as described above."]
    pub fn time_window_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_window_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_scaled_in_replicas` after provisioning.\n"]
    pub fn max_scaled_in_replicas(
        &self,
    ) -> ListRef<ComputeAutoscalerAutoscalingPolicyElScaleInControlElMaxScaledInReplicasElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_scaled_in_replicas", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeAutoscalerAutoscalingPolicyElScalingSchedulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    duration_sec: PrimField<f64>,
    min_required_replicas: PrimField<f64>,
    name: PrimField<String>,
    schedule: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
}
impl ComputeAutoscalerAutoscalingPolicyElScalingSchedulesEl {
    #[doc = "Set the field `description`.\nA description of a scaling schedule."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nA boolean value that specifies if a scaling schedule can influence autoscaler recommendations. If set to true, then a scaling schedule has no effect."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `time_zone`.\nThe time zone to be used when interpreting the schedule. The value of this field must be a time zone name from the tz database: http://en.wikipedia.org/wiki/Tz_database."]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeAutoscalerAutoscalingPolicyElScalingSchedulesEl {
    type O = BlockAssignable<ComputeAutoscalerAutoscalingPolicyElScalingSchedulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeAutoscalerAutoscalingPolicyElScalingSchedulesEl {
    #[doc = "The duration of time intervals (in seconds) for which this scaling schedule will be running. The minimum allowed value is 300."]
    pub duration_sec: PrimField<f64>,
    #[doc = "Minimum number of VM instances that autoscaler will recommend in time intervals starting according to schedule."]
    pub min_required_replicas: PrimField<f64>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = "The start timestamps of time intervals when this scaling schedule should provide a scaling signal. This field uses the extended cron format (with an optional year field)."]
    pub schedule: PrimField<String>,
}
impl BuildComputeAutoscalerAutoscalingPolicyElScalingSchedulesEl {
    pub fn build(self) -> ComputeAutoscalerAutoscalingPolicyElScalingSchedulesEl {
        ComputeAutoscalerAutoscalingPolicyElScalingSchedulesEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            duration_sec: self.duration_sec,
            min_required_replicas: self.min_required_replicas,
            name: self.name,
            schedule: self.schedule,
            time_zone: core::default::Default::default(),
        }
    }
}
pub struct ComputeAutoscalerAutoscalingPolicyElScalingSchedulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeAutoscalerAutoscalingPolicyElScalingSchedulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeAutoscalerAutoscalingPolicyElScalingSchedulesElRef {
        ComputeAutoscalerAutoscalingPolicyElScalingSchedulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeAutoscalerAutoscalingPolicyElScalingSchedulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of a scaling schedule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nA boolean value that specifies if a scaling schedule can influence autoscaler recommendations. If set to true, then a scaling schedule has no effect."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `duration_sec` after provisioning.\nThe duration of time intervals (in seconds) for which this scaling schedule will be running. The minimum allowed value is 300."]
    pub fn duration_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration_sec", self.base))
    }
    #[doc = "Get a reference to the value of field `min_required_replicas` after provisioning.\nMinimum number of VM instances that autoscaler will recommend in time intervals starting according to schedule."]
    pub fn min_required_replicas(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_required_replicas", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nThe start timestamps of time intervals when this scaling schedule should provide a scaling signal. This field uses the extended cron format (with an optional year field)."]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone to be used when interpreting the schedule. The value of this field must be a time zone name from the tz database: http://en.wikipedia.org/wiki/Tz_database."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeAutoscalerAutoscalingPolicyElDynamic {
    cpu_utilization: Option<DynamicBlock<ComputeAutoscalerAutoscalingPolicyElCpuUtilizationEl>>,
    load_balancing_utilization:
        Option<DynamicBlock<ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl>>,
    metric: Option<DynamicBlock<ComputeAutoscalerAutoscalingPolicyElMetricEl>>,
    scale_in_control: Option<DynamicBlock<ComputeAutoscalerAutoscalingPolicyElScaleInControlEl>>,
    scaling_schedules: Option<DynamicBlock<ComputeAutoscalerAutoscalingPolicyElScalingSchedulesEl>>,
}
#[derive(Serialize)]
pub struct ComputeAutoscalerAutoscalingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cooldown_period: Option<PrimField<f64>>,
    max_replicas: PrimField<f64>,
    min_replicas: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stabilization_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_utilization: Option<Vec<ComputeAutoscalerAutoscalingPolicyElCpuUtilizationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancing_utilization:
        Option<Vec<ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric: Option<Vec<ComputeAutoscalerAutoscalingPolicyElMetricEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_in_control: Option<Vec<ComputeAutoscalerAutoscalingPolicyElScaleInControlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_schedules: Option<Vec<ComputeAutoscalerAutoscalingPolicyElScalingSchedulesEl>>,
    dynamic: ComputeAutoscalerAutoscalingPolicyElDynamic,
}
impl ComputeAutoscalerAutoscalingPolicyEl {
    #[doc = "Set the field `cooldown_period`.\nThe number of seconds that the autoscaler should wait before it\nstarts collecting information from a new instance. This prevents\nthe autoscaler from collecting information when the instance is\ninitializing, during which the collected usage would not be\nreliable. The default time autoscaler waits is 60 seconds.\n\nVirtual machine initialization times might vary because of\nnumerous factors. We recommend that you test how long an\ninstance may take to initialize. To do this, create an instance\nand time the startup process."]
    pub fn set_cooldown_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cooldown_period = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nDefines operating mode for this policy."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `stabilization_period`.\nThe number of seconds that the autoscaler waits for load stabilization\nbefore making scale-in decisions.\n\nThis might appear as a delay in scaling in but it is an important mechanism\nfor your application to not have fluctuating size due to short term load\nfluctuations."]
    pub fn set_stabilization_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.stabilization_period = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_utilization`.\n"]
    pub fn set_cpu_utilization(
        mut self,
        v: impl Into<BlockAssignable<ComputeAutoscalerAutoscalingPolicyElCpuUtilizationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cpu_utilization = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cpu_utilization = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `load_balancing_utilization`.\n"]
    pub fn set_load_balancing_utilization(
        mut self,
        v: impl Into<BlockAssignable<ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.load_balancing_utilization = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.load_balancing_utilization = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(
        mut self,
        v: impl Into<BlockAssignable<ComputeAutoscalerAutoscalingPolicyElMetricEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scale_in_control`.\n"]
    pub fn set_scale_in_control(
        mut self,
        v: impl Into<BlockAssignable<ComputeAutoscalerAutoscalingPolicyElScaleInControlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scale_in_control = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scale_in_control = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scaling_schedules`.\n"]
    pub fn set_scaling_schedules(
        mut self,
        v: impl Into<BlockAssignable<ComputeAutoscalerAutoscalingPolicyElScalingSchedulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scaling_schedules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scaling_schedules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeAutoscalerAutoscalingPolicyEl {
    type O = BlockAssignable<ComputeAutoscalerAutoscalingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeAutoscalerAutoscalingPolicyEl {
    #[doc = "The maximum number of instances that the autoscaler can scale up\nto. This is required when creating or updating an autoscaler. The\nmaximum number of replicas should not be lower than minimal number\nof replicas."]
    pub max_replicas: PrimField<f64>,
    #[doc = "The minimum number of replicas that the autoscaler can scale down\nto. This cannot be less than 0. If not provided, autoscaler will\nchoose a default value depending on maximum number of instances\nallowed."]
    pub min_replicas: PrimField<f64>,
}
impl BuildComputeAutoscalerAutoscalingPolicyEl {
    pub fn build(self) -> ComputeAutoscalerAutoscalingPolicyEl {
        ComputeAutoscalerAutoscalingPolicyEl {
            cooldown_period: core::default::Default::default(),
            max_replicas: self.max_replicas,
            min_replicas: self.min_replicas,
            mode: core::default::Default::default(),
            stabilization_period: core::default::Default::default(),
            cpu_utilization: core::default::Default::default(),
            load_balancing_utilization: core::default::Default::default(),
            metric: core::default::Default::default(),
            scale_in_control: core::default::Default::default(),
            scaling_schedules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeAutoscalerAutoscalingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeAutoscalerAutoscalingPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeAutoscalerAutoscalingPolicyElRef {
        ComputeAutoscalerAutoscalingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeAutoscalerAutoscalingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cooldown_period` after provisioning.\nThe number of seconds that the autoscaler should wait before it\nstarts collecting information from a new instance. This prevents\nthe autoscaler from collecting information when the instance is\ninitializing, during which the collected usage would not be\nreliable. The default time autoscaler waits is 60 seconds.\n\nVirtual machine initialization times might vary because of\nnumerous factors. We recommend that you test how long an\ninstance may take to initialize. To do this, create an instance\nand time the startup process."]
    pub fn cooldown_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cooldown_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_replicas` after provisioning.\nThe maximum number of instances that the autoscaler can scale up\nto. This is required when creating or updating an autoscaler. The\nmaximum number of replicas should not be lower than minimal number\nof replicas."]
    pub fn max_replicas(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_replicas", self.base))
    }
    #[doc = "Get a reference to the value of field `min_replicas` after provisioning.\nThe minimum number of replicas that the autoscaler can scale down\nto. This cannot be less than 0. If not provided, autoscaler will\nchoose a default value depending on maximum number of instances\nallowed."]
    pub fn min_replicas(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_replicas", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nDefines operating mode for this policy."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `stabilization_period` after provisioning.\nThe number of seconds that the autoscaler waits for load stabilization\nbefore making scale-in decisions.\n\nThis might appear as a delay in scaling in but it is an important mechanism\nfor your application to not have fluctuating size due to short term load\nfluctuations."]
    pub fn stabilization_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stabilization_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_utilization` after provisioning.\n"]
    pub fn cpu_utilization(
        &self,
    ) -> ListRef<ComputeAutoscalerAutoscalingPolicyElCpuUtilizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cpu_utilization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `load_balancing_utilization` after provisioning.\n"]
    pub fn load_balancing_utilization(
        &self,
    ) -> ListRef<ComputeAutoscalerAutoscalingPolicyElLoadBalancingUtilizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.load_balancing_utilization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> ListRef<ComputeAutoscalerAutoscalingPolicyElMetricElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
    #[doc = "Get a reference to the value of field `scale_in_control` after provisioning.\n"]
    pub fn scale_in_control(
        &self,
    ) -> ListRef<ComputeAutoscalerAutoscalingPolicyElScaleInControlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scale_in_control", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeAutoscalerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeAutoscalerTimeoutsEl {
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
impl ToListMappable for ComputeAutoscalerTimeoutsEl {
    type O = BlockAssignable<ComputeAutoscalerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeAutoscalerTimeoutsEl {}
impl BuildComputeAutoscalerTimeoutsEl {
    pub fn build(self) -> ComputeAutoscalerTimeoutsEl {
        ComputeAutoscalerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeAutoscalerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeAutoscalerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeAutoscalerTimeoutsElRef {
        ComputeAutoscalerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeAutoscalerTimeoutsElRef {
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
struct ComputeAutoscalerDynamic {
    autoscaling_policy: Option<DynamicBlock<ComputeAutoscalerAutoscalingPolicyEl>>,
}
