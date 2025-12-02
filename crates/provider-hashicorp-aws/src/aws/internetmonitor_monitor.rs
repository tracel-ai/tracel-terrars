use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct InternetmonitorMonitorData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    max_city_networks_to_monitor: Option<PrimField<f64>>,
    monitor_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_percentage_to_monitor: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_events_config: Option<Vec<InternetmonitorMonitorHealthEventsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internet_measurements_log_delivery:
        Option<Vec<InternetmonitorMonitorInternetMeasurementsLogDeliveryEl>>,
    dynamic: InternetmonitorMonitorDynamic,
}
struct InternetmonitorMonitor_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<InternetmonitorMonitorData>,
}
#[derive(Clone)]
pub struct InternetmonitorMonitor(Rc<InternetmonitorMonitor_>);
impl InternetmonitorMonitor {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `max_city_networks_to_monitor`.\n"]
    pub fn set_max_city_networks_to_monitor(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_city_networks_to_monitor = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resources = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
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
    #[doc = "Set the field `traffic_percentage_to_monitor`.\n"]
    pub fn set_traffic_percentage_to_monitor(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().traffic_percentage_to_monitor = Some(v.into());
        self
    }
    #[doc = "Set the field `health_events_config`.\n"]
    pub fn set_health_events_config(
        self,
        v: impl Into<BlockAssignable<InternetmonitorMonitorHealthEventsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().health_events_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.health_events_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `internet_measurements_log_delivery`.\n"]
    pub fn set_internet_measurements_log_delivery(
        self,
        v: impl Into<BlockAssignable<InternetmonitorMonitorInternetMeasurementsLogDeliveryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().internet_measurements_log_delivery = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .internet_measurements_log_delivery = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `max_city_networks_to_monitor` after provisioning.\n"]
    pub fn max_city_networks_to_monitor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_city_networks_to_monitor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitor_name` after provisioning.\n"]
    pub fn monitor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitor_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `traffic_percentage_to_monitor` after provisioning.\n"]
    pub fn traffic_percentage_to_monitor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_percentage_to_monitor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_events_config` after provisioning.\n"]
    pub fn health_events_config(&self) -> ListRef<InternetmonitorMonitorHealthEventsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_events_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `internet_measurements_log_delivery` after provisioning.\n"]
    pub fn internet_measurements_log_delivery(
        &self,
    ) -> ListRef<InternetmonitorMonitorInternetMeasurementsLogDeliveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.internet_measurements_log_delivery", self.extract_ref()),
        )
    }
}
impl Referable for InternetmonitorMonitor {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for InternetmonitorMonitor {}
impl ToListMappable for InternetmonitorMonitor {
    type O = ListRef<InternetmonitorMonitorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for InternetmonitorMonitor_ {
    fn extract_resource_type(&self) -> String {
        "aws_internetmonitor_monitor".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildInternetmonitorMonitor {
    pub tf_id: String,
    #[doc = ""]
    pub monitor_name: PrimField<String>,
}
impl BuildInternetmonitorMonitor {
    pub fn build(self, stack: &mut Stack) -> InternetmonitorMonitor {
        let out = InternetmonitorMonitor(Rc::new(InternetmonitorMonitor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(InternetmonitorMonitorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                max_city_networks_to_monitor: core::default::Default::default(),
                monitor_name: self.monitor_name,
                region: core::default::Default::default(),
                resources: core::default::Default::default(),
                status: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                traffic_percentage_to_monitor: core::default::Default::default(),
                health_events_config: core::default::Default::default(),
                internet_measurements_log_delivery: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct InternetmonitorMonitorRef {
    shared: StackShared,
    base: String,
}
impl Ref for InternetmonitorMonitorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl InternetmonitorMonitorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `max_city_networks_to_monitor` after provisioning.\n"]
    pub fn max_city_networks_to_monitor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_city_networks_to_monitor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitor_name` after provisioning.\n"]
    pub fn monitor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitor_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `traffic_percentage_to_monitor` after provisioning.\n"]
    pub fn traffic_percentage_to_monitor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_percentage_to_monitor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_events_config` after provisioning.\n"]
    pub fn health_events_config(&self) -> ListRef<InternetmonitorMonitorHealthEventsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_events_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `internet_measurements_log_delivery` after provisioning.\n"]
    pub fn internet_measurements_log_delivery(
        &self,
    ) -> ListRef<InternetmonitorMonitorInternetMeasurementsLogDeliveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.internet_measurements_log_delivery", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct InternetmonitorMonitorHealthEventsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_score_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_score_threshold: Option<PrimField<f64>>,
}
impl InternetmonitorMonitorHealthEventsConfigEl {
    #[doc = "Set the field `availability_score_threshold`.\n"]
    pub fn set_availability_score_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.availability_score_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `performance_score_threshold`.\n"]
    pub fn set_performance_score_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.performance_score_threshold = Some(v.into());
        self
    }
}
impl ToListMappable for InternetmonitorMonitorHealthEventsConfigEl {
    type O = BlockAssignable<InternetmonitorMonitorHealthEventsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildInternetmonitorMonitorHealthEventsConfigEl {}
impl BuildInternetmonitorMonitorHealthEventsConfigEl {
    pub fn build(self) -> InternetmonitorMonitorHealthEventsConfigEl {
        InternetmonitorMonitorHealthEventsConfigEl {
            availability_score_threshold: core::default::Default::default(),
            performance_score_threshold: core::default::Default::default(),
        }
    }
}
pub struct InternetmonitorMonitorHealthEventsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for InternetmonitorMonitorHealthEventsConfigElRef {
    fn new(shared: StackShared, base: String) -> InternetmonitorMonitorHealthEventsConfigElRef {
        InternetmonitorMonitorHealthEventsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl InternetmonitorMonitorHealthEventsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `availability_score_threshold` after provisioning.\n"]
    pub fn availability_score_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_score_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `performance_score_threshold` after provisioning.\n"]
    pub fn performance_score_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_score_threshold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_delivery_status: Option<PrimField<String>>,
}
impl InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `log_delivery_status`.\n"]
    pub fn set_log_delivery_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_delivery_status = Some(v.into());
        self
    }
}
impl ToListMappable for InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigEl {
    type O = BlockAssignable<InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildInternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
}
impl BuildInternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigEl {
    pub fn build(self) -> InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigEl {
        InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigEl {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            log_delivery_status: core::default::Default::default(),
        }
    }
}
pub struct InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigElRef {
        InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_delivery_status` after provisioning.\n"]
    pub fn log_delivery_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_delivery_status", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct InternetmonitorMonitorInternetMeasurementsLogDeliveryElDynamic {
    s3_config:
        Option<DynamicBlock<InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigEl>>,
}
#[derive(Serialize)]
pub struct InternetmonitorMonitorInternetMeasurementsLogDeliveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_config: Option<Vec<InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigEl>>,
    dynamic: InternetmonitorMonitorInternetMeasurementsLogDeliveryElDynamic,
}
impl InternetmonitorMonitorInternetMeasurementsLogDeliveryEl {
    #[doc = "Set the field `s3_config`.\n"]
    pub fn set_s3_config(
        mut self,
        v: impl Into<BlockAssignable<InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for InternetmonitorMonitorInternetMeasurementsLogDeliveryEl {
    type O = BlockAssignable<InternetmonitorMonitorInternetMeasurementsLogDeliveryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildInternetmonitorMonitorInternetMeasurementsLogDeliveryEl {}
impl BuildInternetmonitorMonitorInternetMeasurementsLogDeliveryEl {
    pub fn build(self) -> InternetmonitorMonitorInternetMeasurementsLogDeliveryEl {
        InternetmonitorMonitorInternetMeasurementsLogDeliveryEl {
            s3_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct InternetmonitorMonitorInternetMeasurementsLogDeliveryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for InternetmonitorMonitorInternetMeasurementsLogDeliveryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> InternetmonitorMonitorInternetMeasurementsLogDeliveryElRef {
        InternetmonitorMonitorInternetMeasurementsLogDeliveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl InternetmonitorMonitorInternetMeasurementsLogDeliveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_config` after provisioning.\n"]
    pub fn s3_config(
        &self,
    ) -> ListRef<InternetmonitorMonitorInternetMeasurementsLogDeliveryElS3ConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct InternetmonitorMonitorDynamic {
    health_events_config: Option<DynamicBlock<InternetmonitorMonitorHealthEventsConfigEl>>,
    internet_measurements_log_delivery:
        Option<DynamicBlock<InternetmonitorMonitorInternetMeasurementsLogDeliveryEl>>,
}
