use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudwatchLogMetricFilterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apply_on_transformed_logs: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    log_group_name: PrimField<String>,
    name: PrimField<String>,
    pattern: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_transformation: Option<Vec<CloudwatchLogMetricFilterMetricTransformationEl>>,
    dynamic: CloudwatchLogMetricFilterDynamic,
}

struct CloudwatchLogMetricFilter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudwatchLogMetricFilterData>,
}

#[derive(Clone)]
pub struct CloudwatchLogMetricFilter(Rc<CloudwatchLogMetricFilter_>);

impl CloudwatchLogMetricFilter {
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

    #[doc = "Set the field `apply_on_transformed_logs`.\n"]
    pub fn set_apply_on_transformed_logs(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().apply_on_transformed_logs = Some(v.into());
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

    #[doc = "Set the field `metric_transformation`.\n"]
    pub fn set_metric_transformation(
        self,
        v: impl Into<BlockAssignable<CloudwatchLogMetricFilterMetricTransformationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metric_transformation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metric_transformation = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `apply_on_transformed_logs` after provisioning.\n"]
    pub fn apply_on_transformed_logs(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.apply_on_transformed_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_transformation` after provisioning.\n"]
    pub fn metric_transformation(&self) -> ListRef<CloudwatchLogMetricFilterMetricTransformationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metric_transformation", self.extract_ref()))
    }
}

impl Referable for CloudwatchLogMetricFilter {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CloudwatchLogMetricFilter { }

impl ToListMappable for CloudwatchLogMetricFilter {
    type O = ListRef<CloudwatchLogMetricFilterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudwatchLogMetricFilter_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudwatch_log_metric_filter".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchLogMetricFilter {
    pub tf_id: String,
    #[doc = ""]
    pub log_group_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub pattern: PrimField<String>,
}

impl BuildCloudwatchLogMetricFilter {
    pub fn build(self, stack: &mut Stack) -> CloudwatchLogMetricFilter {
        let out = CloudwatchLogMetricFilter(Rc::new(CloudwatchLogMetricFilter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudwatchLogMetricFilterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                apply_on_transformed_logs: core::default::Default::default(),
                id: core::default::Default::default(),
                log_group_name: self.log_group_name,
                name: self.name,
                pattern: self.pattern,
                region: core::default::Default::default(),
                metric_transformation: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchLogMetricFilterRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchLogMetricFilterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CloudwatchLogMetricFilterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apply_on_transformed_logs` after provisioning.\n"]
    pub fn apply_on_transformed_logs(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.apply_on_transformed_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_transformation` after provisioning.\n"]
    pub fn metric_transformation(&self) -> ListRef<CloudwatchLogMetricFilterMetricTransformationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metric_transformation", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogMetricFilterMetricTransformationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    namespace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    value: PrimField<String>,
}

impl CloudwatchLogMetricFilterMetricTransformationEl {
    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_value = Some(v.into());
        self
    }

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

impl ToListMappable for CloudwatchLogMetricFilterMetricTransformationEl {
    type O = BlockAssignable<CloudwatchLogMetricFilterMetricTransformationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogMetricFilterMetricTransformationEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub namespace: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildCloudwatchLogMetricFilterMetricTransformationEl {
    pub fn build(self) -> CloudwatchLogMetricFilterMetricTransformationEl {
        CloudwatchLogMetricFilterMetricTransformationEl {
            default_value: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            name: self.name,
            namespace: self.namespace,
            unit: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct CloudwatchLogMetricFilterMetricTransformationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchLogMetricFilterMetricTransformationElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchLogMetricFilterMetricTransformationElRef {
        CloudwatchLogMetricFilterMetricTransformationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogMetricFilterMetricTransformationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchLogMetricFilterDynamic {
    metric_transformation: Option<DynamicBlock<CloudwatchLogMetricFilterMetricTransformationEl>>,
}
