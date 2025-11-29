use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppautoscalingPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_id: PrimField<String>,
    scalable_dimension: PrimField<String>,
    service_namespace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predictive_scaling_policy_configuration: Option<Vec<AppautoscalingPolicyPredictiveScalingPolicyConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step_scaling_policy_configuration: Option<Vec<AppautoscalingPolicyStepScalingPolicyConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_tracking_scaling_policy_configuration: Option<
        Vec<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl>,
    >,
    dynamic: AppautoscalingPolicyDynamic,
}

struct AppautoscalingPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppautoscalingPolicyData>,
}

#[derive(Clone)]
pub struct AppautoscalingPolicy(Rc<AppautoscalingPolicy_>);

impl AppautoscalingPolicy {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `policy_type`.\n"]
    pub fn set_policy_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_type = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `predictive_scaling_policy_configuration`.\n"]
    pub fn set_predictive_scaling_policy_configuration(
        self,
        v: impl Into<BlockAssignable<AppautoscalingPolicyPredictiveScalingPolicyConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().predictive_scaling_policy_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.predictive_scaling_policy_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `step_scaling_policy_configuration`.\n"]
    pub fn set_step_scaling_policy_configuration(
        self,
        v: impl Into<BlockAssignable<AppautoscalingPolicyStepScalingPolicyConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().step_scaling_policy_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.step_scaling_policy_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_tracking_scaling_policy_configuration`.\n"]
    pub fn set_target_tracking_scaling_policy_configuration(
        self,
        v: impl Into<BlockAssignable<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_tracking_scaling_policy_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_tracking_scaling_policy_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `alarm_arns` after provisioning.\n"]
    pub fn alarm_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.alarm_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scalable_dimension` after provisioning.\n"]
    pub fn scalable_dimension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scalable_dimension", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_namespace` after provisioning.\n"]
    pub fn service_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predictive_scaling_policy_configuration` after provisioning.\n"]
    pub fn predictive_scaling_policy_configuration(
        &self,
    ) -> ListRef<AppautoscalingPolicyPredictiveScalingPolicyConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predictive_scaling_policy_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `step_scaling_policy_configuration` after provisioning.\n"]
    pub fn step_scaling_policy_configuration(&self) -> ListRef<AppautoscalingPolicyStepScalingPolicyConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.step_scaling_policy_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_tracking_scaling_policy_configuration` after provisioning.\n"]
    pub fn target_tracking_scaling_policy_configuration(
        &self,
    ) -> ListRef<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_tracking_scaling_policy_configuration", self.extract_ref()),
        )
    }
}

impl Referable for AppautoscalingPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AppautoscalingPolicy { }

impl ToListMappable for AppautoscalingPolicy {
    type O = ListRef<AppautoscalingPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppautoscalingPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_appautoscaling_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppautoscalingPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub resource_id: PrimField<String>,
    #[doc = ""]
    pub scalable_dimension: PrimField<String>,
    #[doc = ""]
    pub service_namespace: PrimField<String>,
}

impl BuildAppautoscalingPolicy {
    pub fn build(self, stack: &mut Stack) -> AppautoscalingPolicy {
        let out = AppautoscalingPolicy(Rc::new(AppautoscalingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppautoscalingPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                policy_type: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_id: self.resource_id,
                scalable_dimension: self.scalable_dimension,
                service_namespace: self.service_namespace,
                predictive_scaling_policy_configuration: core::default::Default::default(),
                step_scaling_policy_configuration: core::default::Default::default(),
                target_tracking_scaling_policy_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppautoscalingPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AppautoscalingPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarm_arns` after provisioning.\n"]
    pub fn alarm_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.alarm_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scalable_dimension` after provisioning.\n"]
    pub fn scalable_dimension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scalable_dimension", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_namespace` after provisioning.\n"]
    pub fn service_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predictive_scaling_policy_configuration` after provisioning.\n"]
    pub fn predictive_scaling_policy_configuration(
        &self,
    ) -> ListRef<AppautoscalingPolicyPredictiveScalingPolicyConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predictive_scaling_policy_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `step_scaling_policy_configuration` after provisioning.\n"]
    pub fn step_scaling_policy_configuration(&self) -> ListRef<AppautoscalingPolicyStepScalingPolicyConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.step_scaling_policy_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_tracking_scaling_policy_configuration` after provisioning.\n"]
    pub fn target_tracking_scaling_policy_configuration(
        &self,
    ) -> ListRef<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_tracking_scaling_policy_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {

}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDynamic {
    dimension: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
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

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
            metric_name: core::default::Default::default(),
            namespace: core::default::Default::default(),
            dimension: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
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
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElDynamic {
    metric: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatEl {
    stat: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatEl {
    #[doc = ""]
    pub stat: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatEl {
            stat: self.stat,
            unit: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stat` after provisioning.\n"]
    pub fn stat(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stat", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElDynamic {
    metric_stat: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_data: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_stat: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryEl {
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
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_stat = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_stat = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryEl {
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryEl {
            expression: core::default::Default::default(),
            id: self.id,
            label: core::default::Default::default(),
            return_data: core::default::Default::default(),
            metric_stat: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElRef {
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

    #[doc = "Get a reference to the value of field `metric_stat` after provisioning.\n"]
    pub fn metric_stat(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElMetricStatElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.metric_stat", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElDynamic {
    metric_data_query: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_data_query: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {
    #[doc = "Set the field `metric_data_query`.\n"]
    pub fn set_metric_data_query(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_data_query = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_data_query = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl {
            metric_data_query: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_data_query` after provisioning.\n"]
    pub fn metric_data_query(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElMetricDataQueryElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.metric_data_query", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {

}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDynamic {
    dimension: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
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

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
            metric_name: core::default::Default::default(),
            namespace: core::default::Default::default(),
            dimension: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
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
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElDynamic {
    metric: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatEl {
    stat: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatEl {
    #[doc = ""]
    pub stat: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatEl {
            stat: self.stat,
            unit: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stat` after provisioning.\n"]
    pub fn stat(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stat", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElDynamic {
    metric_stat: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_data: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_stat: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryEl {
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
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_stat = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_stat = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryEl {
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryEl {
            expression: core::default::Default::default(),
            id: self.id,
            label: core::default::Default::default(),
            return_data: core::default::Default::default(),
            metric_stat: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElRef {
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

    #[doc = "Get a reference to the value of field `metric_stat` after provisioning.\n"]
    pub fn metric_stat(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElMetricStatElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.metric_stat", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElDynamic {
    metric_data_query: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_data_query: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {
    #[doc = "Set the field `metric_data_query`.\n"]
    pub fn set_metric_data_query(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_data_query = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_data_query = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl {
            metric_data_query: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_data_query` after provisioning.\n"]
    pub fn metric_data_query(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElMetricDataQueryElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.metric_data_query", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {

}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDynamic {
    dimension: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
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

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElDimensionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricEl {
            metric_name: core::default::Default::default(),
            namespace: core::default::Default::default(),
            dimension: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef {
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
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElDynamic {
    metric: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatEl {
    stat: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatEl {
    #[doc = ""]
    pub stat: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatEl {
            stat: self.stat,
            unit: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stat` after provisioning.\n"]
    pub fn stat(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stat", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElMetricElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElDynamic {
    metric_stat: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_data: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_stat: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryEl {
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
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_stat = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_stat = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryEl {
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryEl {
            expression: core::default::Default::default(),
            id: self.id,
            label: core::default::Default::default(),
            return_data: core::default::Default::default(),
            metric_stat: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElRef {
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

    #[doc = "Get a reference to the value of field `metric_stat` after provisioning.\n"]
    pub fn metric_stat(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElMetricStatElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.metric_stat", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElDynamic {
    metric_data_query: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_data_query: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {
    #[doc = "Set the field `metric_data_query`.\n"]
    pub fn set_metric_data_query(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_data_query = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_data_query = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl {
            metric_data_query: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_data_query` after provisioning.\n"]
    pub fn metric_data_query(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElMetricDataQueryElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.metric_data_query", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
    predefined_metric_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_label: Option<PrimField<String>>,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_label = Some(v.into());
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
    #[doc = ""]
    pub predefined_metric_type: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"]
    pub fn predefined_metric_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.predefined_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
    predefined_metric_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_label: Option<PrimField<String>>,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_label = Some(v.into());
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
    #[doc = ""]
    pub predefined_metric_type: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"]
    pub fn predefined_metric_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.predefined_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
    predefined_metric_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_label: Option<PrimField<String>>,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_label = Some(v.into());
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
    #[doc = ""]
    pub predefined_metric_type: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"]
    pub fn predefined_metric_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.predefined_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElDynamic {
    customized_capacity_metric_specification: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl,
        >,
    >,
    customized_load_metric_specification: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl,
        >,
    >,
    customized_scaling_metric_specification: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl,
        >,
    >,
    predefined_load_metric_specification: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl,
        >,
    >,
    predefined_metric_pair_specification: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl,
        >,
    >,
    predefined_scaling_metric_specification: Option<
        DynamicBlock<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationEl {
    target_value: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customized_capacity_metric_specification: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    customized_load_metric_specification: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    customized_scaling_metric_specification: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_load_metric_specification: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_metric_pair_specification: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_scaling_metric_specification: Option<
        Vec<
            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl,
        >,
    >,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationEl {
    #[doc = "Set the field `customized_capacity_metric_specification`.\n"]
    pub fn set_customized_capacity_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customized_capacity_metric_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_capacity_metric_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `customized_load_metric_specification`.\n"]
    pub fn set_customized_load_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customized_load_metric_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_load_metric_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `customized_scaling_metric_specification`.\n"]
    pub fn set_customized_scaling_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customized_scaling_metric_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_scaling_metric_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predefined_load_metric_specification`.\n"]
    pub fn set_predefined_load_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.predefined_load_metric_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_load_metric_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predefined_metric_pair_specification`.\n"]
    pub fn set_predefined_metric_pair_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.predefined_metric_pair_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_metric_pair_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predefined_scaling_metric_specification`.\n"]
    pub fn set_predefined_scaling_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.predefined_scaling_metric_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_scaling_metric_specification = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationEl {
    type O = BlockAssignable<AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationEl {
    #[doc = ""]
    pub target_value: PrimField<String>,
}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationEl {
    pub fn build(self) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationEl {
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

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_capacity_metric_specification` after provisioning.\n"]
    pub fn customized_capacity_metric_specification(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedCapacityMetricSpecificationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.customized_capacity_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_load_metric_specification` after provisioning.\n"]
    pub fn customized_load_metric_specification(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedLoadMetricSpecificationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.customized_load_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_scaling_metric_specification` after provisioning.\n"]
    pub fn customized_scaling_metric_specification(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElCustomizedScalingMetricSpecificationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.customized_scaling_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_load_metric_specification` after provisioning.\n"]
    pub fn predefined_load_metric_specification(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedLoadMetricSpecificationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.predefined_load_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_metric_pair_specification` after provisioning.\n"]
    pub fn predefined_metric_pair_specification(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedMetricPairSpecificationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.predefined_metric_pair_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_scaling_metric_specification` after provisioning.\n"]
    pub fn predefined_scaling_metric_specification(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElPredefinedScalingMetricSpecificationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.predefined_scaling_metric_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElDynamic {
    metric_specification: Option<
        DynamicBlock<AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationEl>,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_capacity_breach_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_capacity_buffer: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduling_buffer_time: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_specification: Option<Vec<AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationEl>>,
    dynamic: AppautoscalingPolicyPredictiveScalingPolicyConfigurationElDynamic,
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationEl {
    #[doc = "Set the field `max_capacity_breach_behavior`.\n"]
    pub fn set_max_capacity_breach_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_capacity_breach_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `max_capacity_buffer`.\n"]
    pub fn set_max_capacity_buffer(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_capacity_buffer = Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc = "Set the field `scheduling_buffer_time`.\n"]
    pub fn set_scheduling_buffer_time(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scheduling_buffer_time = Some(v.into());
        self
    }

    #[doc = "Set the field `metric_specification`.\n"]
    pub fn set_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_specification = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyPredictiveScalingPolicyConfigurationEl {
    type O = BlockAssignable<AppautoscalingPolicyPredictiveScalingPolicyConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationEl {}

impl BuildAppautoscalingPolicyPredictiveScalingPolicyConfigurationEl {
    pub fn build(self) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationEl {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationEl {
            max_capacity_breach_behavior: core::default::Default::default(),
            max_capacity_buffer: core::default::Default::default(),
            mode: core::default::Default::default(),
            scheduling_buffer_time: core::default::Default::default(),
            metric_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyPredictiveScalingPolicyConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyPredictiveScalingPolicyConfigurationElRef {
    fn new(shared: StackShared, base: String) -> AppautoscalingPolicyPredictiveScalingPolicyConfigurationElRef {
        AppautoscalingPolicyPredictiveScalingPolicyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyPredictiveScalingPolicyConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_capacity_breach_behavior` after provisioning.\n"]
    pub fn max_capacity_breach_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_capacity_breach_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `max_capacity_buffer` after provisioning.\n"]
    pub fn max_capacity_buffer(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_capacity_buffer", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduling_buffer_time` after provisioning.\n"]
    pub fn scheduling_buffer_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheduling_buffer_time", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_specification` after provisioning.\n"]
    pub fn metric_specification(
        &self,
    ) -> ListRef<AppautoscalingPolicyPredictiveScalingPolicyConfigurationElMetricSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metric_specification", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_interval_lower_bound: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_interval_upper_bound: Option<PrimField<String>>,
    scaling_adjustment: PrimField<f64>,
}

impl AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
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

impl ToListMappable for AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
    type O = BlockAssignable<AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
    #[doc = ""]
    pub scaling_adjustment: PrimField<f64>,
}

impl BuildAppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
    pub fn build(self) -> AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
        AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl {
            metric_interval_lower_bound: core::default::Default::default(),
            metric_interval_upper_bound: core::default::Default::default(),
            scaling_adjustment: self.scaling_adjustment,
        }
    }
}

pub struct AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
        AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_interval_lower_bound` after provisioning.\n"]
    pub fn metric_interval_lower_bound(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_interval_lower_bound", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_interval_upper_bound` after provisioning.\n"]
    pub fn metric_interval_upper_bound(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_interval_upper_bound", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_adjustment` after provisioning.\n"]
    pub fn scaling_adjustment(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scaling_adjustment", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyStepScalingPolicyConfigurationElDynamic {
    step_adjustment: Option<DynamicBlock<AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl>>,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyStepScalingPolicyConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    adjustment_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cooldown: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_aggregation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_adjustment_magnitude: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step_adjustment: Option<Vec<AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl>>,
    dynamic: AppautoscalingPolicyStepScalingPolicyConfigurationElDynamic,
}

impl AppautoscalingPolicyStepScalingPolicyConfigurationEl {
    #[doc = "Set the field `adjustment_type`.\n"]
    pub fn set_adjustment_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.adjustment_type = Some(v.into());
        self
    }

    #[doc = "Set the field `cooldown`.\n"]
    pub fn set_cooldown(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cooldown = Some(v.into());
        self
    }

    #[doc = "Set the field `metric_aggregation_type`.\n"]
    pub fn set_metric_aggregation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_aggregation_type = Some(v.into());
        self
    }

    #[doc = "Set the field `min_adjustment_magnitude`.\n"]
    pub fn set_min_adjustment_magnitude(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_adjustment_magnitude = Some(v.into());
        self
    }

    #[doc = "Set the field `step_adjustment`.\n"]
    pub fn set_step_adjustment(
        mut self,
        v: impl Into<BlockAssignable<AppautoscalingPolicyStepScalingPolicyConfigurationElStepAdjustmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.step_adjustment = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.step_adjustment = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyStepScalingPolicyConfigurationEl {
    type O = BlockAssignable<AppautoscalingPolicyStepScalingPolicyConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyStepScalingPolicyConfigurationEl {}

impl BuildAppautoscalingPolicyStepScalingPolicyConfigurationEl {
    pub fn build(self) -> AppautoscalingPolicyStepScalingPolicyConfigurationEl {
        AppautoscalingPolicyStepScalingPolicyConfigurationEl {
            adjustment_type: core::default::Default::default(),
            cooldown: core::default::Default::default(),
            metric_aggregation_type: core::default::Default::default(),
            min_adjustment_magnitude: core::default::Default::default(),
            step_adjustment: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
    fn new(shared: StackShared, base: String) -> AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
        AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyStepScalingPolicyConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `adjustment_type` after provisioning.\n"]
    pub fn adjustment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.adjustment_type", self.base))
    }

    #[doc = "Get a reference to the value of field `cooldown` after provisioning.\n"]
    pub fn cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cooldown", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_aggregation_type` after provisioning.\n"]
    pub fn metric_aggregation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_aggregation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `min_adjustment_magnitude` after provisioning.\n"]
    pub fn min_adjustment_magnitude(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_adjustment_magnitude", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl { }

impl ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl {

}

impl ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDynamic {
    dimensions: Option<
        DynamicBlock<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl {
    metric_name: PrimField<String>,
    namespace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<
        Vec<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl,
        >,
    >,
    dynamic: AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDynamic,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElDimensionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimensions = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl {
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub namespace: PrimField<String>,
}

impl BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl {
            metric_name: self.metric_name,
            namespace: self.namespace,
            dimensions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElRef {
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
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElDynamic {
    metric: Option<
        DynamicBlock<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl {
    stat: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric: Option<
        Vec<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl,
        >,
    >,
    dynamic: AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElDynamic,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl {
    #[doc = ""]
    pub stat: PrimField<String>,
}

impl BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl {
            stat: self.stat,
            unit: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stat` after provisioning.\n"]
    pub fn stat(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stat", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElMetricElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElDynamic {
    metric_stat: Option<
        DynamicBlock<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_data: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_stat: Option<
        Vec<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl,
        >,
    >,
    dynamic: AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElDynamic,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsEl {
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
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_stat = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_stat = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsEl {
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsEl {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsEl {
            expression: core::default::Default::default(),
            id: self.id,
            label: core::default::Default::default(),
            return_data: core::default::Default::default(),
            metric_stat: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElRef {
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

    #[doc = "Get a reference to the value of field `metric_stat` after provisioning.\n"]
    pub fn metric_stat(
        &self,
    ) -> ListRef<
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsElMetricStatElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.metric_stat", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDynamic {
    dimensions: Option<
        DynamicBlock<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl,
        >,
    >,
    metrics: Option<
        DynamicBlock<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statistic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<
        Vec<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics: Option<
        Vec<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsEl>,
    >,
    dynamic: AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDynamic,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
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

    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElDimensionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimensions = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metrics = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {}

impl BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl {
            metric_name: core::default::Default::default(),
            namespace: core::default::Default::default(),
            statistic: core::default::Default::default(),
            unit: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            metrics: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef {
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

    #[doc = "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    predefined_metric_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_label: Option<PrimField<String>>,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_label = Some(v.into());
        self
    }
}

impl ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    type O =
        BlockAssignable<
            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    #[doc = ""]
    pub predefined_metric_type: PrimField<String>,
}

impl BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl {
            predefined_metric_type: self.predefined_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"]
    pub fn predefined_metric_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.predefined_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElDynamic {
    customized_metric_specification: Option<
        DynamicBlock<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl>,
    >,
    predefined_metric_specification: Option<
        DynamicBlock<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl>,
    >,
}

#[derive(Serialize)]
pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_scale_in: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_in_cooldown: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_out_cooldown: Option<PrimField<f64>>,
    target_value: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customized_metric_specification: Option<
        Vec<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_metric_specification: Option<
        Vec<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl>,
    >,
    dynamic: AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElDynamic,
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    #[doc = "Set the field `disable_scale_in`.\n"]
    pub fn set_disable_scale_in(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_scale_in = Some(v.into());
        self
    }

    #[doc = "Set the field `scale_in_cooldown`.\n"]
    pub fn set_scale_in_cooldown(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_in_cooldown = Some(v.into());
        self
    }

    #[doc = "Set the field `scale_out_cooldown`.\n"]
    pub fn set_scale_out_cooldown(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_out_cooldown = Some(v.into());
        self
    }

    #[doc = "Set the field `customized_metric_specification`.\n"]
    pub fn set_customized_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customized_metric_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customized_metric_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predefined_metric_specification`.\n"]
    pub fn set_predefined_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.predefined_metric_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.predefined_metric_specification = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    type O = BlockAssignable<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    #[doc = ""]
    pub target_value: PrimField<f64>,
}

impl BuildAppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
    pub fn build(self) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl {
            disable_scale_in: core::default::Default::default(),
            scale_in_cooldown: core::default::Default::default(),
            scale_out_cooldown: core::default::Default::default(),
            target_value: self.target_value,
            customized_metric_specification: core::default::Default::default(),
            predefined_metric_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
    fn new(shared: StackShared, base: String) -> AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
        AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_scale_in` after provisioning.\n"]
    pub fn disable_scale_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_scale_in", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_in_cooldown` after provisioning.\n"]
    pub fn scale_in_cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_in_cooldown", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out_cooldown` after provisioning.\n"]
    pub fn scale_out_cooldown(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_out_cooldown", self.base))
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_metric_specification` after provisioning.\n"]
    pub fn customized_metric_specification(
        &self,
    ) -> ListRef<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElCustomizedMetricSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.customized_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_metric_specification` after provisioning.\n"]
    pub fn predefined_metric_specification(
        &self,
    ) -> ListRef<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationElPredefinedMetricSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.predefined_metric_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingPolicyDynamic {
    predictive_scaling_policy_configuration: Option<
        DynamicBlock<AppautoscalingPolicyPredictiveScalingPolicyConfigurationEl>,
    >,
    step_scaling_policy_configuration: Option<DynamicBlock<AppautoscalingPolicyStepScalingPolicyConfigurationEl>>,
    target_tracking_scaling_policy_configuration: Option<
        DynamicBlock<AppautoscalingPolicyTargetTrackingScalingPolicyConfigurationEl>,
    >,
}
