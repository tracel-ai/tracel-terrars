use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LoggingMetricData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    filter: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label_extractors: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_extractor: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_options: Option<Vec<LoggingMetricBucketOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_descriptor: Option<Vec<LoggingMetricMetricDescriptorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LoggingMetricTimeoutsEl>,
    dynamic: LoggingMetricDynamic,
}
struct LoggingMetric_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LoggingMetricData>,
}
#[derive(Clone)]
pub struct LoggingMetric(Rc<LoggingMetric_>);
impl LoggingMetric {
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
    #[doc = "Set the field `bucket_name`.\nThe resource name of the Log Bucket that owns the Log Metric. Only Log Buckets in projects\nare supported. The bucket has to be in the same project as the metric."]
    pub fn set_bucket_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bucket_name = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA description of this metric, which is used in documentation. The maximum length of the\ndescription is 8000 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nIf set to True, then this metric is disabled and it does not generate any points."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `label_extractors`.\nA map from a label key string to an extractor expression which is used to extract data from a log\nentry field and assign as the label value. Each label key specified in the LabelDescriptor must\nhave an associated extractor expression in this map. The syntax of the extractor expression is\nthe same as for the valueExtractor field."]
    pub fn set_label_extractors(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().label_extractors = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `value_extractor`.\nA valueExtractor is required when using a distribution logs-based metric to extract the values to\nrecord from a log entry. Two functions are supported for value extraction - EXTRACT(field) or\nREGEXP_EXTRACT(field, regex). The argument are 1. field - The name of the log entry field from which\nthe value is to be extracted. 2. regex - A regular expression using the Google RE2 syntax\n(https://github.com/google/re2/wiki/Syntax) with a single capture group to extract data from the specified\nlog entry field. The value of the field is converted to a string before applying the regex. It is an\nerror to specify a regex that does not include exactly one capture group."]
    pub fn set_value_extractor(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().value_extractor = Some(v.into());
        self
    }
    #[doc = "Set the field `bucket_options`.\n"]
    pub fn set_bucket_options(
        self,
        v: impl Into<BlockAssignable<LoggingMetricBucketOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bucket_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bucket_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metric_descriptor`.\n"]
    pub fn set_metric_descriptor(
        self,
        v: impl Into<BlockAssignable<LoggingMetricMetricDescriptorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metric_descriptor = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metric_descriptor = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LoggingMetricTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe resource name of the Log Bucket that owns the Log Metric. Only Log Buckets in projects\nare supported. The bucket has to be in the same project as the metric."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of this metric, which is used in documentation. The maximum length of the\ndescription is 8000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf set to True, then this metric is disabled and it does not generate any points."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nAn advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-filters) which\nis used to match log entries."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `label_extractors` after provisioning.\nA map from a label key string to an extractor expression which is used to extract data from a log\nentry field and assign as the label value. Each label key specified in the LabelDescriptor must\nhave an associated extractor expression in this map. The syntax of the extractor expression is\nthe same as for the valueExtractor field."]
    pub fn label_extractors(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.label_extractors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe client-assigned metric identifier. Examples - \"error_count\", \"nginx/requests\".\nMetric identifiers are limited to 100 characters and can include only the following\ncharacters A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash\ncharacter (/) denotes a hierarchy of name pieces, and it cannot be the first character\nof the name."]
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
    #[doc = "Get a reference to the value of field `value_extractor` after provisioning.\nA valueExtractor is required when using a distribution logs-based metric to extract the values to\nrecord from a log entry. Two functions are supported for value extraction - EXTRACT(field) or\nREGEXP_EXTRACT(field, regex). The argument are 1. field - The name of the log entry field from which\nthe value is to be extracted. 2. regex - A regular expression using the Google RE2 syntax\n(https://github.com/google/re2/wiki/Syntax) with a single capture group to extract data from the specified\nlog entry field. The value of the field is converted to a string before applying the regex. It is an\nerror to specify a regex that does not include exactly one capture group."]
    pub fn value_extractor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.value_extractor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bucket_options` after provisioning.\n"]
    pub fn bucket_options(&self) -> ListRef<LoggingMetricBucketOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metric_descriptor` after provisioning.\n"]
    pub fn metric_descriptor(&self) -> ListRef<LoggingMetricMetricDescriptorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_descriptor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LoggingMetricTimeoutsElRef {
        LoggingMetricTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for LoggingMetric {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LoggingMetric {}
impl ToListMappable for LoggingMetric {
    type O = ListRef<LoggingMetricRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LoggingMetric_ {
    fn extract_resource_type(&self) -> String {
        "google_logging_metric".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLoggingMetric {
    pub tf_id: String,
    #[doc = "An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-filters) which\nis used to match log entries."]
    pub filter: PrimField<String>,
    #[doc = "The client-assigned metric identifier. Examples - \"error_count\", \"nginx/requests\".\nMetric identifiers are limited to 100 characters and can include only the following\ncharacters A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash\ncharacter (/) denotes a hierarchy of name pieces, and it cannot be the first character\nof the name."]
    pub name: PrimField<String>,
}
impl BuildLoggingMetric {
    pub fn build(self, stack: &mut Stack) -> LoggingMetric {
        let out = LoggingMetric(Rc::new(LoggingMetric_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LoggingMetricData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket_name: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                filter: self.filter,
                id: core::default::Default::default(),
                label_extractors: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                value_extractor: core::default::Default::default(),
                bucket_options: core::default::Default::default(),
                metric_descriptor: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LoggingMetricRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingMetricRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LoggingMetricRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe resource name of the Log Bucket that owns the Log Metric. Only Log Buckets in projects\nare supported. The bucket has to be in the same project as the metric."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of this metric, which is used in documentation. The maximum length of the\ndescription is 8000 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf set to True, then this metric is disabled and it does not generate any points."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nAn advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-filters) which\nis used to match log entries."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `label_extractors` after provisioning.\nA map from a label key string to an extractor expression which is used to extract data from a log\nentry field and assign as the label value. Each label key specified in the LabelDescriptor must\nhave an associated extractor expression in this map. The syntax of the extractor expression is\nthe same as for the valueExtractor field."]
    pub fn label_extractors(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.label_extractors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe client-assigned metric identifier. Examples - \"error_count\", \"nginx/requests\".\nMetric identifiers are limited to 100 characters and can include only the following\ncharacters A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash\ncharacter (/) denotes a hierarchy of name pieces, and it cannot be the first character\nof the name."]
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
    #[doc = "Get a reference to the value of field `value_extractor` after provisioning.\nA valueExtractor is required when using a distribution logs-based metric to extract the values to\nrecord from a log entry. Two functions are supported for value extraction - EXTRACT(field) or\nREGEXP_EXTRACT(field, regex). The argument are 1. field - The name of the log entry field from which\nthe value is to be extracted. 2. regex - A regular expression using the Google RE2 syntax\n(https://github.com/google/re2/wiki/Syntax) with a single capture group to extract data from the specified\nlog entry field. The value of the field is converted to a string before applying the regex. It is an\nerror to specify a regex that does not include exactly one capture group."]
    pub fn value_extractor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.value_extractor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bucket_options` after provisioning.\n"]
    pub fn bucket_options(&self) -> ListRef<LoggingMetricBucketOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metric_descriptor` after provisioning.\n"]
    pub fn metric_descriptor(&self) -> ListRef<LoggingMetricMetricDescriptorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_descriptor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LoggingMetricTimeoutsElRef {
        LoggingMetricTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LoggingMetricBucketOptionsElExplicitBucketsEl {
    bounds: ListField<PrimField<f64>>,
}
impl LoggingMetricBucketOptionsElExplicitBucketsEl {}
impl ToListMappable for LoggingMetricBucketOptionsElExplicitBucketsEl {
    type O = BlockAssignable<LoggingMetricBucketOptionsElExplicitBucketsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingMetricBucketOptionsElExplicitBucketsEl {
    #[doc = "The values must be monotonically increasing."]
    pub bounds: ListField<PrimField<f64>>,
}
impl BuildLoggingMetricBucketOptionsElExplicitBucketsEl {
    pub fn build(self) -> LoggingMetricBucketOptionsElExplicitBucketsEl {
        LoggingMetricBucketOptionsElExplicitBucketsEl {
            bounds: self.bounds,
        }
    }
}
pub struct LoggingMetricBucketOptionsElExplicitBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingMetricBucketOptionsElExplicitBucketsElRef {
    fn new(shared: StackShared, base: String) -> LoggingMetricBucketOptionsElExplicitBucketsElRef {
        LoggingMetricBucketOptionsElExplicitBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingMetricBucketOptionsElExplicitBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bounds` after provisioning.\nThe values must be monotonically increasing."]
    pub fn bounds(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.bounds", self.base))
    }
}
#[derive(Serialize)]
pub struct LoggingMetricBucketOptionsElExponentialBucketsEl {
    growth_factor: PrimField<f64>,
    num_finite_buckets: PrimField<f64>,
    scale: PrimField<f64>,
}
impl LoggingMetricBucketOptionsElExponentialBucketsEl {}
impl ToListMappable for LoggingMetricBucketOptionsElExponentialBucketsEl {
    type O = BlockAssignable<LoggingMetricBucketOptionsElExponentialBucketsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingMetricBucketOptionsElExponentialBucketsEl {
    #[doc = "Must be greater than 1."]
    pub growth_factor: PrimField<f64>,
    #[doc = "Must be greater than 0."]
    pub num_finite_buckets: PrimField<f64>,
    #[doc = "Must be greater than 0."]
    pub scale: PrimField<f64>,
}
impl BuildLoggingMetricBucketOptionsElExponentialBucketsEl {
    pub fn build(self) -> LoggingMetricBucketOptionsElExponentialBucketsEl {
        LoggingMetricBucketOptionsElExponentialBucketsEl {
            growth_factor: self.growth_factor,
            num_finite_buckets: self.num_finite_buckets,
            scale: self.scale,
        }
    }
}
pub struct LoggingMetricBucketOptionsElExponentialBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingMetricBucketOptionsElExponentialBucketsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LoggingMetricBucketOptionsElExponentialBucketsElRef {
        LoggingMetricBucketOptionsElExponentialBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingMetricBucketOptionsElExponentialBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `growth_factor` after provisioning.\nMust be greater than 1."]
    pub fn growth_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.growth_factor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_finite_buckets` after provisioning.\nMust be greater than 0."]
    pub fn num_finite_buckets(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_finite_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scale` after provisioning.\nMust be greater than 0."]
    pub fn scale(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale", self.base))
    }
}
#[derive(Serialize)]
pub struct LoggingMetricBucketOptionsElLinearBucketsEl {
    num_finite_buckets: PrimField<f64>,
    offset: PrimField<f64>,
    width: PrimField<f64>,
}
impl LoggingMetricBucketOptionsElLinearBucketsEl {}
impl ToListMappable for LoggingMetricBucketOptionsElLinearBucketsEl {
    type O = BlockAssignable<LoggingMetricBucketOptionsElLinearBucketsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingMetricBucketOptionsElLinearBucketsEl {
    #[doc = "Must be greater than 0."]
    pub num_finite_buckets: PrimField<f64>,
    #[doc = "Lower bound of the first bucket."]
    pub offset: PrimField<f64>,
    #[doc = "Must be greater than 0."]
    pub width: PrimField<f64>,
}
impl BuildLoggingMetricBucketOptionsElLinearBucketsEl {
    pub fn build(self) -> LoggingMetricBucketOptionsElLinearBucketsEl {
        LoggingMetricBucketOptionsElLinearBucketsEl {
            num_finite_buckets: self.num_finite_buckets,
            offset: self.offset,
            width: self.width,
        }
    }
}
pub struct LoggingMetricBucketOptionsElLinearBucketsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingMetricBucketOptionsElLinearBucketsElRef {
    fn new(shared: StackShared, base: String) -> LoggingMetricBucketOptionsElLinearBucketsElRef {
        LoggingMetricBucketOptionsElLinearBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingMetricBucketOptionsElLinearBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `num_finite_buckets` after provisioning.\nMust be greater than 0."]
    pub fn num_finite_buckets(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_finite_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `offset` after provisioning.\nLower bound of the first bucket."]
    pub fn offset(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.offset", self.base))
    }
    #[doc = "Get a reference to the value of field `width` after provisioning.\nMust be greater than 0."]
    pub fn width(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.width", self.base))
    }
}
#[derive(Serialize, Default)]
struct LoggingMetricBucketOptionsElDynamic {
    explicit_buckets: Option<DynamicBlock<LoggingMetricBucketOptionsElExplicitBucketsEl>>,
    exponential_buckets: Option<DynamicBlock<LoggingMetricBucketOptionsElExponentialBucketsEl>>,
    linear_buckets: Option<DynamicBlock<LoggingMetricBucketOptionsElLinearBucketsEl>>,
}
#[derive(Serialize)]
pub struct LoggingMetricBucketOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    explicit_buckets: Option<Vec<LoggingMetricBucketOptionsElExplicitBucketsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exponential_buckets: Option<Vec<LoggingMetricBucketOptionsElExponentialBucketsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linear_buckets: Option<Vec<LoggingMetricBucketOptionsElLinearBucketsEl>>,
    dynamic: LoggingMetricBucketOptionsElDynamic,
}
impl LoggingMetricBucketOptionsEl {
    #[doc = "Set the field `explicit_buckets`.\n"]
    pub fn set_explicit_buckets(
        mut self,
        v: impl Into<BlockAssignable<LoggingMetricBucketOptionsElExplicitBucketsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.explicit_buckets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.explicit_buckets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `exponential_buckets`.\n"]
    pub fn set_exponential_buckets(
        mut self,
        v: impl Into<BlockAssignable<LoggingMetricBucketOptionsElExponentialBucketsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exponential_buckets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exponential_buckets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `linear_buckets`.\n"]
    pub fn set_linear_buckets(
        mut self,
        v: impl Into<BlockAssignable<LoggingMetricBucketOptionsElLinearBucketsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.linear_buckets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.linear_buckets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LoggingMetricBucketOptionsEl {
    type O = BlockAssignable<LoggingMetricBucketOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingMetricBucketOptionsEl {}
impl BuildLoggingMetricBucketOptionsEl {
    pub fn build(self) -> LoggingMetricBucketOptionsEl {
        LoggingMetricBucketOptionsEl {
            explicit_buckets: core::default::Default::default(),
            exponential_buckets: core::default::Default::default(),
            linear_buckets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LoggingMetricBucketOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingMetricBucketOptionsElRef {
    fn new(shared: StackShared, base: String) -> LoggingMetricBucketOptionsElRef {
        LoggingMetricBucketOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingMetricBucketOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `explicit_buckets` after provisioning.\n"]
    pub fn explicit_buckets(&self) -> ListRef<LoggingMetricBucketOptionsElExplicitBucketsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.explicit_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exponential_buckets` after provisioning.\n"]
    pub fn exponential_buckets(
        &self,
    ) -> ListRef<LoggingMetricBucketOptionsElExponentialBucketsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exponential_buckets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `linear_buckets` after provisioning.\n"]
    pub fn linear_buckets(&self) -> ListRef<LoggingMetricBucketOptionsElLinearBucketsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linear_buckets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LoggingMetricMetricDescriptorElLabelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_type: Option<PrimField<String>>,
}
impl LoggingMetricMetricDescriptorElLabelsEl {
    #[doc = "Set the field `description`.\nA human-readable description for the label."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `value_type`.\nThe type of data that can be assigned to the label. Default value: \"STRING\" Possible values: [\"BOOL\", \"INT64\", \"STRING\"]"]
    pub fn set_value_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value_type = Some(v.into());
        self
    }
}
impl ToListMappable for LoggingMetricMetricDescriptorElLabelsEl {
    type O = BlockAssignable<LoggingMetricMetricDescriptorElLabelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingMetricMetricDescriptorElLabelsEl {
    #[doc = "The label key."]
    pub key: PrimField<String>,
}
impl BuildLoggingMetricMetricDescriptorElLabelsEl {
    pub fn build(self) -> LoggingMetricMetricDescriptorElLabelsEl {
        LoggingMetricMetricDescriptorElLabelsEl {
            description: core::default::Default::default(),
            key: self.key,
            value_type: core::default::Default::default(),
        }
    }
}
pub struct LoggingMetricMetricDescriptorElLabelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingMetricMetricDescriptorElLabelsElRef {
    fn new(shared: StackShared, base: String) -> LoggingMetricMetricDescriptorElLabelsElRef {
        LoggingMetricMetricDescriptorElLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingMetricMetricDescriptorElLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description for the label."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe label key."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value_type` after provisioning.\nThe type of data that can be assigned to the label. Default value: \"STRING\" Possible values: [\"BOOL\", \"INT64\", \"STRING\"]"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct LoggingMetricMetricDescriptorElDynamic {
    labels: Option<DynamicBlock<LoggingMetricMetricDescriptorElLabelsEl>>,
}
#[derive(Serialize)]
pub struct LoggingMetricMetricDescriptorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    metric_kind: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    value_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<LoggingMetricMetricDescriptorElLabelsEl>>,
    dynamic: LoggingMetricMetricDescriptorElDynamic,
}
impl LoggingMetricMetricDescriptorEl {
    #[doc = "Set the field `display_name`.\nA concise name for the metric, which can be displayed in user interfaces. Use sentence case\nwithout an ending period, for example \"Request count\". This field is optional but it is\nrecommended to be set for any metrics associated with user-visible concepts, such as Quota."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `unit`.\nThe unit in which the metric value is reported. It is only applicable if the valueType is\n'INT64', 'DOUBLE', or 'DISTRIBUTION'. The supported units are a subset of\n[The Unified Code for Units of Measure](http://unitsofmeasure.org/ucum.html) standard"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl Into<BlockAssignable<LoggingMetricMetricDescriptorElLabelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.labels = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LoggingMetricMetricDescriptorEl {
    type O = BlockAssignable<LoggingMetricMetricDescriptorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingMetricMetricDescriptorEl {
    #[doc = "Whether the metric records instantaneous values, changes to a value, etc.\nSome combinations of metricKind and valueType might not be supported.\nFor counter metrics, set this to DELTA. Possible values: [\"DELTA\", \"GAUGE\", \"CUMULATIVE\"]"]
    pub metric_kind: PrimField<String>,
    #[doc = "Whether the measurement is an integer, a floating-point number, etc.\nSome combinations of metricKind and valueType might not be supported.\nFor counter metrics, set this to INT64. Possible values: [\"BOOL\", \"INT64\", \"DOUBLE\", \"STRING\", \"DISTRIBUTION\", \"MONEY\"]"]
    pub value_type: PrimField<String>,
}
impl BuildLoggingMetricMetricDescriptorEl {
    pub fn build(self) -> LoggingMetricMetricDescriptorEl {
        LoggingMetricMetricDescriptorEl {
            display_name: core::default::Default::default(),
            metric_kind: self.metric_kind,
            unit: core::default::Default::default(),
            value_type: self.value_type,
            labels: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LoggingMetricMetricDescriptorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingMetricMetricDescriptorElRef {
    fn new(shared: StackShared, base: String) -> LoggingMetricMetricDescriptorElRef {
        LoggingMetricMetricDescriptorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingMetricMetricDescriptorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA concise name for the metric, which can be displayed in user interfaces. Use sentence case\nwithout an ending period, for example \"Request count\". This field is optional but it is\nrecommended to be set for any metrics associated with user-visible concepts, such as Quota."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `metric_kind` after provisioning.\nWhether the metric records instantaneous values, changes to a value, etc.\nSome combinations of metricKind and valueType might not be supported.\nFor counter metrics, set this to DELTA. Possible values: [\"DELTA\", \"GAUGE\", \"CUMULATIVE\"]"]
    pub fn metric_kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_kind", self.base))
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\nThe unit in which the metric value is reported. It is only applicable if the valueType is\n'INT64', 'DOUBLE', or 'DISTRIBUTION'. The supported units are a subset of\n[The Unified Code for Units of Measure](http://unitsofmeasure.org/ucum.html) standard"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
    #[doc = "Get a reference to the value of field `value_type` after provisioning.\nWhether the measurement is an integer, a floating-point number, etc.\nSome combinations of metricKind and valueType might not be supported.\nFor counter metrics, set this to INT64. Possible values: [\"BOOL\", \"INT64\", \"DOUBLE\", \"STRING\", \"DISTRIBUTION\", \"MONEY\"]"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
}
#[derive(Serialize)]
pub struct LoggingMetricTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl LoggingMetricTimeoutsEl {
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
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for LoggingMetricTimeoutsEl {
    type O = BlockAssignable<LoggingMetricTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLoggingMetricTimeoutsEl {}
impl BuildLoggingMetricTimeoutsEl {
    pub fn build(self) -> LoggingMetricTimeoutsEl {
        LoggingMetricTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct LoggingMetricTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LoggingMetricTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LoggingMetricTimeoutsElRef {
        LoggingMetricTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LoggingMetricTimeoutsElRef {
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
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct LoggingMetricDynamic {
    bucket_options: Option<DynamicBlock<LoggingMetricBucketOptionsEl>>,
    metric_descriptor: Option<DynamicBlock<LoggingMetricMetricDescriptorEl>>,
}
