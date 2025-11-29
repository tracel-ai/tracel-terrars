use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudwatchLogAnomalyDetectorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anomaly_visibility_time: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detector_name: Option<PrimField<String>>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_frequency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_pattern: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    log_group_arn_list: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct CloudwatchLogAnomalyDetector_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudwatchLogAnomalyDetectorData>,
}

#[derive(Clone)]
pub struct CloudwatchLogAnomalyDetector(Rc<CloudwatchLogAnomalyDetector_>);

impl CloudwatchLogAnomalyDetector {
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

    #[doc = "Set the field `anomaly_visibility_time`.\n"]
    pub fn set_anomaly_visibility_time(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().anomaly_visibility_time = Some(v.into());
        self
    }

    #[doc = "Set the field `detector_name`.\n"]
    pub fn set_detector_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().detector_name = Some(v.into());
        self
    }

    #[doc = "Set the field `evaluation_frequency`.\n"]
    pub fn set_evaluation_frequency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().evaluation_frequency = Some(v.into());
        self
    }

    #[doc = "Set the field `filter_pattern`.\n"]
    pub fn set_filter_pattern(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter_pattern = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `anomaly_visibility_time` after provisioning.\n"]
    pub fn anomaly_visibility_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.anomaly_visibility_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detector_name` after provisioning.\n"]
    pub fn detector_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detector_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_frequency` after provisioning.\n"]
    pub fn evaluation_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.evaluation_frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_pattern` after provisioning.\n"]
    pub fn filter_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_pattern", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_group_arn_list` after provisioning.\n"]
    pub fn log_group_arn_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.log_group_arn_list", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl Referable for CloudwatchLogAnomalyDetector {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CloudwatchLogAnomalyDetector { }

impl ToListMappable for CloudwatchLogAnomalyDetector {
    type O = ListRef<CloudwatchLogAnomalyDetectorRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudwatchLogAnomalyDetector_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudwatch_log_anomaly_detector".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchLogAnomalyDetector {
    pub tf_id: String,
    #[doc = ""]
    pub enabled: PrimField<bool>,
    #[doc = ""]
    pub log_group_arn_list: ListField<PrimField<String>>,
}

impl BuildCloudwatchLogAnomalyDetector {
    pub fn build(self, stack: &mut Stack) -> CloudwatchLogAnomalyDetector {
        let out = CloudwatchLogAnomalyDetector(Rc::new(CloudwatchLogAnomalyDetector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudwatchLogAnomalyDetectorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                anomaly_visibility_time: core::default::Default::default(),
                detector_name: core::default::Default::default(),
                enabled: self.enabled,
                evaluation_frequency: core::default::Default::default(),
                filter_pattern: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                log_group_arn_list: self.log_group_arn_list,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchLogAnomalyDetectorRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchLogAnomalyDetectorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CloudwatchLogAnomalyDetectorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `anomaly_visibility_time` after provisioning.\n"]
    pub fn anomaly_visibility_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.anomaly_visibility_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detector_name` after provisioning.\n"]
    pub fn detector_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detector_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_frequency` after provisioning.\n"]
    pub fn evaluation_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.evaluation_frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_pattern` after provisioning.\n"]
    pub fn filter_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_pattern", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_group_arn_list` after provisioning.\n"]
    pub fn log_group_arn_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.log_group_arn_list", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}
