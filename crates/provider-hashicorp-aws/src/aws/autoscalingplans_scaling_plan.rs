use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AutoscalingplansScalingPlanData {
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
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_source: Option<Vec<AutoscalingplansScalingPlanApplicationSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_instruction: Option<Vec<AutoscalingplansScalingPlanScalingInstructionEl>>,
    dynamic: AutoscalingplansScalingPlanDynamic,
}

struct AutoscalingplansScalingPlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AutoscalingplansScalingPlanData>,
}

#[derive(Clone)]
pub struct AutoscalingplansScalingPlan(Rc<AutoscalingplansScalingPlan_>);

impl AutoscalingplansScalingPlan {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `application_source`.\n"]
    pub fn set_application_source(
        self,
        v: impl Into<BlockAssignable<AutoscalingplansScalingPlanApplicationSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().application_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.application_source = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaling_instruction`.\n"]
    pub fn set_scaling_instruction(
        self,
        v: impl Into<BlockAssignable<AutoscalingplansScalingPlanScalingInstructionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_instruction = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_instruction = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_plan_version` after provisioning.\n"]
    pub fn scaling_plan_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scaling_plan_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_source` after provisioning.\n"]
    pub fn application_source(&self) -> ListRef<AutoscalingplansScalingPlanApplicationSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.application_source", self.extract_ref()))
    }
}

impl Referable for AutoscalingplansScalingPlan {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AutoscalingplansScalingPlan { }

impl ToListMappable for AutoscalingplansScalingPlan {
    type O = ListRef<AutoscalingplansScalingPlanRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AutoscalingplansScalingPlan_ {
    fn extract_resource_type(&self) -> String {
        "aws_autoscalingplans_scaling_plan".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAutoscalingplansScalingPlan {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAutoscalingplansScalingPlan {
    pub fn build(self, stack: &mut Stack) -> AutoscalingplansScalingPlan {
        let out = AutoscalingplansScalingPlan(Rc::new(AutoscalingplansScalingPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AutoscalingplansScalingPlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                application_source: core::default::Default::default(),
                scaling_instruction: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AutoscalingplansScalingPlanRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingplansScalingPlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AutoscalingplansScalingPlanRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_plan_version` after provisioning.\n"]
    pub fn scaling_plan_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scaling_plan_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_source` after provisioning.\n"]
    pub fn application_source(&self) -> ListRef<AutoscalingplansScalingPlanApplicationSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.application_source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlanApplicationSourceElTagFilterEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl AutoscalingplansScalingPlanApplicationSourceElTagFilterEl {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingplansScalingPlanApplicationSourceElTagFilterEl {
    type O = BlockAssignable<AutoscalingplansScalingPlanApplicationSourceElTagFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlanApplicationSourceElTagFilterEl {
    #[doc = ""]
    pub key: PrimField<String>,
}

impl BuildAutoscalingplansScalingPlanApplicationSourceElTagFilterEl {
    pub fn build(self) -> AutoscalingplansScalingPlanApplicationSourceElTagFilterEl {
        AutoscalingplansScalingPlanApplicationSourceElTagFilterEl {
            key: self.key,
            values: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingplansScalingPlanApplicationSourceElTagFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingplansScalingPlanApplicationSourceElTagFilterElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingplansScalingPlanApplicationSourceElTagFilterElRef {
        AutoscalingplansScalingPlanApplicationSourceElTagFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlanApplicationSourceElTagFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingplansScalingPlanApplicationSourceElDynamic {
    tag_filter: Option<DynamicBlock<AutoscalingplansScalingPlanApplicationSourceElTagFilterEl>>,
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlanApplicationSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudformation_stack_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_filter: Option<Vec<AutoscalingplansScalingPlanApplicationSourceElTagFilterEl>>,
    dynamic: AutoscalingplansScalingPlanApplicationSourceElDynamic,
}

impl AutoscalingplansScalingPlanApplicationSourceEl {
    #[doc = "Set the field `cloudformation_stack_arn`.\n"]
    pub fn set_cloudformation_stack_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloudformation_stack_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `tag_filter`.\n"]
    pub fn set_tag_filter(
        mut self,
        v: impl Into<BlockAssignable<AutoscalingplansScalingPlanApplicationSourceElTagFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_filter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AutoscalingplansScalingPlanApplicationSourceEl {
    type O = BlockAssignable<AutoscalingplansScalingPlanApplicationSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlanApplicationSourceEl {}

impl BuildAutoscalingplansScalingPlanApplicationSourceEl {
    pub fn build(self) -> AutoscalingplansScalingPlanApplicationSourceEl {
        AutoscalingplansScalingPlanApplicationSourceEl {
            cloudformation_stack_arn: core::default::Default::default(),
            tag_filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AutoscalingplansScalingPlanApplicationSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingplansScalingPlanApplicationSourceElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingplansScalingPlanApplicationSourceElRef {
        AutoscalingplansScalingPlanApplicationSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlanApplicationSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudformation_stack_arn` after provisioning.\n"]
    pub fn cloudformation_stack_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudformation_stack_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<RecField<PrimField<String>>>,
    metric_name: PrimField<String>,
    namespace: PrimField<String>,
    statistic: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
}

impl AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationEl {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.dimensions = Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationEl {
    type O = BlockAssignable<AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationEl {
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub namespace: PrimField<String>,
    #[doc = ""]
    pub statistic: PrimField<String>,
}

impl BuildAutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationEl {
    pub fn build(self) -> AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationEl {
        AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationEl {
            dimensions: core::default::Default::default(),
            metric_name: self.metric_name,
            namespace: self.namespace,
            statistic: self.statistic,
            unit: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationElRef {
        AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
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
pub struct AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationEl {
    predefined_load_metric_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_label: Option<PrimField<String>>,
}

impl AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationEl {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_label = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationEl {
    type O = BlockAssignable<AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationEl {
    #[doc = ""]
    pub predefined_load_metric_type: PrimField<String>,
}

impl BuildAutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationEl {
    pub fn build(self) -> AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationEl {
        AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationEl {
            predefined_load_metric_type: self.predefined_load_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationElRef {
        AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_load_metric_type` after provisioning.\n"]
    pub fn predefined_load_metric_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.predefined_load_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<RecField<PrimField<String>>>,
    metric_name: PrimField<String>,
    namespace: PrimField<String>,
    statistic: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
}

impl AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationEl {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.dimensions = Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationEl {
    type O =
        BlockAssignable<
            AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationEl {
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub namespace: PrimField<String>,
    #[doc = ""]
    pub statistic: PrimField<String>,
}

impl BuildAutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationEl {
        AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationEl {
            dimensions: core::default::Default::default(),
            metric_name: self.metric_name,
            namespace: self.namespace,
            statistic: self.statistic,
            unit: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationElRef {
        AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
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
pub struct AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationEl {
    predefined_scaling_metric_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_label: Option<PrimField<String>>,
}

impl AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationEl {
    #[doc = "Set the field `resource_label`.\n"]
    pub fn set_resource_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_label = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationEl {
    type O =
        BlockAssignable<
            AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationEl {
    #[doc = ""]
    pub predefined_scaling_metric_type: PrimField<String>,
}

impl BuildAutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationEl {
    pub fn build(
        self,
    ) -> AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationEl {
        AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationEl {
            predefined_scaling_metric_type: self.predefined_scaling_metric_type,
            resource_label: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationElRef {
        AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_scaling_metric_type` after provisioning.\n"]
    pub fn predefined_scaling_metric_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.predefined_scaling_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_label` after provisioning.\n"]
    pub fn resource_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_label", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElDynamic {
    customized_scaling_metric_specification: Option<
        DynamicBlock<
            AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationEl,
        >,
    >,
    predefined_scaling_metric_specification: Option<
        DynamicBlock<
            AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_scale_in: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_instance_warmup: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_in_cooldown: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_out_cooldown: Option<PrimField<f64>>,
    target_value: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customized_scaling_metric_specification: Option<
        Vec<
            AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_scaling_metric_specification: Option<
        Vec<
            AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationEl,
        >,
    >,
    dynamic: AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElDynamic,
}

impl AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationEl {
    #[doc = "Set the field `disable_scale_in`.\n"]
    pub fn set_disable_scale_in(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_scale_in = Some(v.into());
        self
    }

    #[doc = "Set the field `estimated_instance_warmup`.\n"]
    pub fn set_estimated_instance_warmup(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.estimated_instance_warmup = Some(v.into());
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

    #[doc = "Set the field `customized_scaling_metric_specification`.\n"]
    pub fn set_customized_scaling_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationEl,
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

    #[doc = "Set the field `predefined_scaling_metric_specification`.\n"]
    pub fn set_predefined_scaling_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationEl,
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

impl ToListMappable for AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationEl {
    type O = BlockAssignable<AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationEl {
    #[doc = ""]
    pub target_value: PrimField<f64>,
}

impl BuildAutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationEl {
    pub fn build(self) -> AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationEl {
        AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationEl {
            disable_scale_in: core::default::Default::default(),
            estimated_instance_warmup: core::default::Default::default(),
            scale_in_cooldown: core::default::Default::default(),
            scale_out_cooldown: core::default::Default::default(),
            target_value: self.target_value,
            customized_scaling_metric_specification: core::default::Default::default(),
            predefined_scaling_metric_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElRef {
        AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_scale_in` after provisioning.\n"]
    pub fn disable_scale_in(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_scale_in", self.base))
    }

    #[doc = "Get a reference to the value of field `estimated_instance_warmup` after provisioning.\n"]
    pub fn estimated_instance_warmup(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.estimated_instance_warmup", self.base))
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

    #[doc = "Get a reference to the value of field `customized_scaling_metric_specification` after provisioning.\n"]
    pub fn customized_scaling_metric_specification(
        &self,
    ) -> ListRef<
        AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElCustomizedScalingMetricSpecificationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.customized_scaling_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_scaling_metric_specification` after provisioning.\n"]
    pub fn predefined_scaling_metric_specification(
        &self,
    ) -> ListRef<
        AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationElPredefinedScalingMetricSpecificationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.predefined_scaling_metric_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingplansScalingPlanScalingInstructionElDynamic {
    customized_load_metric_specification: Option<
        DynamicBlock<AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationEl>,
    >,
    predefined_load_metric_specification: Option<
        DynamicBlock<AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationEl>,
    >,
    target_tracking_configuration: Option<
        DynamicBlock<AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct AutoscalingplansScalingPlanScalingInstructionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_dynamic_scaling: Option<PrimField<bool>>,
    max_capacity: PrimField<f64>,
    min_capacity: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predictive_scaling_max_capacity_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predictive_scaling_max_capacity_buffer: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predictive_scaling_mode: Option<PrimField<String>>,
    resource_id: PrimField<String>,
    scalable_dimension: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_policy_update_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_action_buffer_time: Option<PrimField<f64>>,
    service_namespace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customized_load_metric_specification: Option<
        Vec<AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_load_metric_specification: Option<
        Vec<AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_tracking_configuration: Option<
        Vec<AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationEl>,
    >,
    dynamic: AutoscalingplansScalingPlanScalingInstructionElDynamic,
}

impl AutoscalingplansScalingPlanScalingInstructionEl {
    #[doc = "Set the field `disable_dynamic_scaling`.\n"]
    pub fn set_disable_dynamic_scaling(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_dynamic_scaling = Some(v.into());
        self
    }

    #[doc = "Set the field `predictive_scaling_max_capacity_behavior`.\n"]
    pub fn set_predictive_scaling_max_capacity_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.predictive_scaling_max_capacity_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `predictive_scaling_max_capacity_buffer`.\n"]
    pub fn set_predictive_scaling_max_capacity_buffer(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.predictive_scaling_max_capacity_buffer = Some(v.into());
        self
    }

    #[doc = "Set the field `predictive_scaling_mode`.\n"]
    pub fn set_predictive_scaling_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.predictive_scaling_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `scaling_policy_update_behavior`.\n"]
    pub fn set_scaling_policy_update_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scaling_policy_update_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `scheduled_action_buffer_time`.\n"]
    pub fn set_scheduled_action_buffer_time(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scheduled_action_buffer_time = Some(v.into());
        self
    }

    #[doc = "Set the field `customized_load_metric_specification`.\n"]
    pub fn set_customized_load_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationEl,
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

    #[doc = "Set the field `predefined_load_metric_specification`.\n"]
    pub fn set_predefined_load_metric_specification(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationEl,
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

    #[doc = "Set the field `target_tracking_configuration`.\n"]
    pub fn set_target_tracking_configuration(
        mut self,
        v: impl Into<BlockAssignable<AutoscalingplansScalingPlanScalingInstructionElTargetTrackingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_tracking_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_tracking_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AutoscalingplansScalingPlanScalingInstructionEl {
    type O = BlockAssignable<AutoscalingplansScalingPlanScalingInstructionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingplansScalingPlanScalingInstructionEl {
    #[doc = ""]
    pub max_capacity: PrimField<f64>,
    #[doc = ""]
    pub min_capacity: PrimField<f64>,
    #[doc = ""]
    pub resource_id: PrimField<String>,
    #[doc = ""]
    pub scalable_dimension: PrimField<String>,
    #[doc = ""]
    pub service_namespace: PrimField<String>,
}

impl BuildAutoscalingplansScalingPlanScalingInstructionEl {
    pub fn build(self) -> AutoscalingplansScalingPlanScalingInstructionEl {
        AutoscalingplansScalingPlanScalingInstructionEl {
            disable_dynamic_scaling: core::default::Default::default(),
            max_capacity: self.max_capacity,
            min_capacity: self.min_capacity,
            predictive_scaling_max_capacity_behavior: core::default::Default::default(),
            predictive_scaling_max_capacity_buffer: core::default::Default::default(),
            predictive_scaling_mode: core::default::Default::default(),
            resource_id: self.resource_id,
            scalable_dimension: self.scalable_dimension,
            scaling_policy_update_behavior: core::default::Default::default(),
            scheduled_action_buffer_time: core::default::Default::default(),
            service_namespace: self.service_namespace,
            customized_load_metric_specification: core::default::Default::default(),
            predefined_load_metric_specification: core::default::Default::default(),
            target_tracking_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AutoscalingplansScalingPlanScalingInstructionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingplansScalingPlanScalingInstructionElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingplansScalingPlanScalingInstructionElRef {
        AutoscalingplansScalingPlanScalingInstructionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingplansScalingPlanScalingInstructionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_dynamic_scaling` after provisioning.\n"]
    pub fn disable_dynamic_scaling(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_dynamic_scaling", self.base))
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `min_capacity` after provisioning.\n"]
    pub fn min_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `predictive_scaling_max_capacity_behavior` after provisioning.\n"]
    pub fn predictive_scaling_max_capacity_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.predictive_scaling_max_capacity_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `predictive_scaling_max_capacity_buffer` after provisioning.\n"]
    pub fn predictive_scaling_max_capacity_buffer(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.predictive_scaling_max_capacity_buffer", self.base))
    }

    #[doc = "Get a reference to the value of field `predictive_scaling_mode` after provisioning.\n"]
    pub fn predictive_scaling_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.predictive_scaling_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }

    #[doc = "Get a reference to the value of field `scalable_dimension` after provisioning.\n"]
    pub fn scalable_dimension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scalable_dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_policy_update_behavior` after provisioning.\n"]
    pub fn scaling_policy_update_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scaling_policy_update_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduled_action_buffer_time` after provisioning.\n"]
    pub fn scheduled_action_buffer_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheduled_action_buffer_time", self.base))
    }

    #[doc = "Get a reference to the value of field `service_namespace` after provisioning.\n"]
    pub fn service_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `customized_load_metric_specification` after provisioning.\n"]
    pub fn customized_load_metric_specification(
        &self,
    ) -> ListRef<AutoscalingplansScalingPlanScalingInstructionElCustomizedLoadMetricSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.customized_load_metric_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `predefined_load_metric_specification` after provisioning.\n"]
    pub fn predefined_load_metric_specification(
        &self,
    ) -> ListRef<AutoscalingplansScalingPlanScalingInstructionElPredefinedLoadMetricSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.predefined_load_metric_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingplansScalingPlanDynamic {
    application_source: Option<DynamicBlock<AutoscalingplansScalingPlanApplicationSourceEl>>,
    scaling_instruction: Option<DynamicBlock<AutoscalingplansScalingPlanScalingInstructionEl>>,
}
