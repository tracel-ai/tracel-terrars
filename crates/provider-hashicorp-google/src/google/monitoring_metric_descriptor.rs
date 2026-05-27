use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MonitoringMetricDescriptorData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_stage: Option<PrimField<String>>,
    metric_kind: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    value_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<MonitoringMetricDescriptorLabelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Vec<MonitoringMetricDescriptorMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MonitoringMetricDescriptorTimeoutsEl>,
    dynamic: MonitoringMetricDescriptorDynamic,
}
struct MonitoringMetricDescriptor_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MonitoringMetricDescriptorData>,
}
#[derive(Clone)]
pub struct MonitoringMetricDescriptor(Rc<MonitoringMetricDescriptor_>);
impl MonitoringMetricDescriptor {
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
    #[doc = "Set the field `description`.\nA detailed description of the metric, which can be used in documentation."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nA concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example \"Request count\"."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `launch_stage`.\nThe launch stage of the metric definition. Possible values: [\"LAUNCH_STAGE_UNSPECIFIED\", \"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn set_launch_stage(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().launch_stage = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `unit`.\nThe units in which the metric value is reported. It is only applicable if the\nvalueType is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of\nthe stored metric values.\n\nDifferent systems may scale the values to be more easily displayed (so a value of\n0.02KBy might be displayed as 20By, and a value of 3523KBy might be displayed as\n3.5MBy). However, if the unit is KBy, then the value of the metric is always in\nthousands of bytes, no matter how it may be displayed.\n\nIf you want a custom metric to record the exact number of CPU-seconds used by a job,\nyou can create an INT64 CUMULATIVE metric whose unit is s{CPU} (or equivalently\n1s{CPU} or just s). If the job uses 12,005 CPU-seconds, then the value is written as\n12005.\n\nAlternatively, if you want a custom metric to record data in a more granular way, you\ncan create a DOUBLE CUMULATIVE metric whose unit is ks{CPU}, and then write the value\n12.005 (which is 12005/1000), or use Kis{CPU} and write 11.723 (which is 12005/1024).\nThe supported units are a subset of The Unified Code for Units of Measure standard.\nMore info can be found in the API documentation\n(https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors)."]
    pub fn set_unit(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().unit = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        self,
        v: impl Into<BlockAssignable<MonitoringMetricDescriptorLabelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.labels = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        self,
        v: impl Into<BlockAssignable<MonitoringMetricDescriptorMetadataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MonitoringMetricDescriptorTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA detailed description of the metric, which can be used in documentation."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example \"Request count\"."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage of the metric definition. Possible values: [\"LAUNCH_STAGE_UNSPECIFIED\", \"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metric_kind` after provisioning.\nWhether the metric records instantaneous values, changes to a value, etc. Some combinations of metricKind and valueType might not be supported. Possible values: [\"METRIC_KIND_UNSPECIFIED\", \"GAUGE\", \"DELTA\", \"CUMULATIVE\"]"]
    pub fn metric_kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitored_resource_types` after provisioning.\nIf present, then a time series, which is identified partially by a metric type and a MonitoredResourceDescriptor, that is associated with this metric type can only be associated with one of the monitored resource types listed here. This field allows time series to be associated with the intersection of this metric type and the monitored resource types in this list."]
    pub fn monitored_resource_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.monitored_resource_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the metric descriptor."]
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe metric type, including its DNS name prefix. The type is not URL-encoded. All service defined metrics must be prefixed with the service name, in the format of {service name}/{relative metric name}, such as cloudsql.googleapis.com/database/cpu/utilization. The relative metric name must have only upper and lower-case letters, digits, '/' and underscores '_' are allowed. Additionally, the maximum number of characters allowed for the relative_metric_name is 100. All user-defined metric types have the DNS name custom.googleapis.com, external.googleapis.com, or logging.googleapis.com/user/."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\nThe units in which the metric value is reported. It is only applicable if the\nvalueType is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of\nthe stored metric values.\n\nDifferent systems may scale the values to be more easily displayed (so a value of\n0.02KBy might be displayed as 20By, and a value of 3523KBy might be displayed as\n3.5MBy). However, if the unit is KBy, then the value of the metric is always in\nthousands of bytes, no matter how it may be displayed.\n\nIf you want a custom metric to record the exact number of CPU-seconds used by a job,\nyou can create an INT64 CUMULATIVE metric whose unit is s{CPU} (or equivalently\n1s{CPU} or just s). If the job uses 12,005 CPU-seconds, then the value is written as\n12005.\n\nAlternatively, if you want a custom metric to record data in a more granular way, you\ncan create a DOUBLE CUMULATIVE metric whose unit is ks{CPU}, and then write the value\n12.005 (which is 12005/1000), or use Kis{CPU} and write 11.723 (which is 12005/1024).\nThe supported units are a subset of The Unified Code for Units of Measure standard.\nMore info can be found in the API documentation\n(https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors)."]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `value_type` after provisioning.\nWhether the measurement is an integer, a floating-point number, etc. Some combinations of metricKind and valueType might not be supported. Possible values: [\"BOOL\", \"INT64\", \"DOUBLE\", \"STRING\", \"DISTRIBUTION\"]"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.value_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> ListRef<MonitoringMetricDescriptorMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringMetricDescriptorTimeoutsElRef {
        MonitoringMetricDescriptorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for MonitoringMetricDescriptor {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MonitoringMetricDescriptor {}
impl ToListMappable for MonitoringMetricDescriptor {
    type O = ListRef<MonitoringMetricDescriptorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MonitoringMetricDescriptor_ {
    fn extract_resource_type(&self) -> String {
        "google_monitoring_metric_descriptor".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMonitoringMetricDescriptor {
    pub tf_id: String,
    #[doc = "Whether the metric records instantaneous values, changes to a value, etc. Some combinations of metricKind and valueType might not be supported. Possible values: [\"METRIC_KIND_UNSPECIFIED\", \"GAUGE\", \"DELTA\", \"CUMULATIVE\"]"]
    pub metric_kind: PrimField<String>,
    #[doc = "The metric type, including its DNS name prefix. The type is not URL-encoded. All service defined metrics must be prefixed with the service name, in the format of {service name}/{relative metric name}, such as cloudsql.googleapis.com/database/cpu/utilization. The relative metric name must have only upper and lower-case letters, digits, '/' and underscores '_' are allowed. Additionally, the maximum number of characters allowed for the relative_metric_name is 100. All user-defined metric types have the DNS name custom.googleapis.com, external.googleapis.com, or logging.googleapis.com/user/."]
    pub type_: PrimField<String>,
    #[doc = "Whether the measurement is an integer, a floating-point number, etc. Some combinations of metricKind and valueType might not be supported. Possible values: [\"BOOL\", \"INT64\", \"DOUBLE\", \"STRING\", \"DISTRIBUTION\"]"]
    pub value_type: PrimField<String>,
}
impl BuildMonitoringMetricDescriptor {
    pub fn build(self, stack: &mut Stack) -> MonitoringMetricDescriptor {
        let out = MonitoringMetricDescriptor(Rc::new(MonitoringMetricDescriptor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MonitoringMetricDescriptorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                launch_stage: core::default::Default::default(),
                metric_kind: self.metric_kind,
                project: core::default::Default::default(),
                type_: self.type_,
                unit: core::default::Default::default(),
                value_type: self.value_type,
                labels: core::default::Default::default(),
                metadata: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MonitoringMetricDescriptorRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringMetricDescriptorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MonitoringMetricDescriptorRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA detailed description of the metric, which can be used in documentation."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example \"Request count\"."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `launch_stage` after provisioning.\nThe launch stage of the metric definition. Possible values: [\"LAUNCH_STAGE_UNSPECIFIED\", \"UNIMPLEMENTED\", \"PRELAUNCH\", \"EARLY_ACCESS\", \"ALPHA\", \"BETA\", \"GA\", \"DEPRECATED\"]"]
    pub fn launch_stage(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_stage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metric_kind` after provisioning.\nWhether the metric records instantaneous values, changes to a value, etc. Some combinations of metricKind and valueType might not be supported. Possible values: [\"METRIC_KIND_UNSPECIFIED\", \"GAUGE\", \"DELTA\", \"CUMULATIVE\"]"]
    pub fn metric_kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitored_resource_types` after provisioning.\nIf present, then a time series, which is identified partially by a metric type and a MonitoredResourceDescriptor, that is associated with this metric type can only be associated with one of the monitored resource types listed here. This field allows time series to be associated with the intersection of this metric type and the monitored resource types in this list."]
    pub fn monitored_resource_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.monitored_resource_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the metric descriptor."]
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe metric type, including its DNS name prefix. The type is not URL-encoded. All service defined metrics must be prefixed with the service name, in the format of {service name}/{relative metric name}, such as cloudsql.googleapis.com/database/cpu/utilization. The relative metric name must have only upper and lower-case letters, digits, '/' and underscores '_' are allowed. Additionally, the maximum number of characters allowed for the relative_metric_name is 100. All user-defined metric types have the DNS name custom.googleapis.com, external.googleapis.com, or logging.googleapis.com/user/."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unit` after provisioning.\nThe units in which the metric value is reported. It is only applicable if the\nvalueType is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of\nthe stored metric values.\n\nDifferent systems may scale the values to be more easily displayed (so a value of\n0.02KBy might be displayed as 20By, and a value of 3523KBy might be displayed as\n3.5MBy). However, if the unit is KBy, then the value of the metric is always in\nthousands of bytes, no matter how it may be displayed.\n\nIf you want a custom metric to record the exact number of CPU-seconds used by a job,\nyou can create an INT64 CUMULATIVE metric whose unit is s{CPU} (or equivalently\n1s{CPU} or just s). If the job uses 12,005 CPU-seconds, then the value is written as\n12005.\n\nAlternatively, if you want a custom metric to record data in a more granular way, you\ncan create a DOUBLE CUMULATIVE metric whose unit is ks{CPU}, and then write the value\n12.005 (which is 12005/1000), or use Kis{CPU} and write 11.723 (which is 12005/1024).\nThe supported units are a subset of The Unified Code for Units of Measure standard.\nMore info can be found in the API documentation\n(https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors)."]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `value_type` after provisioning.\nWhether the measurement is an integer, a floating-point number, etc. Some combinations of metricKind and valueType might not be supported. Possible values: [\"BOOL\", \"INT64\", \"DOUBLE\", \"STRING\", \"DISTRIBUTION\"]"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.value_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> ListRef<MonitoringMetricDescriptorMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringMetricDescriptorTimeoutsElRef {
        MonitoringMetricDescriptorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringMetricDescriptorLabelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_type: Option<PrimField<String>>,
}
impl MonitoringMetricDescriptorLabelsEl {
    #[doc = "Set the field `description`.\nA human-readable description for the label."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `value_type`.\nThe type of data that can be assigned to the label. Default value: \"STRING\" Possible values: [\"STRING\", \"BOOL\", \"INT64\"]"]
    pub fn set_value_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value_type = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringMetricDescriptorLabelsEl {
    type O = BlockAssignable<MonitoringMetricDescriptorLabelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringMetricDescriptorLabelsEl {
    #[doc = "The key for this label. The key must not exceed 100 characters. The first character of the key must be an upper- or lower-case letter, the remaining characters must be letters, digits or underscores, and the key must match the regular expression [a-zA-Z][a-zA-Z0-9_]*"]
    pub key: PrimField<String>,
}
impl BuildMonitoringMetricDescriptorLabelsEl {
    pub fn build(self) -> MonitoringMetricDescriptorLabelsEl {
        MonitoringMetricDescriptorLabelsEl {
            description: core::default::Default::default(),
            key: self.key,
            value_type: core::default::Default::default(),
        }
    }
}
pub struct MonitoringMetricDescriptorLabelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringMetricDescriptorLabelsElRef {
    fn new(shared: StackShared, base: String) -> MonitoringMetricDescriptorLabelsElRef {
        MonitoringMetricDescriptorLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringMetricDescriptorLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description for the label."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe key for this label. The key must not exceed 100 characters. The first character of the key must be an upper- or lower-case letter, the remaining characters must be letters, digits or underscores, and the key must match the regular expression [a-zA-Z][a-zA-Z0-9_]*"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value_type` after provisioning.\nThe type of data that can be assigned to the label. Default value: \"STRING\" Possible values: [\"STRING\", \"BOOL\", \"INT64\"]"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
}
#[derive(Serialize)]
pub struct MonitoringMetricDescriptorMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ingest_delay: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_period: Option<PrimField<String>>,
}
impl MonitoringMetricDescriptorMetadataEl {
    #[doc = "Set the field `ingest_delay`.\nThe delay of data points caused by ingestion. Data points older than this age are guaranteed to be ingested and available to be read, excluding data loss due to errors. In '[duration format](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?&_ga=2.264881487.1507873253.1593446723-935052455.1591817775#google.protobuf.Duration)'."]
    pub fn set_ingest_delay(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ingest_delay = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_period`.\nThe sampling period of metric data points. For metrics which are written periodically, consecutive data points are stored at this time interval, excluding data loss due to errors. Metrics with a higher granularity have a smaller sampling period. In '[duration format](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?&_ga=2.264881487.1507873253.1593446723-935052455.1591817775#google.protobuf.Duration)'."]
    pub fn set_sample_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sample_period = Some(v.into());
        self
    }
}
impl ToListMappable for MonitoringMetricDescriptorMetadataEl {
    type O = BlockAssignable<MonitoringMetricDescriptorMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringMetricDescriptorMetadataEl {}
impl BuildMonitoringMetricDescriptorMetadataEl {
    pub fn build(self) -> MonitoringMetricDescriptorMetadataEl {
        MonitoringMetricDescriptorMetadataEl {
            ingest_delay: core::default::Default::default(),
            sample_period: core::default::Default::default(),
        }
    }
}
pub struct MonitoringMetricDescriptorMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringMetricDescriptorMetadataElRef {
    fn new(shared: StackShared, base: String) -> MonitoringMetricDescriptorMetadataElRef {
        MonitoringMetricDescriptorMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringMetricDescriptorMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ingest_delay` after provisioning.\nThe delay of data points caused by ingestion. Data points older than this age are guaranteed to be ingested and available to be read, excluding data loss due to errors. In '[duration format](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?&_ga=2.264881487.1507873253.1593446723-935052455.1591817775#google.protobuf.Duration)'."]
    pub fn ingest_delay(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingest_delay", self.base))
    }
    #[doc = "Get a reference to the value of field `sample_period` after provisioning.\nThe sampling period of metric data points. For metrics which are written periodically, consecutive data points are stored at this time interval, excluding data loss due to errors. Metrics with a higher granularity have a smaller sampling period. In '[duration format](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?&_ga=2.264881487.1507873253.1593446723-935052455.1591817775#google.protobuf.Duration)'."]
    pub fn sample_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sample_period", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MonitoringMetricDescriptorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MonitoringMetricDescriptorTimeoutsEl {
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
impl ToListMappable for MonitoringMetricDescriptorTimeoutsEl {
    type O = BlockAssignable<MonitoringMetricDescriptorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMonitoringMetricDescriptorTimeoutsEl {}
impl BuildMonitoringMetricDescriptorTimeoutsEl {
    pub fn build(self) -> MonitoringMetricDescriptorTimeoutsEl {
        MonitoringMetricDescriptorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MonitoringMetricDescriptorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MonitoringMetricDescriptorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MonitoringMetricDescriptorTimeoutsElRef {
        MonitoringMetricDescriptorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MonitoringMetricDescriptorTimeoutsElRef {
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
struct MonitoringMetricDescriptorDynamic {
    labels: Option<DynamicBlock<MonitoringMetricDescriptorLabelsEl>>,
    metadata: Option<DynamicBlock<MonitoringMetricDescriptorMetadataEl>>,
}
