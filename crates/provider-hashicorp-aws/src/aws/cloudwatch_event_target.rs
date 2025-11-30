use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CloudwatchEventTargetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_bus_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    rule: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    appsync_target: Option<Vec<CloudwatchEventTargetAppsyncTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_target: Option<Vec<CloudwatchEventTargetBatchTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_letter_config: Option<Vec<CloudwatchEventTargetDeadLetterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecs_target: Option<Vec<CloudwatchEventTargetEcsTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_target: Option<Vec<CloudwatchEventTargetHttpTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_transformer: Option<Vec<CloudwatchEventTargetInputTransformerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_target: Option<Vec<CloudwatchEventTargetKinesisTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redshift_target: Option<Vec<CloudwatchEventTargetRedshiftTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<CloudwatchEventTargetRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_command_targets: Option<Vec<CloudwatchEventTargetRunCommandTargetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_pipeline_target: Option<Vec<CloudwatchEventTargetSagemakerPipelineTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs_target: Option<Vec<CloudwatchEventTargetSqsTargetEl>>,
    dynamic: CloudwatchEventTargetDynamic,
}

struct CloudwatchEventTarget_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudwatchEventTargetData>,
}

#[derive(Clone)]
pub struct CloudwatchEventTarget(Rc<CloudwatchEventTarget_>);

impl CloudwatchEventTarget {
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

    #[doc = "Set the field `event_bus_name`.\n"]
    pub fn set_event_bus_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().event_bus_name = Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `input`.\n"]
    pub fn set_input(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().input = Some(v.into());
        self
    }

    #[doc = "Set the field `input_path`.\n"]
    pub fn set_input_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().input_path = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `target_id`.\n"]
    pub fn set_target_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().target_id = Some(v.into());
        self
    }

    #[doc = "Set the field `appsync_target`.\n"]
    pub fn set_appsync_target(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetAppsyncTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().appsync_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.appsync_target = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `batch_target`.\n"]
    pub fn set_batch_target(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetBatchTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().batch_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.batch_target = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `dead_letter_config`.\n"]
    pub fn set_dead_letter_config(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetDeadLetterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dead_letter_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dead_letter_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ecs_target`.\n"]
    pub fn set_ecs_target(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetEcsTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ecs_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ecs_target = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `http_target`.\n"]
    pub fn set_http_target(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetHttpTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_target = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `input_transformer`.\n"]
    pub fn set_input_transformer(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetInputTransformerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_transformer = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_transformer = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `kinesis_target`.\n"]
    pub fn set_kinesis_target(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetKinesisTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kinesis_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kinesis_target = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `redshift_target`.\n"]
    pub fn set_redshift_target(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetRedshiftTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redshift_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redshift_target = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetRetryPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_policy = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `run_command_targets`.\n"]
    pub fn set_run_command_targets(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetRunCommandTargetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().run_command_targets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.run_command_targets = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `sagemaker_pipeline_target`.\n"]
    pub fn set_sagemaker_pipeline_target(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetSagemakerPipelineTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sagemaker_pipeline_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sagemaker_pipeline_target = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `sqs_target`.\n"]
    pub fn set_sqs_target(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetSqsTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sqs_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sqs_target = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_bus_name` after provisioning.\n"]
    pub fn event_bus_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_bus_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `input_path` after provisioning.\n"]
    pub fn input_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `appsync_target` after provisioning.\n"]
    pub fn appsync_target(&self) -> ListRef<CloudwatchEventTargetAppsyncTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.appsync_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `batch_target` after provisioning.\n"]
    pub fn batch_target(&self) -> ListRef<CloudwatchEventTargetBatchTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.batch_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> ListRef<CloudwatchEventTargetDeadLetterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dead_letter_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ecs_target` after provisioning.\n"]
    pub fn ecs_target(&self) -> ListRef<CloudwatchEventTargetEcsTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ecs_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `http_target` after provisioning.\n"]
    pub fn http_target(&self) -> ListRef<CloudwatchEventTargetHttpTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `input_transformer` after provisioning.\n"]
    pub fn input_transformer(&self) -> ListRef<CloudwatchEventTargetInputTransformerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_transformer", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kinesis_target` after provisioning.\n"]
    pub fn kinesis_target(&self) -> ListRef<CloudwatchEventTargetKinesisTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `redshift_target` after provisioning.\n"]
    pub fn redshift_target(&self) -> ListRef<CloudwatchEventTargetRedshiftTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redshift_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<CloudwatchEventTargetRetryPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `run_command_targets` after provisioning.\n"]
    pub fn run_command_targets(&self) -> ListRef<CloudwatchEventTargetRunCommandTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.run_command_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_pipeline_target` after provisioning.\n"]
    pub fn sagemaker_pipeline_target(
        &self,
    ) -> ListRef<CloudwatchEventTargetSagemakerPipelineTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sagemaker_pipeline_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sqs_target` after provisioning.\n"]
    pub fn sqs_target(&self) -> ListRef<CloudwatchEventTargetSqsTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sqs_target", self.extract_ref()),
        )
    }
}

impl Referable for CloudwatchEventTarget {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CloudwatchEventTarget {}

impl ToListMappable for CloudwatchEventTarget {
    type O = ListRef<CloudwatchEventTargetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudwatchEventTarget_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudwatch_event_target".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchEventTarget {
    pub tf_id: String,
    #[doc = ""]
    pub arn: PrimField<String>,
    #[doc = ""]
    pub rule: PrimField<String>,
}

impl BuildCloudwatchEventTarget {
    pub fn build(self, stack: &mut Stack) -> CloudwatchEventTarget {
        let out = CloudwatchEventTarget(Rc::new(CloudwatchEventTarget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudwatchEventTargetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                arn: self.arn,
                event_bus_name: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                input: core::default::Default::default(),
                input_path: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                rule: self.rule,
                target_id: core::default::Default::default(),
                appsync_target: core::default::Default::default(),
                batch_target: core::default::Default::default(),
                dead_letter_config: core::default::Default::default(),
                ecs_target: core::default::Default::default(),
                http_target: core::default::Default::default(),
                input_transformer: core::default::Default::default(),
                kinesis_target: core::default::Default::default(),
                redshift_target: core::default::Default::default(),
                retry_policy: core::default::Default::default(),
                run_command_targets: core::default::Default::default(),
                sagemaker_pipeline_target: core::default::Default::default(),
                sqs_target: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchEventTargetRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CloudwatchEventTargetRef {
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

    #[doc = "Get a reference to the value of field `event_bus_name` after provisioning.\n"]
    pub fn event_bus_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_bus_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `input_path` after provisioning.\n"]
    pub fn input_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `appsync_target` after provisioning.\n"]
    pub fn appsync_target(&self) -> ListRef<CloudwatchEventTargetAppsyncTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.appsync_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `batch_target` after provisioning.\n"]
    pub fn batch_target(&self) -> ListRef<CloudwatchEventTargetBatchTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.batch_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> ListRef<CloudwatchEventTargetDeadLetterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dead_letter_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ecs_target` after provisioning.\n"]
    pub fn ecs_target(&self) -> ListRef<CloudwatchEventTargetEcsTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ecs_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `http_target` after provisioning.\n"]
    pub fn http_target(&self) -> ListRef<CloudwatchEventTargetHttpTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `input_transformer` after provisioning.\n"]
    pub fn input_transformer(&self) -> ListRef<CloudwatchEventTargetInputTransformerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_transformer", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kinesis_target` after provisioning.\n"]
    pub fn kinesis_target(&self) -> ListRef<CloudwatchEventTargetKinesisTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `redshift_target` after provisioning.\n"]
    pub fn redshift_target(&self) -> ListRef<CloudwatchEventTargetRedshiftTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redshift_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<CloudwatchEventTargetRetryPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `run_command_targets` after provisioning.\n"]
    pub fn run_command_targets(&self) -> ListRef<CloudwatchEventTargetRunCommandTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.run_command_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_pipeline_target` after provisioning.\n"]
    pub fn sagemaker_pipeline_target(
        &self,
    ) -> ListRef<CloudwatchEventTargetSagemakerPipelineTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sagemaker_pipeline_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sqs_target` after provisioning.\n"]
    pub fn sqs_target(&self) -> ListRef<CloudwatchEventTargetSqsTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sqs_target", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetAppsyncTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    graphql_operation: Option<PrimField<String>>,
}

impl CloudwatchEventTargetAppsyncTargetEl {
    #[doc = "Set the field `graphql_operation`.\n"]
    pub fn set_graphql_operation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.graphql_operation = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetAppsyncTargetEl {
    type O = BlockAssignable<CloudwatchEventTargetAppsyncTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetAppsyncTargetEl {}

impl BuildCloudwatchEventTargetAppsyncTargetEl {
    pub fn build(self) -> CloudwatchEventTargetAppsyncTargetEl {
        CloudwatchEventTargetAppsyncTargetEl {
            graphql_operation: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetAppsyncTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetAppsyncTargetElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventTargetAppsyncTargetElRef {
        CloudwatchEventTargetAppsyncTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetAppsyncTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `graphql_operation` after provisioning.\n"]
    pub fn graphql_operation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphql_operation", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetBatchTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    array_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_attempts: Option<PrimField<f64>>,
    job_definition: PrimField<String>,
    job_name: PrimField<String>,
}

impl CloudwatchEventTargetBatchTargetEl {
    #[doc = "Set the field `array_size`.\n"]
    pub fn set_array_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.array_size = Some(v.into());
        self
    }

    #[doc = "Set the field `job_attempts`.\n"]
    pub fn set_job_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.job_attempts = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetBatchTargetEl {
    type O = BlockAssignable<CloudwatchEventTargetBatchTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetBatchTargetEl {
    #[doc = ""]
    pub job_definition: PrimField<String>,
    #[doc = ""]
    pub job_name: PrimField<String>,
}

impl BuildCloudwatchEventTargetBatchTargetEl {
    pub fn build(self) -> CloudwatchEventTargetBatchTargetEl {
        CloudwatchEventTargetBatchTargetEl {
            array_size: core::default::Default::default(),
            job_attempts: core::default::Default::default(),
            job_definition: self.job_definition,
            job_name: self.job_name,
        }
    }
}

pub struct CloudwatchEventTargetBatchTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetBatchTargetElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventTargetBatchTargetElRef {
        CloudwatchEventTargetBatchTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetBatchTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `array_size` after provisioning.\n"]
    pub fn array_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.array_size", self.base))
    }

    #[doc = "Get a reference to the value of field `job_attempts` after provisioning.\n"]
    pub fn job_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `job_definition` after provisioning.\n"]
    pub fn job_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_definition", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetDeadLetterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
}

impl CloudwatchEventTargetDeadLetterConfigEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetDeadLetterConfigEl {
    type O = BlockAssignable<CloudwatchEventTargetDeadLetterConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetDeadLetterConfigEl {}

impl BuildCloudwatchEventTargetDeadLetterConfigEl {
    pub fn build(self) -> CloudwatchEventTargetDeadLetterConfigEl {
        CloudwatchEventTargetDeadLetterConfigEl {
            arn: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetDeadLetterConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetDeadLetterConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventTargetDeadLetterConfigElRef {
        CloudwatchEventTargetDeadLetterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetDeadLetterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<PrimField<f64>>,
    capacity_provider: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.base = Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
    type O = BlockAssignable<CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
    #[doc = ""]
    pub capacity_provider: PrimField<String>,
}

impl BuildCloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
    pub fn build(self) -> CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
        CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetEcsTargetElCapacityProviderStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetEcsTargetElCapacityProviderStrategyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventTargetEcsTargetElCapacityProviderStrategyElRef {
        CloudwatchEventTargetEcsTargetElCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetEcsTargetElCapacityProviderStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_provider", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    subnets: SetField<PrimField<String>>,
}

impl CloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
    #[doc = "Set the field `assign_public_ip`.\n"]
    pub fn set_assign_public_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.assign_public_ip = Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
    type O = BlockAssignable<CloudwatchEventTargetEcsTargetElNetworkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
}

impl BuildCloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
    pub fn build(self) -> CloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
        CloudwatchEventTargetEcsTargetElNetworkConfigurationEl {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: self.subnets,
        }
    }
}

pub struct CloudwatchEventTargetEcsTargetElNetworkConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetEcsTargetElNetworkConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventTargetEcsTargetElNetworkConfigurationElRef {
        CloudwatchEventTargetEcsTargetElNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetEcsTargetElNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_public_ip", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyEl {
    type O = BlockAssignable<CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetEcsTargetElOrderedPlacementStrategyEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildCloudwatchEventTargetEcsTargetElOrderedPlacementStrategyEl {
    pub fn build(self) -> CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyEl {
        CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyEl {
            field: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyElRef {
        CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetEcsTargetElPlacementConstraintEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl CloudwatchEventTargetEcsTargetElPlacementConstraintEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetEcsTargetElPlacementConstraintEl {
    type O = BlockAssignable<CloudwatchEventTargetEcsTargetElPlacementConstraintEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetEcsTargetElPlacementConstraintEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildCloudwatchEventTargetEcsTargetElPlacementConstraintEl {
    pub fn build(self) -> CloudwatchEventTargetEcsTargetElPlacementConstraintEl {
        CloudwatchEventTargetEcsTargetElPlacementConstraintEl {
            expression: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct CloudwatchEventTargetEcsTargetElPlacementConstraintElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetEcsTargetElPlacementConstraintElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventTargetEcsTargetElPlacementConstraintElRef {
        CloudwatchEventTargetEcsTargetElPlacementConstraintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetEcsTargetElPlacementConstraintElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventTargetEcsTargetElDynamic {
    capacity_provider_strategy:
        Option<DynamicBlock<CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl>>,
    network_configuration:
        Option<DynamicBlock<CloudwatchEventTargetEcsTargetElNetworkConfigurationEl>>,
    ordered_placement_strategy:
        Option<DynamicBlock<CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyEl>>,
    placement_constraint:
        Option<DynamicBlock<CloudwatchEventTargetEcsTargetElPlacementConstraintEl>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetEcsTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ecs_managed_tags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_execute_command: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagate_tags: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_count: Option<PrimField<f64>>,
    task_definition_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_provider_strategy:
        Option<Vec<CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configuration: Option<Vec<CloudwatchEventTargetEcsTargetElNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordered_placement_strategy:
        Option<Vec<CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_constraint: Option<Vec<CloudwatchEventTargetEcsTargetElPlacementConstraintEl>>,
    dynamic: CloudwatchEventTargetEcsTargetElDynamic,
}

impl CloudwatchEventTargetEcsTargetEl {
    #[doc = "Set the field `enable_ecs_managed_tags`.\n"]
    pub fn set_enable_ecs_managed_tags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ecs_managed_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_execute_command`.\n"]
    pub fn set_enable_execute_command(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_execute_command = Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_type`.\n"]
    pub fn set_launch_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_type = Some(v.into());
        self
    }

    #[doc = "Set the field `platform_version`.\n"]
    pub fn set_platform_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.platform_version = Some(v.into());
        self
    }

    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.propagate_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }

    #[doc = "Set the field `task_count`.\n"]
    pub fn set_task_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.task_count = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_provider_strategy`.\n"]
    pub fn set_capacity_provider_strategy(
        mut self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetEcsTargetElCapacityProviderStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.capacity_provider_strategy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_provider_strategy = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        mut self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetEcsTargetElNetworkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ordered_placement_strategy`.\n"]
    pub fn set_ordered_placement_strategy(
        mut self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ordered_placement_strategy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ordered_placement_strategy = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `placement_constraint`.\n"]
    pub fn set_placement_constraint(
        mut self,
        v: impl Into<BlockAssignable<CloudwatchEventTargetEcsTargetElPlacementConstraintEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.placement_constraint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_constraint = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CloudwatchEventTargetEcsTargetEl {
    type O = BlockAssignable<CloudwatchEventTargetEcsTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetEcsTargetEl {
    #[doc = ""]
    pub task_definition_arn: PrimField<String>,
}

impl BuildCloudwatchEventTargetEcsTargetEl {
    pub fn build(self) -> CloudwatchEventTargetEcsTargetEl {
        CloudwatchEventTargetEcsTargetEl {
            enable_ecs_managed_tags: core::default::Default::default(),
            enable_execute_command: core::default::Default::default(),
            group: core::default::Default::default(),
            launch_type: core::default::Default::default(),
            platform_version: core::default::Default::default(),
            propagate_tags: core::default::Default::default(),
            tags: core::default::Default::default(),
            task_count: core::default::Default::default(),
            task_definition_arn: self.task_definition_arn,
            capacity_provider_strategy: core::default::Default::default(),
            network_configuration: core::default::Default::default(),
            ordered_placement_strategy: core::default::Default::default(),
            placement_constraint: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetEcsTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetEcsTargetElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventTargetEcsTargetElRef {
        CloudwatchEventTargetEcsTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetEcsTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ecs_managed_tags", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_execute_command", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.base))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_version", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagate_tags", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `task_count` after provisioning.\n"]
    pub fn task_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_count", self.base))
    }

    #[doc = "Get a reference to the value of field `task_definition_arn` after provisioning.\n"]
    pub fn task_definition_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_definition_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(
        &self,
    ) -> ListRef<CloudwatchEventTargetEcsTargetElNetworkConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ordered_placement_strategy` after provisioning.\n"]
    pub fn ordered_placement_strategy(
        &self,
    ) -> ListRef<CloudwatchEventTargetEcsTargetElOrderedPlacementStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ordered_placement_strategy", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetHttpTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_parameter_values: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_parameters: Option<RecField<PrimField<String>>>,
}

impl CloudwatchEventTargetHttpTargetEl {
    #[doc = "Set the field `header_parameters`.\n"]
    pub fn set_header_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.header_parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `path_parameter_values`.\n"]
    pub fn set_path_parameter_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.path_parameter_values = Some(v.into());
        self
    }

    #[doc = "Set the field `query_string_parameters`.\n"]
    pub fn set_query_string_parameters(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.query_string_parameters = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetHttpTargetEl {
    type O = BlockAssignable<CloudwatchEventTargetHttpTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetHttpTargetEl {}

impl BuildCloudwatchEventTargetHttpTargetEl {
    pub fn build(self) -> CloudwatchEventTargetHttpTargetEl {
        CloudwatchEventTargetHttpTargetEl {
            header_parameters: core::default::Default::default(),
            path_parameter_values: core::default::Default::default(),
            query_string_parameters: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetHttpTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetHttpTargetElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventTargetHttpTargetElRef {
        CloudwatchEventTargetHttpTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetHttpTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_parameters` after provisioning.\n"]
    pub fn header_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.header_parameters", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `path_parameter_values` after provisioning.\n"]
    pub fn path_parameter_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.path_parameter_values", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `query_string_parameters` after provisioning.\n"]
    pub fn query_string_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.query_string_parameters", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetInputTransformerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_paths: Option<RecField<PrimField<String>>>,
    input_template: PrimField<String>,
}

impl CloudwatchEventTargetInputTransformerEl {
    #[doc = "Set the field `input_paths`.\n"]
    pub fn set_input_paths(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.input_paths = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetInputTransformerEl {
    type O = BlockAssignable<CloudwatchEventTargetInputTransformerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetInputTransformerEl {
    #[doc = ""]
    pub input_template: PrimField<String>,
}

impl BuildCloudwatchEventTargetInputTransformerEl {
    pub fn build(self) -> CloudwatchEventTargetInputTransformerEl {
        CloudwatchEventTargetInputTransformerEl {
            input_paths: core::default::Default::default(),
            input_template: self.input_template,
        }
    }
}

pub struct CloudwatchEventTargetInputTransformerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetInputTransformerElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventTargetInputTransformerElRef {
        CloudwatchEventTargetInputTransformerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetInputTransformerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_paths` after provisioning.\n"]
    pub fn input_paths(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.input_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `input_template` after provisioning.\n"]
    pub fn input_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_template", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetKinesisTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_key_path: Option<PrimField<String>>,
}

impl CloudwatchEventTargetKinesisTargetEl {
    #[doc = "Set the field `partition_key_path`.\n"]
    pub fn set_partition_key_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.partition_key_path = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetKinesisTargetEl {
    type O = BlockAssignable<CloudwatchEventTargetKinesisTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetKinesisTargetEl {}

impl BuildCloudwatchEventTargetKinesisTargetEl {
    pub fn build(self) -> CloudwatchEventTargetKinesisTargetEl {
        CloudwatchEventTargetKinesisTargetEl {
            partition_key_path: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetKinesisTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetKinesisTargetElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventTargetKinesisTargetElRef {
        CloudwatchEventTargetKinesisTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetKinesisTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partition_key_path` after provisioning.\n"]
    pub fn partition_key_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partition_key_path", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetRedshiftTargetEl {
    database: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets_manager_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_event: Option<PrimField<bool>>,
}

impl CloudwatchEventTargetRedshiftTargetEl {
    #[doc = "Set the field `db_user`.\n"]
    pub fn set_db_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_user = Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_arn`.\n"]
    pub fn set_secrets_manager_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secrets_manager_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sql`.\n"]
    pub fn set_sql(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sql = Some(v.into());
        self
    }

    #[doc = "Set the field `statement_name`.\n"]
    pub fn set_statement_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.statement_name = Some(v.into());
        self
    }

    #[doc = "Set the field `with_event`.\n"]
    pub fn set_with_event(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.with_event = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetRedshiftTargetEl {
    type O = BlockAssignable<CloudwatchEventTargetRedshiftTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetRedshiftTargetEl {
    #[doc = ""]
    pub database: PrimField<String>,
}

impl BuildCloudwatchEventTargetRedshiftTargetEl {
    pub fn build(self) -> CloudwatchEventTargetRedshiftTargetEl {
        CloudwatchEventTargetRedshiftTargetEl {
            database: self.database,
            db_user: core::default::Default::default(),
            secrets_manager_arn: core::default::Default::default(),
            sql: core::default::Default::default(),
            statement_name: core::default::Default::default(),
            with_event: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetRedshiftTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetRedshiftTargetElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventTargetRedshiftTargetElRef {
        CloudwatchEventTargetRedshiftTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetRedshiftTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_arn` after provisioning.\n"]
    pub fn secrets_manager_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secrets_manager_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sql` after provisioning.\n"]
    pub fn sql(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql", self.base))
    }

    #[doc = "Get a reference to the value of field `statement_name` after provisioning.\n"]
    pub fn statement_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.statement_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `with_event` after provisioning.\n"]
    pub fn with_event(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_event", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_event_age_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_retry_attempts: Option<PrimField<f64>>,
}

impl CloudwatchEventTargetRetryPolicyEl {
    #[doc = "Set the field `maximum_event_age_in_seconds`.\n"]
    pub fn set_maximum_event_age_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_event_age_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_retry_attempts`.\n"]
    pub fn set_maximum_retry_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_retry_attempts = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetRetryPolicyEl {
    type O = BlockAssignable<CloudwatchEventTargetRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetRetryPolicyEl {}

impl BuildCloudwatchEventTargetRetryPolicyEl {
    pub fn build(self) -> CloudwatchEventTargetRetryPolicyEl {
        CloudwatchEventTargetRetryPolicyEl {
            maximum_event_age_in_seconds: core::default::Default::default(),
            maximum_retry_attempts: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventTargetRetryPolicyElRef {
        CloudwatchEventTargetRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_event_age_in_seconds` after provisioning.\n"]
    pub fn maximum_event_age_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_event_age_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_retry_attempts", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetRunCommandTargetsEl {
    key: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl CloudwatchEventTargetRunCommandTargetsEl {}

impl ToListMappable for CloudwatchEventTargetRunCommandTargetsEl {
    type O = BlockAssignable<CloudwatchEventTargetRunCommandTargetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetRunCommandTargetsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildCloudwatchEventTargetRunCommandTargetsEl {
    pub fn build(self) -> CloudwatchEventTargetRunCommandTargetsEl {
        CloudwatchEventTargetRunCommandTargetsEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct CloudwatchEventTargetRunCommandTargetsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetRunCommandTargetsElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventTargetRunCommandTargetsElRef {
        CloudwatchEventTargetRunCommandTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetRunCommandTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListEl {}

impl ToListMappable for CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListEl {
    type O = BlockAssignable<CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildCloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListEl {
    pub fn build(self) -> CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListEl {
        CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListElRef {
        CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListElRef {
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

#[derive(Serialize, Default)]
struct CloudwatchEventTargetSagemakerPipelineTargetElDynamic {
    pipeline_parameter_list:
        Option<DynamicBlock<CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListEl>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetSagemakerPipelineTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_parameter_list:
        Option<Vec<CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListEl>>,
    dynamic: CloudwatchEventTargetSagemakerPipelineTargetElDynamic,
}

impl CloudwatchEventTargetSagemakerPipelineTargetEl {
    #[doc = "Set the field `pipeline_parameter_list`.\n"]
    pub fn set_pipeline_parameter_list(
        mut self,
        v: impl Into<
            BlockAssignable<CloudwatchEventTargetSagemakerPipelineTargetElPipelineParameterListEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pipeline_parameter_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pipeline_parameter_list = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CloudwatchEventTargetSagemakerPipelineTargetEl {
    type O = BlockAssignable<CloudwatchEventTargetSagemakerPipelineTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetSagemakerPipelineTargetEl {}

impl BuildCloudwatchEventTargetSagemakerPipelineTargetEl {
    pub fn build(self) -> CloudwatchEventTargetSagemakerPipelineTargetEl {
        CloudwatchEventTargetSagemakerPipelineTargetEl {
            pipeline_parameter_list: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetSagemakerPipelineTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetSagemakerPipelineTargetElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventTargetSagemakerPipelineTargetElRef {
        CloudwatchEventTargetSagemakerPipelineTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetSagemakerPipelineTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventTargetSqsTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_group_id: Option<PrimField<String>>,
}

impl CloudwatchEventTargetSqsTargetEl {
    #[doc = "Set the field `message_group_id`.\n"]
    pub fn set_message_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_group_id = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventTargetSqsTargetEl {
    type O = BlockAssignable<CloudwatchEventTargetSqsTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventTargetSqsTargetEl {}

impl BuildCloudwatchEventTargetSqsTargetEl {
    pub fn build(self) -> CloudwatchEventTargetSqsTargetEl {
        CloudwatchEventTargetSqsTargetEl {
            message_group_id: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventTargetSqsTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventTargetSqsTargetElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventTargetSqsTargetElRef {
        CloudwatchEventTargetSqsTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventTargetSqsTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_group_id` after provisioning.\n"]
    pub fn message_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_group_id", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventTargetDynamic {
    appsync_target: Option<DynamicBlock<CloudwatchEventTargetAppsyncTargetEl>>,
    batch_target: Option<DynamicBlock<CloudwatchEventTargetBatchTargetEl>>,
    dead_letter_config: Option<DynamicBlock<CloudwatchEventTargetDeadLetterConfigEl>>,
    ecs_target: Option<DynamicBlock<CloudwatchEventTargetEcsTargetEl>>,
    http_target: Option<DynamicBlock<CloudwatchEventTargetHttpTargetEl>>,
    input_transformer: Option<DynamicBlock<CloudwatchEventTargetInputTransformerEl>>,
    kinesis_target: Option<DynamicBlock<CloudwatchEventTargetKinesisTargetEl>>,
    redshift_target: Option<DynamicBlock<CloudwatchEventTargetRedshiftTargetEl>>,
    retry_policy: Option<DynamicBlock<CloudwatchEventTargetRetryPolicyEl>>,
    run_command_targets: Option<DynamicBlock<CloudwatchEventTargetRunCommandTargetsEl>>,
    sagemaker_pipeline_target: Option<DynamicBlock<CloudwatchEventTargetSagemakerPipelineTargetEl>>,
    sqs_target: Option<DynamicBlock<CloudwatchEventTargetSqsTargetEl>>,
}
