use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AutoscalingPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    adjustment_type: Option<PrimField<String>>,
    autoscaling_group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cooldown: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_instance_warmup: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_aggregation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_adjustment_magnitude: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_adjustment: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predictive_scaling_configuration:
        Option<Vec<AutoscalingPolicyPredictiveScalingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step_adjustment: Option<Vec<AutoscalingPolicyStepAdjustmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_tracking_configuration: Option<Vec<AutoscalingPolicyTargetTrackingConfigurationEl>>,
    dynamic: AutoscalingPolicyDynamic,
}
struct AutoscalingPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AutoscalingPolicyData>,
}
#[derive(Clone)]
pub struct AutoscalingPolicy(Rc<AutoscalingPolicy_>);
impl AutoscalingPolicy {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `adjustment_type`.\n"]
    pub fn set_adjustment_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().adjustment_type = Some(v.into());
        self
    }
    #[doc = "Set the field `cooldown`.\n"]
    pub fn set_cooldown(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cooldown = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `estimated_instance_warmup`.\n"]
    pub fn set_estimated_instance_warmup(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().estimated_instance_warmup = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_aggregation_type`.\n"]
    pub fn set_metric_aggregation_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().metric_aggregation_type = Some(v.into());
        self
    }
    #[doc = "Set the field `min_adjustment_magnitude`.\n"]
    pub fn set_min_adjustment_magnitude(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_adjustment_magnitude = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_type`.\n"]
    pub fn set_policy_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_type = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `scaling_adjustment`.\n"]
    pub fn set_scaling_adjustment(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().scaling_adjustment = Some(v.into());
        self
    }
    #[doc = "Set the field `predictive_scaling_configuration`.\n"]
    pub fn set_predictive_scaling_configuration(
        self,
        v: impl Into<BlockAssignable<AutoscalingPolicyPredictiveScalingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().predictive_scaling_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .predictive_scaling_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `step_adjustment`.\n"]
    pub fn set_step_adjustment(
        self,
        v: impl Into<BlockAssignable<AutoscalingPolicyStepAdjustmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().step_adjustment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.step_adjustment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target_tracking_configuration`.\n"]
    pub fn set_target_tracking_configuration(
        self,
        v: impl Into<BlockAssignable<AutoscalingPolicyTargetTrackingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_tracking_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .target_tracking_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `adjustment_type` after provisioning.\n"]
    pub fn adjustment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.adjustment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `autoscaling_group_name` after provisioning.\n"]
    pub fn autoscaling_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.autoscaling_group_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cooldown` after provisioning.\n"]
    pub fn cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cooldown", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `estimated_instance_warmup` after provisioning.\n"]
    pub fn estimated_instance_warmup(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.estimated_instance_warmup", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `metric_aggregation_type` after provisioning.\n"]
    pub fn metric_aggregation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_aggregation_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_adjustment_magnitude` after provisioning.\n"]
    pub fn min_adjustment_magnitude(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_adjustment_magnitude", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_adjustment` after provisioning.\n"]
    pub fn scaling_adjustment(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scaling_adjustment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `predictive_scaling_configuration` after provisioning.\n"]
    pub fn predictive_scaling_configuration(
        &self,
    ) -> ListRef<AutoscalingPolicyPredictiveScalingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predictive_scaling_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_tracking_configuration` after provisioning.\n"]
    pub fn target_tracking_configuration(
        &self,
    ) -> ListRef<AutoscalingPolicyTargetTrackingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_tracking_configuration", self.extract_ref()),
        )
    }
}
impl Referable for AutoscalingPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AutoscalingPolicy {}
impl ToListMappable for AutoscalingPolicy {
    type O = ListRef<AutoscalingPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AutoscalingPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_autoscaling_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAutoscalingPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub autoscaling_group_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildAutoscalingPolicy {
    pub fn build(self, stack: &mut Stack) -> AutoscalingPolicy {
        let out = AutoscalingPolicy(Rc::new(AutoscalingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AutoscalingPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                adjustment_type: core::default::Default::default(),
                autoscaling_group_name: self.autoscaling_group_name,
                cooldown: core::default::Default::default(),
                enabled: core::default::Default::default(),
                estimated_instance_warmup: core::default::Default::default(),
                id: core::default::Default::default(),
                metric_aggregation_type: core::default::Default::default(),
                min_adjustment_magnitude: core::default::Default::default(),
                name: self.name,
                policy_type: core::default::Default::default(),
                region: core::default::Default::default(),
                scaling_adjustment: core::default::Default::default(),
                predictive_scaling_configuration: core::default::Default::default(),
                step_adjustment: core::default::Default::default(),
                target_tracking_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AutoscalingPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AutoscalingPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `adjustment_type` after provisioning.\n"]
    pub fn adjustment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.adjustment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `autoscaling_group_name` after provisioning.\n"]
    pub fn autoscaling_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.autoscaling_group_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cooldown` after provisioning.\n"]
    pub fn cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cooldown", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `estimated_instance_warmup` after provisioning.\n"]
    pub fn estimated_instance_warmup(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.estimated_instance_warmup", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `metric_aggregation_type` after provisioning.\n"]
    pub fn metric_aggregation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_aggregation_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_adjustment_magnitude` after provisioning.\n"]
    pub fn min_adjustment_magnitude(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_adjustment_magnitude", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_adjustment` after provisioning.\n"]
    pub fn scaling_adjustment(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scaling_adjustment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `predictive_scaling_configuration` after provisioning.\n"]
    pub fn predictive_scaling_configuration(
        &self,
    ) -> ListRef<AutoscalingPolicyPredictiveScalingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predictive_scaling_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_tracking_configuration` after provisioning.\n"]
    pub fn target_tracking_configuration(
        &self,
    ) -> ListRef<AutoscalingPolicyTargetTrackingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_tracking_configuration", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { name : self . name , value : self . value , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDynamic { dimensions : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { metric_name : PrimField < String > , namespace : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] dimensions : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { # [doc = "Set the field `dimensions`.\n"] pub fn set_dimensions (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dimensions = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dimensions = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl
{
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub namespace: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { metric_name : self . metric_name , namespace : self . namespace , dimensions : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metric_name` after provisioning.\n"] pub fn metric_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.metric_name" , self . base)) } # [doc = "Get a reference to the value of field `namespace` after provisioning.\n"] pub fn namespace (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.namespace" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElDynamic { metric : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl { stat : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] unit : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] metric : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl { # [doc = "Set the field `unit`.\n"] pub fn set_unit (mut self , v : impl Into < PrimField < String > >) -> Self { self . unit = Some (v . into ()) ; self } # [doc = "Set the field `metric`.\n"] pub fn set_metric (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . metric = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . metric = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl
{
    #[doc = ""]
    pub stat: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl { stat : self . stat , unit : core :: default :: Default :: default () , metric : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `stat` after provisioning.\n"] pub fn stat (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.stat" , self . base)) } # [doc = "Get a reference to the value of field `unit` after provisioning.\n"] pub fn unit (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.unit" , self . base)) } # [doc = "Get a reference to the value of field `metric` after provisioning.\n"] pub fn metric (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.metric" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElDynamic { metric_stat : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl { # [serde (skip_serializing_if = "Option::is_none")] expression : Option < PrimField < String > > , id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] label : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] return_data : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] metric_stat : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl { # [doc = "Set the field `expression`.\n"] pub fn set_expression (mut self , v : impl Into < PrimField < String > >) -> Self { self . expression = Some (v . into ()) ; self } # [doc = "Set the field `label`.\n"] pub fn set_label (mut self , v : impl Into < PrimField < String > >) -> Self { self . label = Some (v . into ()) ; self } # [doc = "Set the field `return_data`.\n"] pub fn set_return_data (mut self , v : impl Into < PrimField < bool > >) -> Self { self . return_data = Some (v . into ()) ; self } # [doc = "Set the field `metric_stat`.\n"] pub fn set_metric_stat (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . metric_stat = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . metric_stat = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl
{
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl { expression : core :: default :: Default :: default () , id : self . id , label : core :: default :: Default :: default () , return_data : core :: default :: Default :: default () , metric_stat : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `expression` after provisioning.\n"] pub fn expression (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.expression" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\n"] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } # [doc = "Get a reference to the value of field `label` after provisioning.\n"] pub fn label (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.label" , self . base)) } # [doc = "Get a reference to the value of field `return_data` after provisioning.\n"] pub fn return_data (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.return_data" , self . base)) } # [doc = "Get a reference to the value of field `metric_stat` after provisioning.\n"] pub fn metric_stat (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElMetricStatElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.metric_stat" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElDynamic { metric_data_queries : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl { # [serde (skip_serializing_if = "Option::is_none")] metric_data_queries : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl { # [doc = "Set the field `metric_data_queries`.\n"] pub fn set_metric_data_queries (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . metric_data_queries = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . metric_data_queries = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl
{}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl { metric_data_queries : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metric_data_queries` after provisioning.\n"] pub fn metric_data_queries (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueriesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.metric_data_queries" , self . base)) } }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { name : self . name , value : self . value , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDynamic { dimensions : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { metric_name : PrimField < String > , namespace : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] dimensions : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { # [doc = "Set the field `dimensions`.\n"] pub fn set_dimensions (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dimensions = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dimensions = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl
{
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub namespace: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { metric_name : self . metric_name , namespace : self . namespace , dimensions : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metric_name` after provisioning.\n"] pub fn metric_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.metric_name" , self . base)) } # [doc = "Get a reference to the value of field `namespace` after provisioning.\n"] pub fn namespace (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.namespace" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElDynamic { metric : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl { stat : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] unit : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] metric : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl { # [doc = "Set the field `unit`.\n"] pub fn set_unit (mut self , v : impl Into < PrimField < String > >) -> Self { self . unit = Some (v . into ()) ; self } # [doc = "Set the field `metric`.\n"] pub fn set_metric (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . metric = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . metric = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl
{
    #[doc = ""]
    pub stat: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl { stat : self . stat , unit : core :: default :: Default :: default () , metric : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `stat` after provisioning.\n"] pub fn stat (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.stat" , self . base)) } # [doc = "Get a reference to the value of field `unit` after provisioning.\n"] pub fn unit (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.unit" , self . base)) } # [doc = "Get a reference to the value of field `metric` after provisioning.\n"] pub fn metric (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.metric" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElDynamic { metric_stat : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl { # [serde (skip_serializing_if = "Option::is_none")] expression : Option < PrimField < String > > , id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] label : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] return_data : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] metric_stat : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl { # [doc = "Set the field `expression`.\n"] pub fn set_expression (mut self , v : impl Into < PrimField < String > >) -> Self { self . expression = Some (v . into ()) ; self } # [doc = "Set the field `label`.\n"] pub fn set_label (mut self , v : impl Into < PrimField < String > >) -> Self { self . label = Some (v . into ()) ; self } # [doc = "Set the field `return_data`.\n"] pub fn set_return_data (mut self , v : impl Into < PrimField < bool > >) -> Self { self . return_data = Some (v . into ()) ; self } # [doc = "Set the field `metric_stat`.\n"] pub fn set_metric_stat (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . metric_stat = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . metric_stat = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl
{
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl { expression : core :: default :: Default :: default () , id : self . id , label : core :: default :: Default :: default () , return_data : core :: default :: Default :: default () , metric_stat : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `expression` after provisioning.\n"] pub fn expression (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.expression" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\n"] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } # [doc = "Get a reference to the value of field `label` after provisioning.\n"] pub fn label (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.label" , self . base)) } # [doc = "Get a reference to the value of field `return_data` after provisioning.\n"] pub fn return_data (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.return_data" , self . base)) } # [doc = "Get a reference to the value of field `metric_stat` after provisioning.\n"] pub fn metric_stat (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElMetricStatElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.metric_stat" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElDynamic { metric_data_queries : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl { # [serde (skip_serializing_if = "Option::is_none")] metric_data_queries : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl { # [doc = "Set the field `metric_data_queries`.\n"] pub fn set_metric_data_queries (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . metric_data_queries = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . metric_data_queries = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl
{}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl { metric_data_queries : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metric_data_queries` after provisioning.\n"] pub fn metric_data_queries (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueriesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.metric_data_queries" , self . base)) } }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl { name : self . name , value : self . value , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDynamic { dimensions : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { metric_name : PrimField < String > , namespace : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] dimensions : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { # [doc = "Set the field `dimensions`.\n"] pub fn set_dimensions (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElDimensionsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dimensions = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dimensions = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl
{
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub namespace: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl { metric_name : self . metric_name , namespace : self . namespace , dimensions : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metric_name` after provisioning.\n"] pub fn metric_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.metric_name" , self . base)) } # [doc = "Get a reference to the value of field `namespace` after provisioning.\n"] pub fn namespace (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.namespace" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElDynamic { metric : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl { stat : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] unit : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] metric : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl { # [doc = "Set the field `unit`.\n"] pub fn set_unit (mut self , v : impl Into < PrimField < String > >) -> Self { self . unit = Some (v . into ()) ; self } # [doc = "Set the field `metric`.\n"] pub fn set_metric (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . metric = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . metric = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl
{
    #[doc = ""]
    pub stat: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl { stat : self . stat , unit : core :: default :: Default :: default () , metric : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `stat` after provisioning.\n"] pub fn stat (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.stat" , self . base)) } # [doc = "Get a reference to the value of field `unit` after provisioning.\n"] pub fn unit (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.unit" , self . base)) } # [doc = "Get a reference to the value of field `metric` after provisioning.\n"] pub fn metric (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElMetricElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.metric" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElDynamic { metric_stat : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl { # [serde (skip_serializing_if = "Option::is_none")] expression : Option < PrimField < String > > , id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] label : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] return_data : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] metric_stat : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl { # [doc = "Set the field `expression`.\n"] pub fn set_expression (mut self , v : impl Into < PrimField < String > >) -> Self { self . expression = Some (v . into ()) ; self } # [doc = "Set the field `label`.\n"] pub fn set_label (mut self , v : impl Into < PrimField < String > >) -> Self { self . label = Some (v . into ()) ; self } # [doc = "Set the field `return_data`.\n"] pub fn set_return_data (mut self , v : impl Into < PrimField < bool > >) -> Self { self . return_data = Some (v . into ()) ; self } # [doc = "Set the field `metric_stat`.\n"] pub fn set_metric_stat (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . metric_stat = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . metric_stat = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl
{
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl { expression : core :: default :: Default :: default () , id : self . id , label : core :: default :: Default :: default () , return_data : core :: default :: Default :: default () , metric_stat : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `expression` after provisioning.\n"] pub fn expression (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.expression" , self . base)) } # [doc = "Get a reference to the value of field `id` after provisioning.\n"] pub fn id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.id" , self . base)) } # [doc = "Get a reference to the value of field `label` after provisioning.\n"] pub fn label (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.label" , self . base)) } # [doc = "Get a reference to the value of field `return_data` after provisioning.\n"] pub fn return_data (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.return_data" , self . base)) } # [doc = "Get a reference to the value of field `metric_stat` after provisioning.\n"] pub fn metric_stat (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElMetricStatElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.metric_stat" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElDynamic { metric_data_queries : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl { # [serde (skip_serializing_if = "Option::is_none")] metric_data_queries : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl { # [doc = "Set the field `metric_data_queries`.\n"] pub fn set_metric_data_queries (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . metric_data_queries = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . metric_data_queries = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl
{}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl { metric_data_queries : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metric_data_queries` after provisioning.\n"] pub fn metric_data_queries (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueriesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.metric_data_queries" , self . base)) } }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl
{
    predefined_metric_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_label: Option<PrimField<String>>,
}
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl { # [doc = "Set the field `resource_label`.\n"] pub fn set_resource_label (mut self , v : impl Into < PrimField < String > >) -> Self { self . resource_label = Some (v . into ()) ; self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl
{
    #[doc = ""]
    pub predefined_metric_type: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl { predefined_metric_type : self . predefined_metric_type , resource_label : core :: default :: Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"] pub fn predefined_metric_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.predefined_metric_type" , self . base)) } # [doc = "Get a reference to the value of field `resource_label` after provisioning.\n"] pub fn resource_label (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.resource_label" , self . base)) } }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl
{
    predefined_metric_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_label: Option<PrimField<String>>,
}
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl { # [doc = "Set the field `resource_label`.\n"] pub fn set_resource_label (mut self , v : impl Into < PrimField < String > >) -> Self { self . resource_label = Some (v . into ()) ; self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl
{
    #[doc = ""]
    pub predefined_metric_type: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl { predefined_metric_type : self . predefined_metric_type , resource_label : core :: default :: Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"] pub fn predefined_metric_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.predefined_metric_type" , self . base)) } # [doc = "Get a reference to the value of field `resource_label` after provisioning.\n"] pub fn resource_label (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.resource_label" , self . base)) } }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl
{
    predefined_metric_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_label: Option<PrimField<String>>,
}
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl { # [doc = "Set the field `resource_label`.\n"] pub fn set_resource_label (mut self , v : impl Into < PrimField < String > >) -> Self { self . resource_label = Some (v . into ()) ; self } }
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl { type O = BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl
{
    #[doc = ""]
    pub predefined_metric_type: PrimField<String>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl { pub fn build (self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl { predefined_metric_type : self . predefined_metric_type , resource_label : core :: default :: Default :: default () , } } }
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef { AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"] pub fn predefined_metric_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.predefined_metric_type" , self . base)) } # [doc = "Get a reference to the value of field `resource_label` after provisioning.\n"] pub fn resource_label (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.resource_label" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElDynamic { customized_capacity_metric_specification : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl >> , customized_load_metric_specification : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl >> , customized_scaling_metric_specification : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl >> , predefined_load_metric_specification : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl >> , predefined_metric_pair_specification : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl >> , predefined_scaling_metric_specification : Option < DynamicBlock < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl { target_value : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] customized_capacity_metric_specification : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl > > , # [serde (skip_serializing_if = "Option::is_none")] customized_load_metric_specification : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl > > , # [serde (skip_serializing_if = "Option::is_none")] customized_scaling_metric_specification : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl > > , # [serde (skip_serializing_if = "Option::is_none")] predefined_load_metric_specification : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl > > , # [serde (skip_serializing_if = "Option::is_none")] predefined_metric_pair_specification : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl > > , # [serde (skip_serializing_if = "Option::is_none")] predefined_scaling_metric_specification : Option < Vec < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl > > , dynamic : AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElDynamic , }
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
    #[doc = "Set the field `customized_capacity_metric_specification`.\n"]
    pub fn set_customized_capacity_metric_specification(
        mut self,
        v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customized_capacity_metric_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_capacity_metric_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `customized_load_metric_specification`.\n"]
    pub fn set_customized_load_metric_specification(
        mut self,
        v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customized_load_metric_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_load_metric_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `customized_scaling_metric_specification`.\n"]
    pub fn set_customized_scaling_metric_specification(
        mut self,
        v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customized_scaling_metric_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_scaling_metric_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `predefined_load_metric_specification`.\n"]
    pub fn set_predefined_load_metric_specification(
        mut self,
        v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.predefined_load_metric_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_load_metric_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `predefined_metric_pair_specification`.\n"]
    pub fn set_predefined_metric_pair_specification(
        mut self,
        v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.predefined_metric_pair_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_metric_pair_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `predefined_scaling_metric_specification`.\n"]
    pub fn set_predefined_scaling_metric_specification(
        mut self,
        v : impl Into < BlockAssignable < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.predefined_scaling_metric_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_scaling_metric_specification = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
    type O =
        BlockAssignable<AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
    #[doc = ""]
    pub target_value: PrimField<f64>,
}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
    pub fn build(self) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl {
            target_value: self.target_value,
            customized_capacity_metric_specification: core::default::Default::default(),
            customized_load_metric_specification: core::default::Default::default(),
            customized_scaling_metric_specification: core::default::Default::default(),
            predefined_load_metric_specification: core::default::Default::default(),
            predefined_metric_pair_specification: core::default::Default::default(),
            predefined_scaling_metric_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }
    #[doc = "Get a reference to the value of field `customized_capacity_metric_specification` after provisioning.\n"]    pub fn customized_capacity_metric_specification (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.customized_capacity_metric_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `customized_load_metric_specification` after provisioning.\n"]    pub fn customized_load_metric_specification (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.customized_load_metric_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `customized_scaling_metric_specification` after provisioning.\n"]    pub fn customized_scaling_metric_specification (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.customized_scaling_metric_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `predefined_load_metric_specification` after provisioning.\n"]    pub fn predefined_load_metric_specification (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_load_metric_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `predefined_metric_pair_specification` after provisioning.\n"]    pub fn predefined_metric_pair_specification (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_metric_pair_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `predefined_scaling_metric_specification` after provisioning.\n"]    pub fn predefined_scaling_metric_specification (& self) -> ListRef < AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_scaling_metric_specification", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AutoscalingPolicyPredictiveScalingConfigurationElDynamic {
    metric_specification: Option<
        DynamicBlock<AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl>,
    >,
}
#[derive(Serialize)]
pub struct AutoscalingPolicyPredictiveScalingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_capacity_breach_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_capacity_buffer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduling_buffer_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_specification:
        Option<Vec<AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl>>,
    dynamic: AutoscalingPolicyPredictiveScalingConfigurationElDynamic,
}
impl AutoscalingPolicyPredictiveScalingConfigurationEl {
    #[doc = "Set the field `max_capacity_breach_behavior`.\n"]
    pub fn set_max_capacity_breach_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_capacity_breach_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `max_capacity_buffer`.\n"]
    pub fn set_max_capacity_buffer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_capacity_buffer = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `scheduling_buffer_time`.\n"]
    pub fn set_scheduling_buffer_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scheduling_buffer_time = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_specification`.\n"]
    pub fn set_metric_specification(
        mut self,
        v: impl Into<
            BlockAssignable<AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_specification = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AutoscalingPolicyPredictiveScalingConfigurationEl {
    type O = BlockAssignable<AutoscalingPolicyPredictiveScalingConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAutoscalingPolicyPredictiveScalingConfigurationEl {}
impl BuildAutoscalingPolicyPredictiveScalingConfigurationEl {
    pub fn build(self) -> AutoscalingPolicyPredictiveScalingConfigurationEl {
        AutoscalingPolicyPredictiveScalingConfigurationEl {
            max_capacity_breach_behavior: core::default::Default::default(),
            max_capacity_buffer: core::default::Default::default(),
            mode: core::default::Default::default(),
            scheduling_buffer_time: core::default::Default::default(),
            metric_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AutoscalingPolicyPredictiveScalingConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyPredictiveScalingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingPolicyPredictiveScalingConfigurationElRef {
        AutoscalingPolicyPredictiveScalingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AutoscalingPolicyPredictiveScalingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_capacity_breach_behavior` after provisioning.\n"]
    pub fn max_capacity_breach_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_capacity_breach_behavior", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_capacity_buffer` after provisioning.\n"]
    pub fn max_capacity_buffer(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_capacity_buffer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `scheduling_buffer_time` after provisioning.\n"]
    pub fn scheduling_buffer_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduling_buffer_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric_specification` after provisioning.\n"]
    pub fn metric_specification(
        &self,
    ) -> ListRef<AutoscalingPolicyPredictiveScalingConfigurationElMetricSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_specification", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AutoscalingPolicyStepAdjustmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_interval_lower_bound: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_interval_upper_bound: Option<PrimField<String>>,
    scaling_adjustment: PrimField<f64>,
}
impl AutoscalingPolicyStepAdjustmentEl {
    #[doc = "Set the field `metric_interval_lower_bound`.\n"]
    pub fn set_metric_interval_lower_bound(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_interval_lower_bound = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_interval_upper_bound`.\n"]
    pub fn set_metric_interval_upper_bound(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_interval_upper_bound = Some(v.into());
        self
    }
}
impl ToListMappable for AutoscalingPolicyStepAdjustmentEl {
    type O = BlockAssignable<AutoscalingPolicyStepAdjustmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAutoscalingPolicyStepAdjustmentEl {
    #[doc = ""]
    pub scaling_adjustment: PrimField<f64>,
}
impl BuildAutoscalingPolicyStepAdjustmentEl {
    pub fn build(self) -> AutoscalingPolicyStepAdjustmentEl {
        AutoscalingPolicyStepAdjustmentEl {
            metric_interval_lower_bound: core::default::Default::default(),
            metric_interval_upper_bound: core::default::Default::default(),
            scaling_adjustment: self.scaling_adjustment,
        }
    }
}
pub struct AutoscalingPolicyStepAdjustmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyStepAdjustmentElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingPolicyStepAdjustmentElRef {
        AutoscalingPolicyStepAdjustmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AutoscalingPolicyStepAdjustmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metric_interval_lower_bound` after provisioning.\n"]
    pub fn metric_interval_lower_bound(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_interval_lower_bound", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric_interval_upper_bound` after provisioning.\n"]
    pub fn metric_interval_upper_bound(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_interval_upper_bound", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scaling_adjustment` after provisioning.\n"]
    pub fn scaling_adjustment(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scaling_adjustment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl
    AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl
{
}
impl ToListMappable for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl { type O = BlockAssignable < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl { pub fn build (self) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl { AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl { name : self . name , value : self . value , } } }
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionElRef { AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl
{
    name: PrimField<String>,
    value: PrimField<String>,
}
impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl { }
impl ToListMappable for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl { type O = BlockAssignable < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl { pub fn build (self) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl { AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl { name : self . name , value : self . value , } } }
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsElRef { AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDynamic { dimensions : Option < DynamicBlock < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl { metric_name : PrimField < String > , namespace : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] dimensions : Option < Vec < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl > > , dynamic : AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDynamic , }
impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl { # [doc = "Set the field `dimensions`.\n"] pub fn set_dimensions (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dimensions = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dimensions = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl { type O = BlockAssignable < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl
{
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub namespace: PrimField<String>,
}
impl BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl { pub fn build (self) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl { AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl { metric_name : self . metric_name , namespace : self . namespace , dimensions : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElRef { AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `metric_name` after provisioning.\n"] pub fn metric_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.metric_name" , self . base)) } # [doc = "Get a reference to the value of field `namespace` after provisioning.\n"] pub fn namespace (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.namespace" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElDynamic { metric : Option < DynamicBlock < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl { # [serde (skip_serializing_if = "Option::is_none")] period : Option < PrimField < f64 > > , stat : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] unit : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] metric : Option < Vec < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl > > , dynamic : AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElDynamic , }
impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl { # [doc = "Set the field `period`.\n"] pub fn set_period (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . period = Some (v . into ()) ; self } # [doc = "Set the field `unit`.\n"] pub fn set_unit (mut self , v : impl Into < PrimField < String > >) -> Self { self . unit = Some (v . into ()) ; self } # [doc = "Set the field `metric`.\n"] pub fn set_metric (mut self , v : impl Into < BlockAssignable < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . metric = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . metric = Some (d) ; } } self } }
impl ToListMappable for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl { type O = BlockAssignable < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl
{
    #[doc = ""]
    pub stat: PrimField<String>,
}
impl BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl { pub fn build (self) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl { AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl { period : core :: default :: Default :: default () , stat : self . stat , unit : core :: default :: Default :: default () , metric : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElRef { fn new (shared : StackShared , base : String) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElRef { AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElRef { shared : shared , base : base . to_string () , } } }
impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `period` after provisioning.\n"] pub fn period (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.period" , self . base)) } # [doc = "Get a reference to the value of field `stat` after provisioning.\n"] pub fn stat (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.stat" , self . base)) } # [doc = "Get a reference to the value of field `unit` after provisioning.\n"] pub fn unit (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.unit" , self . base)) } # [doc = "Get a reference to the value of field `metric` after provisioning.\n"] pub fn metric (& self) -> ListRef < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.metric" , self . base)) } }
#[derive(Serialize, Default)]
struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElDynamic { metric_stat : Option < DynamicBlock < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsEl { # [serde (skip_serializing_if = "Option::is_none")] expression : Option < PrimField < String > > , id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] label : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] return_data : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] metric_stat : Option < Vec < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl > > , dynamic : AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElDynamic , }
impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.label = Some(v.into());
        self
    }
    #[doc = "Set the field `return_data`.\n"]
    pub fn set_return_data(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.return_data = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_stat`.\n"]
    pub fn set_metric_stat(
        mut self,
        v : impl Into < BlockAssignable < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_stat = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_stat = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsEl
{
    type O = BlockAssignable<
        AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsEl
{
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsEl
    {
        AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsEl {
            expression: core::default::Default::default(),
            id: self.id,
            label: core::default::Default::default(),
            return_data: core::default::Default::default(),
            metric_stat: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElRef
    {
        AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
    #[doc = "Get a reference to the value of field `return_data` after provisioning.\n"]
    pub fn return_data(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.return_data", self.base))
    }
    #[doc = "Get a reference to the value of field `metric_stat` after provisioning.\n"]    pub fn metric_stat (& self) -> ListRef < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElRef >{
        ListRef::new(self.shared().clone(), format!("{}.metric_stat", self.base))
    }
}
#[derive(Serialize, Default)]
struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElDynamic { metric_dimension : Option < DynamicBlock < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl >> , metrics : Option < DynamicBlock < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsEl >> , }
#[derive(Serialize)]
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl { # [serde (skip_serializing_if = "Option::is_none")] metric_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] namespace : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] period : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] statistic : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] unit : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] metric_dimension : Option < Vec < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl > > , # [serde (skip_serializing_if = "Option::is_none")] metrics : Option < Vec < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsEl > > , dynamic : AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElDynamic , }
impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl {
    #[doc = "Set the field `metric_name`.\n"]
    pub fn set_metric_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_name = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `period`.\n"]
    pub fn set_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period = Some(v.into());
        self
    }
    #[doc = "Set the field `statistic`.\n"]
    pub fn set_statistic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.statistic = Some(v.into());
        self
    }
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_dimension`.\n"]
    pub fn set_metric_dimension(
        mut self,
        v : impl Into < BlockAssignable < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_dimension = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_dimension = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v : impl Into < BlockAssignable < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metrics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metrics = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl
{
    type O = BlockAssignable<
        AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl {}
impl BuildAutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl {
        AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl {
            metric_name: core::default::Default::default(),
            namespace: core::default::Default::default(),
            period: core::default::Default::default(),
            statistic: core::default::Default::default(),
            unit: core::default::Default::default(),
            metric_dimension: core::default::Default::default(),
            metrics: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElRef {
        AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }
    #[doc = "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.base))
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `metric_dimension` after provisioning.\n"]    pub fn metric_dimension (& self) -> ListRef < AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElMetricDimensionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_dimension", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
    predefined_metric_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_label: Option<PrimField<String>>,
}
impl AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_label = Some(v.into());
        self
    }
}
impl ToListMappable
    for AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl
{
    type O = BlockAssignable<
        AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
    #[doc = ""]
    pub predefined_metric_type: PrimField<String>,
}
impl BuildAutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
        AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}
pub struct AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationElRef {
        AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"]
    pub fn predefined_metric_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.predefined_metric_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_label", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AutoscalingPolicyTargetTrackingConfigurationElDynamic {
    customized_metric_specification: Option<
        DynamicBlock<AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl>,
    >,
    predefined_metric_specification: Option<
        DynamicBlock<AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl>,
    >,
}
#[derive(Serialize)]
pub struct AutoscalingPolicyTargetTrackingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_scale_in: Option<PrimField<bool>>,
    target_value: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customized_metric_specification:
        Option<Vec<AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_metric_specification:
        Option<Vec<AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl>>,
    dynamic: AutoscalingPolicyTargetTrackingConfigurationElDynamic,
}
impl AutoscalingPolicyTargetTrackingConfigurationEl {
    #[doc = "Set the field `disable_scale_in`.\n"]
    pub fn set_disable_scale_in(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_scale_in = Some(v.into());
        self
    }
    #[doc = "Set the field `customized_metric_specification`.\n"]
    pub fn set_customized_metric_specification(
        mut self,
        v: impl Into<
            BlockAssignable<
                AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customized_metric_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_metric_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `predefined_metric_specification`.\n"]
    pub fn set_predefined_metric_specification(
        mut self,
        v: impl Into<
            BlockAssignable<
                AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.predefined_metric_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_metric_specification = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AutoscalingPolicyTargetTrackingConfigurationEl {
    type O = BlockAssignable<AutoscalingPolicyTargetTrackingConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAutoscalingPolicyTargetTrackingConfigurationEl {
    #[doc = ""]
    pub target_value: PrimField<f64>,
}
impl BuildAutoscalingPolicyTargetTrackingConfigurationEl {
    pub fn build(self) -> AutoscalingPolicyTargetTrackingConfigurationEl {
        AutoscalingPolicyTargetTrackingConfigurationEl {
            disable_scale_in: core::default::Default::default(),
            target_value: self.target_value,
            customized_metric_specification: core::default::Default::default(),
            predefined_metric_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AutoscalingPolicyTargetTrackingConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AutoscalingPolicyTargetTrackingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingPolicyTargetTrackingConfigurationElRef {
        AutoscalingPolicyTargetTrackingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AutoscalingPolicyTargetTrackingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_scale_in` after provisioning.\n"]
    pub fn disable_scale_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_scale_in", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }
    #[doc = "Get a reference to the value of field `customized_metric_specification` after provisioning.\n"]
    pub fn customized_metric_specification(
        &self,
    ) -> ListRef<AutoscalingPolicyTargetTrackingConfigurationElCustomizedMetricSpecificationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customized_metric_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `predefined_metric_specification` after provisioning.\n"]
    pub fn predefined_metric_specification(
        &self,
    ) -> ListRef<AutoscalingPolicyTargetTrackingConfigurationElPredefinedMetricSpecificationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_metric_specification", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AutoscalingPolicyDynamic {
    predictive_scaling_configuration:
        Option<DynamicBlock<AutoscalingPolicyPredictiveScalingConfigurationEl>>,
    step_adjustment: Option<DynamicBlock<AutoscalingPolicyStepAdjustmentEl>>,
    target_tracking_configuration:
        Option<DynamicBlock<AutoscalingPolicyTargetTrackingConfigurationEl>>,
}
