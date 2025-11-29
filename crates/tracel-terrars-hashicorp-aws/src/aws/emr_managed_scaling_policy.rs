use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrManagedScalingPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    utilization_performance_index: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_limits: Option<Vec<EmrManagedScalingPolicyComputeLimitsEl>>,
    dynamic: EmrManagedScalingPolicyDynamic,
}

struct EmrManagedScalingPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmrManagedScalingPolicyData>,
}

#[derive(Clone)]
pub struct EmrManagedScalingPolicy(Rc<EmrManagedScalingPolicy_>);

impl EmrManagedScalingPolicy {
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

    #[doc = "Set the field `scaling_strategy`.\n"]
    pub fn set_scaling_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().scaling_strategy = Some(v.into());
        self
    }

    #[doc = "Set the field `utilization_performance_index`.\n"]
    pub fn set_utilization_performance_index(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().utilization_performance_index = Some(v.into());
        self
    }

    #[doc = "Set the field `compute_limits`.\n"]
    pub fn set_compute_limits(self, v: impl Into<BlockAssignable<EmrManagedScalingPolicyComputeLimitsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_limits = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_limits = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_strategy` after provisioning.\n"]
    pub fn scaling_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scaling_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `utilization_performance_index` after provisioning.\n"]
    pub fn utilization_performance_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.utilization_performance_index", self.extract_ref()))
    }
}

impl Referable for EmrManagedScalingPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EmrManagedScalingPolicy { }

impl ToListMappable for EmrManagedScalingPolicy {
    type O = ListRef<EmrManagedScalingPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EmrManagedScalingPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_emr_managed_scaling_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrManagedScalingPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_id: PrimField<String>,
}

impl BuildEmrManagedScalingPolicy {
    pub fn build(self, stack: &mut Stack) -> EmrManagedScalingPolicy {
        let out = EmrManagedScalingPolicy(Rc::new(EmrManagedScalingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmrManagedScalingPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                scaling_strategy: core::default::Default::default(),
                utilization_performance_index: core::default::Default::default(),
                compute_limits: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrManagedScalingPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrManagedScalingPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EmrManagedScalingPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_strategy` after provisioning.\n"]
    pub fn scaling_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scaling_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `utilization_performance_index` after provisioning.\n"]
    pub fn utilization_performance_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.utilization_performance_index", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EmrManagedScalingPolicyComputeLimitsEl {
    maximum_capacity_units: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_core_capacity_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_ondemand_capacity_units: Option<PrimField<f64>>,
    minimum_capacity_units: PrimField<f64>,
    unit_type: PrimField<String>,
}

impl EmrManagedScalingPolicyComputeLimitsEl {
    #[doc = "Set the field `maximum_core_capacity_units`.\n"]
    pub fn set_maximum_core_capacity_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_core_capacity_units = Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_ondemand_capacity_units`.\n"]
    pub fn set_maximum_ondemand_capacity_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_ondemand_capacity_units = Some(v.into());
        self
    }
}

impl ToListMappable for EmrManagedScalingPolicyComputeLimitsEl {
    type O = BlockAssignable<EmrManagedScalingPolicyComputeLimitsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrManagedScalingPolicyComputeLimitsEl {
    #[doc = ""]
    pub maximum_capacity_units: PrimField<f64>,
    #[doc = ""]
    pub minimum_capacity_units: PrimField<f64>,
    #[doc = ""]
    pub unit_type: PrimField<String>,
}

impl BuildEmrManagedScalingPolicyComputeLimitsEl {
    pub fn build(self) -> EmrManagedScalingPolicyComputeLimitsEl {
        EmrManagedScalingPolicyComputeLimitsEl {
            maximum_capacity_units: self.maximum_capacity_units,
            maximum_core_capacity_units: core::default::Default::default(),
            maximum_ondemand_capacity_units: core::default::Default::default(),
            minimum_capacity_units: self.minimum_capacity_units,
            unit_type: self.unit_type,
        }
    }
}

pub struct EmrManagedScalingPolicyComputeLimitsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrManagedScalingPolicyComputeLimitsElRef {
    fn new(shared: StackShared, base: String) -> EmrManagedScalingPolicyComputeLimitsElRef {
        EmrManagedScalingPolicyComputeLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrManagedScalingPolicyComputeLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_capacity_units` after provisioning.\n"]
    pub fn maximum_capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_core_capacity_units` after provisioning.\n"]
    pub fn maximum_core_capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_core_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_ondemand_capacity_units` after provisioning.\n"]
    pub fn maximum_ondemand_capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_ondemand_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_capacity_units` after provisioning.\n"]
    pub fn minimum_capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_type` after provisioning.\n"]
    pub fn unit_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrManagedScalingPolicyDynamic {
    compute_limits: Option<DynamicBlock<EmrManagedScalingPolicyComputeLimitsEl>>,
}
