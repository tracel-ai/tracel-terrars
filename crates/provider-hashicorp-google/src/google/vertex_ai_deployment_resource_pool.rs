use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VertexAiDeploymentResourcePoolData {
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
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dedicated_resources: Option<Vec<VertexAiDeploymentResourcePoolDedicatedResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VertexAiDeploymentResourcePoolTimeoutsEl>,
    dynamic: VertexAiDeploymentResourcePoolDynamic,
}
struct VertexAiDeploymentResourcePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VertexAiDeploymentResourcePoolData>,
}
#[derive(Clone)]
pub struct VertexAiDeploymentResourcePool(Rc<VertexAiDeploymentResourcePool_>);
impl VertexAiDeploymentResourcePool {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of deployment resource pool. eg us-central1"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `dedicated_resources`.\n"]
    pub fn set_dedicated_resources(
        self,
        v: impl Into<BlockAssignable<VertexAiDeploymentResourcePoolDedicatedResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dedicated_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dedicated_resources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VertexAiDeploymentResourcePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of deployment resource pool. The maximum length is 63 characters, and valid characters are '/^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$/'."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of deployment resource pool. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated_resources` after provisioning.\n"]
    pub fn dedicated_resources(
        &self,
    ) -> ListRef<VertexAiDeploymentResourcePoolDedicatedResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dedicated_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiDeploymentResourcePoolTimeoutsElRef {
        VertexAiDeploymentResourcePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VertexAiDeploymentResourcePool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VertexAiDeploymentResourcePool {}
impl ToListMappable for VertexAiDeploymentResourcePool {
    type O = ListRef<VertexAiDeploymentResourcePoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VertexAiDeploymentResourcePool_ {
    fn extract_resource_type(&self) -> String {
        "google_vertex_ai_deployment_resource_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVertexAiDeploymentResourcePool {
    pub tf_id: String,
    #[doc = "The resource name of deployment resource pool. The maximum length is 63 characters, and valid characters are '/^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$/'."]
    pub name: PrimField<String>,
}
impl BuildVertexAiDeploymentResourcePool {
    pub fn build(self, stack: &mut Stack) -> VertexAiDeploymentResourcePool {
        let out = VertexAiDeploymentResourcePool(Rc::new(VertexAiDeploymentResourcePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VertexAiDeploymentResourcePoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                dedicated_resources: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VertexAiDeploymentResourcePoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiDeploymentResourcePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VertexAiDeploymentResourcePoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of deployment resource pool. The maximum length is 63 characters, and valid characters are '/^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$/'."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of deployment resource pool. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated_resources` after provisioning.\n"]
    pub fn dedicated_resources(
        &self,
    ) -> ListRef<VertexAiDeploymentResourcePoolDedicatedResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dedicated_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiDeploymentResourcePoolTimeoutsElRef {
        VertexAiDeploymentResourcePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsEl {
    metric_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<f64>>,
}
impl VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsEl {
    #[doc = "Set the field `target`.\nThe target resource utilization in percentage (1% - 100%) for the given metric; once the real usage deviates from the target by a certain percentage, the machine replicas change. The default value is 60 (representing 60%) if not provided."]
    pub fn set_target(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsEl {
    type O =
        BlockAssignable<VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsEl {
    #[doc = "The resource metric name. Supported metrics: For Online Prediction: * 'aiplatform.googleapis.com/prediction/online/accelerator/duty_cycle' * 'aiplatform.googleapis.com/prediction/online/cpu/utilization'"]
    pub metric_name: PrimField<String>,
}
impl BuildVertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsEl {
    pub fn build(
        self,
    ) -> VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsEl {
        VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsEl {
            metric_name: self.metric_name,
            target: core::default::Default::default(),
        }
    }
}
pub struct VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsElRef {
        VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\nThe resource metric name. Supported metrics: For Online Prediction: * 'aiplatform.googleapis.com/prediction/online/accelerator/duty_cycle' * 'aiplatform.googleapis.com/prediction/online/cpu/utilization'"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\nThe target resource utilization in percentage (1% - 100%) for the given metric; once the real usage deviates from the target by a certain percentage, the machine replicas change. The default value is 60 (representing 60%) if not provided."]
    pub fn target(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
}
impl VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecEl {
    #[doc = "Set the field `accelerator_count`.\nThe number of accelerators to attach to the machine."]
    pub fn set_accelerator_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.accelerator_count = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_type`.\nThe type of accelerator(s) that may be attached to the machine as per accelerator_count. See possible values [here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/MachineSpec#AcceleratorType)."]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe type of the machine. See the [list of machine types supported for prediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecEl {
    type O = BlockAssignable<VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecEl {}
impl BuildVertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecEl {
    pub fn build(self) -> VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecEl {
        VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecEl {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}
pub struct VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecElRef {
        VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of accelerators to attach to the machine."]
    pub fn accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\nThe type of accelerator(s) that may be attached to the machine as per accelerator_count. See possible values [here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/MachineSpec#AcceleratorType)."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe type of the machine. See the [list of machine types supported for prediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiDeploymentResourcePoolDedicatedResourcesElDynamic {
    autoscaling_metric_specs: Option<
        DynamicBlock<VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsEl>,
    >,
    machine_spec:
        Option<DynamicBlock<VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecEl>>,
}
#[derive(Serialize)]
pub struct VertexAiDeploymentResourcePoolDedicatedResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_replica_count: Option<PrimField<f64>>,
    min_replica_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_metric_specs:
        Option<Vec<VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_spec: Option<Vec<VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecEl>>,
    dynamic: VertexAiDeploymentResourcePoolDedicatedResourcesElDynamic,
}
impl VertexAiDeploymentResourcePoolDedicatedResourcesEl {
    #[doc = "Set the field `max_replica_count`.\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If the requested value is too large, the deployment will error, but if deployment succeeds then the ability to scale the model to that many replicas is guaranteed (barring service outages). If traffic against the DeployedModel increases beyond what its replicas at maximum may handle, a portion of the traffic will be dropped. If this value is not provided, will use min_replica_count as the default value. The value of this field impacts the charge against Vertex CPU and GPU quotas. Specifically, you will be charged for max_replica_count * number of cores in the selected machine type) and (max_replica_count * number of GPUs per replica in the selected machine type)."]
    pub fn set_max_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_replica_count = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_metric_specs`.\n"]
    pub fn set_autoscaling_metric_specs(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoscaling_metric_specs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_metric_specs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        mut self,
        v: impl Into<BlockAssignable<VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.machine_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.machine_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiDeploymentResourcePoolDedicatedResourcesEl {
    type O = BlockAssignable<VertexAiDeploymentResourcePoolDedicatedResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiDeploymentResourcePoolDedicatedResourcesEl {
    #[doc = "The minimum number of machine replicas this DeployedModel will be always deployed on. This value must be greater than or equal to 1. If traffic against the DeployedModel increases, it may dynamically be deployed onto more replicas, and as traffic decreases, some of these extra replicas may be freed."]
    pub min_replica_count: PrimField<f64>,
}
impl BuildVertexAiDeploymentResourcePoolDedicatedResourcesEl {
    pub fn build(self) -> VertexAiDeploymentResourcePoolDedicatedResourcesEl {
        VertexAiDeploymentResourcePoolDedicatedResourcesEl {
            max_replica_count: core::default::Default::default(),
            min_replica_count: self.min_replica_count,
            autoscaling_metric_specs: core::default::Default::default(),
            machine_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiDeploymentResourcePoolDedicatedResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiDeploymentResourcePoolDedicatedResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiDeploymentResourcePoolDedicatedResourcesElRef {
        VertexAiDeploymentResourcePoolDedicatedResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiDeploymentResourcePoolDedicatedResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_replica_count` after provisioning.\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If the requested value is too large, the deployment will error, but if deployment succeeds then the ability to scale the model to that many replicas is guaranteed (barring service outages). If traffic against the DeployedModel increases beyond what its replicas at maximum may handle, a portion of the traffic will be dropped. If this value is not provided, will use min_replica_count as the default value. The value of this field impacts the charge against Vertex CPU and GPU quotas. Specifically, you will be charged for max_replica_count * number of cores in the selected machine type) and (max_replica_count * number of GPUs per replica in the selected machine type)."]
    pub fn max_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_replica_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_replica_count` after provisioning.\nThe minimum number of machine replicas this DeployedModel will be always deployed on. This value must be greater than or equal to 1. If traffic against the DeployedModel increases, it may dynamically be deployed onto more replicas, and as traffic decreases, some of these extra replicas may be freed."]
    pub fn min_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_replica_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_metric_specs` after provisioning.\n"]
    pub fn autoscaling_metric_specs(
        &self,
    ) -> ListRef<VertexAiDeploymentResourcePoolDedicatedResourcesElAutoscalingMetricSpecsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_metric_specs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(
        &self,
    ) -> ListRef<VertexAiDeploymentResourcePoolDedicatedResourcesElMachineSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiDeploymentResourcePoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl VertexAiDeploymentResourcePoolTimeoutsEl {
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
}
impl ToListMappable for VertexAiDeploymentResourcePoolTimeoutsEl {
    type O = BlockAssignable<VertexAiDeploymentResourcePoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiDeploymentResourcePoolTimeoutsEl {}
impl BuildVertexAiDeploymentResourcePoolTimeoutsEl {
    pub fn build(self) -> VertexAiDeploymentResourcePoolTimeoutsEl {
        VertexAiDeploymentResourcePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct VertexAiDeploymentResourcePoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiDeploymentResourcePoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VertexAiDeploymentResourcePoolTimeoutsElRef {
        VertexAiDeploymentResourcePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiDeploymentResourcePoolTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct VertexAiDeploymentResourcePoolDynamic {
    dedicated_resources: Option<DynamicBlock<VertexAiDeploymentResourcePoolDedicatedResourcesEl>>,
}
