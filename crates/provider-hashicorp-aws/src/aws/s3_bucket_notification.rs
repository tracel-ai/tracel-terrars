use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct S3BucketNotificationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eventbridge: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function: Option<Vec<S3BucketNotificationLambdaFunctionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue: Option<Vec<S3BucketNotificationQueueEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<Vec<S3BucketNotificationTopicEl>>,
    dynamic: S3BucketNotificationDynamic,
}

struct S3BucketNotification_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketNotificationData>,
}

#[derive(Clone)]
pub struct S3BucketNotification(Rc<S3BucketNotification_>);

impl S3BucketNotification {
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

    #[doc = "Set the field `eventbridge`.\n"]
    pub fn set_eventbridge(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().eventbridge = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_function`.\n"]
    pub fn set_lambda_function(
        self,
        v: impl Into<BlockAssignable<S3BucketNotificationLambdaFunctionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lambda_function = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lambda_function = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `queue`.\n"]
    pub fn set_queue(self, v: impl Into<BlockAssignable<S3BucketNotificationQueueEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().queue = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.queue = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(self, v: impl Into<BlockAssignable<S3BucketNotificationTopicEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().topic = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.topic = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `eventbridge` after provisioning.\n"]
    pub fn eventbridge(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.eventbridge", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_function` after provisioning.\n"]
    pub fn lambda_function(&self) -> ListRef<S3BucketNotificationLambdaFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_function", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `queue` after provisioning.\n"]
    pub fn queue(&self) -> ListRef<S3BucketNotificationQueueElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.queue", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> ListRef<S3BucketNotificationTopicElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topic", self.extract_ref()),
        )
    }
}

impl Referable for S3BucketNotification {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for S3BucketNotification {}

impl ToListMappable for S3BucketNotification {
    type O = ListRef<S3BucketNotificationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3BucketNotification_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket_notification".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketNotification {
    pub tf_id: String,
    #[doc = ""]
    pub bucket: PrimField<String>,
}

impl BuildS3BucketNotification {
    pub fn build(self, stack: &mut Stack) -> S3BucketNotification {
        let out = S3BucketNotification(Rc::new(S3BucketNotification_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketNotificationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                eventbridge: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                lambda_function: core::default::Default::default(),
                queue: core::default::Default::default(),
                topic: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketNotificationRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketNotificationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl S3BucketNotificationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `eventbridge` after provisioning.\n"]
    pub fn eventbridge(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.eventbridge", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_function` after provisioning.\n"]
    pub fn lambda_function(&self) -> ListRef<S3BucketNotificationLambdaFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_function", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `queue` after provisioning.\n"]
    pub fn queue(&self) -> ListRef<S3BucketNotificationQueueElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.queue", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> ListRef<S3BucketNotificationTopicElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topic", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct S3BucketNotificationLambdaFunctionEl {
    events: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_arn: Option<PrimField<String>>,
}

impl S3BucketNotificationLambdaFunctionEl {
    #[doc = "Set the field `filter_prefix`.\n"]
    pub fn set_filter_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `filter_suffix`.\n"]
    pub fn set_filter_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_suffix = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_function_arn`.\n"]
    pub fn set_lambda_function_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda_function_arn = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketNotificationLambdaFunctionEl {
    type O = BlockAssignable<S3BucketNotificationLambdaFunctionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketNotificationLambdaFunctionEl {
    #[doc = ""]
    pub events: SetField<PrimField<String>>,
}

impl BuildS3BucketNotificationLambdaFunctionEl {
    pub fn build(self) -> S3BucketNotificationLambdaFunctionEl {
        S3BucketNotificationLambdaFunctionEl {
            events: self.events,
            filter_prefix: core::default::Default::default(),
            filter_suffix: core::default::Default::default(),
            id: core::default::Default::default(),
            lambda_function_arn: core::default::Default::default(),
        }
    }
}

pub struct S3BucketNotificationLambdaFunctionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketNotificationLambdaFunctionElRef {
    fn new(shared: StackShared, base: String) -> S3BucketNotificationLambdaFunctionElRef {
        S3BucketNotificationLambdaFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketNotificationLambdaFunctionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.events", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_prefix` after provisioning.\n"]
    pub fn filter_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_prefix", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `filter_suffix` after provisioning.\n"]
    pub fn filter_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_suffix", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_function_arn` after provisioning.\n"]
    pub fn lambda_function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_function_arn", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct S3BucketNotificationQueueEl {
    events: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    queue_arn: PrimField<String>,
}

impl S3BucketNotificationQueueEl {
    #[doc = "Set the field `filter_prefix`.\n"]
    pub fn set_filter_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `filter_suffix`.\n"]
    pub fn set_filter_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_suffix = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketNotificationQueueEl {
    type O = BlockAssignable<S3BucketNotificationQueueEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketNotificationQueueEl {
    #[doc = ""]
    pub events: SetField<PrimField<String>>,
    #[doc = ""]
    pub queue_arn: PrimField<String>,
}

impl BuildS3BucketNotificationQueueEl {
    pub fn build(self) -> S3BucketNotificationQueueEl {
        S3BucketNotificationQueueEl {
            events: self.events,
            filter_prefix: core::default::Default::default(),
            filter_suffix: core::default::Default::default(),
            id: core::default::Default::default(),
            queue_arn: self.queue_arn,
        }
    }
}

pub struct S3BucketNotificationQueueElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketNotificationQueueElRef {
    fn new(shared: StackShared, base: String) -> S3BucketNotificationQueueElRef {
        S3BucketNotificationQueueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketNotificationQueueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.events", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_prefix` after provisioning.\n"]
    pub fn filter_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_prefix", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `filter_suffix` after provisioning.\n"]
    pub fn filter_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_suffix", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_arn` after provisioning.\n"]
    pub fn queue_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketNotificationTopicEl {
    events: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    topic_arn: PrimField<String>,
}

impl S3BucketNotificationTopicEl {
    #[doc = "Set the field `filter_prefix`.\n"]
    pub fn set_filter_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `filter_suffix`.\n"]
    pub fn set_filter_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_suffix = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketNotificationTopicEl {
    type O = BlockAssignable<S3BucketNotificationTopicEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketNotificationTopicEl {
    #[doc = ""]
    pub events: SetField<PrimField<String>>,
    #[doc = ""]
    pub topic_arn: PrimField<String>,
}

impl BuildS3BucketNotificationTopicEl {
    pub fn build(self) -> S3BucketNotificationTopicEl {
        S3BucketNotificationTopicEl {
            events: self.events,
            filter_prefix: core::default::Default::default(),
            filter_suffix: core::default::Default::default(),
            id: core::default::Default::default(),
            topic_arn: self.topic_arn,
        }
    }
}

pub struct S3BucketNotificationTopicElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketNotificationTopicElRef {
    fn new(shared: StackShared, base: String) -> S3BucketNotificationTopicElRef {
        S3BucketNotificationTopicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketNotificationTopicElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.events", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_prefix` after provisioning.\n"]
    pub fn filter_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_prefix", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `filter_suffix` after provisioning.\n"]
    pub fn filter_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_suffix", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketNotificationDynamic {
    lambda_function: Option<DynamicBlock<S3BucketNotificationLambdaFunctionEl>>,
    queue: Option<DynamicBlock<S3BucketNotificationQueueEl>>,
    topic: Option<DynamicBlock<S3BucketNotificationTopicEl>>,
}
