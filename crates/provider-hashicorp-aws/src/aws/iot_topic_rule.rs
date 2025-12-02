use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IotTopicRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    sql: PrimField<String>,
    sql_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_alarm: Option<Vec<IotTopicRuleCloudwatchAlarmEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logs: Option<Vec<IotTopicRuleCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_metric: Option<Vec<IotTopicRuleCloudwatchMetricEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamodb: Option<Vec<IotTopicRuleDynamodbEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamodbv2: Option<Vec<IotTopicRuleDynamodbv2El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch: Option<Vec<IotTopicRuleElasticsearchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_action: Option<Vec<IotTopicRuleErrorActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firehose: Option<Vec<IotTopicRuleFirehoseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http: Option<Vec<IotTopicRuleHttpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iot_analytics: Option<Vec<IotTopicRuleIotAnalyticsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iot_events: Option<Vec<IotTopicRuleIotEventsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kafka: Option<Vec<IotTopicRuleKafkaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis: Option<Vec<IotTopicRuleKinesisEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda: Option<Vec<IotTopicRuleLambdaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    republish: Option<Vec<IotTopicRuleRepublishEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<IotTopicRuleS3El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns: Option<Vec<IotTopicRuleSnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs: Option<Vec<IotTopicRuleSqsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step_functions: Option<Vec<IotTopicRuleStepFunctionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestream: Option<Vec<IotTopicRuleTimestreamEl>>,
    dynamic: IotTopicRuleDynamic,
}
struct IotTopicRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IotTopicRuleData>,
}
#[derive(Clone)]
pub struct IotTopicRule(Rc<IotTopicRule_>);
impl IotTopicRule {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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
    #[doc = "Set the field `cloudwatch_alarm`.\n"]
    pub fn set_cloudwatch_alarm(
        self,
        v: impl Into<BlockAssignable<IotTopicRuleCloudwatchAlarmEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_alarm = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_alarm = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        self,
        v: impl Into<BlockAssignable<IotTopicRuleCloudwatchLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_logs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloudwatch_metric`.\n"]
    pub fn set_cloudwatch_metric(
        self,
        v: impl Into<BlockAssignable<IotTopicRuleCloudwatchMetricEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_metric = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_metric = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dynamodb`.\n"]
    pub fn set_dynamodb(self, v: impl Into<BlockAssignable<IotTopicRuleDynamodbEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dynamodb = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dynamodb = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dynamodbv2`.\n"]
    pub fn set_dynamodbv2(self, v: impl Into<BlockAssignable<IotTopicRuleDynamodbv2El>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dynamodbv2 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dynamodbv2 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `elasticsearch`.\n"]
    pub fn set_elasticsearch(
        self,
        v: impl Into<BlockAssignable<IotTopicRuleElasticsearchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elasticsearch = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elasticsearch = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `error_action`.\n"]
    pub fn set_error_action(
        self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().error_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.error_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(self, v: impl Into<BlockAssignable<IotTopicRuleFirehoseEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().firehose = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.firehose = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http`.\n"]
    pub fn set_http(self, v: impl Into<BlockAssignable<IotTopicRuleHttpEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `iot_analytics`.\n"]
    pub fn set_iot_analytics(
        self,
        v: impl Into<BlockAssignable<IotTopicRuleIotAnalyticsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iot_analytics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iot_analytics = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `iot_events`.\n"]
    pub fn set_iot_events(self, v: impl Into<BlockAssignable<IotTopicRuleIotEventsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iot_events = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iot_events = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kafka`.\n"]
    pub fn set_kafka(self, v: impl Into<BlockAssignable<IotTopicRuleKafkaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kafka = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kinesis`.\n"]
    pub fn set_kinesis(self, v: impl Into<BlockAssignable<IotTopicRuleKinesisEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kinesis = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kinesis = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(self, v: impl Into<BlockAssignable<IotTopicRuleLambdaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lambda = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lambda = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `republish`.\n"]
    pub fn set_republish(self, v: impl Into<BlockAssignable<IotTopicRuleRepublishEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().republish = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.republish = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(self, v: impl Into<BlockAssignable<IotTopicRuleS3El>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sns`.\n"]
    pub fn set_sns(self, v: impl Into<BlockAssignable<IotTopicRuleSnsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sns = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sqs`.\n"]
    pub fn set_sqs(self, v: impl Into<BlockAssignable<IotTopicRuleSqsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sqs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sqs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `step_functions`.\n"]
    pub fn set_step_functions(
        self,
        v: impl Into<BlockAssignable<IotTopicRuleStepFunctionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().step_functions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.step_functions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timestream`.\n"]
    pub fn set_timestream(self, v: impl Into<BlockAssignable<IotTopicRuleTimestreamEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().timestream = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.timestream = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sql` after provisioning.\n"]
    pub fn sql(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `sql_version` after provisioning.\n"]
    pub fn sql_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_version", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `error_action` after provisioning.\n"]
    pub fn error_action(&self) -> ListRef<IotTopicRuleErrorActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error_action", self.extract_ref()),
        )
    }
}
impl Referable for IotTopicRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IotTopicRule {}
impl ToListMappable for IotTopicRule {
    type O = ListRef<IotTopicRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IotTopicRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_iot_topic_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIotTopicRule {
    pub tf_id: String,
    #[doc = ""]
    pub enabled: PrimField<bool>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub sql: PrimField<String>,
    #[doc = ""]
    pub sql_version: PrimField<String>,
}
impl BuildIotTopicRule {
    pub fn build(self, stack: &mut Stack) -> IotTopicRule {
        let out = IotTopicRule(Rc::new(IotTopicRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IotTopicRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                enabled: self.enabled,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                sql: self.sql,
                sql_version: self.sql_version,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                cloudwatch_alarm: core::default::Default::default(),
                cloudwatch_logs: core::default::Default::default(),
                cloudwatch_metric: core::default::Default::default(),
                dynamodb: core::default::Default::default(),
                dynamodbv2: core::default::Default::default(),
                elasticsearch: core::default::Default::default(),
                error_action: core::default::Default::default(),
                firehose: core::default::Default::default(),
                http: core::default::Default::default(),
                iot_analytics: core::default::Default::default(),
                iot_events: core::default::Default::default(),
                kafka: core::default::Default::default(),
                kinesis: core::default::Default::default(),
                lambda: core::default::Default::default(),
                republish: core::default::Default::default(),
                s3: core::default::Default::default(),
                sns: core::default::Default::default(),
                sqs: core::default::Default::default(),
                step_functions: core::default::Default::default(),
                timestream: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IotTopicRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IotTopicRuleRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sql` after provisioning.\n"]
    pub fn sql(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `sql_version` after provisioning.\n"]
    pub fn sql_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_version", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `error_action` after provisioning.\n"]
    pub fn error_action(&self) -> ListRef<IotTopicRuleErrorActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error_action", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleCloudwatchAlarmEl {
    alarm_name: PrimField<String>,
    role_arn: PrimField<String>,
    state_reason: PrimField<String>,
    state_value: PrimField<String>,
}
impl IotTopicRuleCloudwatchAlarmEl {}
impl ToListMappable for IotTopicRuleCloudwatchAlarmEl {
    type O = BlockAssignable<IotTopicRuleCloudwatchAlarmEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleCloudwatchAlarmEl {
    #[doc = ""]
    pub alarm_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub state_reason: PrimField<String>,
    #[doc = ""]
    pub state_value: PrimField<String>,
}
impl BuildIotTopicRuleCloudwatchAlarmEl {
    pub fn build(self) -> IotTopicRuleCloudwatchAlarmEl {
        IotTopicRuleCloudwatchAlarmEl {
            alarm_name: self.alarm_name,
            role_arn: self.role_arn,
            state_reason: self.state_reason,
            state_value: self.state_value,
        }
    }
}
pub struct IotTopicRuleCloudwatchAlarmElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleCloudwatchAlarmElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleCloudwatchAlarmElRef {
        IotTopicRuleCloudwatchAlarmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleCloudwatchAlarmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alarm_name` after provisioning.\n"]
    pub fn alarm_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alarm_name", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\n"]
    pub fn state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.base))
    }
    #[doc = "Get a reference to the value of field `state_value` after provisioning.\n"]
    pub fn state_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_value", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleCloudwatchLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_mode: Option<PrimField<bool>>,
    log_group_name: PrimField<String>,
    role_arn: PrimField<String>,
}
impl IotTopicRuleCloudwatchLogsEl {
    #[doc = "Set the field `batch_mode`.\n"]
    pub fn set_batch_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.batch_mode = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleCloudwatchLogsEl {
    type O = BlockAssignable<IotTopicRuleCloudwatchLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleCloudwatchLogsEl {
    #[doc = ""]
    pub log_group_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleCloudwatchLogsEl {
    pub fn build(self) -> IotTopicRuleCloudwatchLogsEl {
        IotTopicRuleCloudwatchLogsEl {
            batch_mode: core::default::Default::default(),
            log_group_name: self.log_group_name,
            role_arn: self.role_arn,
        }
    }
}
pub struct IotTopicRuleCloudwatchLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleCloudwatchLogsElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleCloudwatchLogsElRef {
        IotTopicRuleCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleCloudwatchLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_mode` after provisioning.\n"]
    pub fn batch_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_group_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleCloudwatchMetricEl {
    metric_name: PrimField<String>,
    metric_namespace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_timestamp: Option<PrimField<String>>,
    metric_unit: PrimField<String>,
    metric_value: PrimField<String>,
    role_arn: PrimField<String>,
}
impl IotTopicRuleCloudwatchMetricEl {
    #[doc = "Set the field `metric_timestamp`.\n"]
    pub fn set_metric_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_timestamp = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleCloudwatchMetricEl {
    type O = BlockAssignable<IotTopicRuleCloudwatchMetricEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleCloudwatchMetricEl {
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub metric_namespace: PrimField<String>,
    #[doc = ""]
    pub metric_unit: PrimField<String>,
    #[doc = ""]
    pub metric_value: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleCloudwatchMetricEl {
    pub fn build(self) -> IotTopicRuleCloudwatchMetricEl {
        IotTopicRuleCloudwatchMetricEl {
            metric_name: self.metric_name,
            metric_namespace: self.metric_namespace,
            metric_timestamp: core::default::Default::default(),
            metric_unit: self.metric_unit,
            metric_value: self.metric_value,
            role_arn: self.role_arn,
        }
    }
}
pub struct IotTopicRuleCloudwatchMetricElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleCloudwatchMetricElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleCloudwatchMetricElRef {
        IotTopicRuleCloudwatchMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleCloudwatchMetricElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }
    #[doc = "Get a reference to the value of field `metric_namespace` after provisioning.\n"]
    pub fn metric_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_namespace", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric_timestamp` after provisioning.\n"]
    pub fn metric_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_timestamp", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric_unit` after provisioning.\n"]
    pub fn metric_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_unit", self.base))
    }
    #[doc = "Get a reference to the value of field `metric_value` after provisioning.\n"]
    pub fn metric_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_value", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleDynamodbEl {
    hash_key_field: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hash_key_type: Option<PrimField<String>>,
    hash_key_value: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_key_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_key_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_key_value: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    table_name: PrimField<String>,
}
impl IotTopicRuleDynamodbEl {
    #[doc = "Set the field `hash_key_type`.\n"]
    pub fn set_hash_key_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hash_key_type = Some(v.into());
        self
    }
    #[doc = "Set the field `operation`.\n"]
    pub fn set_operation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operation = Some(v.into());
        self
    }
    #[doc = "Set the field `payload_field`.\n"]
    pub fn set_payload_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.payload_field = Some(v.into());
        self
    }
    #[doc = "Set the field `range_key_field`.\n"]
    pub fn set_range_key_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range_key_field = Some(v.into());
        self
    }
    #[doc = "Set the field `range_key_type`.\n"]
    pub fn set_range_key_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range_key_type = Some(v.into());
        self
    }
    #[doc = "Set the field `range_key_value`.\n"]
    pub fn set_range_key_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range_key_value = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleDynamodbEl {
    type O = BlockAssignable<IotTopicRuleDynamodbEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleDynamodbEl {
    #[doc = ""]
    pub hash_key_field: PrimField<String>,
    #[doc = ""]
    pub hash_key_value: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
}
impl BuildIotTopicRuleDynamodbEl {
    pub fn build(self) -> IotTopicRuleDynamodbEl {
        IotTopicRuleDynamodbEl {
            hash_key_field: self.hash_key_field,
            hash_key_type: core::default::Default::default(),
            hash_key_value: self.hash_key_value,
            operation: core::default::Default::default(),
            payload_field: core::default::Default::default(),
            range_key_field: core::default::Default::default(),
            range_key_type: core::default::Default::default(),
            range_key_value: core::default::Default::default(),
            role_arn: self.role_arn,
            table_name: self.table_name,
        }
    }
}
pub struct IotTopicRuleDynamodbElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleDynamodbElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleDynamodbElRef {
        IotTopicRuleDynamodbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleDynamodbElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hash_key_field` after provisioning.\n"]
    pub fn hash_key_field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hash_key_field", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hash_key_type` after provisioning.\n"]
    pub fn hash_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hash_key_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hash_key_value` after provisioning.\n"]
    pub fn hash_key_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hash_key_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
    #[doc = "Get a reference to the value of field `payload_field` after provisioning.\n"]
    pub fn payload_field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.payload_field", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `range_key_field` after provisioning.\n"]
    pub fn range_key_field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.range_key_field", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `range_key_type` after provisioning.\n"]
    pub fn range_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.range_key_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `range_key_value` after provisioning.\n"]
    pub fn range_key_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.range_key_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleDynamodbv2ElPutItemEl {
    table_name: PrimField<String>,
}
impl IotTopicRuleDynamodbv2ElPutItemEl {}
impl ToListMappable for IotTopicRuleDynamodbv2ElPutItemEl {
    type O = BlockAssignable<IotTopicRuleDynamodbv2ElPutItemEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleDynamodbv2ElPutItemEl {
    #[doc = ""]
    pub table_name: PrimField<String>,
}
impl BuildIotTopicRuleDynamodbv2ElPutItemEl {
    pub fn build(self) -> IotTopicRuleDynamodbv2ElPutItemEl {
        IotTopicRuleDynamodbv2ElPutItemEl {
            table_name: self.table_name,
        }
    }
}
pub struct IotTopicRuleDynamodbv2ElPutItemElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleDynamodbv2ElPutItemElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleDynamodbv2ElPutItemElRef {
        IotTopicRuleDynamodbv2ElPutItemElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleDynamodbv2ElPutItemElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct IotTopicRuleDynamodbv2ElDynamic {
    put_item: Option<DynamicBlock<IotTopicRuleDynamodbv2ElPutItemEl>>,
}
#[derive(Serialize)]
pub struct IotTopicRuleDynamodbv2El {
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    put_item: Option<Vec<IotTopicRuleDynamodbv2ElPutItemEl>>,
    dynamic: IotTopicRuleDynamodbv2ElDynamic,
}
impl IotTopicRuleDynamodbv2El {
    #[doc = "Set the field `put_item`.\n"]
    pub fn set_put_item(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleDynamodbv2ElPutItemEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.put_item = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.put_item = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IotTopicRuleDynamodbv2El {
    type O = BlockAssignable<IotTopicRuleDynamodbv2El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleDynamodbv2El {
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleDynamodbv2El {
    pub fn build(self) -> IotTopicRuleDynamodbv2El {
        IotTopicRuleDynamodbv2El {
            role_arn: self.role_arn,
            put_item: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IotTopicRuleDynamodbv2ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleDynamodbv2ElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleDynamodbv2ElRef {
        IotTopicRuleDynamodbv2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleDynamodbv2ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `put_item` after provisioning.\n"]
    pub fn put_item(&self) -> ListRef<IotTopicRuleDynamodbv2ElPutItemElRef> {
        ListRef::new(self.shared().clone(), format!("{}.put_item", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleElasticsearchEl {
    endpoint: PrimField<String>,
    id: PrimField<String>,
    index: PrimField<String>,
    role_arn: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl IotTopicRuleElasticsearchEl {}
impl ToListMappable for IotTopicRuleElasticsearchEl {
    type O = BlockAssignable<IotTopicRuleElasticsearchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleElasticsearchEl {
    #[doc = ""]
    pub endpoint: PrimField<String>,
    #[doc = ""]
    pub id: PrimField<String>,
    #[doc = ""]
    pub index: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildIotTopicRuleElasticsearchEl {
    pub fn build(self) -> IotTopicRuleElasticsearchEl {
        IotTopicRuleElasticsearchEl {
            endpoint: self.endpoint,
            id: self.id,
            index: self.index,
            role_arn: self.role_arn,
            type_: self.type_,
        }
    }
}
pub struct IotTopicRuleElasticsearchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleElasticsearchElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleElasticsearchElRef {
        IotTopicRuleElasticsearchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleElasticsearchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `index` after provisioning.\n"]
    pub fn index(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElCloudwatchAlarmEl {
    alarm_name: PrimField<String>,
    role_arn: PrimField<String>,
    state_reason: PrimField<String>,
    state_value: PrimField<String>,
}
impl IotTopicRuleErrorActionElCloudwatchAlarmEl {}
impl ToListMappable for IotTopicRuleErrorActionElCloudwatchAlarmEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElCloudwatchAlarmEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElCloudwatchAlarmEl {
    #[doc = ""]
    pub alarm_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub state_reason: PrimField<String>,
    #[doc = ""]
    pub state_value: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElCloudwatchAlarmEl {
    pub fn build(self) -> IotTopicRuleErrorActionElCloudwatchAlarmEl {
        IotTopicRuleErrorActionElCloudwatchAlarmEl {
            alarm_name: self.alarm_name,
            role_arn: self.role_arn,
            state_reason: self.state_reason,
            state_value: self.state_value,
        }
    }
}
pub struct IotTopicRuleErrorActionElCloudwatchAlarmElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElCloudwatchAlarmElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElCloudwatchAlarmElRef {
        IotTopicRuleErrorActionElCloudwatchAlarmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElCloudwatchAlarmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alarm_name` after provisioning.\n"]
    pub fn alarm_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alarm_name", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\n"]
    pub fn state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.base))
    }
    #[doc = "Get a reference to the value of field `state_value` after provisioning.\n"]
    pub fn state_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_value", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElCloudwatchLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_mode: Option<PrimField<bool>>,
    log_group_name: PrimField<String>,
    role_arn: PrimField<String>,
}
impl IotTopicRuleErrorActionElCloudwatchLogsEl {
    #[doc = "Set the field `batch_mode`.\n"]
    pub fn set_batch_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.batch_mode = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElCloudwatchLogsEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElCloudwatchLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElCloudwatchLogsEl {
    #[doc = ""]
    pub log_group_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElCloudwatchLogsEl {
    pub fn build(self) -> IotTopicRuleErrorActionElCloudwatchLogsEl {
        IotTopicRuleErrorActionElCloudwatchLogsEl {
            batch_mode: core::default::Default::default(),
            log_group_name: self.log_group_name,
            role_arn: self.role_arn,
        }
    }
}
pub struct IotTopicRuleErrorActionElCloudwatchLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElCloudwatchLogsElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElCloudwatchLogsElRef {
        IotTopicRuleErrorActionElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElCloudwatchLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_mode` after provisioning.\n"]
    pub fn batch_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_group_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElCloudwatchMetricEl {
    metric_name: PrimField<String>,
    metric_namespace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_timestamp: Option<PrimField<String>>,
    metric_unit: PrimField<String>,
    metric_value: PrimField<String>,
    role_arn: PrimField<String>,
}
impl IotTopicRuleErrorActionElCloudwatchMetricEl {
    #[doc = "Set the field `metric_timestamp`.\n"]
    pub fn set_metric_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_timestamp = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElCloudwatchMetricEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElCloudwatchMetricEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElCloudwatchMetricEl {
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub metric_namespace: PrimField<String>,
    #[doc = ""]
    pub metric_unit: PrimField<String>,
    #[doc = ""]
    pub metric_value: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElCloudwatchMetricEl {
    pub fn build(self) -> IotTopicRuleErrorActionElCloudwatchMetricEl {
        IotTopicRuleErrorActionElCloudwatchMetricEl {
            metric_name: self.metric_name,
            metric_namespace: self.metric_namespace,
            metric_timestamp: core::default::Default::default(),
            metric_unit: self.metric_unit,
            metric_value: self.metric_value,
            role_arn: self.role_arn,
        }
    }
}
pub struct IotTopicRuleErrorActionElCloudwatchMetricElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElCloudwatchMetricElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElCloudwatchMetricElRef {
        IotTopicRuleErrorActionElCloudwatchMetricElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElCloudwatchMetricElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }
    #[doc = "Get a reference to the value of field `metric_namespace` after provisioning.\n"]
    pub fn metric_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_namespace", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric_timestamp` after provisioning.\n"]
    pub fn metric_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_timestamp", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric_unit` after provisioning.\n"]
    pub fn metric_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_unit", self.base))
    }
    #[doc = "Get a reference to the value of field `metric_value` after provisioning.\n"]
    pub fn metric_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_value", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElDynamodbEl {
    hash_key_field: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hash_key_type: Option<PrimField<String>>,
    hash_key_value: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_key_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_key_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_key_value: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    table_name: PrimField<String>,
}
impl IotTopicRuleErrorActionElDynamodbEl {
    #[doc = "Set the field `hash_key_type`.\n"]
    pub fn set_hash_key_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hash_key_type = Some(v.into());
        self
    }
    #[doc = "Set the field `operation`.\n"]
    pub fn set_operation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operation = Some(v.into());
        self
    }
    #[doc = "Set the field `payload_field`.\n"]
    pub fn set_payload_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.payload_field = Some(v.into());
        self
    }
    #[doc = "Set the field `range_key_field`.\n"]
    pub fn set_range_key_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range_key_field = Some(v.into());
        self
    }
    #[doc = "Set the field `range_key_type`.\n"]
    pub fn set_range_key_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range_key_type = Some(v.into());
        self
    }
    #[doc = "Set the field `range_key_value`.\n"]
    pub fn set_range_key_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range_key_value = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElDynamodbEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElDynamodbEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElDynamodbEl {
    #[doc = ""]
    pub hash_key_field: PrimField<String>,
    #[doc = ""]
    pub hash_key_value: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElDynamodbEl {
    pub fn build(self) -> IotTopicRuleErrorActionElDynamodbEl {
        IotTopicRuleErrorActionElDynamodbEl {
            hash_key_field: self.hash_key_field,
            hash_key_type: core::default::Default::default(),
            hash_key_value: self.hash_key_value,
            operation: core::default::Default::default(),
            payload_field: core::default::Default::default(),
            range_key_field: core::default::Default::default(),
            range_key_type: core::default::Default::default(),
            range_key_value: core::default::Default::default(),
            role_arn: self.role_arn,
            table_name: self.table_name,
        }
    }
}
pub struct IotTopicRuleErrorActionElDynamodbElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElDynamodbElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElDynamodbElRef {
        IotTopicRuleErrorActionElDynamodbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElDynamodbElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hash_key_field` after provisioning.\n"]
    pub fn hash_key_field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hash_key_field", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hash_key_type` after provisioning.\n"]
    pub fn hash_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hash_key_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hash_key_value` after provisioning.\n"]
    pub fn hash_key_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hash_key_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
    #[doc = "Get a reference to the value of field `payload_field` after provisioning.\n"]
    pub fn payload_field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.payload_field", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `range_key_field` after provisioning.\n"]
    pub fn range_key_field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.range_key_field", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `range_key_type` after provisioning.\n"]
    pub fn range_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.range_key_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `range_key_value` after provisioning.\n"]
    pub fn range_key_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.range_key_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElDynamodbv2ElPutItemEl {
    table_name: PrimField<String>,
}
impl IotTopicRuleErrorActionElDynamodbv2ElPutItemEl {}
impl ToListMappable for IotTopicRuleErrorActionElDynamodbv2ElPutItemEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElDynamodbv2ElPutItemEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElDynamodbv2ElPutItemEl {
    #[doc = ""]
    pub table_name: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElDynamodbv2ElPutItemEl {
    pub fn build(self) -> IotTopicRuleErrorActionElDynamodbv2ElPutItemEl {
        IotTopicRuleErrorActionElDynamodbv2ElPutItemEl {
            table_name: self.table_name,
        }
    }
}
pub struct IotTopicRuleErrorActionElDynamodbv2ElPutItemElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElDynamodbv2ElPutItemElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElDynamodbv2ElPutItemElRef {
        IotTopicRuleErrorActionElDynamodbv2ElPutItemElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElDynamodbv2ElPutItemElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct IotTopicRuleErrorActionElDynamodbv2ElDynamic {
    put_item: Option<DynamicBlock<IotTopicRuleErrorActionElDynamodbv2ElPutItemEl>>,
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElDynamodbv2El {
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    put_item: Option<Vec<IotTopicRuleErrorActionElDynamodbv2ElPutItemEl>>,
    dynamic: IotTopicRuleErrorActionElDynamodbv2ElDynamic,
}
impl IotTopicRuleErrorActionElDynamodbv2El {
    #[doc = "Set the field `put_item`.\n"]
    pub fn set_put_item(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElDynamodbv2ElPutItemEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.put_item = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.put_item = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElDynamodbv2El {
    type O = BlockAssignable<IotTopicRuleErrorActionElDynamodbv2El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElDynamodbv2El {
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElDynamodbv2El {
    pub fn build(self) -> IotTopicRuleErrorActionElDynamodbv2El {
        IotTopicRuleErrorActionElDynamodbv2El {
            role_arn: self.role_arn,
            put_item: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IotTopicRuleErrorActionElDynamodbv2ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElDynamodbv2ElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElDynamodbv2ElRef {
        IotTopicRuleErrorActionElDynamodbv2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElDynamodbv2ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `put_item` after provisioning.\n"]
    pub fn put_item(&self) -> ListRef<IotTopicRuleErrorActionElDynamodbv2ElPutItemElRef> {
        ListRef::new(self.shared().clone(), format!("{}.put_item", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElElasticsearchEl {
    endpoint: PrimField<String>,
    id: PrimField<String>,
    index: PrimField<String>,
    role_arn: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl IotTopicRuleErrorActionElElasticsearchEl {}
impl ToListMappable for IotTopicRuleErrorActionElElasticsearchEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElElasticsearchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElElasticsearchEl {
    #[doc = ""]
    pub endpoint: PrimField<String>,
    #[doc = ""]
    pub id: PrimField<String>,
    #[doc = ""]
    pub index: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElElasticsearchEl {
    pub fn build(self) -> IotTopicRuleErrorActionElElasticsearchEl {
        IotTopicRuleErrorActionElElasticsearchEl {
            endpoint: self.endpoint,
            id: self.id,
            index: self.index,
            role_arn: self.role_arn,
            type_: self.type_,
        }
    }
}
pub struct IotTopicRuleErrorActionElElasticsearchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElElasticsearchElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElElasticsearchElRef {
        IotTopicRuleErrorActionElElasticsearchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElElasticsearchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `index` after provisioning.\n"]
    pub fn index(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElFirehoseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_mode: Option<PrimField<bool>>,
    delivery_stream_name: PrimField<String>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    separator: Option<PrimField<String>>,
}
impl IotTopicRuleErrorActionElFirehoseEl {
    #[doc = "Set the field `batch_mode`.\n"]
    pub fn set_batch_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.batch_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `separator`.\n"]
    pub fn set_separator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.separator = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElFirehoseEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElFirehoseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElFirehoseEl {
    #[doc = ""]
    pub delivery_stream_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElFirehoseEl {
    pub fn build(self) -> IotTopicRuleErrorActionElFirehoseEl {
        IotTopicRuleErrorActionElFirehoseEl {
            batch_mode: core::default::Default::default(),
            delivery_stream_name: self.delivery_stream_name,
            role_arn: self.role_arn,
            separator: core::default::Default::default(),
        }
    }
}
pub struct IotTopicRuleErrorActionElFirehoseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElFirehoseElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElFirehoseElRef {
        IotTopicRuleErrorActionElFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElFirehoseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_mode` after provisioning.\n"]
    pub fn batch_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `delivery_stream_name` after provisioning.\n"]
    pub fn delivery_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_stream_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `separator` after provisioning.\n"]
    pub fn separator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.separator", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElHttpElHttpHeaderEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl IotTopicRuleErrorActionElHttpElHttpHeaderEl {}
impl ToListMappable for IotTopicRuleErrorActionElHttpElHttpHeaderEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElHttpElHttpHeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElHttpElHttpHeaderEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElHttpElHttpHeaderEl {
    pub fn build(self) -> IotTopicRuleErrorActionElHttpElHttpHeaderEl {
        IotTopicRuleErrorActionElHttpElHttpHeaderEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct IotTopicRuleErrorActionElHttpElHttpHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElHttpElHttpHeaderElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElHttpElHttpHeaderElRef {
        IotTopicRuleErrorActionElHttpElHttpHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElHttpElHttpHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct IotTopicRuleErrorActionElHttpElDynamic {
    http_header: Option<DynamicBlock<IotTopicRuleErrorActionElHttpElHttpHeaderEl>>,
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElHttpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confirmation_url: Option<PrimField<String>>,
    url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_header: Option<Vec<IotTopicRuleErrorActionElHttpElHttpHeaderEl>>,
    dynamic: IotTopicRuleErrorActionElHttpElDynamic,
}
impl IotTopicRuleErrorActionElHttpEl {
    #[doc = "Set the field `confirmation_url`.\n"]
    pub fn set_confirmation_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confirmation_url = Some(v.into());
        self
    }
    #[doc = "Set the field `http_header`.\n"]
    pub fn set_http_header(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElHttpElHttpHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_header = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElHttpEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElHttpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElHttpEl {
    #[doc = ""]
    pub url: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElHttpEl {
    pub fn build(self) -> IotTopicRuleErrorActionElHttpEl {
        IotTopicRuleErrorActionElHttpEl {
            confirmation_url: core::default::Default::default(),
            url: self.url,
            http_header: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IotTopicRuleErrorActionElHttpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElHttpElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElHttpElRef {
        IotTopicRuleErrorActionElHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElHttpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confirmation_url` after provisioning.\n"]
    pub fn confirmation_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confirmation_url", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
    #[doc = "Get a reference to the value of field `http_header` after provisioning.\n"]
    pub fn http_header(&self) -> ListRef<IotTopicRuleErrorActionElHttpElHttpHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_header", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElIotAnalyticsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_mode: Option<PrimField<bool>>,
    channel_name: PrimField<String>,
    role_arn: PrimField<String>,
}
impl IotTopicRuleErrorActionElIotAnalyticsEl {
    #[doc = "Set the field `batch_mode`.\n"]
    pub fn set_batch_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.batch_mode = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElIotAnalyticsEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElIotAnalyticsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElIotAnalyticsEl {
    #[doc = ""]
    pub channel_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElIotAnalyticsEl {
    pub fn build(self) -> IotTopicRuleErrorActionElIotAnalyticsEl {
        IotTopicRuleErrorActionElIotAnalyticsEl {
            batch_mode: core::default::Default::default(),
            channel_name: self.channel_name,
            role_arn: self.role_arn,
        }
    }
}
pub struct IotTopicRuleErrorActionElIotAnalyticsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElIotAnalyticsElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElIotAnalyticsElRef {
        IotTopicRuleErrorActionElIotAnalyticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElIotAnalyticsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_mode` after provisioning.\n"]
    pub fn batch_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `channel_name` after provisioning.\n"]
    pub fn channel_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel_name", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElIotEventsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_mode: Option<PrimField<bool>>,
    input_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<PrimField<String>>,
    role_arn: PrimField<String>,
}
impl IotTopicRuleErrorActionElIotEventsEl {
    #[doc = "Set the field `batch_mode`.\n"]
    pub fn set_batch_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.batch_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `message_id`.\n"]
    pub fn set_message_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_id = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElIotEventsEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElIotEventsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElIotEventsEl {
    #[doc = ""]
    pub input_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElIotEventsEl {
    pub fn build(self) -> IotTopicRuleErrorActionElIotEventsEl {
        IotTopicRuleErrorActionElIotEventsEl {
            batch_mode: core::default::Default::default(),
            input_name: self.input_name,
            message_id: core::default::Default::default(),
            role_arn: self.role_arn,
        }
    }
}
pub struct IotTopicRuleErrorActionElIotEventsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElIotEventsElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElIotEventsElRef {
        IotTopicRuleErrorActionElIotEventsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElIotEventsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_mode` after provisioning.\n"]
    pub fn batch_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `input_name` after provisioning.\n"]
    pub fn input_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_name", self.base))
    }
    #[doc = "Get a reference to the value of field `message_id` after provisioning.\n"]
    pub fn message_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_id", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElKafkaElHeaderEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl IotTopicRuleErrorActionElKafkaElHeaderEl {}
impl ToListMappable for IotTopicRuleErrorActionElKafkaElHeaderEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElKafkaElHeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElKafkaElHeaderEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElKafkaElHeaderEl {
    pub fn build(self) -> IotTopicRuleErrorActionElKafkaElHeaderEl {
        IotTopicRuleErrorActionElKafkaElHeaderEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct IotTopicRuleErrorActionElKafkaElHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElKafkaElHeaderElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElKafkaElHeaderElRef {
        IotTopicRuleErrorActionElKafkaElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElKafkaElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct IotTopicRuleErrorActionElKafkaElDynamic {
    header: Option<DynamicBlock<IotTopicRuleErrorActionElKafkaElHeaderEl>>,
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElKafkaEl {
    client_properties: RecField<PrimField<String>>,
    destination_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition: Option<PrimField<String>>,
    topic: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<Vec<IotTopicRuleErrorActionElKafkaElHeaderEl>>,
    dynamic: IotTopicRuleErrorActionElKafkaElDynamic,
}
impl IotTopicRuleErrorActionElKafkaEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `partition`.\n"]
    pub fn set_partition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.partition = Some(v.into());
        self
    }
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElKafkaElHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElKafkaEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElKafkaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElKafkaEl {
    #[doc = ""]
    pub client_properties: RecField<PrimField<String>>,
    #[doc = ""]
    pub destination_arn: PrimField<String>,
    #[doc = ""]
    pub topic: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElKafkaEl {
    pub fn build(self) -> IotTopicRuleErrorActionElKafkaEl {
        IotTopicRuleErrorActionElKafkaEl {
            client_properties: self.client_properties,
            destination_arn: self.destination_arn,
            key: core::default::Default::default(),
            partition: core::default::Default::default(),
            topic: self.topic,
            header: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IotTopicRuleErrorActionElKafkaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElKafkaElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElKafkaElRef {
        IotTopicRuleErrorActionElKafkaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElKafkaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_properties` after provisioning.\n"]
    pub fn client_properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.client_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `partition` after provisioning.\n"]
    pub fn partition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.partition", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> ListRef<IotTopicRuleErrorActionElKafkaElHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElKinesisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_key: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    stream_name: PrimField<String>,
}
impl IotTopicRuleErrorActionElKinesisEl {
    #[doc = "Set the field `partition_key`.\n"]
    pub fn set_partition_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.partition_key = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElKinesisEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElKinesisEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElKinesisEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub stream_name: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElKinesisEl {
    pub fn build(self) -> IotTopicRuleErrorActionElKinesisEl {
        IotTopicRuleErrorActionElKinesisEl {
            partition_key: core::default::Default::default(),
            role_arn: self.role_arn,
            stream_name: self.stream_name,
        }
    }
}
pub struct IotTopicRuleErrorActionElKinesisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElKinesisElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElKinesisElRef {
        IotTopicRuleErrorActionElKinesisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElKinesisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partition_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `stream_name` after provisioning.\n"]
    pub fn stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_name", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElLambdaEl {
    function_arn: PrimField<String>,
}
impl IotTopicRuleErrorActionElLambdaEl {}
impl ToListMappable for IotTopicRuleErrorActionElLambdaEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElLambdaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElLambdaEl {
    #[doc = ""]
    pub function_arn: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElLambdaEl {
    pub fn build(self) -> IotTopicRuleErrorActionElLambdaEl {
        IotTopicRuleErrorActionElLambdaEl {
            function_arn: self.function_arn,
        }
    }
}
pub struct IotTopicRuleErrorActionElLambdaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElLambdaElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElLambdaElRef {
        IotTopicRuleErrorActionElLambdaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElLambdaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElRepublishEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    qos: Option<PrimField<f64>>,
    role_arn: PrimField<String>,
    topic: PrimField<String>,
}
impl IotTopicRuleErrorActionElRepublishEl {
    #[doc = "Set the field `qos`.\n"]
    pub fn set_qos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.qos = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElRepublishEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElRepublishEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElRepublishEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub topic: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElRepublishEl {
    pub fn build(self) -> IotTopicRuleErrorActionElRepublishEl {
        IotTopicRuleErrorActionElRepublishEl {
            qos: core::default::Default::default(),
            role_arn: self.role_arn,
            topic: self.topic,
        }
    }
}
pub struct IotTopicRuleErrorActionElRepublishElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElRepublishElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElRepublishElRef {
        IotTopicRuleErrorActionElRepublishElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElRepublishElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `qos` after provisioning.\n"]
    pub fn qos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.qos", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElS3El {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canned_acl: Option<PrimField<String>>,
    key: PrimField<String>,
    role_arn: PrimField<String>,
}
impl IotTopicRuleErrorActionElS3El {
    #[doc = "Set the field `canned_acl`.\n"]
    pub fn set_canned_acl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.canned_acl = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElS3El {
    type O = BlockAssignable<IotTopicRuleErrorActionElS3El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElS3El {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElS3El {
    pub fn build(self) -> IotTopicRuleErrorActionElS3El {
        IotTopicRuleErrorActionElS3El {
            bucket_name: self.bucket_name,
            canned_acl: core::default::Default::default(),
            key: self.key,
            role_arn: self.role_arn,
        }
    }
}
pub struct IotTopicRuleErrorActionElS3ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElS3ElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElS3ElRef {
        IotTopicRuleErrorActionElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `canned_acl` after provisioning.\n"]
    pub fn canned_acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.canned_acl", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElSnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    target_arn: PrimField<String>,
}
impl IotTopicRuleErrorActionElSnsEl {
    #[doc = "Set the field `message_format`.\n"]
    pub fn set_message_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElSnsEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElSnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElSnsEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub target_arn: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElSnsEl {
    pub fn build(self) -> IotTopicRuleErrorActionElSnsEl {
        IotTopicRuleErrorActionElSnsEl {
            message_format: core::default::Default::default(),
            role_arn: self.role_arn,
            target_arn: self.target_arn,
        }
    }
}
pub struct IotTopicRuleErrorActionElSnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElSnsElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElSnsElRef {
        IotTopicRuleErrorActionElSnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElSnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_format` after provisioning.\n"]
    pub fn message_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `target_arn` after provisioning.\n"]
    pub fn target_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElSqsEl {
    queue_url: PrimField<String>,
    role_arn: PrimField<String>,
    use_base64: PrimField<bool>,
}
impl IotTopicRuleErrorActionElSqsEl {}
impl ToListMappable for IotTopicRuleErrorActionElSqsEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElSqsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElSqsEl {
    #[doc = ""]
    pub queue_url: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub use_base64: PrimField<bool>,
}
impl BuildIotTopicRuleErrorActionElSqsEl {
    pub fn build(self) -> IotTopicRuleErrorActionElSqsEl {
        IotTopicRuleErrorActionElSqsEl {
            queue_url: self.queue_url,
            role_arn: self.role_arn,
            use_base64: self.use_base64,
        }
    }
}
pub struct IotTopicRuleErrorActionElSqsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElSqsElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElSqsElRef {
        IotTopicRuleErrorActionElSqsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElSqsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `queue_url` after provisioning.\n"]
    pub fn queue_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_url", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `use_base64` after provisioning.\n"]
    pub fn use_base64(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_base64", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElStepFunctionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_name_prefix: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    state_machine_name: PrimField<String>,
}
impl IotTopicRuleErrorActionElStepFunctionsEl {
    #[doc = "Set the field `execution_name_prefix`.\n"]
    pub fn set_execution_name_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_name_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElStepFunctionsEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElStepFunctionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElStepFunctionsEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub state_machine_name: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElStepFunctionsEl {
    pub fn build(self) -> IotTopicRuleErrorActionElStepFunctionsEl {
        IotTopicRuleErrorActionElStepFunctionsEl {
            execution_name_prefix: core::default::Default::default(),
            role_arn: self.role_arn,
            state_machine_name: self.state_machine_name,
        }
    }
}
pub struct IotTopicRuleErrorActionElStepFunctionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElStepFunctionsElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElStepFunctionsElRef {
        IotTopicRuleErrorActionElStepFunctionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElStepFunctionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `execution_name_prefix` after provisioning.\n"]
    pub fn execution_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_name_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `state_machine_name` after provisioning.\n"]
    pub fn state_machine_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_machine_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElTimestreamElDimensionEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl IotTopicRuleErrorActionElTimestreamElDimensionEl {}
impl ToListMappable for IotTopicRuleErrorActionElTimestreamElDimensionEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElTimestreamElDimensionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElTimestreamElDimensionEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElTimestreamElDimensionEl {
    pub fn build(self) -> IotTopicRuleErrorActionElTimestreamElDimensionEl {
        IotTopicRuleErrorActionElTimestreamElDimensionEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct IotTopicRuleErrorActionElTimestreamElDimensionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElTimestreamElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IotTopicRuleErrorActionElTimestreamElDimensionElRef {
        IotTopicRuleErrorActionElTimestreamElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElTimestreamElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElTimestreamElTimestampEl {
    unit: PrimField<String>,
    value: PrimField<String>,
}
impl IotTopicRuleErrorActionElTimestreamElTimestampEl {}
impl ToListMappable for IotTopicRuleErrorActionElTimestreamElTimestampEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElTimestreamElTimestampEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElTimestreamElTimestampEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElTimestreamElTimestampEl {
    pub fn build(self) -> IotTopicRuleErrorActionElTimestreamElTimestampEl {
        IotTopicRuleErrorActionElTimestreamElTimestampEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct IotTopicRuleErrorActionElTimestreamElTimestampElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElTimestreamElTimestampElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IotTopicRuleErrorActionElTimestreamElTimestampElRef {
        IotTopicRuleErrorActionElTimestreamElTimestampElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElTimestreamElTimestampElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
struct IotTopicRuleErrorActionElTimestreamElDynamic {
    dimension: Option<DynamicBlock<IotTopicRuleErrorActionElTimestreamElDimensionEl>>,
    timestamp: Option<DynamicBlock<IotTopicRuleErrorActionElTimestreamElTimestampEl>>,
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionElTimestreamEl {
    database_name: PrimField<String>,
    role_arn: PrimField<String>,
    table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<IotTopicRuleErrorActionElTimestreamElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<Vec<IotTopicRuleErrorActionElTimestreamElTimestampEl>>,
    dynamic: IotTopicRuleErrorActionElTimestreamElDynamic,
}
impl IotTopicRuleErrorActionElTimestreamEl {
    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElTimestreamElDimensionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timestamp`.\n"]
    pub fn set_timestamp(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElTimestreamElTimestampEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timestamp = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timestamp = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionElTimestreamEl {
    type O = BlockAssignable<IotTopicRuleErrorActionElTimestreamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionElTimestreamEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
}
impl BuildIotTopicRuleErrorActionElTimestreamEl {
    pub fn build(self) -> IotTopicRuleErrorActionElTimestreamEl {
        IotTopicRuleErrorActionElTimestreamEl {
            database_name: self.database_name,
            role_arn: self.role_arn,
            table_name: self.table_name,
            dimension: core::default::Default::default(),
            timestamp: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IotTopicRuleErrorActionElTimestreamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElTimestreamElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElTimestreamElRef {
        IotTopicRuleErrorActionElTimestreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElTimestreamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
    #[doc = "Get a reference to the value of field `timestamp` after provisioning.\n"]
    pub fn timestamp(&self) -> ListRef<IotTopicRuleErrorActionElTimestreamElTimestampElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }
}
#[derive(Serialize, Default)]
struct IotTopicRuleErrorActionElDynamic {
    cloudwatch_alarm: Option<DynamicBlock<IotTopicRuleErrorActionElCloudwatchAlarmEl>>,
    cloudwatch_logs: Option<DynamicBlock<IotTopicRuleErrorActionElCloudwatchLogsEl>>,
    cloudwatch_metric: Option<DynamicBlock<IotTopicRuleErrorActionElCloudwatchMetricEl>>,
    dynamodb: Option<DynamicBlock<IotTopicRuleErrorActionElDynamodbEl>>,
    dynamodbv2: Option<DynamicBlock<IotTopicRuleErrorActionElDynamodbv2El>>,
    elasticsearch: Option<DynamicBlock<IotTopicRuleErrorActionElElasticsearchEl>>,
    firehose: Option<DynamicBlock<IotTopicRuleErrorActionElFirehoseEl>>,
    http: Option<DynamicBlock<IotTopicRuleErrorActionElHttpEl>>,
    iot_analytics: Option<DynamicBlock<IotTopicRuleErrorActionElIotAnalyticsEl>>,
    iot_events: Option<DynamicBlock<IotTopicRuleErrorActionElIotEventsEl>>,
    kafka: Option<DynamicBlock<IotTopicRuleErrorActionElKafkaEl>>,
    kinesis: Option<DynamicBlock<IotTopicRuleErrorActionElKinesisEl>>,
    lambda: Option<DynamicBlock<IotTopicRuleErrorActionElLambdaEl>>,
    republish: Option<DynamicBlock<IotTopicRuleErrorActionElRepublishEl>>,
    s3: Option<DynamicBlock<IotTopicRuleErrorActionElS3El>>,
    sns: Option<DynamicBlock<IotTopicRuleErrorActionElSnsEl>>,
    sqs: Option<DynamicBlock<IotTopicRuleErrorActionElSqsEl>>,
    step_functions: Option<DynamicBlock<IotTopicRuleErrorActionElStepFunctionsEl>>,
    timestream: Option<DynamicBlock<IotTopicRuleErrorActionElTimestreamEl>>,
}
#[derive(Serialize)]
pub struct IotTopicRuleErrorActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_alarm: Option<Vec<IotTopicRuleErrorActionElCloudwatchAlarmEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logs: Option<Vec<IotTopicRuleErrorActionElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_metric: Option<Vec<IotTopicRuleErrorActionElCloudwatchMetricEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamodb: Option<Vec<IotTopicRuleErrorActionElDynamodbEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamodbv2: Option<Vec<IotTopicRuleErrorActionElDynamodbv2El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch: Option<Vec<IotTopicRuleErrorActionElElasticsearchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firehose: Option<Vec<IotTopicRuleErrorActionElFirehoseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http: Option<Vec<IotTopicRuleErrorActionElHttpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iot_analytics: Option<Vec<IotTopicRuleErrorActionElIotAnalyticsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iot_events: Option<Vec<IotTopicRuleErrorActionElIotEventsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kafka: Option<Vec<IotTopicRuleErrorActionElKafkaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis: Option<Vec<IotTopicRuleErrorActionElKinesisEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda: Option<Vec<IotTopicRuleErrorActionElLambdaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    republish: Option<Vec<IotTopicRuleErrorActionElRepublishEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<IotTopicRuleErrorActionElS3El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns: Option<Vec<IotTopicRuleErrorActionElSnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs: Option<Vec<IotTopicRuleErrorActionElSqsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step_functions: Option<Vec<IotTopicRuleErrorActionElStepFunctionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestream: Option<Vec<IotTopicRuleErrorActionElTimestreamEl>>,
    dynamic: IotTopicRuleErrorActionElDynamic,
}
impl IotTopicRuleErrorActionEl {
    #[doc = "Set the field `cloudwatch_alarm`.\n"]
    pub fn set_cloudwatch_alarm(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElCloudwatchAlarmEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_alarm = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_alarm = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElCloudwatchLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloudwatch_metric`.\n"]
    pub fn set_cloudwatch_metric(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElCloudwatchMetricEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_metric = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_metric = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dynamodb`.\n"]
    pub fn set_dynamodb(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElDynamodbEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dynamodb = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dynamodb = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dynamodbv2`.\n"]
    pub fn set_dynamodbv2(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElDynamodbv2El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dynamodbv2 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dynamodbv2 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `elasticsearch`.\n"]
    pub fn set_elasticsearch(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElElasticsearchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.elasticsearch = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.elasticsearch = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElFirehoseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.firehose = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http`.\n"]
    pub fn set_http(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElHttpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `iot_analytics`.\n"]
    pub fn set_iot_analytics(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElIotAnalyticsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iot_analytics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iot_analytics = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `iot_events`.\n"]
    pub fn set_iot_events(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElIotEventsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iot_events = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iot_events = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kafka`.\n"]
    pub fn set_kafka(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElKafkaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kafka = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kafka = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kinesis`.\n"]
    pub fn set_kinesis(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElKinesisEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElLambdaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `republish`.\n"]
    pub fn set_republish(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElRepublishEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.republish = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.republish = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl Into<BlockAssignable<IotTopicRuleErrorActionElS3El>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sns`.\n"]
    pub fn set_sns(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElSnsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sns = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sqs`.\n"]
    pub fn set_sqs(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElSqsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sqs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `step_functions`.\n"]
    pub fn set_step_functions(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElStepFunctionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.step_functions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.step_functions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timestream`.\n"]
    pub fn set_timestream(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleErrorActionElTimestreamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timestream = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timestream = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IotTopicRuleErrorActionEl {
    type O = BlockAssignable<IotTopicRuleErrorActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleErrorActionEl {}
impl BuildIotTopicRuleErrorActionEl {
    pub fn build(self) -> IotTopicRuleErrorActionEl {
        IotTopicRuleErrorActionEl {
            cloudwatch_alarm: core::default::Default::default(),
            cloudwatch_logs: core::default::Default::default(),
            cloudwatch_metric: core::default::Default::default(),
            dynamodb: core::default::Default::default(),
            dynamodbv2: core::default::Default::default(),
            elasticsearch: core::default::Default::default(),
            firehose: core::default::Default::default(),
            http: core::default::Default::default(),
            iot_analytics: core::default::Default::default(),
            iot_events: core::default::Default::default(),
            kafka: core::default::Default::default(),
            kinesis: core::default::Default::default(),
            lambda: core::default::Default::default(),
            republish: core::default::Default::default(),
            s3: core::default::Default::default(),
            sns: core::default::Default::default(),
            sqs: core::default::Default::default(),
            step_functions: core::default::Default::default(),
            timestream: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IotTopicRuleErrorActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleErrorActionElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleErrorActionElRef {
        IotTopicRuleErrorActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleErrorActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloudwatch_alarm` after provisioning.\n"]
    pub fn cloudwatch_alarm(&self) -> ListRef<IotTopicRuleErrorActionElCloudwatchAlarmElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_alarm", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(&self) -> ListRef<IotTopicRuleErrorActionElCloudwatchLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloudwatch_metric` after provisioning.\n"]
    pub fn cloudwatch_metric(&self) -> ListRef<IotTopicRuleErrorActionElCloudwatchMetricElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_metric", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dynamodb` after provisioning.\n"]
    pub fn dynamodb(&self) -> ListRef<IotTopicRuleErrorActionElDynamodbElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dynamodb", self.base))
    }
    #[doc = "Get a reference to the value of field `dynamodbv2` after provisioning.\n"]
    pub fn dynamodbv2(&self) -> ListRef<IotTopicRuleErrorActionElDynamodbv2ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dynamodbv2", self.base))
    }
    #[doc = "Get a reference to the value of field `elasticsearch` after provisioning.\n"]
    pub fn elasticsearch(&self) -> ListRef<IotTopicRuleErrorActionElElasticsearchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.elasticsearch", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(&self) -> ListRef<IotTopicRuleErrorActionElFirehoseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }
    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> ListRef<IotTopicRuleErrorActionElHttpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }
    #[doc = "Get a reference to the value of field `iot_analytics` after provisioning.\n"]
    pub fn iot_analytics(&self) -> ListRef<IotTopicRuleErrorActionElIotAnalyticsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.iot_analytics", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `iot_events` after provisioning.\n"]
    pub fn iot_events(&self) -> ListRef<IotTopicRuleErrorActionElIotEventsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iot_events", self.base))
    }
    #[doc = "Get a reference to the value of field `kafka` after provisioning.\n"]
    pub fn kafka(&self) -> ListRef<IotTopicRuleErrorActionElKafkaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kafka", self.base))
    }
    #[doc = "Get a reference to the value of field `kinesis` after provisioning.\n"]
    pub fn kinesis(&self) -> ListRef<IotTopicRuleErrorActionElKinesisElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kinesis", self.base))
    }
    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> ListRef<IotTopicRuleErrorActionElLambdaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }
    #[doc = "Get a reference to the value of field `republish` after provisioning.\n"]
    pub fn republish(&self) -> ListRef<IotTopicRuleErrorActionElRepublishElRef> {
        ListRef::new(self.shared().clone(), format!("{}.republish", self.base))
    }
    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<IotTopicRuleErrorActionElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
    #[doc = "Get a reference to the value of field `sns` after provisioning.\n"]
    pub fn sns(&self) -> ListRef<IotTopicRuleErrorActionElSnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sns", self.base))
    }
    #[doc = "Get a reference to the value of field `sqs` after provisioning.\n"]
    pub fn sqs(&self) -> ListRef<IotTopicRuleErrorActionElSqsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sqs", self.base))
    }
    #[doc = "Get a reference to the value of field `step_functions` after provisioning.\n"]
    pub fn step_functions(&self) -> ListRef<IotTopicRuleErrorActionElStepFunctionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.step_functions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timestream` after provisioning.\n"]
    pub fn timestream(&self) -> ListRef<IotTopicRuleErrorActionElTimestreamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timestream", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleFirehoseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_mode: Option<PrimField<bool>>,
    delivery_stream_name: PrimField<String>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    separator: Option<PrimField<String>>,
}
impl IotTopicRuleFirehoseEl {
    #[doc = "Set the field `batch_mode`.\n"]
    pub fn set_batch_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.batch_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `separator`.\n"]
    pub fn set_separator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.separator = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleFirehoseEl {
    type O = BlockAssignable<IotTopicRuleFirehoseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleFirehoseEl {
    #[doc = ""]
    pub delivery_stream_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleFirehoseEl {
    pub fn build(self) -> IotTopicRuleFirehoseEl {
        IotTopicRuleFirehoseEl {
            batch_mode: core::default::Default::default(),
            delivery_stream_name: self.delivery_stream_name,
            role_arn: self.role_arn,
            separator: core::default::Default::default(),
        }
    }
}
pub struct IotTopicRuleFirehoseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleFirehoseElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleFirehoseElRef {
        IotTopicRuleFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleFirehoseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_mode` after provisioning.\n"]
    pub fn batch_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `delivery_stream_name` after provisioning.\n"]
    pub fn delivery_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_stream_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `separator` after provisioning.\n"]
    pub fn separator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.separator", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleHttpElHttpHeaderEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl IotTopicRuleHttpElHttpHeaderEl {}
impl ToListMappable for IotTopicRuleHttpElHttpHeaderEl {
    type O = BlockAssignable<IotTopicRuleHttpElHttpHeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleHttpElHttpHeaderEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildIotTopicRuleHttpElHttpHeaderEl {
    pub fn build(self) -> IotTopicRuleHttpElHttpHeaderEl {
        IotTopicRuleHttpElHttpHeaderEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct IotTopicRuleHttpElHttpHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleHttpElHttpHeaderElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleHttpElHttpHeaderElRef {
        IotTopicRuleHttpElHttpHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleHttpElHttpHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct IotTopicRuleHttpElDynamic {
    http_header: Option<DynamicBlock<IotTopicRuleHttpElHttpHeaderEl>>,
}
#[derive(Serialize)]
pub struct IotTopicRuleHttpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confirmation_url: Option<PrimField<String>>,
    url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_header: Option<Vec<IotTopicRuleHttpElHttpHeaderEl>>,
    dynamic: IotTopicRuleHttpElDynamic,
}
impl IotTopicRuleHttpEl {
    #[doc = "Set the field `confirmation_url`.\n"]
    pub fn set_confirmation_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confirmation_url = Some(v.into());
        self
    }
    #[doc = "Set the field `http_header`.\n"]
    pub fn set_http_header(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleHttpElHttpHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_header = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IotTopicRuleHttpEl {
    type O = BlockAssignable<IotTopicRuleHttpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleHttpEl {
    #[doc = ""]
    pub url: PrimField<String>,
}
impl BuildIotTopicRuleHttpEl {
    pub fn build(self) -> IotTopicRuleHttpEl {
        IotTopicRuleHttpEl {
            confirmation_url: core::default::Default::default(),
            url: self.url,
            http_header: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IotTopicRuleHttpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleHttpElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleHttpElRef {
        IotTopicRuleHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleHttpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confirmation_url` after provisioning.\n"]
    pub fn confirmation_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confirmation_url", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
    #[doc = "Get a reference to the value of field `http_header` after provisioning.\n"]
    pub fn http_header(&self) -> ListRef<IotTopicRuleHttpElHttpHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_header", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleIotAnalyticsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_mode: Option<PrimField<bool>>,
    channel_name: PrimField<String>,
    role_arn: PrimField<String>,
}
impl IotTopicRuleIotAnalyticsEl {
    #[doc = "Set the field `batch_mode`.\n"]
    pub fn set_batch_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.batch_mode = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleIotAnalyticsEl {
    type O = BlockAssignable<IotTopicRuleIotAnalyticsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleIotAnalyticsEl {
    #[doc = ""]
    pub channel_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleIotAnalyticsEl {
    pub fn build(self) -> IotTopicRuleIotAnalyticsEl {
        IotTopicRuleIotAnalyticsEl {
            batch_mode: core::default::Default::default(),
            channel_name: self.channel_name,
            role_arn: self.role_arn,
        }
    }
}
pub struct IotTopicRuleIotAnalyticsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleIotAnalyticsElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleIotAnalyticsElRef {
        IotTopicRuleIotAnalyticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleIotAnalyticsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_mode` after provisioning.\n"]
    pub fn batch_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `channel_name` after provisioning.\n"]
    pub fn channel_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel_name", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleIotEventsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_mode: Option<PrimField<bool>>,
    input_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<PrimField<String>>,
    role_arn: PrimField<String>,
}
impl IotTopicRuleIotEventsEl {
    #[doc = "Set the field `batch_mode`.\n"]
    pub fn set_batch_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.batch_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `message_id`.\n"]
    pub fn set_message_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_id = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleIotEventsEl {
    type O = BlockAssignable<IotTopicRuleIotEventsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleIotEventsEl {
    #[doc = ""]
    pub input_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleIotEventsEl {
    pub fn build(self) -> IotTopicRuleIotEventsEl {
        IotTopicRuleIotEventsEl {
            batch_mode: core::default::Default::default(),
            input_name: self.input_name,
            message_id: core::default::Default::default(),
            role_arn: self.role_arn,
        }
    }
}
pub struct IotTopicRuleIotEventsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleIotEventsElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleIotEventsElRef {
        IotTopicRuleIotEventsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleIotEventsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_mode` after provisioning.\n"]
    pub fn batch_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `input_name` after provisioning.\n"]
    pub fn input_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_name", self.base))
    }
    #[doc = "Get a reference to the value of field `message_id` after provisioning.\n"]
    pub fn message_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_id", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleKafkaElHeaderEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl IotTopicRuleKafkaElHeaderEl {}
impl ToListMappable for IotTopicRuleKafkaElHeaderEl {
    type O = BlockAssignable<IotTopicRuleKafkaElHeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleKafkaElHeaderEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildIotTopicRuleKafkaElHeaderEl {
    pub fn build(self) -> IotTopicRuleKafkaElHeaderEl {
        IotTopicRuleKafkaElHeaderEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct IotTopicRuleKafkaElHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleKafkaElHeaderElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleKafkaElHeaderElRef {
        IotTopicRuleKafkaElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleKafkaElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct IotTopicRuleKafkaElDynamic {
    header: Option<DynamicBlock<IotTopicRuleKafkaElHeaderEl>>,
}
#[derive(Serialize)]
pub struct IotTopicRuleKafkaEl {
    client_properties: RecField<PrimField<String>>,
    destination_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition: Option<PrimField<String>>,
    topic: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<Vec<IotTopicRuleKafkaElHeaderEl>>,
    dynamic: IotTopicRuleKafkaElDynamic,
}
impl IotTopicRuleKafkaEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `partition`.\n"]
    pub fn set_partition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.partition = Some(v.into());
        self
    }
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleKafkaElHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IotTopicRuleKafkaEl {
    type O = BlockAssignable<IotTopicRuleKafkaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleKafkaEl {
    #[doc = ""]
    pub client_properties: RecField<PrimField<String>>,
    #[doc = ""]
    pub destination_arn: PrimField<String>,
    #[doc = ""]
    pub topic: PrimField<String>,
}
impl BuildIotTopicRuleKafkaEl {
    pub fn build(self) -> IotTopicRuleKafkaEl {
        IotTopicRuleKafkaEl {
            client_properties: self.client_properties,
            destination_arn: self.destination_arn,
            key: core::default::Default::default(),
            partition: core::default::Default::default(),
            topic: self.topic,
            header: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IotTopicRuleKafkaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleKafkaElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleKafkaElRef {
        IotTopicRuleKafkaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleKafkaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_properties` after provisioning.\n"]
    pub fn client_properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.client_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `partition` after provisioning.\n"]
    pub fn partition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.partition", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> ListRef<IotTopicRuleKafkaElHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleKinesisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_key: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    stream_name: PrimField<String>,
}
impl IotTopicRuleKinesisEl {
    #[doc = "Set the field `partition_key`.\n"]
    pub fn set_partition_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.partition_key = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleKinesisEl {
    type O = BlockAssignable<IotTopicRuleKinesisEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleKinesisEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub stream_name: PrimField<String>,
}
impl BuildIotTopicRuleKinesisEl {
    pub fn build(self) -> IotTopicRuleKinesisEl {
        IotTopicRuleKinesisEl {
            partition_key: core::default::Default::default(),
            role_arn: self.role_arn,
            stream_name: self.stream_name,
        }
    }
}
pub struct IotTopicRuleKinesisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleKinesisElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleKinesisElRef {
        IotTopicRuleKinesisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleKinesisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partition_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `stream_name` after provisioning.\n"]
    pub fn stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_name", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleLambdaEl {
    function_arn: PrimField<String>,
}
impl IotTopicRuleLambdaEl {}
impl ToListMappable for IotTopicRuleLambdaEl {
    type O = BlockAssignable<IotTopicRuleLambdaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleLambdaEl {
    #[doc = ""]
    pub function_arn: PrimField<String>,
}
impl BuildIotTopicRuleLambdaEl {
    pub fn build(self) -> IotTopicRuleLambdaEl {
        IotTopicRuleLambdaEl {
            function_arn: self.function_arn,
        }
    }
}
pub struct IotTopicRuleLambdaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleLambdaElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleLambdaElRef {
        IotTopicRuleLambdaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleLambdaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleRepublishEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    qos: Option<PrimField<f64>>,
    role_arn: PrimField<String>,
    topic: PrimField<String>,
}
impl IotTopicRuleRepublishEl {
    #[doc = "Set the field `qos`.\n"]
    pub fn set_qos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.qos = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleRepublishEl {
    type O = BlockAssignable<IotTopicRuleRepublishEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleRepublishEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub topic: PrimField<String>,
}
impl BuildIotTopicRuleRepublishEl {
    pub fn build(self) -> IotTopicRuleRepublishEl {
        IotTopicRuleRepublishEl {
            qos: core::default::Default::default(),
            role_arn: self.role_arn,
            topic: self.topic,
        }
    }
}
pub struct IotTopicRuleRepublishElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleRepublishElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleRepublishElRef {
        IotTopicRuleRepublishElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleRepublishElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `qos` after provisioning.\n"]
    pub fn qos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.qos", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleS3El {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canned_acl: Option<PrimField<String>>,
    key: PrimField<String>,
    role_arn: PrimField<String>,
}
impl IotTopicRuleS3El {
    #[doc = "Set the field `canned_acl`.\n"]
    pub fn set_canned_acl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.canned_acl = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleS3El {
    type O = BlockAssignable<IotTopicRuleS3El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleS3El {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildIotTopicRuleS3El {
    pub fn build(self) -> IotTopicRuleS3El {
        IotTopicRuleS3El {
            bucket_name: self.bucket_name,
            canned_acl: core::default::Default::default(),
            key: self.key,
            role_arn: self.role_arn,
        }
    }
}
pub struct IotTopicRuleS3ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleS3ElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleS3ElRef {
        IotTopicRuleS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `canned_acl` after provisioning.\n"]
    pub fn canned_acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.canned_acl", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleSnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    target_arn: PrimField<String>,
}
impl IotTopicRuleSnsEl {
    #[doc = "Set the field `message_format`.\n"]
    pub fn set_message_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleSnsEl {
    type O = BlockAssignable<IotTopicRuleSnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleSnsEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub target_arn: PrimField<String>,
}
impl BuildIotTopicRuleSnsEl {
    pub fn build(self) -> IotTopicRuleSnsEl {
        IotTopicRuleSnsEl {
            message_format: core::default::Default::default(),
            role_arn: self.role_arn,
            target_arn: self.target_arn,
        }
    }
}
pub struct IotTopicRuleSnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleSnsElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleSnsElRef {
        IotTopicRuleSnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleSnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_format` after provisioning.\n"]
    pub fn message_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `target_arn` after provisioning.\n"]
    pub fn target_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleSqsEl {
    queue_url: PrimField<String>,
    role_arn: PrimField<String>,
    use_base64: PrimField<bool>,
}
impl IotTopicRuleSqsEl {}
impl ToListMappable for IotTopicRuleSqsEl {
    type O = BlockAssignable<IotTopicRuleSqsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleSqsEl {
    #[doc = ""]
    pub queue_url: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub use_base64: PrimField<bool>,
}
impl BuildIotTopicRuleSqsEl {
    pub fn build(self) -> IotTopicRuleSqsEl {
        IotTopicRuleSqsEl {
            queue_url: self.queue_url,
            role_arn: self.role_arn,
            use_base64: self.use_base64,
        }
    }
}
pub struct IotTopicRuleSqsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleSqsElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleSqsElRef {
        IotTopicRuleSqsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleSqsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `queue_url` after provisioning.\n"]
    pub fn queue_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_url", self.base))
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `use_base64` after provisioning.\n"]
    pub fn use_base64(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_base64", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleStepFunctionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_name_prefix: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    state_machine_name: PrimField<String>,
}
impl IotTopicRuleStepFunctionsEl {
    #[doc = "Set the field `execution_name_prefix`.\n"]
    pub fn set_execution_name_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_name_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for IotTopicRuleStepFunctionsEl {
    type O = BlockAssignable<IotTopicRuleStepFunctionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleStepFunctionsEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub state_machine_name: PrimField<String>,
}
impl BuildIotTopicRuleStepFunctionsEl {
    pub fn build(self) -> IotTopicRuleStepFunctionsEl {
        IotTopicRuleStepFunctionsEl {
            execution_name_prefix: core::default::Default::default(),
            role_arn: self.role_arn,
            state_machine_name: self.state_machine_name,
        }
    }
}
pub struct IotTopicRuleStepFunctionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleStepFunctionsElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleStepFunctionsElRef {
        IotTopicRuleStepFunctionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleStepFunctionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `execution_name_prefix` after provisioning.\n"]
    pub fn execution_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_name_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `state_machine_name` after provisioning.\n"]
    pub fn state_machine_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_machine_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleTimestreamElDimensionEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl IotTopicRuleTimestreamElDimensionEl {}
impl ToListMappable for IotTopicRuleTimestreamElDimensionEl {
    type O = BlockAssignable<IotTopicRuleTimestreamElDimensionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleTimestreamElDimensionEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildIotTopicRuleTimestreamElDimensionEl {
    pub fn build(self) -> IotTopicRuleTimestreamElDimensionEl {
        IotTopicRuleTimestreamElDimensionEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct IotTopicRuleTimestreamElDimensionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleTimestreamElDimensionElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleTimestreamElDimensionElRef {
        IotTopicRuleTimestreamElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleTimestreamElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct IotTopicRuleTimestreamElTimestampEl {
    unit: PrimField<String>,
    value: PrimField<String>,
}
impl IotTopicRuleTimestreamElTimestampEl {}
impl ToListMappable for IotTopicRuleTimestreamElTimestampEl {
    type O = BlockAssignable<IotTopicRuleTimestreamElTimestampEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleTimestreamElTimestampEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildIotTopicRuleTimestreamElTimestampEl {
    pub fn build(self) -> IotTopicRuleTimestreamElTimestampEl {
        IotTopicRuleTimestreamElTimestampEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct IotTopicRuleTimestreamElTimestampElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleTimestreamElTimestampElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleTimestreamElTimestampElRef {
        IotTopicRuleTimestreamElTimestampElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleTimestreamElTimestampElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
struct IotTopicRuleTimestreamElDynamic {
    dimension: Option<DynamicBlock<IotTopicRuleTimestreamElDimensionEl>>,
    timestamp: Option<DynamicBlock<IotTopicRuleTimestreamElTimestampEl>>,
}
#[derive(Serialize)]
pub struct IotTopicRuleTimestreamEl {
    database_name: PrimField<String>,
    role_arn: PrimField<String>,
    table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<IotTopicRuleTimestreamElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<Vec<IotTopicRuleTimestreamElTimestampEl>>,
    dynamic: IotTopicRuleTimestreamElDynamic,
}
impl IotTopicRuleTimestreamEl {
    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleTimestreamElDimensionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timestamp`.\n"]
    pub fn set_timestamp(
        mut self,
        v: impl Into<BlockAssignable<IotTopicRuleTimestreamElTimestampEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timestamp = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timestamp = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IotTopicRuleTimestreamEl {
    type O = BlockAssignable<IotTopicRuleTimestreamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIotTopicRuleTimestreamEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
}
impl BuildIotTopicRuleTimestreamEl {
    pub fn build(self) -> IotTopicRuleTimestreamEl {
        IotTopicRuleTimestreamEl {
            database_name: self.database_name,
            role_arn: self.role_arn,
            table_name: self.table_name,
            dimension: core::default::Default::default(),
            timestamp: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IotTopicRuleTimestreamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IotTopicRuleTimestreamElRef {
    fn new(shared: StackShared, base: String) -> IotTopicRuleTimestreamElRef {
        IotTopicRuleTimestreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IotTopicRuleTimestreamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
    #[doc = "Get a reference to the value of field `timestamp` after provisioning.\n"]
    pub fn timestamp(&self) -> ListRef<IotTopicRuleTimestreamElTimestampElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }
}
#[derive(Serialize, Default)]
struct IotTopicRuleDynamic {
    cloudwatch_alarm: Option<DynamicBlock<IotTopicRuleCloudwatchAlarmEl>>,
    cloudwatch_logs: Option<DynamicBlock<IotTopicRuleCloudwatchLogsEl>>,
    cloudwatch_metric: Option<DynamicBlock<IotTopicRuleCloudwatchMetricEl>>,
    dynamodb: Option<DynamicBlock<IotTopicRuleDynamodbEl>>,
    dynamodbv2: Option<DynamicBlock<IotTopicRuleDynamodbv2El>>,
    elasticsearch: Option<DynamicBlock<IotTopicRuleElasticsearchEl>>,
    error_action: Option<DynamicBlock<IotTopicRuleErrorActionEl>>,
    firehose: Option<DynamicBlock<IotTopicRuleFirehoseEl>>,
    http: Option<DynamicBlock<IotTopicRuleHttpEl>>,
    iot_analytics: Option<DynamicBlock<IotTopicRuleIotAnalyticsEl>>,
    iot_events: Option<DynamicBlock<IotTopicRuleIotEventsEl>>,
    kafka: Option<DynamicBlock<IotTopicRuleKafkaEl>>,
    kinesis: Option<DynamicBlock<IotTopicRuleKinesisEl>>,
    lambda: Option<DynamicBlock<IotTopicRuleLambdaEl>>,
    republish: Option<DynamicBlock<IotTopicRuleRepublishEl>>,
    s3: Option<DynamicBlock<IotTopicRuleS3El>>,
    sns: Option<DynamicBlock<IotTopicRuleSnsEl>>,
    sqs: Option<DynamicBlock<IotTopicRuleSqsEl>>,
    step_functions: Option<DynamicBlock<IotTopicRuleStepFunctionsEl>>,
    timestream: Option<DynamicBlock<IotTopicRuleTimestreamEl>>,
}
