use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ComputeoptimizerRecommendationPreferencesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_infrastructure_metrics: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inferred_workload_types: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    look_back_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    savings_estimation_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_metrics_preference: Option<Vec<ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_resource: Option<Vec<ComputeoptimizerRecommendationPreferencesPreferredResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<Vec<ComputeoptimizerRecommendationPreferencesScopeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    utilization_preference: Option<Vec<ComputeoptimizerRecommendationPreferencesUtilizationPreferenceEl>>,
    dynamic: ComputeoptimizerRecommendationPreferencesDynamic,
}

struct ComputeoptimizerRecommendationPreferences_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeoptimizerRecommendationPreferencesData>,
}

#[derive(Clone)]
pub struct ComputeoptimizerRecommendationPreferences(Rc<ComputeoptimizerRecommendationPreferences_>);

impl ComputeoptimizerRecommendationPreferences {
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

    #[doc = "Set the field `enhanced_infrastructure_metrics`.\n"]
    pub fn set_enhanced_infrastructure_metrics(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().enhanced_infrastructure_metrics = Some(v.into());
        self
    }

    #[doc = "Set the field `inferred_workload_types`.\n"]
    pub fn set_inferred_workload_types(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().inferred_workload_types = Some(v.into());
        self
    }

    #[doc = "Set the field `look_back_period`.\n"]
    pub fn set_look_back_period(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().look_back_period = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `savings_estimation_mode`.\n"]
    pub fn set_savings_estimation_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().savings_estimation_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `external_metrics_preference`.\n"]
    pub fn set_external_metrics_preference(
        self,
        v: impl Into<BlockAssignable<ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_metrics_preference = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_metrics_preference = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `preferred_resource`.\n"]
    pub fn set_preferred_resource(
        self,
        v: impl Into<BlockAssignable<ComputeoptimizerRecommendationPreferencesPreferredResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().preferred_resource = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.preferred_resource = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(self, v: impl Into<BlockAssignable<ComputeoptimizerRecommendationPreferencesScopeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scope = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scope = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `utilization_preference`.\n"]
    pub fn set_utilization_preference(
        self,
        v: impl Into<BlockAssignable<ComputeoptimizerRecommendationPreferencesUtilizationPreferenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().utilization_preference = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.utilization_preference = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `enhanced_infrastructure_metrics` after provisioning.\n"]
    pub fn enhanced_infrastructure_metrics(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enhanced_infrastructure_metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inferred_workload_types` after provisioning.\n"]
    pub fn inferred_workload_types(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.inferred_workload_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `look_back_period` after provisioning.\n"]
    pub fn look_back_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.look_back_period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savings_estimation_mode` after provisioning.\n"]
    pub fn savings_estimation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.savings_estimation_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_metrics_preference` after provisioning.\n"]
    pub fn external_metrics_preference(
        &self,
    ) -> ListRef<ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_metrics_preference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_resource` after provisioning.\n"]
    pub fn preferred_resource(&self) -> ListRef<ComputeoptimizerRecommendationPreferencesPreferredResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.preferred_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> ListRef<ComputeoptimizerRecommendationPreferencesScopeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `utilization_preference` after provisioning.\n"]
    pub fn utilization_preference(&self) -> ListRef<ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.utilization_preference", self.extract_ref()))
    }
}

impl Referable for ComputeoptimizerRecommendationPreferences {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeoptimizerRecommendationPreferences { }

impl ToListMappable for ComputeoptimizerRecommendationPreferences {
    type O = ListRef<ComputeoptimizerRecommendationPreferencesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeoptimizerRecommendationPreferences_ {
    fn extract_resource_type(&self) -> String {
        "aws_computeoptimizer_recommendation_preferences".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeoptimizerRecommendationPreferences {
    pub tf_id: String,
    #[doc = ""]
    pub resource_type: PrimField<String>,
}

impl BuildComputeoptimizerRecommendationPreferences {
    pub fn build(self, stack: &mut Stack) -> ComputeoptimizerRecommendationPreferences {
        let out = ComputeoptimizerRecommendationPreferences(Rc::new(ComputeoptimizerRecommendationPreferences_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeoptimizerRecommendationPreferencesData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enhanced_infrastructure_metrics: core::default::Default::default(),
                inferred_workload_types: core::default::Default::default(),
                look_back_period: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_type: self.resource_type,
                savings_estimation_mode: core::default::Default::default(),
                external_metrics_preference: core::default::Default::default(),
                preferred_resource: core::default::Default::default(),
                scope: core::default::Default::default(),
                utilization_preference: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeoptimizerRecommendationPreferencesRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeoptimizerRecommendationPreferencesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ComputeoptimizerRecommendationPreferencesRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enhanced_infrastructure_metrics` after provisioning.\n"]
    pub fn enhanced_infrastructure_metrics(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enhanced_infrastructure_metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inferred_workload_types` after provisioning.\n"]
    pub fn inferred_workload_types(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.inferred_workload_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `look_back_period` after provisioning.\n"]
    pub fn look_back_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.look_back_period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savings_estimation_mode` after provisioning.\n"]
    pub fn savings_estimation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.savings_estimation_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_metrics_preference` after provisioning.\n"]
    pub fn external_metrics_preference(
        &self,
    ) -> ListRef<ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_metrics_preference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_resource` after provisioning.\n"]
    pub fn preferred_resource(&self) -> ListRef<ComputeoptimizerRecommendationPreferencesPreferredResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.preferred_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> ListRef<ComputeoptimizerRecommendationPreferencesScopeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `utilization_preference` after provisioning.\n"]
    pub fn utilization_preference(&self) -> ListRef<ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.utilization_preference", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceEl {
    source: PrimField<String>,
}

impl ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceEl { }

impl ToListMappable for ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceEl {
    type O = BlockAssignable<ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceEl {
    #[doc = ""]
    pub source: PrimField<String>,
}

impl BuildComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceEl {
    pub fn build(self) -> ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceEl {
        ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceEl { source: self.source }
    }
}

pub struct ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceElRef {
        ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeoptimizerRecommendationPreferencesPreferredResourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_list: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_list: Option<SetField<PrimField<String>>>,
    name: PrimField<String>,
}

impl ComputeoptimizerRecommendationPreferencesPreferredResourceEl {
    #[doc = "Set the field `exclude_list`.\n"]
    pub fn set_exclude_list(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exclude_list = Some(v.into());
        self
    }

    #[doc = "Set the field `include_list`.\n"]
    pub fn set_include_list(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.include_list = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeoptimizerRecommendationPreferencesPreferredResourceEl {
    type O = BlockAssignable<ComputeoptimizerRecommendationPreferencesPreferredResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeoptimizerRecommendationPreferencesPreferredResourceEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildComputeoptimizerRecommendationPreferencesPreferredResourceEl {
    pub fn build(self) -> ComputeoptimizerRecommendationPreferencesPreferredResourceEl {
        ComputeoptimizerRecommendationPreferencesPreferredResourceEl {
            exclude_list: core::default::Default::default(),
            include_list: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct ComputeoptimizerRecommendationPreferencesPreferredResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeoptimizerRecommendationPreferencesPreferredResourceElRef {
    fn new(shared: StackShared, base: String) -> ComputeoptimizerRecommendationPreferencesPreferredResourceElRef {
        ComputeoptimizerRecommendationPreferencesPreferredResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeoptimizerRecommendationPreferencesPreferredResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude_list` after provisioning.\n"]
    pub fn exclude_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exclude_list", self.base))
    }

    #[doc = "Get a reference to the value of field `include_list` after provisioning.\n"]
    pub fn include_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.include_list", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeoptimizerRecommendationPreferencesScopeEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl ComputeoptimizerRecommendationPreferencesScopeEl { }

impl ToListMappable for ComputeoptimizerRecommendationPreferencesScopeEl {
    type O = BlockAssignable<ComputeoptimizerRecommendationPreferencesScopeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeoptimizerRecommendationPreferencesScopeEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildComputeoptimizerRecommendationPreferencesScopeEl {
    pub fn build(self) -> ComputeoptimizerRecommendationPreferencesScopeEl {
        ComputeoptimizerRecommendationPreferencesScopeEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct ComputeoptimizerRecommendationPreferencesScopeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeoptimizerRecommendationPreferencesScopeElRef {
    fn new(shared: StackShared, base: String) -> ComputeoptimizerRecommendationPreferencesScopeElRef {
        ComputeoptimizerRecommendationPreferencesScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeoptimizerRecommendationPreferencesScopeElRef {
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
pub struct ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersEl {
    headroom: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<PrimField<String>>,
}

impl ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersEl {
    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.threshold = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersEl {
    type O = BlockAssignable<ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersEl {
    #[doc = ""]
    pub headroom: PrimField<String>,
}

impl BuildComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersEl {
    pub fn build(self) -> ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersEl {
        ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersEl {
            headroom: self.headroom,
            threshold: core::default::Default::default(),
        }
    }
}

pub struct ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersElRef {
        ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `headroom` after provisioning.\n"]
    pub fn headroom(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.headroom", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElDynamic {
    metric_parameters: Option<
        DynamicBlock<ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeoptimizerRecommendationPreferencesUtilizationPreferenceEl {
    metric_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_parameters: Option<Vec<ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersEl>>,
    dynamic: ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElDynamic,
}

impl ComputeoptimizerRecommendationPreferencesUtilizationPreferenceEl {
    #[doc = "Set the field `metric_parameters`.\n"]
    pub fn set_metric_parameters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeoptimizerRecommendationPreferencesUtilizationPreferenceEl {
    type O = BlockAssignable<ComputeoptimizerRecommendationPreferencesUtilizationPreferenceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeoptimizerRecommendationPreferencesUtilizationPreferenceEl {
    #[doc = ""]
    pub metric_name: PrimField<String>,
}

impl BuildComputeoptimizerRecommendationPreferencesUtilizationPreferenceEl {
    pub fn build(self) -> ComputeoptimizerRecommendationPreferencesUtilizationPreferenceEl {
        ComputeoptimizerRecommendationPreferencesUtilizationPreferenceEl {
            metric_name: self.metric_name,
            metric_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElRef {
    fn new(shared: StackShared, base: String) -> ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElRef {
        ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_parameters` after provisioning.\n"]
    pub fn metric_parameters(
        &self,
    ) -> ListRef<ComputeoptimizerRecommendationPreferencesUtilizationPreferenceElMetricParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metric_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeoptimizerRecommendationPreferencesDynamic {
    external_metrics_preference: Option<
        DynamicBlock<ComputeoptimizerRecommendationPreferencesExternalMetricsPreferenceEl>,
    >,
    preferred_resource: Option<DynamicBlock<ComputeoptimizerRecommendationPreferencesPreferredResourceEl>>,
    scope: Option<DynamicBlock<ComputeoptimizerRecommendationPreferencesScopeEl>>,
    utilization_preference: Option<DynamicBlock<ComputeoptimizerRecommendationPreferencesUtilizationPreferenceEl>>,
}
