use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VertexAiEndpointWithModelGardenDeploymentData {
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
    hugging_face_model_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publisher_model_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_config: Option<Vec<VertexAiEndpointWithModelGardenDeploymentDeployConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_config: Option<Vec<VertexAiEndpointWithModelGardenDeploymentEndpointConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_config: Option<Vec<VertexAiEndpointWithModelGardenDeploymentModelConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VertexAiEndpointWithModelGardenDeploymentTimeoutsEl>,
    dynamic: VertexAiEndpointWithModelGardenDeploymentDynamic,
}
struct VertexAiEndpointWithModelGardenDeployment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VertexAiEndpointWithModelGardenDeploymentData>,
}
#[derive(Clone)]
pub struct VertexAiEndpointWithModelGardenDeployment(
    Rc<VertexAiEndpointWithModelGardenDeployment_>,
);
impl VertexAiEndpointWithModelGardenDeployment {
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
    #[doc = "Set the field `hugging_face_model_id`.\nThe Hugging Face model to deploy.\nFormat: Hugging Face model ID like 'google/gemma-2-2b-it'."]
    pub fn set_hugging_face_model_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().hugging_face_model_id = Some(v.into());
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
    #[doc = "Set the field `publisher_model_name`.\nThe Model Garden model to deploy.\nFormat:\n'publishers/{publisher}/models/{publisher_model}@{version_id}', or\n'publishers/hf-{hugging-face-author}/models/{hugging-face-model-name}@001'."]
    pub fn set_publisher_model_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().publisher_model_name = Some(v.into());
        self
    }
    #[doc = "Set the field `deploy_config`.\n"]
    pub fn set_deploy_config(
        self,
        v: impl Into<BlockAssignable<VertexAiEndpointWithModelGardenDeploymentDeployConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deploy_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deploy_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `endpoint_config`.\n"]
    pub fn set_endpoint_config(
        self,
        v: impl Into<BlockAssignable<VertexAiEndpointWithModelGardenDeploymentEndpointConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `model_config`.\n"]
    pub fn set_model_config(
        self,
        v: impl Into<BlockAssignable<VertexAiEndpointWithModelGardenDeploymentModelConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<VertexAiEndpointWithModelGardenDeploymentTimeoutsEl>,
    ) -> Self {
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
    #[doc = "Get a reference to the value of field `deployed_model_display_name` after provisioning.\nOutput only. The display name assigned to the model deployed to the endpoint.\nThis is not required to delete the resource but is used for debug logging."]
    pub fn deployed_model_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployed_model_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployed_model_id` after provisioning.\nOutput only. The unique numeric ID that Vertex AI assigns to the model at the time it is deployed to the endpoint.\nIt is required to undeploy the model from the endpoint during resource deletion as described in\nhttps://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.endpoints/undeployModel."]
    pub fn deployed_model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployed_model_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nResource ID segment making up resource 'endpoint'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hugging_face_model_id` after provisioning.\nThe Hugging Face model to deploy.\nFormat: Hugging Face model ID like 'google/gemma-2-2b-it'."]
    pub fn hugging_face_model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hugging_face_model_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'location'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publisher_model_name` after provisioning.\nThe Model Garden model to deploy.\nFormat:\n'publishers/{publisher}/models/{publisher_model}@{version_id}', or\n'publishers/hf-{hugging-face-author}/models/{hugging-face-model-name}@001'."]
    pub fn publisher_model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publisher_model_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deploy_config` after provisioning.\n"]
    pub fn deploy_config(
        &self,
    ) -> ListRef<VertexAiEndpointWithModelGardenDeploymentDeployConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deploy_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_config` after provisioning.\n"]
    pub fn endpoint_config(
        &self,
    ) -> ListRef<VertexAiEndpointWithModelGardenDeploymentEndpointConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model_config` after provisioning.\n"]
    pub fn model_config(
        &self,
    ) -> ListRef<VertexAiEndpointWithModelGardenDeploymentModelConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiEndpointWithModelGardenDeploymentTimeoutsElRef {
        VertexAiEndpointWithModelGardenDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VertexAiEndpointWithModelGardenDeployment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VertexAiEndpointWithModelGardenDeployment {}
impl ToListMappable for VertexAiEndpointWithModelGardenDeployment {
    type O = ListRef<VertexAiEndpointWithModelGardenDeploymentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VertexAiEndpointWithModelGardenDeployment_ {
    fn extract_resource_type(&self) -> String {
        "google_vertex_ai_endpoint_with_model_garden_deployment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeployment {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'location'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildVertexAiEndpointWithModelGardenDeployment {
    pub fn build(self, stack: &mut Stack) -> VertexAiEndpointWithModelGardenDeployment {
        let out = VertexAiEndpointWithModelGardenDeployment(Rc::new(
            VertexAiEndpointWithModelGardenDeployment_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(VertexAiEndpointWithModelGardenDeploymentData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    hugging_face_model_id: core::default::Default::default(),
                    id: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                    publisher_model_name: core::default::Default::default(),
                    deploy_config: core::default::Default::default(),
                    endpoint_config: core::default::Default::default(),
                    model_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentRef {
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
    #[doc = "Get a reference to the value of field `deployed_model_display_name` after provisioning.\nOutput only. The display name assigned to the model deployed to the endpoint.\nThis is not required to delete the resource but is used for debug logging."]
    pub fn deployed_model_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployed_model_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployed_model_id` after provisioning.\nOutput only. The unique numeric ID that Vertex AI assigns to the model at the time it is deployed to the endpoint.\nIt is required to undeploy the model from the endpoint during resource deletion as described in\nhttps://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.endpoints/undeployModel."]
    pub fn deployed_model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployed_model_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nResource ID segment making up resource 'endpoint'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hugging_face_model_id` after provisioning.\nThe Hugging Face model to deploy.\nFormat: Hugging Face model ID like 'google/gemma-2-2b-it'."]
    pub fn hugging_face_model_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hugging_face_model_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'location'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publisher_model_name` after provisioning.\nThe Model Garden model to deploy.\nFormat:\n'publishers/{publisher}/models/{publisher_model}@{version_id}', or\n'publishers/hf-{hugging-face-author}/models/{hugging-face-model-name}@001'."]
    pub fn publisher_model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publisher_model_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deploy_config` after provisioning.\n"]
    pub fn deploy_config(
        &self,
    ) -> ListRef<VertexAiEndpointWithModelGardenDeploymentDeployConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deploy_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_config` after provisioning.\n"]
    pub fn endpoint_config(
        &self,
    ) -> ListRef<VertexAiEndpointWithModelGardenDeploymentEndpointConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model_config` after provisioning.\n"]
    pub fn model_config(
        &self,
    ) -> ListRef<VertexAiEndpointWithModelGardenDeploymentModelConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiEndpointWithModelGardenDeploymentTimeoutsElRef {
        VertexAiEndpointWithModelGardenDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsEl
{
    metric_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<f64>>,
}
impl VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsEl { # [doc = "Set the field `target`.\nThe target resource utilization in percentage (1% - 100%) for the given\nmetric; once the real usage deviates from the target by a certain\npercentage, the machine replicas change. The default value is 60\n(representing 60%) if not provided."] pub fn set_target (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . target = Some (v . into ()) ; self } }
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsEl { type O = BlockAssignable < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildVertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsEl
{
    #[doc = "The resource metric name.\nSupported metrics:\n\n* For Online Prediction:\n* 'aiplatform.googleapis.com/prediction/online/accelerator/duty_cycle'\n* 'aiplatform.googleapis.com/prediction/online/cpu/utilization'"]
    pub metric_name: PrimField<String>,
}
impl BuildVertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsEl { pub fn build (self) -> VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsEl { VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsEl { metric_name : self . metric_name , target : core :: default :: Default :: default () , } } }
pub struct VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsElRef { VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsElRef { shared : shared , base : base . to_string () , } } }
impl VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metric_name` after provisioning.\nThe resource metric name.\nSupported metrics:\n\n* For Online Prediction:\n* 'aiplatform.googleapis.com/prediction/online/accelerator/duty_cycle'\n* 'aiplatform.googleapis.com/prediction/online/cpu/utilization'"] pub fn metric_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.metric_name" , self . base)) } # [doc = "Get a reference to the value of field `target` after provisioning.\nThe target resource utilization in percentage (1% - 100%) for the given\nmetric; once the real usage deviates from the target by a certain\npercentage, the machine replicas change. The default value is 60\n(representing 60%) if not provided."] pub fn target (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.target" , self . base)) } }
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    reservation_affinity_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityEl { # [doc = "Set the field `key`.\nCorresponds to the label key of a reservation resource. To target a\nSPECIFIC_RESERVATION by name, use 'compute.googleapis.com/reservation-name'\nas the key and specify the name of your reservation as its value."] pub fn set_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . key = Some (v . into ()) ; self } # [doc = "Set the field `values`.\nCorresponds to the label values of a reservation resource. This must be the\nfull resource name of the reservation or reservation block."] pub fn set_values (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . values = Some (v . into ()) ; self } }
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityEl { type O = BlockAssignable < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildVertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityEl
{
    #[doc = "Specifies the reservation affinity type.\nPossible values:\nTYPE_UNSPECIFIED\nNO_RESERVATION\nANY_RESERVATION\nSPECIFIC_RESERVATION"]
    pub reservation_affinity_type: PrimField<String>,
}
impl BuildVertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityEl { pub fn build (self) -> VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityEl { VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityEl { key : core :: default :: Default :: default () , reservation_affinity_type : self . reservation_affinity_type , values : core :: default :: Default :: default () , } } }
pub struct VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityElRef { VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityElRef { shared : shared , base : base . to_string () , } } }
impl VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `key` after provisioning.\nCorresponds to the label key of a reservation resource. To target a\nSPECIFIC_RESERVATION by name, use 'compute.googleapis.com/reservation-name'\nas the key and specify the name of your reservation as its value."] pub fn key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.key" , self . base)) } # [doc = "Get a reference to the value of field `reservation_affinity_type` after provisioning.\nSpecifies the reservation affinity type.\nPossible values:\nTYPE_UNSPECIFIED\nNO_RESERVATION\nANY_RESERVATION\nSPECIFIC_RESERVATION"] pub fn reservation_affinity_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.reservation_affinity_type" , self . base)) } # [doc = "Get a reference to the value of field `values` after provisioning.\nCorresponds to the label values of a reservation resource. This must be the\nfull resource name of the reservation or reservation block."] pub fn values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.values" , self . base)) } }
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElDynamic { reservation_affinity : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityEl >> , }
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecEl { # [serde (skip_serializing_if = "Option::is_none")] accelerator_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] accelerator_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] machine_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] multihost_gpu_node_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] tpu_topology : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] reservation_affinity : Option < Vec < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityEl > > , dynamic : VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElDynamic , }
impl VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecEl {
    #[doc = "Set the field `accelerator_count`.\nThe number of accelerators to attach to the machine."]
    pub fn set_accelerator_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.accelerator_count = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_type`.\nPossible values:\nACCELERATOR_TYPE_UNSPECIFIED\nNVIDIA_TESLA_K80\nNVIDIA_TESLA_P100\nNVIDIA_TESLA_V100\nNVIDIA_TESLA_P4\nNVIDIA_TESLA_T4\nNVIDIA_TESLA_A100\nNVIDIA_A100_80GB\nNVIDIA_L4\nNVIDIA_H100_80GB\nNVIDIA_H100_MEGA_80GB\nNVIDIA_H200_141GB\nNVIDIA_B200\nTPU_V2\nTPU_V3\nTPU_V4_POD\nTPU_V5_LITEPOD"]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe type of the machine.\n\nSee the [list of machine types supported for\nprediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)\n\nSee the [list of machine types supported for custom\ntraining](https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types).\n\nFor DeployedModel this field is optional, and the default\nvalue is 'n1-standard-2'. For BatchPredictionJob or as part of\nWorkerPoolSpec this field is required."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `multihost_gpu_node_count`.\nThe number of nodes per replica for multihost GPU deployments."]
    pub fn set_multihost_gpu_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.multihost_gpu_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `tpu_topology`.\nThe topology of the TPUs. Corresponds to the TPU topologies available from\nGKE. (Example: tpu_topology: \"2x2x1\")."]
    pub fn set_tpu_topology(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tpu_topology = Some(v.into());
        self
    }
    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.reservation_affinity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.reservation_affinity = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecEl {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecEl
    {
        VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecEl {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            multihost_gpu_node_count: core::default::Default::default(),
            tpu_topology: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElRef
    {
        VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElRef { shared : shared , base : base . to_string () , }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElRef {
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
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\nPossible values:\nACCELERATOR_TYPE_UNSPECIFIED\nNVIDIA_TESLA_K80\nNVIDIA_TESLA_P100\nNVIDIA_TESLA_V100\nNVIDIA_TESLA_P4\nNVIDIA_TESLA_T4\nNVIDIA_TESLA_A100\nNVIDIA_A100_80GB\nNVIDIA_L4\nNVIDIA_H100_80GB\nNVIDIA_H100_MEGA_80GB\nNVIDIA_H200_141GB\nNVIDIA_B200\nTPU_V2\nTPU_V3\nTPU_V4_POD\nTPU_V5_LITEPOD"]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe type of the machine.\n\nSee the [list of machine types supported for\nprediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)\n\nSee the [list of machine types supported for custom\ntraining](https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types).\n\nFor DeployedModel this field is optional, and the default\nvalue is 'n1-standard-2'. For BatchPredictionJob or as part of\nWorkerPoolSpec this field is required."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `multihost_gpu_node_count` after provisioning.\nThe number of nodes per replica for multihost GPU deployments."]
    pub fn multihost_gpu_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multihost_gpu_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tpu_topology` after provisioning.\nThe topology of the TPUs. Corresponds to the TPU topologies available from\nGKE. (Example: tpu_topology: \"2x2x1\")."]
    pub fn tpu_topology(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tpu_topology", self.base))
    }
    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]    pub fn reservation_affinity (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElReservationAffinityElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_affinity", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElDynamic { autoscaling_metric_specs : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsEl >> , machine_spec : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecEl >> , }
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesEl { # [serde (skip_serializing_if = "Option::is_none")] max_replica_count : Option < PrimField < f64 > > , min_replica_count : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] required_replica_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] spot : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] autoscaling_metric_specs : Option < Vec < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsEl > > , # [serde (skip_serializing_if = "Option::is_none")] machine_spec : Option < Vec < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecEl > > , dynamic : VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElDynamic , }
impl VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesEl {
    #[doc = "Set the field `max_replica_count`.\nThe maximum number of replicas that may be deployed on when the traffic\nagainst it increases. If the requested value is too large, the deployment\nwill error, but if deployment succeeds then the ability to scale to that\nmany replicas is guaranteed (barring service outages). If traffic increases\nbeyond what its replicas at maximum may handle, a portion of the traffic\nwill be dropped. If this value is not provided, will use\nmin_replica_count as the default value.\n\nThe value of this field impacts the charge against Vertex CPU and GPU\nquotas. Specifically, you will be charged for (max_replica_count *\nnumber of cores in the selected machine type) and (max_replica_count *\nnumber of GPUs per replica in the selected machine type)."]
    pub fn set_max_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_replica_count = Some(v.into());
        self
    }
    #[doc = "Set the field `required_replica_count`.\nNumber of required available replicas for the deployment to succeed.\nThis field is only needed when partial deployment/mutation is\ndesired. If set, the deploy/mutate operation will succeed once\navailable_replica_count reaches required_replica_count, and the rest of\nthe replicas will be retried. If not set, the default\nrequired_replica_count will be min_replica_count."]
    pub fn set_required_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.required_replica_count = Some(v.into());
        self
    }
    #[doc = "Set the field `spot`.\nIf true, schedule the deployment workload on [spot\nVMs](https://cloud.google.com/kubernetes-engine/docs/concepts/spot-vms)."]
    pub fn set_spot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.spot = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_metric_specs`.\n"]
    pub fn set_autoscaling_metric_specs(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsEl >>,
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
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecEl >>,
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
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesEl {
    #[doc = "The minimum number of machine replicas that will be always deployed on.\nThis value must be greater than or equal to 1.\n\nIf traffic increases, it may dynamically be deployed onto more replicas,\nand as traffic decreases, some of these extra replicas may be freed."]
    pub min_replica_count: PrimField<f64>,
}
impl BuildVertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesEl {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesEl {
        VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesEl {
            max_replica_count: core::default::Default::default(),
            min_replica_count: self.min_replica_count,
            required_replica_count: core::default::Default::default(),
            spot: core::default::Default::default(),
            autoscaling_metric_specs: core::default::Default::default(),
            machine_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElRef {
        VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_replica_count` after provisioning.\nThe maximum number of replicas that may be deployed on when the traffic\nagainst it increases. If the requested value is too large, the deployment\nwill error, but if deployment succeeds then the ability to scale to that\nmany replicas is guaranteed (barring service outages). If traffic increases\nbeyond what its replicas at maximum may handle, a portion of the traffic\nwill be dropped. If this value is not provided, will use\nmin_replica_count as the default value.\n\nThe value of this field impacts the charge against Vertex CPU and GPU\nquotas. Specifically, you will be charged for (max_replica_count *\nnumber of cores in the selected machine type) and (max_replica_count *\nnumber of GPUs per replica in the selected machine type)."]
    pub fn max_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_replica_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_replica_count` after provisioning.\nThe minimum number of machine replicas that will be always deployed on.\nThis value must be greater than or equal to 1.\n\nIf traffic increases, it may dynamically be deployed onto more replicas,\nand as traffic decreases, some of these extra replicas may be freed."]
    pub fn min_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_replica_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `required_replica_count` after provisioning.\nNumber of required available replicas for the deployment to succeed.\nThis field is only needed when partial deployment/mutation is\ndesired. If set, the deploy/mutate operation will succeed once\navailable_replica_count reaches required_replica_count, and the rest of\nthe replicas will be retried. If not set, the default\nrequired_replica_count will be min_replica_count."]
    pub fn required_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.required_replica_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spot` after provisioning.\nIf true, schedule the deployment workload on [spot\nVMs](https://cloud.google.com/kubernetes-engine/docs/concepts/spot-vms)."]
    pub fn spot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot", self.base))
    }
    #[doc = "Get a reference to the value of field `autoscaling_metric_specs` after provisioning.\n"]    pub fn autoscaling_metric_specs (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElAutoscalingMetricSpecsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_metric_specs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(
        &self,
    ) -> ListRef<
        VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElMachineSpecElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentDeployConfigElDynamic {
    dedicated_resources: Option<
        DynamicBlock<VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesEl>,
    >,
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentDeployConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_tryout_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dedicated_resources:
        Option<Vec<VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesEl>>,
    dynamic: VertexAiEndpointWithModelGardenDeploymentDeployConfigElDynamic,
}
impl VertexAiEndpointWithModelGardenDeploymentDeployConfigEl {
    #[doc = "Set the field `fast_tryout_enabled`.\nIf true, enable the QMT fast tryout feature for this model if possible."]
    pub fn set_fast_tryout_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fast_tryout_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `system_labels`.\nSystem labels for Model Garden deployments.\nThese labels are managed by Google and for tracking purposes only."]
    pub fn set_system_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.system_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `dedicated_resources`.\n"]
    pub fn set_dedicated_resources(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dedicated_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dedicated_resources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentDeployConfigEl {
    type O = BlockAssignable<VertexAiEndpointWithModelGardenDeploymentDeployConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentDeployConfigEl {}
impl BuildVertexAiEndpointWithModelGardenDeploymentDeployConfigEl {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeploymentDeployConfigEl {
        VertexAiEndpointWithModelGardenDeploymentDeployConfigEl {
            fast_tryout_enabled: core::default::Default::default(),
            system_labels: core::default::Default::default(),
            dedicated_resources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentDeployConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentDeployConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentDeployConfigElRef {
        VertexAiEndpointWithModelGardenDeploymentDeployConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentDeployConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fast_tryout_enabled` after provisioning.\nIf true, enable the QMT fast tryout feature for this model if possible."]
    pub fn fast_tryout_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fast_tryout_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `system_labels` after provisioning.\nSystem labels for Model Garden deployments.\nThese labels are managed by Google and for tracking purposes only."]
    pub fn system_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.system_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated_resources` after provisioning.\n"]
    pub fn dedicated_resources(
        &self,
    ) -> ListRef<VertexAiEndpointWithModelGardenDeploymentDeployConfigElDedicatedResourcesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dedicated_resources", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsEl
{
    network: PrimField<String>,
    project_id: PrimField<String>,
}
impl VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsEl { }
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsEl { type O = BlockAssignable < VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildVertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsEl
{
    #[doc = "Required. The full name of the Google Compute Engine network.\nFormat: projects/{project}/global/networks/{network}."]
    pub network: PrimField<String>,
    #[doc = "Required. Project id used to create forwarding rule."]
    pub project_id: PrimField<String>,
}
impl BuildVertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsEl { pub fn build (self) -> VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsEl { VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsEl { network : self . network , project_id : self . project_id , } } }
pub struct VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsElRef { VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsElRef { shared : shared , base : base . to_string () , } } }
impl VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `error_message` after provisioning.\nOutput only. Error message if the PSC service automation failed."] pub fn error_message (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.error_message" , self . base)) } # [doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\nOutput only. Forwarding rule created by the PSC service automation."] pub fn forwarding_rule (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.forwarding_rule" , self . base)) } # [doc = "Get a reference to the value of field `ip_address` after provisioning.\nOutput only. IP address rule created by the PSC service automation."] pub fn ip_address (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ip_address" , self . base)) } # [doc = "Get a reference to the value of field `network` after provisioning.\nRequired. The full name of the Google Compute Engine network.\nFormat: projects/{project}/global/networks/{network}."] pub fn network (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.network" , self . base)) } # [doc = "Get a reference to the value of field `project_id` after provisioning.\nRequired. Project id used to create forwarding rule."] pub fn project_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.project_id" , self . base)) } # [doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the PSC service automation."] pub fn state (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.state" , self . base)) } }
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElDynamic { psc_automation_configs : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsEl >> , }
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigEl { enable_private_service_connect : PrimField < bool > , # [serde (skip_serializing_if = "Option::is_none")] project_allowlist : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] psc_automation_configs : Option < Vec < VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsEl > > , dynamic : VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElDynamic , }
impl VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigEl {
    #[doc = "Set the field `project_allowlist`.\nA list of Projects from which the forwarding rule will target the service attachment."]
    pub fn set_project_allowlist(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.project_allowlist = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_automation_configs`.\n"]
    pub fn set_psc_automation_configs(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.psc_automation_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_automation_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigEl
{
    #[doc = "Required. If true, expose the IndexEndpoint via private service connect."]
    pub enable_private_service_connect: PrimField<bool>,
}
impl BuildVertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigEl {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigEl
    {
        VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigEl {
            enable_private_service_connect: self.enable_private_service_connect,
            project_allowlist: core::default::Default::default(),
            psc_automation_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElRef
    {
        VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_private_service_connect` after provisioning.\nRequired. If true, expose the IndexEndpoint via private service connect."]
    pub fn enable_private_service_connect(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_service_connect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project_allowlist` after provisioning.\nA list of Projects from which the forwarding rule will target the service attachment."]
    pub fn project_allowlist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.project_allowlist", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\nOutput only. The name of the generated service attachment resource.\nThis is only populated if the endpoint is deployed with PrivateServiceConnect."]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_automation_configs` after provisioning.\n"]    pub fn psc_automation_configs (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElPscAutomationConfigsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_automation_configs", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentEndpointConfigElDynamic {
    private_service_connect_config: Option<
        DynamicBlock<
            VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentEndpointConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dedicated_endpoint_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_service_connect_config: Option<
        Vec<VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigEl>,
    >,
    dynamic: VertexAiEndpointWithModelGardenDeploymentEndpointConfigElDynamic,
}
impl VertexAiEndpointWithModelGardenDeploymentEndpointConfigEl {
    #[doc = "Set the field `dedicated_endpoint_enabled`.\nIf true, the endpoint will be exposed through a dedicated\nDNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS\nwill be isolated from other users' traffic and will have better\nperformance and reliability. Note: Once you enabled dedicated endpoint,\nyou won't be able to send request to the shared DNS\n{region}-aiplatform.googleapis.com. The limitations will be removed soon."]
    pub fn set_dedicated_endpoint_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dedicated_endpoint_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint_display_name`.\nThe user-specified display name of the endpoint. If not set, a\ndefault name will be used."]
    pub fn set_endpoint_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `private_service_connect_config`.\n"]
    pub fn set_private_service_connect_config(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.private_service_connect_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.private_service_connect_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentEndpointConfigEl {
    type O = BlockAssignable<VertexAiEndpointWithModelGardenDeploymentEndpointConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentEndpointConfigEl {}
impl BuildVertexAiEndpointWithModelGardenDeploymentEndpointConfigEl {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeploymentEndpointConfigEl {
        VertexAiEndpointWithModelGardenDeploymentEndpointConfigEl {
            dedicated_endpoint_enabled: core::default::Default::default(),
            endpoint_display_name: core::default::Default::default(),
            private_service_connect_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentEndpointConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentEndpointConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentEndpointConfigElRef {
        VertexAiEndpointWithModelGardenDeploymentEndpointConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentEndpointConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dedicated_endpoint_enabled` after provisioning.\nIf true, the endpoint will be exposed through a dedicated\nDNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS\nwill be isolated from other users' traffic and will have better\nperformance and reliability. Note: Once you enabled dedicated endpoint,\nyou won't be able to send request to the shared DNS\n{region}-aiplatform.googleapis.com. The limitations will be removed soon."]
    pub fn dedicated_endpoint_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dedicated_endpoint_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_display_name` after provisioning.\nThe user-specified display name of the endpoint. If not set, a\ndefault name will be used."]
    pub fn endpoint_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_display_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_service_connect_config` after provisioning.\n"]
    pub fn private_service_connect_config(
        &self,
    ) -> ListRef<
        VertexAiEndpointWithModelGardenDeploymentEndpointConfigElPrivateServiceConnectConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_service_connect_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvEl {}
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvEl {
    type O =
        BlockAssignable<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvEl {
    #[doc = "Name of the environment variable. Must be a valid C identifier."]
    pub name: PrimField<String>,
    #[doc = "Variables that reference a $(VAR_NAME) are expanded\nusing the previous defined environment variables in the container and\nany service environment variables. If a variable cannot be resolved,\nthe reference in the input string will be unchanged. The $(VAR_NAME)\nsyntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped\nreferences will never be expanded, regardless of whether the variable\nexists or not."]
    pub value: PrimField<String>,
}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvEl {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvEl {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvElRef {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the environment variable. Must be a valid C identifier."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nVariables that reference a $(VAR_NAME) are expanded\nusing the previous defined environment variables in the container and\nany service environment variables. If a variable cannot be resolved,\nthe reference in the input string will be unchanged. The $(VAR_NAME)\nsyntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped\nreferences will never be expanded, regardless of whether the variable\nexists or not."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_port: Option<PrimField<f64>>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsEl {
    #[doc = "Set the field `container_port`.\nThe number of the port to expose on the pod's IP address.\nMust be a valid port number, between 1 and 65535 inclusive."]
    pub fn set_container_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_port = Some(v.into());
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsEl {
}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsEl {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsEl {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsEl {
            container_port: core::default::Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsElRef {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_port` after provisioning.\nThe number of the port to expose on the pod's IP address.\nMust be a valid port number, between 1 and 65535 inclusive."]
    pub fn container_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_port", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecEl {
    #[doc = "Set the field `command`.\nCommand is the command line to execute inside the container, the working\ndirectory for the command is root ('/') in the container's filesystem.\nThe command is simply exec'd, it is not run inside a shell, so\ntraditional shell instructions ('|', etc) won't work. To use a shell, you\nneed to explicitly call out to that shell. Exit status of 0 is treated as\nlive/healthy and non-zero is unhealthy."]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecEl {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecEl
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecEl {
            command: core::default::Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecElRef
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecElRef { shared : shared , base : base . to_string () , }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\nCommand is the command line to execute inside the container, the working\ndirectory for the command is root ('/') in the container's filesystem.\nThe command is simply exec'd, it is not run inside a shell, so\ntraditional shell instructions ('|', etc) won't work. To use a shell, you\nneed to explicitly call out to that shell. Exit status of 0 is treated as\nlive/healthy and non-zero is unhealthy."]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcEl {
    #[doc = "Set the field `port`.\nPort number of the gRPC service. Number must be in the range 1 to 65535."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nService is the name of the service to place in the gRPC\nHealthCheckRequest. See\nhttps://github.com/grpc/grpc/blob/master/doc/health-checking.md.\n\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcEl {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcEl
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcElRef
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcElRef { shared : shared , base : base . to_string () , }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number of the gRPC service. Number must be in the range 1 to 65535."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nService is the name of the service to place in the gRPC\nHealthCheckRequest. See\nhttps://github.com/grpc/grpc/blob/master/doc/health-checking.md.\n\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersEl { # [doc = "Set the field `name`.\nThe header field name.\nThis will be canonicalized upon output, so case-variant names will be\nunderstood as the same header."] pub fn set_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . name = Some (v . into ()) ; self } # [doc = "Set the field `value`.\nThe header field value"] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersEl { type O = BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersEl { pub fn build (self) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersEl { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersEl { name : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersElRef { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersElRef { shared : shared , base : base . to_string () , } } }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nThe header field name.\nThis will be canonicalized upon output, so case-variant names will be\nunderstood as the same header."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\nThe header field value"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElDynamic { http_headers : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersEl >> , }
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetEl { # [serde (skip_serializing_if = "Option::is_none")] host : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] port : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] scheme : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] http_headers : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersEl > > , dynamic : VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElDynamic , }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetEl {
    #[doc = "Set the field `host`.\nHost name to connect to, defaults to the model serving container's IP.\nYou probably want to set \"Host\" in httpHeaders instead."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nPath to access on the HTTP server."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `scheme`.\nScheme to use for connecting to the host.\nDefaults to HTTP. Acceptable values are \"HTTP\" or \"HTTPS\"."]
    pub fn set_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_headers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetEl
{}
impl
    BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetEl
{
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetEl
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetEl { host : core :: default :: Default :: default () , path : core :: default :: Default :: default () , port : core :: default :: Default :: default () , scheme : core :: default :: Default :: default () , http_headers : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElRef { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElRef { shared : shared , base : base . to_string () , } } }
impl
    VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nHost name to connect to, defaults to the model serving container's IP.\nYou probably want to set \"Host\" in httpHeaders instead."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to access on the HTTP server."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `scheme` after provisioning.\nScheme to use for connecting to the host.\nDefaults to HTTP. Acceptable values are \"HTTP\" or \"HTTPS\"."]
    pub fn scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]    pub fn http_headers (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElHttpHeadersElRef >{
        ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketEl {
    #[doc = "Set the field `host`.\nOptional: Host name to connect to, defaults to the model serving\ncontainer's IP."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketEl { type O = BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketEl { pub fn build (self) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketEl { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketEl { host : core :: default :: Default :: default () , port : core :: default :: Default :: default () , } } }
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketElRef { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketElRef { shared : shared , base : base . to_string () , } } }
impl
    VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nOptional: Host name to connect to, defaults to the model serving\ncontainer's IP."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElDynamic { exec : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecEl >> , grpc : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcEl >> , http_get : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetEl >> , tcp_socket : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketEl >> , }
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeEl { # [serde (skip_serializing_if = "Option::is_none")] failure_threshold : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] initial_delay_seconds : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] period_seconds : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] success_threshold : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] timeout_seconds : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] exec : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecEl > > , # [serde (skip_serializing_if = "Option::is_none")] grpc : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcEl > > , # [serde (skip_serializing_if = "Option::is_none")] http_get : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetEl > > , # [serde (skip_serializing_if = "Option::is_none")] tcp_socket : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketEl > > , dynamic : VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElDynamic , }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeEl {
    #[doc = "Set the field `failure_threshold`.\nNumber of consecutive failures before the probe is considered failed.\nDefaults to 3. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'failureThreshold'."]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_delay_seconds`.\nNumber of seconds to wait before starting the probe. Defaults to 0.\nMinimum value is 0.\n\nMaps to Kubernetes probe argument 'initialDelaySeconds'."]
    pub fn set_initial_delay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe. Default to 10 seconds.\nMinimum value is 1. Must be less than timeout_seconds.\n\nMaps to Kubernetes probe argument 'periodSeconds'."]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `success_threshold`.\nNumber of consecutive successes before the probe is considered successful.\nDefaults to 1. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'successThreshold'."]
    pub fn set_success_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out. Defaults to 1 second.\nMinimum value is 1. Must be greater or equal to period_seconds.\n\nMaps to Kubernetes probe argument 'timeoutSeconds'."]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `exec`.\n"]
    pub fn set_exec(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_get = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tcp_socket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_socket = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeEl {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeEl {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeEl {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            success_threshold: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            exec: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElRef {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\nNumber of consecutive failures before the probe is considered failed.\nDefaults to 3. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'failureThreshold'."]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds to wait before starting the probe. Defaults to 0.\nMinimum value is 0.\n\nMaps to Kubernetes probe argument 'initialDelaySeconds'."]
    pub fn initial_delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe. Default to 10 seconds.\nMinimum value is 1. Must be less than timeout_seconds.\n\nMaps to Kubernetes probe argument 'periodSeconds'."]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success_threshold` after provisioning.\nNumber of consecutive successes before the probe is considered successful.\nDefaults to 1. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'successThreshold'."]
    pub fn success_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out. Defaults to 1 second.\nMinimum value is 1. Must be greater or equal to period_seconds.\n\nMaps to Kubernetes probe argument 'timeoutSeconds'."]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exec` after provisioning.\n"]
    pub fn exec(
        &self,
    ) -> ListRef<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElExecElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.exec", self.base))
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(
        &self,
    ) -> ListRef<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElGrpcElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]    pub fn http_get (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElHttpGetElRef >{
        ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]    pub fn tcp_socket (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElTcpSocketElRef >{
        ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecEl {
    #[doc = "Set the field `command`.\nCommand is the command line to execute inside the container, the working\ndirectory for the command is root ('/') in the container's filesystem.\nThe command is simply exec'd, it is not run inside a shell, so\ntraditional shell instructions ('|', etc) won't work. To use a shell, you\nneed to explicitly call out to that shell. Exit status of 0 is treated as\nlive/healthy and non-zero is unhealthy."]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecEl
{}
impl
    BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecEl
{
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecEl
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecEl {
            command: core::default::Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecElRef { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecElRef { shared : shared , base : base . to_string () , } } }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\nCommand is the command line to execute inside the container, the working\ndirectory for the command is root ('/') in the container's filesystem.\nThe command is simply exec'd, it is not run inside a shell, so\ntraditional shell instructions ('|', etc) won't work. To use a shell, you\nneed to explicitly call out to that shell. Exit status of 0 is treated as\nlive/healthy and non-zero is unhealthy."]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcEl {
    #[doc = "Set the field `port`.\nPort number of the gRPC service. Number must be in the range 1 to 65535."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nService is the name of the service to place in the gRPC\nHealthCheckRequest. See\nhttps://github.com/grpc/grpc/blob/master/doc/health-checking.md.\n\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcEl
{}
impl
    BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcEl
{
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcEl
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcElRef { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcElRef { shared : shared , base : base . to_string () , } } }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number of the gRPC service. Number must be in the range 1 to 65535."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nService is the name of the service to place in the gRPC\nHealthCheckRequest. See\nhttps://github.com/grpc/grpc/blob/master/doc/health-checking.md.\n\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersEl { # [doc = "Set the field `name`.\nThe header field name.\nThis will be canonicalized upon output, so case-variant names will be\nunderstood as the same header."] pub fn set_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . name = Some (v . into ()) ; self } # [doc = "Set the field `value`.\nThe header field value"] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersEl { type O = BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersEl { pub fn build (self) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersEl { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersEl { name : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersElRef { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersElRef { shared : shared , base : base . to_string () , } } }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nThe header field name.\nThis will be canonicalized upon output, so case-variant names will be\nunderstood as the same header."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\nThe header field value"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElDynamic { http_headers : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersEl >> , }
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetEl { # [serde (skip_serializing_if = "Option::is_none")] host : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] port : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] scheme : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] http_headers : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersEl > > , dynamic : VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElDynamic , }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetEl {
    #[doc = "Set the field `host`.\nHost name to connect to, defaults to the model serving container's IP.\nYou probably want to set \"Host\" in httpHeaders instead."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nPath to access on the HTTP server."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `scheme`.\nScheme to use for connecting to the host.\nDefaults to HTTP. Acceptable values are \"HTTP\" or \"HTTPS\"."]
    pub fn set_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_headers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetEl { type O = BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetEl { pub fn build (self) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetEl { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetEl { host : core :: default :: Default :: default () , path : core :: default :: Default :: default () , port : core :: default :: Default :: default () , scheme : core :: default :: Default :: default () , http_headers : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElRef { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElRef { shared : shared , base : base . to_string () , } } }
impl
    VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nHost name to connect to, defaults to the model serving container's IP.\nYou probably want to set \"Host\" in httpHeaders instead."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to access on the HTTP server."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `scheme` after provisioning.\nScheme to use for connecting to the host.\nDefaults to HTTP. Acceptable values are \"HTTP\" or \"HTTPS\"."]
    pub fn scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]    pub fn http_headers (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElHttpHeadersElRef >{
        ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl
    VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketEl
{
    #[doc = "Set the field `host`.\nOptional: Host name to connect to, defaults to the model serving\ncontainer's IP."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketEl { type O = BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketEl { pub fn build (self) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketEl { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketEl { host : core :: default :: Default :: default () , port : core :: default :: Default :: default () , } } }
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketElRef { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketElRef { shared : shared , base : base . to_string () , } } }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `host` after provisioning.\nOptional: Host name to connect to, defaults to the model serving\ncontainer's IP."] pub fn host (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.host" , self . base)) } # [doc = "Get a reference to the value of field `port` after provisioning.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."] pub fn port (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.port" , self . base)) } }
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElDynamic { exec : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecEl >> , grpc : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcEl >> , http_get : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetEl >> , tcp_socket : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketEl >> , }
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeEl { # [serde (skip_serializing_if = "Option::is_none")] failure_threshold : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] initial_delay_seconds : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] period_seconds : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] success_threshold : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] timeout_seconds : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] exec : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecEl > > , # [serde (skip_serializing_if = "Option::is_none")] grpc : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcEl > > , # [serde (skip_serializing_if = "Option::is_none")] http_get : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetEl > > , # [serde (skip_serializing_if = "Option::is_none")] tcp_socket : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketEl > > , dynamic : VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElDynamic , }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeEl {
    #[doc = "Set the field `failure_threshold`.\nNumber of consecutive failures before the probe is considered failed.\nDefaults to 3. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'failureThreshold'."]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_delay_seconds`.\nNumber of seconds to wait before starting the probe. Defaults to 0.\nMinimum value is 0.\n\nMaps to Kubernetes probe argument 'initialDelaySeconds'."]
    pub fn set_initial_delay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe. Default to 10 seconds.\nMinimum value is 1. Must be less than timeout_seconds.\n\nMaps to Kubernetes probe argument 'periodSeconds'."]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `success_threshold`.\nNumber of consecutive successes before the probe is considered successful.\nDefaults to 1. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'successThreshold'."]
    pub fn set_success_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out. Defaults to 1 second.\nMinimum value is 1. Must be greater or equal to period_seconds.\n\nMaps to Kubernetes probe argument 'timeoutSeconds'."]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `exec`.\n"]
    pub fn set_exec(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_get = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tcp_socket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_socket = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeEl {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeEl {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeEl {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            success_threshold: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            exec: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElRef
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\nNumber of consecutive failures before the probe is considered failed.\nDefaults to 3. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'failureThreshold'."]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds to wait before starting the probe. Defaults to 0.\nMinimum value is 0.\n\nMaps to Kubernetes probe argument 'initialDelaySeconds'."]
    pub fn initial_delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe. Default to 10 seconds.\nMinimum value is 1. Must be less than timeout_seconds.\n\nMaps to Kubernetes probe argument 'periodSeconds'."]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success_threshold` after provisioning.\nNumber of consecutive successes before the probe is considered successful.\nDefaults to 1. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'successThreshold'."]
    pub fn success_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out. Defaults to 1 second.\nMinimum value is 1. Must be greater or equal to period_seconds.\n\nMaps to Kubernetes probe argument 'timeoutSeconds'."]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exec` after provisioning.\n"]    pub fn exec (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElExecElRef >{
        ListRef::new(self.shared().clone(), format!("{}.exec", self.base))
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]    pub fn grpc (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElGrpcElRef >{
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]    pub fn http_get (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElHttpGetElRef >{
        ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]    pub fn tcp_socket (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElTcpSocketElRef >{
        ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_port: Option<PrimField<f64>>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsEl {
    #[doc = "Set the field `container_port`.\nThe number of the port to expose on the pod's IP address.\nMust be a valid port number, between 1 and 65535 inclusive."]
    pub fn set_container_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_port = Some(v.into());
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsEl {}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsEl {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsEl {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsEl {
            container_port: core::default::Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsElRef {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_port` after provisioning.\nThe number of the port to expose on the pod's IP address.\nMust be a valid port number, between 1 and 65535 inclusive."]
    pub fn container_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_port", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecEl {
    #[doc = "Set the field `command`.\nCommand is the command line to execute inside the container, the working\ndirectory for the command is root ('/') in the container's filesystem.\nThe command is simply exec'd, it is not run inside a shell, so\ntraditional shell instructions ('|', etc) won't work. To use a shell, you\nneed to explicitly call out to that shell. Exit status of 0 is treated as\nlive/healthy and non-zero is unhealthy."]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecEl
{}
impl
    BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecEl
{
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecEl
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecEl {
            command: core::default::Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecElRef
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecElRef { shared : shared , base : base . to_string () , }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\nCommand is the command line to execute inside the container, the working\ndirectory for the command is root ('/') in the container's filesystem.\nThe command is simply exec'd, it is not run inside a shell, so\ntraditional shell instructions ('|', etc) won't work. To use a shell, you\nneed to explicitly call out to that shell. Exit status of 0 is treated as\nlive/healthy and non-zero is unhealthy."]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcEl {
    #[doc = "Set the field `port`.\nPort number of the gRPC service. Number must be in the range 1 to 65535."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nService is the name of the service to place in the gRPC\nHealthCheckRequest. See\nhttps://github.com/grpc/grpc/blob/master/doc/health-checking.md.\n\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcEl
{}
impl
    BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcEl
{
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcEl
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcEl {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcElRef
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcElRef { shared : shared , base : base . to_string () , }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort number of the gRPC service. Number must be in the range 1 to 65535."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nService is the name of the service to place in the gRPC\nHealthCheckRequest. See\nhttps://github.com/grpc/grpc/blob/master/doc/health-checking.md.\n\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersEl { # [doc = "Set the field `name`.\nThe header field name.\nThis will be canonicalized upon output, so case-variant names will be\nunderstood as the same header."] pub fn set_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . name = Some (v . into ()) ; self } # [doc = "Set the field `value`.\nThe header field value"] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersEl { type O = BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersEl { pub fn build (self) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersEl { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersEl { name : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersElRef { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersElRef { shared : shared , base : base . to_string () , } } }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nThe header field name.\nThis will be canonicalized upon output, so case-variant names will be\nunderstood as the same header."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\nThe header field value"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElDynamic { http_headers : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersEl >> , }
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetEl { # [serde (skip_serializing_if = "Option::is_none")] host : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] path : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] port : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] scheme : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] http_headers : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersEl > > , dynamic : VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElDynamic , }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetEl {
    #[doc = "Set the field `host`.\nHost name to connect to, defaults to the model serving container's IP.\nYou probably want to set \"Host\" in httpHeaders instead."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nPath to access on the HTTP server."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `scheme`.\nScheme to use for connecting to the host.\nDefaults to HTTP. Acceptable values are \"HTTP\" or \"HTTPS\"."]
    pub fn set_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_headers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetEl
{
    type O = BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetEl { pub fn build (self) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetEl { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetEl { host : core :: default :: Default :: default () , path : core :: default :: Default :: default () , port : core :: default :: Default :: default () , scheme : core :: default :: Default :: default () , http_headers : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElRef { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElRef { shared : shared , base : base . to_string () , } } }
impl
    VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nHost name to connect to, defaults to the model serving container's IP.\nYou probably want to set \"Host\" in httpHeaders instead."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to access on the HTTP server."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `scheme` after provisioning.\nScheme to use for connecting to the host.\nDefaults to HTTP. Acceptable values are \"HTTP\" or \"HTTPS\"."]
    pub fn scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]    pub fn http_headers (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElHttpHeadersElRef >{
        ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl
    VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketEl
{
    #[doc = "Set the field `host`.\nOptional: Host name to connect to, defaults to the model serving\ncontainer's IP."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketEl { type O = BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketEl { pub fn build (self) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketEl { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketEl { host : core :: default :: Default :: default () , port : core :: default :: Default :: default () , } } }
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketElRef { fn new (shared : StackShared , base : String) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketElRef { VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketElRef { shared : shared , base : base . to_string () , } } }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `host` after provisioning.\nOptional: Host name to connect to, defaults to the model serving\ncontainer's IP."] pub fn host (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.host" , self . base)) } # [doc = "Get a reference to the value of field `port` after provisioning.\nNumber of the port to access on the container.\nNumber must be in the range 1 to 65535."] pub fn port (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.port" , self . base)) } }
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElDynamic { exec : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecEl >> , grpc : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcEl >> , http_get : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetEl >> , tcp_socket : Option < DynamicBlock < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketEl >> , }
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeEl { # [serde (skip_serializing_if = "Option::is_none")] failure_threshold : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] initial_delay_seconds : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] period_seconds : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] success_threshold : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] timeout_seconds : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] exec : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecEl > > , # [serde (skip_serializing_if = "Option::is_none")] grpc : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcEl > > , # [serde (skip_serializing_if = "Option::is_none")] http_get : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetEl > > , # [serde (skip_serializing_if = "Option::is_none")] tcp_socket : Option < Vec < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketEl > > , dynamic : VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElDynamic , }
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeEl {
    #[doc = "Set the field `failure_threshold`.\nNumber of consecutive failures before the probe is considered failed.\nDefaults to 3. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'failureThreshold'."]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_delay_seconds`.\nNumber of seconds to wait before starting the probe. Defaults to 0.\nMinimum value is 0.\n\nMaps to Kubernetes probe argument 'initialDelaySeconds'."]
    pub fn set_initial_delay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe. Default to 10 seconds.\nMinimum value is 1. Must be less than timeout_seconds.\n\nMaps to Kubernetes probe argument 'periodSeconds'."]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `success_threshold`.\nNumber of consecutive successes before the probe is considered successful.\nDefaults to 1. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'successThreshold'."]
    pub fn set_success_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out. Defaults to 1 second.\nMinimum value is 1. Must be greater or equal to period_seconds.\n\nMaps to Kubernetes probe argument 'timeoutSeconds'."]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `exec`.\n"]
    pub fn set_exec(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_get = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tcp_socket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_socket = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeEl
{
    type O = BlockAssignable<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeEl
{}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeEl {
    pub fn build(
        self,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeEl {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeEl {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            success_threshold: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            exec: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElRef
    {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\nNumber of consecutive failures before the probe is considered failed.\nDefaults to 3. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'failureThreshold'."]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds to wait before starting the probe. Defaults to 0.\nMinimum value is 0.\n\nMaps to Kubernetes probe argument 'initialDelaySeconds'."]
    pub fn initial_delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_delay_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe. Default to 10 seconds.\nMinimum value is 1. Must be less than timeout_seconds.\n\nMaps to Kubernetes probe argument 'periodSeconds'."]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success_threshold` after provisioning.\nNumber of consecutive successes before the probe is considered successful.\nDefaults to 1. Minimum value is 1.\n\nMaps to Kubernetes probe argument 'successThreshold'."]
    pub fn success_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out. Defaults to 1 second.\nMinimum value is 1. Must be greater or equal to period_seconds.\n\nMaps to Kubernetes probe argument 'timeoutSeconds'."]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exec` after provisioning.\n"]    pub fn exec (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElExecElRef >{
        ListRef::new(self.shared().clone(), format!("{}.exec", self.base))
    }
    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]    pub fn grpc (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElGrpcElRef >{
        ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }
    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]    pub fn http_get (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElHttpGetElRef >{
        ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]    pub fn tcp_socket (& self) -> ListRef < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElTcpSocketElRef >{
        ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElDynamic {
    env: Option<
        DynamicBlock<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvEl>,
    >,
    grpc_ports: Option<
        DynamicBlock<
            VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsEl,
        >,
    >,
    health_probe: Option<
        DynamicBlock<
            VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeEl,
        >,
    >,
    liveness_probe: Option<
        DynamicBlock<
            VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeEl,
        >,
    >,
    ports: Option<
        DynamicBlock<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsEl>,
    >,
    startup_probe: Option<
        DynamicBlock<
            VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_route: Option<PrimField<String>>,
    image_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predict_route: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_memory_size_mb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_ports: Option<
        Vec<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_probe: Option<
        Vec<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    liveness_probe: Option<
        Vec<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports:
        Option<Vec<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_probe: Option<
        Vec<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeEl>,
    >,
    dynamic: VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElDynamic,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecEl {
    #[doc = "Set the field `args`.\nSpecifies arguments for the command that runs when the container starts.\nThis overrides the container's\n['CMD'](https://docs.docker.com/engine/reference/builder/#cmd). Specify\nthis field as an array of executable and arguments, similar to a Docker\n'CMD''s \"default parameters\" form.\n\nIf you don't specify this field but do specify the\ncommand field, then the command from the\n'command' field runs without any additional arguments. See the\n[Kubernetes documentation about how the\n'command' and 'args' fields interact with a container's 'ENTRYPOINT' and\n'CMD'](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#notes).\n\nIf you don't specify this field and don't specify the 'command' field,\nthen the container's\n['ENTRYPOINT'](https://docs.docker.com/engine/reference/builder/#cmd) and\n'CMD' determine what runs based on their default behavior. See the Docker\ndocumentation about [how 'CMD' and 'ENTRYPOINT'\ninteract](https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact).\n\nIn this field, you can reference [environment variables\nset by Vertex\nAI](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables)\nand environment variables set in the env field.\nYou cannot reference environment variables set in the Docker image. In\norder for environment variables to be expanded, reference them by using the\nfollowing syntax:$(VARIABLE_NAME)\nNote that this differs from Bash variable expansion, which does not use\nparentheses. If a variable cannot be resolved, the reference in the input\nstring is used unchanged. To avoid variable expansion, you can escape this\nsyntax with '$$'; for example:$$(VARIABLE_NAME)\nThis field corresponds to the 'args' field of the Kubernetes Containers\n[v1 core\nAPI](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.23/#container-v1-core)."]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `command`.\nSpecifies the command that runs when the container starts. This overrides\nthe container's\n[ENTRYPOINT](https://docs.docker.com/engine/reference/builder/#entrypoint).\nSpecify this field as an array of executable and arguments, similar to a\nDocker 'ENTRYPOINT''s \"exec\" form, not its \"shell\" form.\n\nIf you do not specify this field, then the container's 'ENTRYPOINT' runs,\nin conjunction with the args field or the\ncontainer's ['CMD'](https://docs.docker.com/engine/reference/builder/#cmd),\nif either exists. If this field is not specified and the container does not\nhave an 'ENTRYPOINT', then refer to the Docker documentation about [how\n'CMD' and 'ENTRYPOINT'\ninteract](https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact).\n\nIf you specify this field, then you can also specify the 'args' field to\nprovide additional arguments for this command. However, if you specify this\nfield, then the container's 'CMD' is ignored. See the\n[Kubernetes documentation about how the\n'command' and 'args' fields interact with a container's 'ENTRYPOINT' and\n'CMD'](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#notes).\n\nIn this field, you can reference [environment variables set by Vertex\nAI](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables)\nand environment variables set in the env field.\nYou cannot reference environment variables set in the Docker image. In\norder for environment variables to be expanded, reference them by using the\nfollowing syntax:$(VARIABLE_NAME)\nNote that this differs from Bash variable expansion, which does not use\nparentheses. If a variable cannot be resolved, the reference in the input\nstring is used unchanged. To avoid variable expansion, you can escape this\nsyntax with '$$'; for example:$$(VARIABLE_NAME)\nThis field corresponds to the 'command' field of the Kubernetes Containers\n[v1 core\nAPI](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.23/#container-v1-core)."]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `deployment_timeout`.\nDeployment timeout.\nLimit for deployment timeout is 2 hours."]
    pub fn set_deployment_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deployment_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `health_route`.\nHTTP path on the container to send health checks to. Vertex AI\nintermittently sends GET requests to this path on the container's IP\naddress and port to check that the container is healthy. Read more about\n[health\nchecks](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#health).\n\nFor example, if you set this field to '/bar', then Vertex AI\nintermittently sends a GET request to the '/bar' path on the port of your\ncontainer specified by the first value of this 'ModelContainerSpec''s\nports field.\n\nIf you don't specify this field, it defaults to the following value when\nyou deploy this Model to an Endpoint:/v1/endpoints/ENDPOINT/deployedModels/DEPLOYED_MODEL:predict\nThe placeholders in this value are replaced as follows:\n\n* ENDPOINT: The last segment (following 'endpoints/')of the\nEndpoint.name][] field of the Endpoint where this Model has been\ndeployed. (Vertex AI makes this value available to your container code\nas the ['AIP_ENDPOINT_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)\n\n* DEPLOYED_MODEL: DeployedModel.id of the 'DeployedModel'.\n(Vertex AI makes this value available to your container code as the\n['AIP_DEPLOYED_MODEL_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)"]
    pub fn set_health_route(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.health_route = Some(v.into());
        self
    }
    #[doc = "Set the field `predict_route`.\nHTTP path on the container to send prediction requests to. Vertex AI\nforwards requests sent using\nprojects.locations.endpoints.predict to this\npath on the container's IP address and port. Vertex AI then returns the\ncontainer's response in the API response.\n\nFor example, if you set this field to '/foo', then when Vertex AI\nreceives a prediction request, it forwards the request body in a POST\nrequest to the '/foo' path on the port of your container specified by the\nfirst value of this 'ModelContainerSpec''s\nports field.\n\nIf you don't specify this field, it defaults to the following value when\nyou deploy this Model to an Endpoint:/v1/endpoints/ENDPOINT/deployedModels/DEPLOYED_MODEL:predict\nThe placeholders in this value are replaced as follows:\n\n* ENDPOINT: The last segment (following 'endpoints/')of the\nEndpoint.name][] field of the Endpoint where this Model has been\ndeployed. (Vertex AI makes this value available to your container code\nas the ['AIP_ENDPOINT_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)\n\n* DEPLOYED_MODEL: DeployedModel.id of the 'DeployedModel'.\n(Vertex AI makes this value available to your container code\nas the ['AIP_DEPLOYED_MODEL_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)"]
    pub fn set_predict_route(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.predict_route = Some(v.into());
        self
    }
    #[doc = "Set the field `shared_memory_size_mb`.\nThe amount of the VM memory to reserve as the shared memory for the model\nin megabytes."]
    pub fn set_shared_memory_size_mb(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shared_memory_size_mb = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `grpc_ports`.\n"]
    pub fn set_grpc_ports(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc_ports = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_ports = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `health_probe`.\n"]
    pub fn set_health_probe(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.health_probe = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.health_probe = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `liveness_probe`.\n"]
    pub fn set_liveness_probe(
        mut self,
        v : impl Into < BlockAssignable < VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.liveness_probe = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.liveness_probe = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ports = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ports = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `startup_probe`.\n"]
    pub fn set_startup_probe(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.startup_probe = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.startup_probe = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecEl {
    type O = BlockAssignable<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecEl {
    #[doc = "URI of the Docker image to be used as the custom container for serving\npredictions. This URI must identify an image in Artifact Registry or\nContainer Registry. Learn more about the [container publishing\nrequirements](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#publishing),\nincluding permissions requirements for the Vertex AI Service Agent.\n\nThe container image is ingested upon ModelService.UploadModel, stored\ninternally, and this original path is afterwards not used.\n\nTo learn about the requirements for the Docker image itself, see\n[Custom container\nrequirements](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#).\n\nYou can use the URI to one of Vertex AI's [pre-built container images for\nprediction](https://cloud.google.com/vertex-ai/docs/predictions/pre-built-containers)\nin this field."]
    pub image_uri: PrimField<String>,
}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecEl {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecEl {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            deployment_timeout: core::default::Default::default(),
            health_route: core::default::Default::default(),
            image_uri: self.image_uri,
            predict_route: core::default::Default::default(),
            shared_memory_size_mb: core::default::Default::default(),
            env: core::default::Default::default(),
            grpc_ports: core::default::Default::default(),
            health_probe: core::default::Default::default(),
            liveness_probe: core::default::Default::default(),
            ports: core::default::Default::default(),
            startup_probe: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElRef {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nSpecifies arguments for the command that runs when the container starts.\nThis overrides the container's\n['CMD'](https://docs.docker.com/engine/reference/builder/#cmd). Specify\nthis field as an array of executable and arguments, similar to a Docker\n'CMD''s \"default parameters\" form.\n\nIf you don't specify this field but do specify the\ncommand field, then the command from the\n'command' field runs without any additional arguments. See the\n[Kubernetes documentation about how the\n'command' and 'args' fields interact with a container's 'ENTRYPOINT' and\n'CMD'](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#notes).\n\nIf you don't specify this field and don't specify the 'command' field,\nthen the container's\n['ENTRYPOINT'](https://docs.docker.com/engine/reference/builder/#cmd) and\n'CMD' determine what runs based on their default behavior. See the Docker\ndocumentation about [how 'CMD' and 'ENTRYPOINT'\ninteract](https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact).\n\nIn this field, you can reference [environment variables\nset by Vertex\nAI](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables)\nand environment variables set in the env field.\nYou cannot reference environment variables set in the Docker image. In\norder for environment variables to be expanded, reference them by using the\nfollowing syntax:$(VARIABLE_NAME)\nNote that this differs from Bash variable expansion, which does not use\nparentheses. If a variable cannot be resolved, the reference in the input\nstring is used unchanged. To avoid variable expansion, you can escape this\nsyntax with '$$'; for example:$$(VARIABLE_NAME)\nThis field corresponds to the 'args' field of the Kubernetes Containers\n[v1 core\nAPI](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.23/#container-v1-core)."]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\nSpecifies the command that runs when the container starts. This overrides\nthe container's\n[ENTRYPOINT](https://docs.docker.com/engine/reference/builder/#entrypoint).\nSpecify this field as an array of executable and arguments, similar to a\nDocker 'ENTRYPOINT''s \"exec\" form, not its \"shell\" form.\n\nIf you do not specify this field, then the container's 'ENTRYPOINT' runs,\nin conjunction with the args field or the\ncontainer's ['CMD'](https://docs.docker.com/engine/reference/builder/#cmd),\nif either exists. If this field is not specified and the container does not\nhave an 'ENTRYPOINT', then refer to the Docker documentation about [how\n'CMD' and 'ENTRYPOINT'\ninteract](https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact).\n\nIf you specify this field, then you can also specify the 'args' field to\nprovide additional arguments for this command. However, if you specify this\nfield, then the container's 'CMD' is ignored. See the\n[Kubernetes documentation about how the\n'command' and 'args' fields interact with a container's 'ENTRYPOINT' and\n'CMD'](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#notes).\n\nIn this field, you can reference [environment variables set by Vertex\nAI](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables)\nand environment variables set in the env field.\nYou cannot reference environment variables set in the Docker image. In\norder for environment variables to be expanded, reference them by using the\nfollowing syntax:$(VARIABLE_NAME)\nNote that this differs from Bash variable expansion, which does not use\nparentheses. If a variable cannot be resolved, the reference in the input\nstring is used unchanged. To avoid variable expansion, you can escape this\nsyntax with '$$'; for example:$$(VARIABLE_NAME)\nThis field corresponds to the 'command' field of the Kubernetes Containers\n[v1 core\nAPI](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.23/#container-v1-core)."]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `deployment_timeout` after provisioning.\nDeployment timeout.\nLimit for deployment timeout is 2 hours."]
    pub fn deployment_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_route` after provisioning.\nHTTP path on the container to send health checks to. Vertex AI\nintermittently sends GET requests to this path on the container's IP\naddress and port to check that the container is healthy. Read more about\n[health\nchecks](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#health).\n\nFor example, if you set this field to '/bar', then Vertex AI\nintermittently sends a GET request to the '/bar' path on the port of your\ncontainer specified by the first value of this 'ModelContainerSpec''s\nports field.\n\nIf you don't specify this field, it defaults to the following value when\nyou deploy this Model to an Endpoint:/v1/endpoints/ENDPOINT/deployedModels/DEPLOYED_MODEL:predict\nThe placeholders in this value are replaced as follows:\n\n* ENDPOINT: The last segment (following 'endpoints/')of the\nEndpoint.name][] field of the Endpoint where this Model has been\ndeployed. (Vertex AI makes this value available to your container code\nas the ['AIP_ENDPOINT_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)\n\n* DEPLOYED_MODEL: DeployedModel.id of the 'DeployedModel'.\n(Vertex AI makes this value available to your container code as the\n['AIP_DEPLOYED_MODEL_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)"]
    pub fn health_route(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_route", self.base))
    }
    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\nURI of the Docker image to be used as the custom container for serving\npredictions. This URI must identify an image in Artifact Registry or\nContainer Registry. Learn more about the [container publishing\nrequirements](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#publishing),\nincluding permissions requirements for the Vertex AI Service Agent.\n\nThe container image is ingested upon ModelService.UploadModel, stored\ninternally, and this original path is afterwards not used.\n\nTo learn about the requirements for the Docker image itself, see\n[Custom container\nrequirements](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#).\n\nYou can use the URI to one of Vertex AI's [pre-built container images for\nprediction](https://cloud.google.com/vertex-ai/docs/predictions/pre-built-containers)\nin this field."]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `predict_route` after provisioning.\nHTTP path on the container to send prediction requests to. Vertex AI\nforwards requests sent using\nprojects.locations.endpoints.predict to this\npath on the container's IP address and port. Vertex AI then returns the\ncontainer's response in the API response.\n\nFor example, if you set this field to '/foo', then when Vertex AI\nreceives a prediction request, it forwards the request body in a POST\nrequest to the '/foo' path on the port of your container specified by the\nfirst value of this 'ModelContainerSpec''s\nports field.\n\nIf you don't specify this field, it defaults to the following value when\nyou deploy this Model to an Endpoint:/v1/endpoints/ENDPOINT/deployedModels/DEPLOYED_MODEL:predict\nThe placeholders in this value are replaced as follows:\n\n* ENDPOINT: The last segment (following 'endpoints/')of the\nEndpoint.name][] field of the Endpoint where this Model has been\ndeployed. (Vertex AI makes this value available to your container code\nas the ['AIP_ENDPOINT_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)\n\n* DEPLOYED_MODEL: DeployedModel.id of the 'DeployedModel'.\n(Vertex AI makes this value available to your container code\nas the ['AIP_DEPLOYED_MODEL_ID' environment\nvariable](https://cloud.google.com/vertex-ai/docs/predictions/custom-container-requirements#aip-variables).)"]
    pub fn predict_route(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.predict_route", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shared_memory_size_mb` after provisioning.\nThe amount of the VM memory to reserve as the shared memory for the model\nin megabytes."]
    pub fn shared_memory_size_mb(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shared_memory_size_mb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(
        &self,
    ) -> ListRef<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElEnvElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `grpc_ports` after provisioning.\n"]
    pub fn grpc_ports(
        &self,
    ) -> ListRef<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElGrpcPortsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.grpc_ports", self.base))
    }
    #[doc = "Get a reference to the value of field `health_probe` after provisioning.\n"]
    pub fn health_probe(
        &self,
    ) -> ListRef<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElHealthProbeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.health_probe", self.base))
    }
    #[doc = "Get a reference to the value of field `liveness_probe` after provisioning.\n"]
    pub fn liveness_probe(
        &self,
    ) -> ListRef<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElLivenessProbeElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.liveness_probe", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(
        &self,
    ) -> ListRef<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElPortsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
    #[doc = "Get a reference to the value of field `startup_probe` after provisioning.\n"]
    pub fn startup_probe(
        &self,
    ) -> ListRef<
        VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElStartupProbeElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.startup_probe", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VertexAiEndpointWithModelGardenDeploymentModelConfigElDynamic {
    container_spec:
        Option<DynamicBlock<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecEl>>,
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accept_eula: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hugging_face_access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hugging_face_cache_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_spec:
        Option<Vec<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecEl>>,
    dynamic: VertexAiEndpointWithModelGardenDeploymentModelConfigElDynamic,
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigEl {
    #[doc = "Set the field `accept_eula`.\nWhether the user accepts the End User License Agreement (EULA)\nfor the model."]
    pub fn set_accept_eula(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.accept_eula = Some(v.into());
        self
    }
    #[doc = "Set the field `hugging_face_access_token`.\nThe Hugging Face read access token used to access the model\nartifacts of gated models."]
    pub fn set_hugging_face_access_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hugging_face_access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `hugging_face_cache_enabled`.\nIf true, the model will deploy with a cached version instead of directly\ndownloading the model artifacts from Hugging Face. This is suitable for\nVPC-SC users with limited internet access."]
    pub fn set_hugging_face_cache_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.hugging_face_cache_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `model_display_name`.\nThe user-specified display name of the uploaded model. If not\nset, a default name will be used."]
    pub fn set_model_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `container_spec`.\n"]
    pub fn set_container_spec(
        mut self,
        v: impl Into<
            BlockAssignable<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentModelConfigEl {
    type O = BlockAssignable<VertexAiEndpointWithModelGardenDeploymentModelConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentModelConfigEl {}
impl BuildVertexAiEndpointWithModelGardenDeploymentModelConfigEl {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeploymentModelConfigEl {
        VertexAiEndpointWithModelGardenDeploymentModelConfigEl {
            accept_eula: core::default::Default::default(),
            hugging_face_access_token: core::default::Default::default(),
            hugging_face_cache_enabled: core::default::Default::default(),
            model_display_name: core::default::Default::default(),
            container_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentModelConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentModelConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentModelConfigElRef {
        VertexAiEndpointWithModelGardenDeploymentModelConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentModelConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accept_eula` after provisioning.\nWhether the user accepts the End User License Agreement (EULA)\nfor the model."]
    pub fn accept_eula(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.accept_eula", self.base))
    }
    #[doc = "Get a reference to the value of field `hugging_face_access_token` after provisioning.\nThe Hugging Face read access token used to access the model\nartifacts of gated models."]
    pub fn hugging_face_access_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hugging_face_access_token", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hugging_face_cache_enabled` after provisioning.\nIf true, the model will deploy with a cached version instead of directly\ndownloading the model artifacts from Hugging Face. This is suitable for\nVPC-SC users with limited internet access."]
    pub fn hugging_face_cache_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hugging_face_cache_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_display_name` after provisioning.\nThe user-specified display name of the uploaded model. If not\nset, a default name will be used."]
    pub fn model_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_display_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_spec` after provisioning.\n"]
    pub fn container_spec(
        &self,
    ) -> ListRef<VertexAiEndpointWithModelGardenDeploymentModelConfigElContainerSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_spec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiEndpointWithModelGardenDeploymentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl VertexAiEndpointWithModelGardenDeploymentTimeoutsEl {
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
impl ToListMappable for VertexAiEndpointWithModelGardenDeploymentTimeoutsEl {
    type O = BlockAssignable<VertexAiEndpointWithModelGardenDeploymentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiEndpointWithModelGardenDeploymentTimeoutsEl {}
impl BuildVertexAiEndpointWithModelGardenDeploymentTimeoutsEl {
    pub fn build(self) -> VertexAiEndpointWithModelGardenDeploymentTimeoutsEl {
        VertexAiEndpointWithModelGardenDeploymentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct VertexAiEndpointWithModelGardenDeploymentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiEndpointWithModelGardenDeploymentTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiEndpointWithModelGardenDeploymentTimeoutsElRef {
        VertexAiEndpointWithModelGardenDeploymentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiEndpointWithModelGardenDeploymentTimeoutsElRef {
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
struct VertexAiEndpointWithModelGardenDeploymentDynamic {
    deploy_config: Option<DynamicBlock<VertexAiEndpointWithModelGardenDeploymentDeployConfigEl>>,
    endpoint_config:
        Option<DynamicBlock<VertexAiEndpointWithModelGardenDeploymentEndpointConfigEl>>,
    model_config: Option<DynamicBlock<VertexAiEndpointWithModelGardenDeploymentModelConfigEl>>,
}
