use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudwatchMetricAlarmData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_description: Option<PrimField<String>>,
    alarm_name: PrimField<String>,
    comparison_operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datapoints_to_alarm: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate_low_sample_count_percentiles: Option<PrimField<String>>,
    evaluation_periods: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extended_statistic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insufficient_data_actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ok_actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statistic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold_metric_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    treat_missing_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_query: Option<Vec<CloudwatchMetricAlarmMetricQueryEl>>,
    dynamic: CloudwatchMetricAlarmDynamic,
}
struct CloudwatchMetricAlarm_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudwatchMetricAlarmData>,
}
#[derive(Clone)]
pub struct CloudwatchMetricAlarm(Rc<CloudwatchMetricAlarm_>);
impl CloudwatchMetricAlarm {
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
    #[doc = "Set the field `actions_enabled`.\n"]
    pub fn set_actions_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().actions_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `alarm_actions`.\n"]
    pub fn set_alarm_actions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().alarm_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `alarm_description`.\n"]
    pub fn set_alarm_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().alarm_description = Some(v.into());
        self
    }
    #[doc = "Set the field `datapoints_to_alarm`.\n"]
    pub fn set_datapoints_to_alarm(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().datapoints_to_alarm = Some(v.into());
        self
    }
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().dimensions = Some(v.into());
        self
    }
    #[doc = "Set the field `evaluate_low_sample_count_percentiles`.\n"]
    pub fn set_evaluate_low_sample_count_percentiles(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .evaluate_low_sample_count_percentiles = Some(v.into());
        self
    }
    #[doc = "Set the field `extended_statistic`.\n"]
    pub fn set_extended_statistic(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().extended_statistic = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `insufficient_data_actions`.\n"]
    pub fn set_insufficient_data_actions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().insufficient_data_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_name`.\n"]
    pub fn set_metric_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().metric_name = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `ok_actions`.\n"]
    pub fn set_ok_actions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ok_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `period`.\n"]
    pub fn set_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().period = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `statistic`.\n"]
    pub fn set_statistic(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().statistic = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `threshold_metric_id`.\n"]
    pub fn set_threshold_metric_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().threshold_metric_id = Some(v.into());
        self
    }
    #[doc = "Set the field `treat_missing_data`.\n"]
    pub fn set_treat_missing_data(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().treat_missing_data = Some(v.into());
        self
    }
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().unit = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_query`.\n"]
    pub fn set_metric_query(
        self,
        v: impl Into<BlockAssignable<CloudwatchMetricAlarmMetricQueryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metric_query = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metric_query = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `actions_enabled` after provisioning.\n"]
    pub fn actions_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.actions_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_actions` after provisioning.\n"]
    pub fn alarm_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.alarm_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_description` after provisioning.\n"]
    pub fn alarm_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alarm_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_name` after provisioning.\n"]
    pub fn alarm_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alarm_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `comparison_operator` after provisioning.\n"]
    pub fn comparison_operator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comparison_operator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `datapoints_to_alarm` after provisioning.\n"]
    pub fn datapoints_to_alarm(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.datapoints_to_alarm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.dimensions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `evaluate_low_sample_count_percentiles` after provisioning.\n"]
    pub fn evaluate_low_sample_count_percentiles(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.evaluate_low_sample_count_percentiles",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `evaluation_periods` after provisioning.\n"]
    pub fn evaluation_periods(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_periods", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `extended_statistic` after provisioning.\n"]
    pub fn extended_statistic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.extended_statistic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `insufficient_data_actions` after provisioning.\n"]
    pub fn insufficient_data_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.insufficient_data_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ok_actions` after provisioning.\n"]
    pub fn ok_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ok_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.statistic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threshold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `threshold_metric_id` after provisioning.\n"]
    pub fn threshold_metric_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threshold_metric_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `treat_missing_data` after provisioning.\n"]
    pub fn treat_missing_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.treat_missing_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unit", self.extract_ref()),
        )
    }
}
impl Referable for CloudwatchMetricAlarm {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudwatchMetricAlarm {}
impl ToListMappable for CloudwatchMetricAlarm {
    type O = ListRef<CloudwatchMetricAlarmRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudwatchMetricAlarm_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudwatch_metric_alarm".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudwatchMetricAlarm {
    pub tf_id: String,
    #[doc = ""]
    pub alarm_name: PrimField<String>,
    #[doc = ""]
    pub comparison_operator: PrimField<String>,
    #[doc = ""]
    pub evaluation_periods: PrimField<f64>,
}
impl BuildCloudwatchMetricAlarm {
    pub fn build(self, stack: &mut Stack) -> CloudwatchMetricAlarm {
        let out = CloudwatchMetricAlarm(Rc::new(CloudwatchMetricAlarm_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudwatchMetricAlarmData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                actions_enabled: core::default::Default::default(),
                alarm_actions: core::default::Default::default(),
                alarm_description: core::default::Default::default(),
                alarm_name: self.alarm_name,
                comparison_operator: self.comparison_operator,
                datapoints_to_alarm: core::default::Default::default(),
                dimensions: core::default::Default::default(),
                evaluate_low_sample_count_percentiles: core::default::Default::default(),
                evaluation_periods: self.evaluation_periods,
                extended_statistic: core::default::Default::default(),
                id: core::default::Default::default(),
                insufficient_data_actions: core::default::Default::default(),
                metric_name: core::default::Default::default(),
                namespace: core::default::Default::default(),
                ok_actions: core::default::Default::default(),
                period: core::default::Default::default(),
                region: core::default::Default::default(),
                statistic: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                threshold: core::default::Default::default(),
                threshold_metric_id: core::default::Default::default(),
                treat_missing_data: core::default::Default::default(),
                unit: core::default::Default::default(),
                metric_query: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudwatchMetricAlarmRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchMetricAlarmRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudwatchMetricAlarmRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `actions_enabled` after provisioning.\n"]
    pub fn actions_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.actions_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_actions` after provisioning.\n"]
    pub fn alarm_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.alarm_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_description` after provisioning.\n"]
    pub fn alarm_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alarm_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_name` after provisioning.\n"]
    pub fn alarm_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alarm_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `comparison_operator` after provisioning.\n"]
    pub fn comparison_operator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comparison_operator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `datapoints_to_alarm` after provisioning.\n"]
    pub fn datapoints_to_alarm(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.datapoints_to_alarm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.dimensions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `evaluate_low_sample_count_percentiles` after provisioning.\n"]
    pub fn evaluate_low_sample_count_percentiles(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.evaluate_low_sample_count_percentiles",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `evaluation_periods` after provisioning.\n"]
    pub fn evaluation_periods(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_periods", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `extended_statistic` after provisioning.\n"]
    pub fn extended_statistic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.extended_statistic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `insufficient_data_actions` after provisioning.\n"]
    pub fn insufficient_data_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.insufficient_data_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ok_actions` after provisioning.\n"]
    pub fn ok_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ok_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.statistic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threshold", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `threshold_metric_id` after provisioning.\n"]
    pub fn threshold_metric_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threshold_metric_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `treat_missing_data` after provisioning.\n"]
    pub fn treat_missing_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.treat_missing_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unit", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudwatchMetricAlarmMetricQueryElMetricEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<RecField<PrimField<String>>>,
    metric_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    period: PrimField<f64>,
    stat: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
}
impl CloudwatchMetricAlarmMetricQueryElMetricEl {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.dimensions = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }
}
impl ToListMappable for CloudwatchMetricAlarmMetricQueryElMetricEl {
    type O = BlockAssignable<CloudwatchMetricAlarmMetricQueryElMetricEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchMetricAlarmMetricQueryElMetricEl {
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub period: PrimField<f64>,
    #[doc = ""]
    pub stat: PrimField<String>,
}
impl BuildCloudwatchMetricAlarmMetricQueryElMetricEl {
    pub fn build(self) -> CloudwatchMetricAlarmMetricQueryElMetricEl {
        CloudwatchMetricAlarmMetricQueryElMetricEl {
            dimensions: core::default::Default::default(),
            metric_name: self.metric_name,
            namespace: core::default::Default::default(),
            period: self.period,
            stat: self.stat,
            unit: core::default::Default::default(),
        }
    }
}
pub struct CloudwatchMetricAlarmMetricQueryElMetricElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchMetricAlarmMetricQueryElMetricElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchMetricAlarmMetricQueryElMetricElRef {
        CloudwatchMetricAlarmMetricQueryElMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchMetricAlarmMetricQueryElMetricElRef {
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
    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }
    #[doc = "Get a reference to the value of field `stat` after provisioning.\n"]
    pub fn stat(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stat", self.base))
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudwatchMetricAlarmMetricQueryElDynamic {
    metric: Option<DynamicBlock<CloudwatchMetricAlarmMetricQueryElMetricEl>>,
}
#[derive(Serialize)]
pub struct CloudwatchMetricAlarmMetricQueryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_data: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric: Option<Vec<CloudwatchMetricAlarmMetricQueryElMetricEl>>,
    dynamic: CloudwatchMetricAlarmMetricQueryElDynamic,
}
impl CloudwatchMetricAlarmMetricQueryEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }
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
    #[doc = "Set the field `period`.\n"]
    pub fn set_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period = Some(v.into());
        self
    }
    #[doc = "Set the field `return_data`.\n"]
    pub fn set_return_data(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.return_data = Some(v.into());
        self
    }
    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(
        mut self,
        v: impl Into<BlockAssignable<CloudwatchMetricAlarmMetricQueryElMetricEl>>,
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
}
impl ToListMappable for CloudwatchMetricAlarmMetricQueryEl {
    type O = BlockAssignable<CloudwatchMetricAlarmMetricQueryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchMetricAlarmMetricQueryEl {
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildCloudwatchMetricAlarmMetricQueryEl {
    pub fn build(self) -> CloudwatchMetricAlarmMetricQueryEl {
        CloudwatchMetricAlarmMetricQueryEl {
            account_id: core::default::Default::default(),
            expression: core::default::Default::default(),
            id: self.id,
            label: core::default::Default::default(),
            period: core::default::Default::default(),
            return_data: core::default::Default::default(),
            metric: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudwatchMetricAlarmMetricQueryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchMetricAlarmMetricQueryElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchMetricAlarmMetricQueryElRef {
        CloudwatchMetricAlarmMetricQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchMetricAlarmMetricQueryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
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
    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }
    #[doc = "Get a reference to the value of field `return_data` after provisioning.\n"]
    pub fn return_data(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.return_data", self.base))
    }
    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> ListRef<CloudwatchMetricAlarmMetricQueryElMetricElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metric", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudwatchMetricAlarmDynamic {
    metric_query: Option<DynamicBlock<CloudwatchMetricAlarmMetricQueryEl>>,
}
