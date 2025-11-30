use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SqsQueueData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_based_deduplication: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deduplication_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fifo_queue: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fifo_throughput_limit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_data_key_reuse_period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_master_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_message_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_retention_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receive_wait_time_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redrive_allow_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redrive_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs_managed_sse_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility_timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SqsQueueTimeoutsEl>,
}

struct SqsQueue_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SqsQueueData>,
}

#[derive(Clone)]
pub struct SqsQueue(Rc<SqsQueue_>);

impl SqsQueue {
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

    #[doc = "Set the field `content_based_deduplication`.\n"]
    pub fn set_content_based_deduplication(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().content_based_deduplication = Some(v.into());
        self
    }

    #[doc = "Set the field `deduplication_scope`.\n"]
    pub fn set_deduplication_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deduplication_scope = Some(v.into());
        self
    }

    #[doc = "Set the field `delay_seconds`.\n"]
    pub fn set_delay_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().delay_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `fifo_queue`.\n"]
    pub fn set_fifo_queue(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().fifo_queue = Some(v.into());
        self
    }

    #[doc = "Set the field `fifo_throughput_limit`.\n"]
    pub fn set_fifo_throughput_limit(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fifo_throughput_limit = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_data_key_reuse_period_seconds`.\n"]
    pub fn set_kms_data_key_reuse_period_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().kms_data_key_reuse_period_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_master_key_id`.\n"]
    pub fn set_kms_master_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_master_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `max_message_size`.\n"]
    pub fn set_max_message_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_message_size = Some(v.into());
        self
    }

    #[doc = "Set the field `message_retention_seconds`.\n"]
    pub fn set_message_retention_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().message_retention_seconds = Some(v.into());
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

    #[doc = "Set the field `receive_wait_time_seconds`.\n"]
    pub fn set_receive_wait_time_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().receive_wait_time_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `redrive_allow_policy`.\n"]
    pub fn set_redrive_allow_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().redrive_allow_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `redrive_policy`.\n"]
    pub fn set_redrive_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().redrive_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `sqs_managed_sse_enabled`.\n"]
    pub fn set_sqs_managed_sse_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().sqs_managed_sse_enabled = Some(v.into());
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

    #[doc = "Set the field `visibility_timeout_seconds`.\n"]
    pub fn set_visibility_timeout_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().visibility_timeout_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SqsQueueTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_based_deduplication` after provisioning.\n"]
    pub fn content_based_deduplication(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_based_deduplication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deduplication_scope` after provisioning.\n"]
    pub fn deduplication_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deduplication_scope", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delay_seconds` after provisioning.\n"]
    pub fn delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delay_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fifo_queue` after provisioning.\n"]
    pub fn fifo_queue(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fifo_queue", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fifo_throughput_limit` after provisioning.\n"]
    pub fn fifo_throughput_limit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fifo_throughput_limit", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_data_key_reuse_period_seconds` after provisioning.\n"]
    pub fn kms_data_key_reuse_period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_data_key_reuse_period_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kms_master_key_id` after provisioning.\n"]
    pub fn kms_master_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_master_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_message_size` after provisioning.\n"]
    pub fn max_message_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_message_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `message_retention_seconds` after provisioning.\n"]
    pub fn message_retention_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_retention_seconds", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `receive_wait_time_seconds` after provisioning.\n"]
    pub fn receive_wait_time_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.receive_wait_time_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `redrive_allow_policy` after provisioning.\n"]
    pub fn redrive_allow_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redrive_allow_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `redrive_policy` after provisioning.\n"]
    pub fn redrive_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redrive_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sqs_managed_sse_enabled` after provisioning.\n"]
    pub fn sqs_managed_sse_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sqs_managed_sse_enabled", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visibility_timeout_seconds` after provisioning.\n"]
    pub fn visibility_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visibility_timeout_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SqsQueueTimeoutsElRef {
        SqsQueueTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for SqsQueue {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SqsQueue {}

impl ToListMappable for SqsQueue {
    type O = ListRef<SqsQueueRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SqsQueue_ {
    fn extract_resource_type(&self) -> String {
        "aws_sqs_queue".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSqsQueue {
    pub tf_id: String,
}

impl BuildSqsQueue {
    pub fn build(self, stack: &mut Stack) -> SqsQueue {
        let out = SqsQueue(Rc::new(SqsQueue_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SqsQueueData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                content_based_deduplication: core::default::Default::default(),
                deduplication_scope: core::default::Default::default(),
                delay_seconds: core::default::Default::default(),
                fifo_queue: core::default::Default::default(),
                fifo_throughput_limit: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_data_key_reuse_period_seconds: core::default::Default::default(),
                kms_master_key_id: core::default::Default::default(),
                max_message_size: core::default::Default::default(),
                message_retention_seconds: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                policy: core::default::Default::default(),
                receive_wait_time_seconds: core::default::Default::default(),
                redrive_allow_policy: core::default::Default::default(),
                redrive_policy: core::default::Default::default(),
                region: core::default::Default::default(),
                sqs_managed_sse_enabled: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                visibility_timeout_seconds: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SqsQueueRef {
    shared: StackShared,
    base: String,
}

impl Ref for SqsQueueRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SqsQueueRef {
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

    #[doc = "Get a reference to the value of field `content_based_deduplication` after provisioning.\n"]
    pub fn content_based_deduplication(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_based_deduplication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deduplication_scope` after provisioning.\n"]
    pub fn deduplication_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deduplication_scope", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delay_seconds` after provisioning.\n"]
    pub fn delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delay_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fifo_queue` after provisioning.\n"]
    pub fn fifo_queue(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fifo_queue", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fifo_throughput_limit` after provisioning.\n"]
    pub fn fifo_throughput_limit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fifo_throughput_limit", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_data_key_reuse_period_seconds` after provisioning.\n"]
    pub fn kms_data_key_reuse_period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_data_key_reuse_period_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kms_master_key_id` after provisioning.\n"]
    pub fn kms_master_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_master_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_message_size` after provisioning.\n"]
    pub fn max_message_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_message_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `message_retention_seconds` after provisioning.\n"]
    pub fn message_retention_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_retention_seconds", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `receive_wait_time_seconds` after provisioning.\n"]
    pub fn receive_wait_time_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.receive_wait_time_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `redrive_allow_policy` after provisioning.\n"]
    pub fn redrive_allow_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redrive_allow_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `redrive_policy` after provisioning.\n"]
    pub fn redrive_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redrive_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sqs_managed_sse_enabled` after provisioning.\n"]
    pub fn sqs_managed_sse_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sqs_managed_sse_enabled", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visibility_timeout_seconds` after provisioning.\n"]
    pub fn visibility_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visibility_timeout_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SqsQueueTimeoutsElRef {
        SqsQueueTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SqsQueueTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl SqsQueueTimeoutsEl {
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

impl ToListMappable for SqsQueueTimeoutsEl {
    type O = BlockAssignable<SqsQueueTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqsQueueTimeoutsEl {}

impl BuildSqsQueueTimeoutsEl {
    pub fn build(self) -> SqsQueueTimeoutsEl {
        SqsQueueTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct SqsQueueTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SqsQueueTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SqsQueueTimeoutsElRef {
        SqsQueueTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqsQueueTimeoutsElRef {
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
