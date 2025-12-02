use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SnsSmsPreferencesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_sender_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_sms_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_status_iam_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_status_success_sampling_rate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monthly_spend_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_report_s3_bucket: Option<PrimField<String>>,
}
struct SnsSmsPreferences_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SnsSmsPreferencesData>,
}
#[derive(Clone)]
pub struct SnsSmsPreferences(Rc<SnsSmsPreferences_>);
impl SnsSmsPreferences {
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
    #[doc = "Set the field `default_sender_id`.\n"]
    pub fn set_default_sender_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_sender_id = Some(v.into());
        self
    }
    #[doc = "Set the field `default_sms_type`.\n"]
    pub fn set_default_sms_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_sms_type = Some(v.into());
        self
    }
    #[doc = "Set the field `delivery_status_iam_role_arn`.\n"]
    pub fn set_delivery_status_iam_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delivery_status_iam_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `delivery_status_success_sampling_rate`.\n"]
    pub fn set_delivery_status_success_sampling_rate(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .delivery_status_success_sampling_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `monthly_spend_limit`.\n"]
    pub fn set_monthly_spend_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().monthly_spend_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `usage_report_s3_bucket`.\n"]
    pub fn set_usage_report_s3_bucket(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().usage_report_s3_bucket = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `default_sender_id` after provisioning.\n"]
    pub fn default_sender_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_sender_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_sms_type` after provisioning.\n"]
    pub fn default_sms_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_sms_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delivery_status_iam_role_arn` after provisioning.\n"]
    pub fn delivery_status_iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_status_iam_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delivery_status_success_sampling_rate` after provisioning.\n"]
    pub fn delivery_status_success_sampling_rate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.delivery_status_success_sampling_rate",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `monthly_spend_limit` after provisioning.\n"]
    pub fn monthly_spend_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_spend_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `usage_report_s3_bucket` after provisioning.\n"]
    pub fn usage_report_s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.usage_report_s3_bucket", self.extract_ref()),
        )
    }
}
impl Referable for SnsSmsPreferences {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SnsSmsPreferences {}
impl ToListMappable for SnsSmsPreferences {
    type O = ListRef<SnsSmsPreferencesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SnsSmsPreferences_ {
    fn extract_resource_type(&self) -> String {
        "aws_sns_sms_preferences".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSnsSmsPreferences {
    pub tf_id: String,
}
impl BuildSnsSmsPreferences {
    pub fn build(self, stack: &mut Stack) -> SnsSmsPreferences {
        let out = SnsSmsPreferences(Rc::new(SnsSmsPreferences_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SnsSmsPreferencesData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_sender_id: core::default::Default::default(),
                default_sms_type: core::default::Default::default(),
                delivery_status_iam_role_arn: core::default::Default::default(),
                delivery_status_success_sampling_rate: core::default::Default::default(),
                id: core::default::Default::default(),
                monthly_spend_limit: core::default::Default::default(),
                region: core::default::Default::default(),
                usage_report_s3_bucket: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SnsSmsPreferencesRef {
    shared: StackShared,
    base: String,
}
impl Ref for SnsSmsPreferencesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SnsSmsPreferencesRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_sender_id` after provisioning.\n"]
    pub fn default_sender_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_sender_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_sms_type` after provisioning.\n"]
    pub fn default_sms_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_sms_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delivery_status_iam_role_arn` after provisioning.\n"]
    pub fn delivery_status_iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_status_iam_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delivery_status_success_sampling_rate` after provisioning.\n"]
    pub fn delivery_status_success_sampling_rate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.delivery_status_success_sampling_rate",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `monthly_spend_limit` after provisioning.\n"]
    pub fn monthly_spend_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_spend_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `usage_report_s3_bucket` after provisioning.\n"]
    pub fn usage_report_s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.usage_report_s3_bucket", self.extract_ref()),
        )
    }
}
