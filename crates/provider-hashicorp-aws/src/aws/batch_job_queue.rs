use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct BatchJobQueueData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduling_policy_arn: Option<PrimField<String>>,
    state: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_environment_order: Option<Vec<BatchJobQueueComputeEnvironmentOrderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_state_time_limit_action: Option<Vec<BatchJobQueueJobStateTimeLimitActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BatchJobQueueTimeoutsEl>,
    dynamic: BatchJobQueueDynamic,
}

struct BatchJobQueue_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BatchJobQueueData>,
}

#[derive(Clone)]
pub struct BatchJobQueue(Rc<BatchJobQueue_>);

impl BatchJobQueue {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `scheduling_policy_arn`.\n"]
    pub fn set_scheduling_policy_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().scheduling_policy_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `compute_environment_order`.\n"]
    pub fn set_compute_environment_order(
        self,
        v: impl Into<BlockAssignable<BatchJobQueueComputeEnvironmentOrderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_environment_order = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_environment_order = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `job_state_time_limit_action`.\n"]
    pub fn set_job_state_time_limit_action(
        self,
        v: impl Into<BlockAssignable<BatchJobQueueJobStateTimeLimitActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().job_state_time_limit_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.job_state_time_limit_action = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BatchJobQueueTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scheduling_policy_arn` after provisioning.\n"]
    pub fn scheduling_policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduling_policy_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `compute_environment_order` after provisioning.\n"]
    pub fn compute_environment_order(&self) -> ListRef<BatchJobQueueComputeEnvironmentOrderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_environment_order", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `job_state_time_limit_action` after provisioning.\n"]
    pub fn job_state_time_limit_action(
        &self,
    ) -> ListRef<BatchJobQueueJobStateTimeLimitActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.job_state_time_limit_action", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BatchJobQueueTimeoutsElRef {
        BatchJobQueueTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for BatchJobQueue {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for BatchJobQueue {}

impl ToListMappable for BatchJobQueue {
    type O = ListRef<BatchJobQueueRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BatchJobQueue_ {
    fn extract_resource_type(&self) -> String {
        "aws_batch_job_queue".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBatchJobQueue {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub priority: PrimField<f64>,
    #[doc = ""]
    pub state: PrimField<String>,
}

impl BuildBatchJobQueue {
    pub fn build(self, stack: &mut Stack) -> BatchJobQueue {
        let out = BatchJobQueue(Rc::new(BatchJobQueue_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BatchJobQueueData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                name: self.name,
                priority: self.priority,
                region: core::default::Default::default(),
                scheduling_policy_arn: core::default::Default::default(),
                state: self.state,
                tags: core::default::Default::default(),
                compute_environment_order: core::default::Default::default(),
                job_state_time_limit_action: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BatchJobQueueRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobQueueRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl BatchJobQueueRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scheduling_policy_arn` after provisioning.\n"]
    pub fn scheduling_policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduling_policy_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `compute_environment_order` after provisioning.\n"]
    pub fn compute_environment_order(&self) -> ListRef<BatchJobQueueComputeEnvironmentOrderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_environment_order", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `job_state_time_limit_action` after provisioning.\n"]
    pub fn job_state_time_limit_action(
        &self,
    ) -> ListRef<BatchJobQueueJobStateTimeLimitActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.job_state_time_limit_action", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BatchJobQueueTimeoutsElRef {
        BatchJobQueueTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BatchJobQueueComputeEnvironmentOrderEl {
    compute_environment: PrimField<String>,
    order: PrimField<f64>,
}

impl BatchJobQueueComputeEnvironmentOrderEl {}

impl ToListMappable for BatchJobQueueComputeEnvironmentOrderEl {
    type O = BlockAssignable<BatchJobQueueComputeEnvironmentOrderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobQueueComputeEnvironmentOrderEl {
    #[doc = ""]
    pub compute_environment: PrimField<String>,
    #[doc = ""]
    pub order: PrimField<f64>,
}

impl BuildBatchJobQueueComputeEnvironmentOrderEl {
    pub fn build(self) -> BatchJobQueueComputeEnvironmentOrderEl {
        BatchJobQueueComputeEnvironmentOrderEl {
            compute_environment: self.compute_environment,
            order: self.order,
        }
    }
}

pub struct BatchJobQueueComputeEnvironmentOrderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobQueueComputeEnvironmentOrderElRef {
    fn new(shared: StackShared, base: String) -> BatchJobQueueComputeEnvironmentOrderElRef {
        BatchJobQueueComputeEnvironmentOrderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobQueueComputeEnvironmentOrderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compute_environment` after provisioning.\n"]
    pub fn compute_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_environment", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobQueueJobStateTimeLimitActionEl {
    action: PrimField<String>,
    max_time_seconds: PrimField<f64>,
    reason: PrimField<String>,
    state: PrimField<String>,
}

impl BatchJobQueueJobStateTimeLimitActionEl {}

impl ToListMappable for BatchJobQueueJobStateTimeLimitActionEl {
    type O = BlockAssignable<BatchJobQueueJobStateTimeLimitActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobQueueJobStateTimeLimitActionEl {
    #[doc = ""]
    pub action: PrimField<String>,
    #[doc = ""]
    pub max_time_seconds: PrimField<f64>,
    #[doc = ""]
    pub reason: PrimField<String>,
    #[doc = ""]
    pub state: PrimField<String>,
}

impl BuildBatchJobQueueJobStateTimeLimitActionEl {
    pub fn build(self) -> BatchJobQueueJobStateTimeLimitActionEl {
        BatchJobQueueJobStateTimeLimitActionEl {
            action: self.action,
            max_time_seconds: self.max_time_seconds,
            reason: self.reason,
            state: self.state,
        }
    }
}

pub struct BatchJobQueueJobStateTimeLimitActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobQueueJobStateTimeLimitActionElRef {
    fn new(shared: StackShared, base: String) -> BatchJobQueueJobStateTimeLimitActionElRef {
        BatchJobQueueJobStateTimeLimitActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobQueueJobStateTimeLimitActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `max_time_seconds` after provisioning.\n"]
    pub fn max_time_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_time_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct BatchJobQueueTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BatchJobQueueTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for BatchJobQueueTimeoutsEl {
    type O = BlockAssignable<BatchJobQueueTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchJobQueueTimeoutsEl {}

impl BuildBatchJobQueueTimeoutsEl {
    pub fn build(self) -> BatchJobQueueTimeoutsEl {
        BatchJobQueueTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BatchJobQueueTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BatchJobQueueTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BatchJobQueueTimeoutsElRef {
        BatchJobQueueTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchJobQueueTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchJobQueueDynamic {
    compute_environment_order: Option<DynamicBlock<BatchJobQueueComputeEnvironmentOrderEl>>,
    job_state_time_limit_action: Option<DynamicBlock<BatchJobQueueJobStateTimeLimitActionEl>>,
}
