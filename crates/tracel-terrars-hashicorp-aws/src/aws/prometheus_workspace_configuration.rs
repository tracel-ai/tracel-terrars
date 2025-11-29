use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct PrometheusWorkspaceConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_period_in_days: Option<PrimField<f64>>,
    workspace_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limits_per_label_set: Option<Vec<PrometheusWorkspaceConfigurationLimitsPerLabelSetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PrometheusWorkspaceConfigurationTimeoutsEl>,
    dynamic: PrometheusWorkspaceConfigurationDynamic,
}

struct PrometheusWorkspaceConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PrometheusWorkspaceConfigurationData>,
}

#[derive(Clone)]
pub struct PrometheusWorkspaceConfiguration(Rc<PrometheusWorkspaceConfiguration_>);

impl PrometheusWorkspaceConfiguration {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `retention_period_in_days`.\n"]
    pub fn set_retention_period_in_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retention_period_in_days = Some(v.into());
        self
    }

    #[doc = "Set the field `limits_per_label_set`.\n"]
    pub fn set_limits_per_label_set(
        self,
        v: impl Into<BlockAssignable<PrometheusWorkspaceConfigurationLimitsPerLabelSetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().limits_per_label_set = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.limits_per_label_set = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PrometheusWorkspaceConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_period_in_days` after provisioning.\n"]
    pub fn retention_period_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_period_in_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limits_per_label_set` after provisioning.\n"]
    pub fn limits_per_label_set(&self) -> ListRef<PrometheusWorkspaceConfigurationLimitsPerLabelSetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.limits_per_label_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusWorkspaceConfigurationTimeoutsElRef {
        PrometheusWorkspaceConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for PrometheusWorkspaceConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for PrometheusWorkspaceConfiguration { }

impl ToListMappable for PrometheusWorkspaceConfiguration {
    type O = ListRef<PrometheusWorkspaceConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for PrometheusWorkspaceConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_prometheus_workspace_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPrometheusWorkspaceConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub workspace_id: PrimField<String>,
}

impl BuildPrometheusWorkspaceConfiguration {
    pub fn build(self, stack: &mut Stack) -> PrometheusWorkspaceConfiguration {
        let out = PrometheusWorkspaceConfiguration(Rc::new(PrometheusWorkspaceConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PrometheusWorkspaceConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                retention_period_in_days: core::default::Default::default(),
                workspace_id: self.workspace_id,
                limits_per_label_set: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PrometheusWorkspaceConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for PrometheusWorkspaceConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl PrometheusWorkspaceConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_period_in_days` after provisioning.\n"]
    pub fn retention_period_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_period_in_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limits_per_label_set` after provisioning.\n"]
    pub fn limits_per_label_set(&self) -> ListRef<PrometheusWorkspaceConfigurationLimitsPerLabelSetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.limits_per_label_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusWorkspaceConfigurationTimeoutsElRef {
        PrometheusWorkspaceConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsEl {
    max_series: PrimField<f64>,
}

impl PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsEl { }

impl ToListMappable for PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsEl {
    type O = BlockAssignable<PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsEl {
    #[doc = ""]
    pub max_series: PrimField<f64>,
}

impl BuildPrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsEl {
    pub fn build(self) -> PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsEl {
        PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsEl { max_series: self.max_series }
    }
}

pub struct PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsElRef {
    fn new(shared: StackShared, base: String) -> PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsElRef {
        PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_series` after provisioning.\n"]
    pub fn max_series(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_series", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrometheusWorkspaceConfigurationLimitsPerLabelSetElDynamic {
    limits: Option<DynamicBlock<PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsEl>>,
}

#[derive(Serialize)]
pub struct PrometheusWorkspaceConfigurationLimitsPerLabelSetEl {
    label_set: RecField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<Vec<PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsEl>>,
    dynamic: PrometheusWorkspaceConfigurationLimitsPerLabelSetElDynamic,
}

impl PrometheusWorkspaceConfigurationLimitsPerLabelSetEl {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v: impl Into<BlockAssignable<PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.limits = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.limits = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for PrometheusWorkspaceConfigurationLimitsPerLabelSetEl {
    type O = BlockAssignable<PrometheusWorkspaceConfigurationLimitsPerLabelSetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusWorkspaceConfigurationLimitsPerLabelSetEl {
    #[doc = ""]
    pub label_set: RecField<PrimField<String>>,
}

impl BuildPrometheusWorkspaceConfigurationLimitsPerLabelSetEl {
    pub fn build(self) -> PrometheusWorkspaceConfigurationLimitsPerLabelSetEl {
        PrometheusWorkspaceConfigurationLimitsPerLabelSetEl {
            label_set: self.label_set,
            limits: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct PrometheusWorkspaceConfigurationLimitsPerLabelSetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PrometheusWorkspaceConfigurationLimitsPerLabelSetElRef {
    fn new(shared: StackShared, base: String) -> PrometheusWorkspaceConfigurationLimitsPerLabelSetElRef {
        PrometheusWorkspaceConfigurationLimitsPerLabelSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusWorkspaceConfigurationLimitsPerLabelSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label_set` after provisioning.\n"]
    pub fn label_set(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.label_set", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> ListRef<PrometheusWorkspaceConfigurationLimitsPerLabelSetElLimitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
}

#[derive(Serialize)]
pub struct PrometheusWorkspaceConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl PrometheusWorkspaceConfigurationTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for PrometheusWorkspaceConfigurationTimeoutsEl {
    type O = BlockAssignable<PrometheusWorkspaceConfigurationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrometheusWorkspaceConfigurationTimeoutsEl {}

impl BuildPrometheusWorkspaceConfigurationTimeoutsEl {
    pub fn build(self) -> PrometheusWorkspaceConfigurationTimeoutsEl {
        PrometheusWorkspaceConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct PrometheusWorkspaceConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PrometheusWorkspaceConfigurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PrometheusWorkspaceConfigurationTimeoutsElRef {
        PrometheusWorkspaceConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrometheusWorkspaceConfigurationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrometheusWorkspaceConfigurationDynamic {
    limits_per_label_set: Option<DynamicBlock<PrometheusWorkspaceConfigurationLimitsPerLabelSetEl>>,
}
