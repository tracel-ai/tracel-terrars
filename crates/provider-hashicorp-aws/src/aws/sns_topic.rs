use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SnsTopicData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_failure_feedback_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_success_feedback_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_success_feedback_sample_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_based_deduplication: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fifo_throughput_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fifo_topic: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firehose_failure_feedback_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firehose_success_feedback_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firehose_success_feedback_sample_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_failure_feedback_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_success_feedback_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_success_feedback_sample_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_master_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_failure_feedback_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_success_feedback_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_success_feedback_sample_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs_failure_feedback_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs_success_feedback_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs_success_feedback_sample_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracing_config: Option<PrimField<String>>,
}

struct SnsTopic_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SnsTopicData>,
}

#[derive(Clone)]
pub struct SnsTopic(Rc<SnsTopic_>);

impl SnsTopic {
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

    #[doc = "Set the field `application_failure_feedback_role_arn`.\n"]
    pub fn set_application_failure_feedback_role_arn(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .application_failure_feedback_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `application_success_feedback_role_arn`.\n"]
    pub fn set_application_success_feedback_role_arn(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .application_success_feedback_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `application_success_feedback_sample_rate`.\n"]
    pub fn set_application_success_feedback_sample_rate(
        self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .application_success_feedback_sample_rate = Some(v.into());
        self
    }

    #[doc = "Set the field `archive_policy`.\n"]
    pub fn set_archive_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().archive_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `content_based_deduplication`.\n"]
    pub fn set_content_based_deduplication(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().content_based_deduplication = Some(v.into());
        self
    }

    #[doc = "Set the field `delivery_policy`.\n"]
    pub fn set_delivery_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delivery_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }

    #[doc = "Set the field `fifo_throughput_scope`.\n"]
    pub fn set_fifo_throughput_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fifo_throughput_scope = Some(v.into());
        self
    }

    #[doc = "Set the field `fifo_topic`.\n"]
    pub fn set_fifo_topic(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().fifo_topic = Some(v.into());
        self
    }

    #[doc = "Set the field `firehose_failure_feedback_role_arn`.\n"]
    pub fn set_firehose_failure_feedback_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().firehose_failure_feedback_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `firehose_success_feedback_role_arn`.\n"]
    pub fn set_firehose_success_feedback_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().firehose_success_feedback_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `firehose_success_feedback_sample_rate`.\n"]
    pub fn set_firehose_success_feedback_sample_rate(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .firehose_success_feedback_sample_rate = Some(v.into());
        self
    }

    #[doc = "Set the field `http_failure_feedback_role_arn`.\n"]
    pub fn set_http_failure_feedback_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().http_failure_feedback_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `http_success_feedback_role_arn`.\n"]
    pub fn set_http_success_feedback_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().http_success_feedback_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `http_success_feedback_sample_rate`.\n"]
    pub fn set_http_success_feedback_sample_rate(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().http_success_feedback_sample_rate = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_master_key_id`.\n"]
    pub fn set_kms_master_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_master_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_failure_feedback_role_arn`.\n"]
    pub fn set_lambda_failure_feedback_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().lambda_failure_feedback_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_success_feedback_role_arn`.\n"]
    pub fn set_lambda_success_feedback_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().lambda_success_feedback_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_success_feedback_sample_rate`.\n"]
    pub fn set_lambda_success_feedback_sample_rate(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().lambda_success_feedback_sample_rate = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `signature_version`.\n"]
    pub fn set_signature_version(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().signature_version = Some(v.into());
        self
    }

    #[doc = "Set the field `sqs_failure_feedback_role_arn`.\n"]
    pub fn set_sqs_failure_feedback_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sqs_failure_feedback_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sqs_success_feedback_role_arn`.\n"]
    pub fn set_sqs_success_feedback_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sqs_success_feedback_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sqs_success_feedback_sample_rate`.\n"]
    pub fn set_sqs_success_feedback_sample_rate(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().sqs_success_feedback_sample_rate = Some(v.into());
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

    #[doc = "Set the field `tracing_config`.\n"]
    pub fn set_tracing_config(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tracing_config = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `application_failure_feedback_role_arn` after provisioning.\n"]
    pub fn application_failure_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.application_failure_feedback_role_arn",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `application_success_feedback_role_arn` after provisioning.\n"]
    pub fn application_success_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.application_success_feedback_role_arn",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `application_success_feedback_sample_rate` after provisioning.\n"]
    pub fn application_success_feedback_sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.application_success_feedback_sample_rate",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `archive_policy` after provisioning.\n"]
    pub fn archive_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.archive_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `beginning_archive_time` after provisioning.\n"]
    pub fn beginning_archive_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.beginning_archive_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `content_based_deduplication` after provisioning.\n"]
    pub fn content_based_deduplication(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_based_deduplication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delivery_policy` after provisioning.\n"]
    pub fn delivery_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fifo_throughput_scope` after provisioning.\n"]
    pub fn fifo_throughput_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fifo_throughput_scope", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fifo_topic` after provisioning.\n"]
    pub fn fifo_topic(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fifo_topic", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firehose_failure_feedback_role_arn` after provisioning.\n"]
    pub fn firehose_failure_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firehose_failure_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firehose_success_feedback_role_arn` after provisioning.\n"]
    pub fn firehose_success_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firehose_success_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firehose_success_feedback_sample_rate` after provisioning.\n"]
    pub fn firehose_success_feedback_sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.firehose_success_feedback_sample_rate",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `http_failure_feedback_role_arn` after provisioning.\n"]
    pub fn http_failure_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_failure_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `http_success_feedback_role_arn` after provisioning.\n"]
    pub fn http_success_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_success_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `http_success_feedback_sample_rate` after provisioning.\n"]
    pub fn http_success_feedback_sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_success_feedback_sample_rate", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_master_key_id` after provisioning.\n"]
    pub fn kms_master_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_master_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_failure_feedback_role_arn` after provisioning.\n"]
    pub fn lambda_failure_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_failure_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_success_feedback_role_arn` after provisioning.\n"]
    pub fn lambda_success_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_success_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_success_feedback_sample_rate` after provisioning.\n"]
    pub fn lambda_success_feedback_sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_success_feedback_sample_rate", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `signature_version` after provisioning.\n"]
    pub fn signature_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signature_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sqs_failure_feedback_role_arn` after provisioning.\n"]
    pub fn sqs_failure_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sqs_failure_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sqs_success_feedback_role_arn` after provisioning.\n"]
    pub fn sqs_success_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sqs_success_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sqs_success_feedback_sample_rate` after provisioning.\n"]
    pub fn sqs_success_feedback_sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sqs_success_feedback_sample_rate", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tracing_config", self.extract_ref()),
        )
    }
}

impl Referable for SnsTopic {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SnsTopic {}

impl ToListMappable for SnsTopic {
    type O = ListRef<SnsTopicRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SnsTopic_ {
    fn extract_resource_type(&self) -> String {
        "aws_sns_topic".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSnsTopic {
    pub tf_id: String,
}

impl BuildSnsTopic {
    pub fn build(self, stack: &mut Stack) -> SnsTopic {
        let out = SnsTopic(Rc::new(SnsTopic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SnsTopicData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                application_failure_feedback_role_arn: core::default::Default::default(),
                application_success_feedback_role_arn: core::default::Default::default(),
                application_success_feedback_sample_rate: core::default::Default::default(),
                archive_policy: core::default::Default::default(),
                content_based_deduplication: core::default::Default::default(),
                delivery_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                fifo_throughput_scope: core::default::Default::default(),
                fifo_topic: core::default::Default::default(),
                firehose_failure_feedback_role_arn: core::default::Default::default(),
                firehose_success_feedback_role_arn: core::default::Default::default(),
                firehose_success_feedback_sample_rate: core::default::Default::default(),
                http_failure_feedback_role_arn: core::default::Default::default(),
                http_success_feedback_role_arn: core::default::Default::default(),
                http_success_feedback_sample_rate: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_master_key_id: core::default::Default::default(),
                lambda_failure_feedback_role_arn: core::default::Default::default(),
                lambda_success_feedback_role_arn: core::default::Default::default(),
                lambda_success_feedback_sample_rate: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                policy: core::default::Default::default(),
                region: core::default::Default::default(),
                signature_version: core::default::Default::default(),
                sqs_failure_feedback_role_arn: core::default::Default::default(),
                sqs_success_feedback_role_arn: core::default::Default::default(),
                sqs_success_feedback_sample_rate: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                tracing_config: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SnsTopicRef {
    shared: StackShared,
    base: String,
}

impl Ref for SnsTopicRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SnsTopicRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_failure_feedback_role_arn` after provisioning.\n"]
    pub fn application_failure_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.application_failure_feedback_role_arn",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `application_success_feedback_role_arn` after provisioning.\n"]
    pub fn application_success_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.application_success_feedback_role_arn",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `application_success_feedback_sample_rate` after provisioning.\n"]
    pub fn application_success_feedback_sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.application_success_feedback_sample_rate",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `archive_policy` after provisioning.\n"]
    pub fn archive_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.archive_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `beginning_archive_time` after provisioning.\n"]
    pub fn beginning_archive_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.beginning_archive_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `content_based_deduplication` after provisioning.\n"]
    pub fn content_based_deduplication(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_based_deduplication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delivery_policy` after provisioning.\n"]
    pub fn delivery_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fifo_throughput_scope` after provisioning.\n"]
    pub fn fifo_throughput_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fifo_throughput_scope", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fifo_topic` after provisioning.\n"]
    pub fn fifo_topic(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fifo_topic", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firehose_failure_feedback_role_arn` after provisioning.\n"]
    pub fn firehose_failure_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firehose_failure_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firehose_success_feedback_role_arn` after provisioning.\n"]
    pub fn firehose_success_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firehose_success_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firehose_success_feedback_sample_rate` after provisioning.\n"]
    pub fn firehose_success_feedback_sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.firehose_success_feedback_sample_rate",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `http_failure_feedback_role_arn` after provisioning.\n"]
    pub fn http_failure_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_failure_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `http_success_feedback_role_arn` after provisioning.\n"]
    pub fn http_success_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_success_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `http_success_feedback_sample_rate` after provisioning.\n"]
    pub fn http_success_feedback_sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_success_feedback_sample_rate", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_master_key_id` after provisioning.\n"]
    pub fn kms_master_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_master_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_failure_feedback_role_arn` after provisioning.\n"]
    pub fn lambda_failure_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_failure_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_success_feedback_role_arn` after provisioning.\n"]
    pub fn lambda_success_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_success_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_success_feedback_sample_rate` after provisioning.\n"]
    pub fn lambda_success_feedback_sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_success_feedback_sample_rate", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `signature_version` after provisioning.\n"]
    pub fn signature_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signature_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sqs_failure_feedback_role_arn` after provisioning.\n"]
    pub fn sqs_failure_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sqs_failure_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sqs_success_feedback_role_arn` after provisioning.\n"]
    pub fn sqs_success_feedback_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sqs_success_feedback_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sqs_success_feedback_sample_rate` after provisioning.\n"]
    pub fn sqs_success_feedback_sample_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sqs_success_feedback_sample_rate", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tracing_config", self.extract_ref()),
        )
    }
}
