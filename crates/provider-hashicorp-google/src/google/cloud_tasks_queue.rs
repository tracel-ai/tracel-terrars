use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudTasksQueueData {
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
    desired_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_engine_routing_override: Option<Vec<CloudTasksQueueAppEngineRoutingOverrideEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_target: Option<Vec<CloudTasksQueueHttpTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limits: Option<Vec<CloudTasksQueueRateLimitsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_config: Option<Vec<CloudTasksQueueRetryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stackdriver_logging_config: Option<Vec<CloudTasksQueueStackdriverLoggingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudTasksQueueTimeoutsEl>,
    dynamic: CloudTasksQueueDynamic,
}
struct CloudTasksQueue_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudTasksQueueData>,
}
#[derive(Clone)]
pub struct CloudTasksQueue(Rc<CloudTasksQueue_>);
impl CloudTasksQueue {
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
    #[doc = "Set the field `desired_state`.\nThe desired state of the queue. Use this to pause and resume the queue.\n\n* RUNNING: The queue is running. Tasks can be dispatched.\n* PAUSED: The queue is paused. Tasks are not dispatched but can be added to the queue. Default value: \"RUNNING\" Possible values: [\"RUNNING\", \"PAUSED\"]"]
    pub fn set_desired_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().desired_state = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `app_engine_routing_override`.\n"]
    pub fn set_app_engine_routing_override(
        self,
        v: impl Into<BlockAssignable<CloudTasksQueueAppEngineRoutingOverrideEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().app_engine_routing_override = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.app_engine_routing_override = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_target`.\n"]
    pub fn set_http_target(
        self,
        v: impl Into<BlockAssignable<CloudTasksQueueHttpTargetEl>>,
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
    #[doc = "Set the field `rate_limits`.\n"]
    pub fn set_rate_limits(
        self,
        v: impl Into<BlockAssignable<CloudTasksQueueRateLimitsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rate_limits = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rate_limits = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `retry_config`.\n"]
    pub fn set_retry_config(
        self,
        v: impl Into<BlockAssignable<CloudTasksQueueRetryConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stackdriver_logging_config`.\n"]
    pub fn set_stackdriver_logging_config(
        self,
        v: impl Into<BlockAssignable<CloudTasksQueueStackdriverLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stackdriver_logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stackdriver_logging_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudTasksQueueTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nThe desired state of the queue. Use this to pause and resume the queue.\n\n* RUNNING: The queue is running. Tasks can be dispatched.\n* PAUSED: The queue is paused. Tasks are not dispatched but can be added to the queue. Default value: \"RUNNING\" Possible values: [\"RUNNING\", \"PAUSED\"]"]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the queue"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe queue name."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the queue."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app_engine_routing_override` after provisioning.\n"]
    pub fn app_engine_routing_override(
        &self,
    ) -> ListRef<CloudTasksQueueAppEngineRoutingOverrideElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_engine_routing_override", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_target` after provisioning.\n"]
    pub fn http_target(&self) -> ListRef<CloudTasksQueueHttpTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rate_limits` after provisioning.\n"]
    pub fn rate_limits(&self) -> ListRef<CloudTasksQueueRateLimitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limits", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_config` after provisioning.\n"]
    pub fn retry_config(&self) -> ListRef<CloudTasksQueueRetryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stackdriver_logging_config` after provisioning.\n"]
    pub fn stackdriver_logging_config(
        &self,
    ) -> ListRef<CloudTasksQueueStackdriverLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stackdriver_logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudTasksQueueTimeoutsElRef {
        CloudTasksQueueTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudTasksQueue {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudTasksQueue {}
impl ToListMappable for CloudTasksQueue {
    type O = ListRef<CloudTasksQueueRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudTasksQueue_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_tasks_queue".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudTasksQueue {
    pub tf_id: String,
    #[doc = "The location of the queue"]
    pub location: PrimField<String>,
    #[doc = "The queue name."]
    pub name: PrimField<String>,
}
impl BuildCloudTasksQueue {
    pub fn build(self, stack: &mut Stack) -> CloudTasksQueue {
        let out = CloudTasksQueue(Rc::new(CloudTasksQueue_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudTasksQueueData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                app_engine_routing_override: core::default::Default::default(),
                http_target: core::default::Default::default(),
                rate_limits: core::default::Default::default(),
                retry_config: core::default::Default::default(),
                stackdriver_logging_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudTasksQueueRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudTasksQueueRef {
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
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nThe desired state of the queue. Use this to pause and resume the queue.\n\n* RUNNING: The queue is running. Tasks can be dispatched.\n* PAUSED: The queue is paused. Tasks are not dispatched but can be added to the queue. Default value: \"RUNNING\" Possible values: [\"RUNNING\", \"PAUSED\"]"]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the queue"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe queue name."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the queue."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app_engine_routing_override` after provisioning.\n"]
    pub fn app_engine_routing_override(
        &self,
    ) -> ListRef<CloudTasksQueueAppEngineRoutingOverrideElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_engine_routing_override", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `http_target` after provisioning.\n"]
    pub fn http_target(&self) -> ListRef<CloudTasksQueueHttpTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rate_limits` after provisioning.\n"]
    pub fn rate_limits(&self) -> ListRef<CloudTasksQueueRateLimitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limits", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_config` after provisioning.\n"]
    pub fn retry_config(&self) -> ListRef<CloudTasksQueueRetryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stackdriver_logging_config` after provisioning.\n"]
    pub fn stackdriver_logging_config(
        &self,
    ) -> ListRef<CloudTasksQueueStackdriverLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stackdriver_logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudTasksQueueTimeoutsElRef {
        CloudTasksQueueTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudTasksQueueAppEngineRoutingOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl CloudTasksQueueAppEngineRoutingOverrideEl {
    #[doc = "Set the field `instance`.\nApp instance.\n\nBy default, the task is sent to an instance which is available when the task is attempted."]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nApp service.\n\nBy default, the task is sent to the service which is the default service when the task is attempted."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nApp version.\n\nBy default, the task is sent to the version which is the default version when the task is attempted."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for CloudTasksQueueAppEngineRoutingOverrideEl {
    type O = BlockAssignable<CloudTasksQueueAppEngineRoutingOverrideEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueAppEngineRoutingOverrideEl {}
impl BuildCloudTasksQueueAppEngineRoutingOverrideEl {
    pub fn build(self) -> CloudTasksQueueAppEngineRoutingOverrideEl {
        CloudTasksQueueAppEngineRoutingOverrideEl {
            instance: core::default::Default::default(),
            service: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct CloudTasksQueueAppEngineRoutingOverrideElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueAppEngineRoutingOverrideElRef {
    fn new(shared: StackShared, base: String) -> CloudTasksQueueAppEngineRoutingOverrideElRef {
        CloudTasksQueueAppEngineRoutingOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueAppEngineRoutingOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe host that the task is sent to."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nApp instance.\n\nBy default, the task is sent to an instance which is available when the task is attempted."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nApp service.\n\nBy default, the task is sent to the service which is the default service when the task is attempted."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nApp version.\n\nBy default, the task is sent to the version which is the default version when the task is attempted."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudTasksQueueHttpTargetElHeaderOverridesElHeaderEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl CloudTasksQueueHttpTargetElHeaderOverridesElHeaderEl {}
impl ToListMappable for CloudTasksQueueHttpTargetElHeaderOverridesElHeaderEl {
    type O = BlockAssignable<CloudTasksQueueHttpTargetElHeaderOverridesElHeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueHttpTargetElHeaderOverridesElHeaderEl {
    #[doc = "The Key of the header."]
    pub key: PrimField<String>,
    #[doc = "The Value of the header."]
    pub value: PrimField<String>,
}
impl BuildCloudTasksQueueHttpTargetElHeaderOverridesElHeaderEl {
    pub fn build(self) -> CloudTasksQueueHttpTargetElHeaderOverridesElHeaderEl {
        CloudTasksQueueHttpTargetElHeaderOverridesElHeaderEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct CloudTasksQueueHttpTargetElHeaderOverridesElHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueHttpTargetElHeaderOverridesElHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudTasksQueueHttpTargetElHeaderOverridesElHeaderElRef {
        CloudTasksQueueHttpTargetElHeaderOverridesElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueHttpTargetElHeaderOverridesElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe Key of the header."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe Value of the header."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudTasksQueueHttpTargetElHeaderOverridesElDynamic {
    header: Option<DynamicBlock<CloudTasksQueueHttpTargetElHeaderOverridesElHeaderEl>>,
}
#[derive(Serialize)]
pub struct CloudTasksQueueHttpTargetElHeaderOverridesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<Vec<CloudTasksQueueHttpTargetElHeaderOverridesElHeaderEl>>,
    dynamic: CloudTasksQueueHttpTargetElHeaderOverridesElDynamic,
}
impl CloudTasksQueueHttpTargetElHeaderOverridesEl {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<BlockAssignable<CloudTasksQueueHttpTargetElHeaderOverridesElHeaderEl>>,
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
impl ToListMappable for CloudTasksQueueHttpTargetElHeaderOverridesEl {
    type O = BlockAssignable<CloudTasksQueueHttpTargetElHeaderOverridesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueHttpTargetElHeaderOverridesEl {}
impl BuildCloudTasksQueueHttpTargetElHeaderOverridesEl {
    pub fn build(self) -> CloudTasksQueueHttpTargetElHeaderOverridesEl {
        CloudTasksQueueHttpTargetElHeaderOverridesEl {
            header: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudTasksQueueHttpTargetElHeaderOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueHttpTargetElHeaderOverridesElRef {
    fn new(shared: StackShared, base: String) -> CloudTasksQueueHttpTargetElHeaderOverridesElRef {
        CloudTasksQueueHttpTargetElHeaderOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueHttpTargetElHeaderOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> ListRef<CloudTasksQueueHttpTargetElHeaderOverridesElHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudTasksQueueHttpTargetElOauthTokenEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    service_account_email: PrimField<String>,
}
impl CloudTasksQueueHttpTargetElOauthTokenEl {
    #[doc = "Set the field `scope`.\nOAuth scope to be used for generating OAuth access token.\nIf not specified, \"https://www.googleapis.com/auth/cloud-platform\" will be used."]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
}
impl ToListMappable for CloudTasksQueueHttpTargetElOauthTokenEl {
    type O = BlockAssignable<CloudTasksQueueHttpTargetElOauthTokenEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueHttpTargetElOauthTokenEl {
    #[doc = "Service account email to be used for generating OAuth token.\nThe service account must be within the same project as the queue.\nThe caller must have iam.serviceAccounts.actAs permission for the service account."]
    pub service_account_email: PrimField<String>,
}
impl BuildCloudTasksQueueHttpTargetElOauthTokenEl {
    pub fn build(self) -> CloudTasksQueueHttpTargetElOauthTokenEl {
        CloudTasksQueueHttpTargetElOauthTokenEl {
            scope: core::default::Default::default(),
            service_account_email: self.service_account_email,
        }
    }
}
pub struct CloudTasksQueueHttpTargetElOauthTokenElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueHttpTargetElOauthTokenElRef {
    fn new(shared: StackShared, base: String) -> CloudTasksQueueHttpTargetElOauthTokenElRef {
        CloudTasksQueueHttpTargetElOauthTokenElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueHttpTargetElOauthTokenElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nOAuth scope to be used for generating OAuth access token.\nIf not specified, \"https://www.googleapis.com/auth/cloud-platform\" will be used."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nService account email to be used for generating OAuth token.\nThe service account must be within the same project as the queue.\nThe caller must have iam.serviceAccounts.actAs permission for the service account."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudTasksQueueHttpTargetElOidcTokenEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audience: Option<PrimField<String>>,
    service_account_email: PrimField<String>,
}
impl CloudTasksQueueHttpTargetElOidcTokenEl {
    #[doc = "Set the field `audience`.\nAudience to be used when generating OIDC token. If not specified, the URI specified in target will be used."]
    pub fn set_audience(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audience = Some(v.into());
        self
    }
}
impl ToListMappable for CloudTasksQueueHttpTargetElOidcTokenEl {
    type O = BlockAssignable<CloudTasksQueueHttpTargetElOidcTokenEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueHttpTargetElOidcTokenEl {
    #[doc = "Service account email to be used for generating OIDC token.\nThe service account must be within the same project as the queue.\nThe caller must have iam.serviceAccounts.actAs permission for the service account."]
    pub service_account_email: PrimField<String>,
}
impl BuildCloudTasksQueueHttpTargetElOidcTokenEl {
    pub fn build(self) -> CloudTasksQueueHttpTargetElOidcTokenEl {
        CloudTasksQueueHttpTargetElOidcTokenEl {
            audience: core::default::Default::default(),
            service_account_email: self.service_account_email,
        }
    }
}
pub struct CloudTasksQueueHttpTargetElOidcTokenElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueHttpTargetElOidcTokenElRef {
    fn new(shared: StackShared, base: String) -> CloudTasksQueueHttpTargetElOidcTokenElRef {
        CloudTasksQueueHttpTargetElOidcTokenElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueHttpTargetElOidcTokenElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audience` after provisioning.\nAudience to be used when generating OIDC token. If not specified, the URI specified in target will be used."]
    pub fn audience(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nService account email to be used for generating OIDC token.\nThe service account must be within the same project as the queue.\nThe caller must have iam.serviceAccounts.actAs permission for the service account."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudTasksQueueHttpTargetElUriOverrideElPathOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}
impl CloudTasksQueueHttpTargetElUriOverrideElPathOverrideEl {
    #[doc = "Set the field `path`.\nThe URI path (e.g., /users/1234). Default is an empty string."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for CloudTasksQueueHttpTargetElUriOverrideElPathOverrideEl {
    type O = BlockAssignable<CloudTasksQueueHttpTargetElUriOverrideElPathOverrideEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueHttpTargetElUriOverrideElPathOverrideEl {}
impl BuildCloudTasksQueueHttpTargetElUriOverrideElPathOverrideEl {
    pub fn build(self) -> CloudTasksQueueHttpTargetElUriOverrideElPathOverrideEl {
        CloudTasksQueueHttpTargetElUriOverrideElPathOverrideEl {
            path: core::default::Default::default(),
        }
    }
}
pub struct CloudTasksQueueHttpTargetElUriOverrideElPathOverrideElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueHttpTargetElUriOverrideElPathOverrideElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudTasksQueueHttpTargetElUriOverrideElPathOverrideElRef {
        CloudTasksQueueHttpTargetElUriOverrideElPathOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueHttpTargetElUriOverrideElPathOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe URI path (e.g., /users/1234). Default is an empty string."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    query_params: Option<PrimField<String>>,
}
impl CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideEl {
    #[doc = "Set the field `query_params`.\nThe query parameters (e.g., qparam1=123&qparam2=456). Default is an empty string."]
    pub fn set_query_params(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_params = Some(v.into());
        self
    }
}
impl ToListMappable for CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideEl {
    type O = BlockAssignable<CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueHttpTargetElUriOverrideElQueryOverrideEl {}
impl BuildCloudTasksQueueHttpTargetElUriOverrideElQueryOverrideEl {
    pub fn build(self) -> CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideEl {
        CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideEl {
            query_params: core::default::Default::default(),
        }
    }
}
pub struct CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideElRef {
        CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `query_params` after provisioning.\nThe query parameters (e.g., qparam1=123&qparam2=456). Default is an empty string."]
    pub fn query_params(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_params", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudTasksQueueHttpTargetElUriOverrideElDynamic {
    path_override: Option<DynamicBlock<CloudTasksQueueHttpTargetElUriOverrideElPathOverrideEl>>,
    query_override: Option<DynamicBlock<CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideEl>>,
}
#[derive(Serialize)]
pub struct CloudTasksQueueHttpTargetElUriOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri_override_enforce_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_override: Option<Vec<CloudTasksQueueHttpTargetElUriOverrideElPathOverrideEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_override: Option<Vec<CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideEl>>,
    dynamic: CloudTasksQueueHttpTargetElUriOverrideElDynamic,
}
impl CloudTasksQueueHttpTargetElUriOverrideEl {
    #[doc = "Set the field `host`.\nHost override.\n\nWhen specified, replaces the host part of the task URL.\nFor example, if the task URL is \"https://www.google.com\", and host value\nis set to \"example.net\", the overridden URI will be changed to \"https://example.net\".\nHost value cannot be an empty string (INVALID_ARGUMENT)."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nPort override.\n\nWhen specified, replaces the port part of the task URI.\nFor instance, for a URI http://www.google.com/foo and port=123, the overridden URI becomes http://www.google.com:123/foo.\nNote that the port value must be a positive integer.\nSetting the port to 0 (Zero) clears the URI port."]
    pub fn set_port(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `scheme`.\nScheme override.\n\nWhen specified, the task URI scheme is replaced by the provided value (HTTP or HTTPS). Possible values: [\"HTTP\", \"HTTPS\"]"]
    pub fn set_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `uri_override_enforce_mode`.\nURI Override Enforce Mode\n\nWhen specified, determines the Target UriOverride mode. If not specified, it defaults to ALWAYS. Possible values: [\"ALWAYS\", \"IF_NOT_EXISTS\"]"]
    pub fn set_uri_override_enforce_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri_override_enforce_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `path_override`.\n"]
    pub fn set_path_override(
        mut self,
        v: impl Into<BlockAssignable<CloudTasksQueueHttpTargetElUriOverrideElPathOverrideEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.path_override = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.path_override = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query_override`.\n"]
    pub fn set_query_override(
        mut self,
        v: impl Into<BlockAssignable<CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_override = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_override = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudTasksQueueHttpTargetElUriOverrideEl {
    type O = BlockAssignable<CloudTasksQueueHttpTargetElUriOverrideEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueHttpTargetElUriOverrideEl {}
impl BuildCloudTasksQueueHttpTargetElUriOverrideEl {
    pub fn build(self) -> CloudTasksQueueHttpTargetElUriOverrideEl {
        CloudTasksQueueHttpTargetElUriOverrideEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
            scheme: core::default::Default::default(),
            uri_override_enforce_mode: core::default::Default::default(),
            path_override: core::default::Default::default(),
            query_override: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudTasksQueueHttpTargetElUriOverrideElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueHttpTargetElUriOverrideElRef {
    fn new(shared: StackShared, base: String) -> CloudTasksQueueHttpTargetElUriOverrideElRef {
        CloudTasksQueueHttpTargetElUriOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueHttpTargetElUriOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nHost override.\n\nWhen specified, replaces the host part of the task URL.\nFor example, if the task URL is \"https://www.google.com\", and host value\nis set to \"example.net\", the overridden URI will be changed to \"https://example.net\".\nHost value cannot be an empty string (INVALID_ARGUMENT)."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort override.\n\nWhen specified, replaces the port part of the task URI.\nFor instance, for a URI http://www.google.com/foo and port=123, the overridden URI becomes http://www.google.com:123/foo.\nNote that the port value must be a positive integer.\nSetting the port to 0 (Zero) clears the URI port."]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `scheme` after provisioning.\nScheme override.\n\nWhen specified, the task URI scheme is replaced by the provided value (HTTP or HTTPS). Possible values: [\"HTTP\", \"HTTPS\"]"]
    pub fn scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }
    #[doc = "Get a reference to the value of field `uri_override_enforce_mode` after provisioning.\nURI Override Enforce Mode\n\nWhen specified, determines the Target UriOverride mode. If not specified, it defaults to ALWAYS. Possible values: [\"ALWAYS\", \"IF_NOT_EXISTS\"]"]
    pub fn uri_override_enforce_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uri_override_enforce_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_override` after provisioning.\n"]
    pub fn path_override(
        &self,
    ) -> ListRef<CloudTasksQueueHttpTargetElUriOverrideElPathOverrideElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.path_override", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_override` after provisioning.\n"]
    pub fn query_override(
        &self,
    ) -> ListRef<CloudTasksQueueHttpTargetElUriOverrideElQueryOverrideElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_override", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudTasksQueueHttpTargetElDynamic {
    header_overrides: Option<DynamicBlock<CloudTasksQueueHttpTargetElHeaderOverridesEl>>,
    oauth_token: Option<DynamicBlock<CloudTasksQueueHttpTargetElOauthTokenEl>>,
    oidc_token: Option<DynamicBlock<CloudTasksQueueHttpTargetElOidcTokenEl>>,
    uri_override: Option<DynamicBlock<CloudTasksQueueHttpTargetElUriOverrideEl>>,
}
#[derive(Serialize)]
pub struct CloudTasksQueueHttpTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_overrides: Option<Vec<CloudTasksQueueHttpTargetElHeaderOverridesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_token: Option<Vec<CloudTasksQueueHttpTargetElOauthTokenEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc_token: Option<Vec<CloudTasksQueueHttpTargetElOidcTokenEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri_override: Option<Vec<CloudTasksQueueHttpTargetElUriOverrideEl>>,
    dynamic: CloudTasksQueueHttpTargetElDynamic,
}
impl CloudTasksQueueHttpTargetEl {
    #[doc = "Set the field `http_method`.\nThe HTTP method to use for the request.\n\nWhen specified, it overrides HttpRequest for the task.\nNote that if the value is set to GET the body of the task will be ignored at execution time. Possible values: [\"HTTP_METHOD_UNSPECIFIED\", \"POST\", \"GET\", \"HEAD\", \"PUT\", \"DELETE\", \"PATCH\", \"OPTIONS\"]"]
    pub fn set_http_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_method = Some(v.into());
        self
    }
    #[doc = "Set the field `header_overrides`.\n"]
    pub fn set_header_overrides(
        mut self,
        v: impl Into<BlockAssignable<CloudTasksQueueHttpTargetElHeaderOverridesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_overrides = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_overrides = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oauth_token`.\n"]
    pub fn set_oauth_token(
        mut self,
        v: impl Into<BlockAssignable<CloudTasksQueueHttpTargetElOauthTokenEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_token = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_token = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oidc_token`.\n"]
    pub fn set_oidc_token(
        mut self,
        v: impl Into<BlockAssignable<CloudTasksQueueHttpTargetElOidcTokenEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oidc_token = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oidc_token = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `uri_override`.\n"]
    pub fn set_uri_override(
        mut self,
        v: impl Into<BlockAssignable<CloudTasksQueueHttpTargetElUriOverrideEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.uri_override = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.uri_override = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudTasksQueueHttpTargetEl {
    type O = BlockAssignable<CloudTasksQueueHttpTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueHttpTargetEl {}
impl BuildCloudTasksQueueHttpTargetEl {
    pub fn build(self) -> CloudTasksQueueHttpTargetEl {
        CloudTasksQueueHttpTargetEl {
            http_method: core::default::Default::default(),
            header_overrides: core::default::Default::default(),
            oauth_token: core::default::Default::default(),
            oidc_token: core::default::Default::default(),
            uri_override: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudTasksQueueHttpTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueHttpTargetElRef {
    fn new(shared: StackShared, base: String) -> CloudTasksQueueHttpTargetElRef {
        CloudTasksQueueHttpTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueHttpTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_method` after provisioning.\nThe HTTP method to use for the request.\n\nWhen specified, it overrides HttpRequest for the task.\nNote that if the value is set to GET the body of the task will be ignored at execution time. Possible values: [\"HTTP_METHOD_UNSPECIFIED\", \"POST\", \"GET\", \"HEAD\", \"PUT\", \"DELETE\", \"PATCH\", \"OPTIONS\"]"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }
    #[doc = "Get a reference to the value of field `header_overrides` after provisioning.\n"]
    pub fn header_overrides(&self) -> ListRef<CloudTasksQueueHttpTargetElHeaderOverridesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_overrides", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_token` after provisioning.\n"]
    pub fn oauth_token(&self) -> ListRef<CloudTasksQueueHttpTargetElOauthTokenElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_token", self.base))
    }
    #[doc = "Get a reference to the value of field `oidc_token` after provisioning.\n"]
    pub fn oidc_token(&self) -> ListRef<CloudTasksQueueHttpTargetElOidcTokenElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oidc_token", self.base))
    }
    #[doc = "Get a reference to the value of field `uri_override` after provisioning.\n"]
    pub fn uri_override(&self) -> ListRef<CloudTasksQueueHttpTargetElUriOverrideElRef> {
        ListRef::new(self.shared().clone(), format!("{}.uri_override", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudTasksQueueRateLimitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_dispatches: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_dispatches_per_second: Option<PrimField<f64>>,
}
impl CloudTasksQueueRateLimitsEl {
    #[doc = "Set the field `max_concurrent_dispatches`.\nThe maximum number of concurrent tasks that Cloud Tasks allows to\nbe dispatched for this queue. After this threshold has been\nreached, Cloud Tasks stops dispatching tasks until the number of\nconcurrent requests decreases."]
    pub fn set_max_concurrent_dispatches(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_dispatches = Some(v.into());
        self
    }
    #[doc = "Set the field `max_dispatches_per_second`.\nThe maximum rate at which tasks are dispatched from this queue.\n\nIf unspecified when the queue is created, Cloud Tasks will pick the default."]
    pub fn set_max_dispatches_per_second(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_dispatches_per_second = Some(v.into());
        self
    }
}
impl ToListMappable for CloudTasksQueueRateLimitsEl {
    type O = BlockAssignable<CloudTasksQueueRateLimitsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueRateLimitsEl {}
impl BuildCloudTasksQueueRateLimitsEl {
    pub fn build(self) -> CloudTasksQueueRateLimitsEl {
        CloudTasksQueueRateLimitsEl {
            max_concurrent_dispatches: core::default::Default::default(),
            max_dispatches_per_second: core::default::Default::default(),
        }
    }
}
pub struct CloudTasksQueueRateLimitsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueRateLimitsElRef {
    fn new(shared: StackShared, base: String) -> CloudTasksQueueRateLimitsElRef {
        CloudTasksQueueRateLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueRateLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_burst_size` after provisioning.\nThe max burst size.\n\nMax burst size limits how fast tasks in queue are processed when many tasks are\nin the queue and the rate is high. This field allows the queue to have a high\nrate so processing starts shortly after a task is enqueued, but still limits\nresource usage when many tasks are enqueued in a short period of time."]
    pub fn max_burst_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_burst_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_concurrent_dispatches` after provisioning.\nThe maximum number of concurrent tasks that Cloud Tasks allows to\nbe dispatched for this queue. After this threshold has been\nreached, Cloud Tasks stops dispatching tasks until the number of\nconcurrent requests decreases."]
    pub fn max_concurrent_dispatches(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_dispatches", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_dispatches_per_second` after provisioning.\nThe maximum rate at which tasks are dispatched from this queue.\n\nIf unspecified when the queue is created, Cloud Tasks will pick the default."]
    pub fn max_dispatches_per_second(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_dispatches_per_second", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudTasksQueueRetryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_backoff: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_doublings: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retry_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_backoff: Option<PrimField<String>>,
}
impl CloudTasksQueueRetryConfigEl {
    #[doc = "Set the field `max_attempts`.\nNumber of attempts per task.\n\nCloud Tasks will attempt the task maxAttempts times (that is, if\nthe first attempt fails, then there will be maxAttempts - 1\nretries). Must be >= -1.\n\nIf unspecified when the queue is created, Cloud Tasks will pick\nthe default.\n\n-1 indicates unlimited attempts."]
    pub fn set_max_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_attempts = Some(v.into());
        self
    }
    #[doc = "Set the field `max_backoff`.\nA task will be scheduled for retry between minBackoff and\nmaxBackoff duration after it fails, if the queue's RetryConfig\nspecifies that the task should be retried."]
    pub fn set_max_backoff(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_backoff = Some(v.into());
        self
    }
    #[doc = "Set the field `max_doublings`.\nThe time between retries will double maxDoublings times.\n\nA task's retry interval starts at minBackoff, then doubles maxDoublings times,\nthen increases linearly, and finally retries retries at intervals of maxBackoff\nup to maxAttempts times."]
    pub fn set_max_doublings(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_doublings = Some(v.into());
        self
    }
    #[doc = "Set the field `max_retry_duration`.\nIf positive, maxRetryDuration specifies the time limit for\nretrying a failed task, measured from when the task was first\nattempted. Once maxRetryDuration time has passed and the task has\nbeen attempted maxAttempts times, no further attempts will be\nmade and the task will be deleted.\n\nIf zero, then the task age is unlimited."]
    pub fn set_max_retry_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_retry_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `min_backoff`.\nA task will be scheduled for retry between minBackoff and\nmaxBackoff duration after it fails, if the queue's RetryConfig\nspecifies that the task should be retried."]
    pub fn set_min_backoff(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_backoff = Some(v.into());
        self
    }
}
impl ToListMappable for CloudTasksQueueRetryConfigEl {
    type O = BlockAssignable<CloudTasksQueueRetryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueRetryConfigEl {}
impl BuildCloudTasksQueueRetryConfigEl {
    pub fn build(self) -> CloudTasksQueueRetryConfigEl {
        CloudTasksQueueRetryConfigEl {
            max_attempts: core::default::Default::default(),
            max_backoff: core::default::Default::default(),
            max_doublings: core::default::Default::default(),
            max_retry_duration: core::default::Default::default(),
            min_backoff: core::default::Default::default(),
        }
    }
}
pub struct CloudTasksQueueRetryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueRetryConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudTasksQueueRetryConfigElRef {
        CloudTasksQueueRetryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueRetryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_attempts` after provisioning.\nNumber of attempts per task.\n\nCloud Tasks will attempt the task maxAttempts times (that is, if\nthe first attempt fails, then there will be maxAttempts - 1\nretries). Must be >= -1.\n\nIf unspecified when the queue is created, Cloud Tasks will pick\nthe default.\n\n-1 indicates unlimited attempts."]
    pub fn max_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }
    #[doc = "Get a reference to the value of field `max_backoff` after provisioning.\nA task will be scheduled for retry between minBackoff and\nmaxBackoff duration after it fails, if the queue's RetryConfig\nspecifies that the task should be retried."]
    pub fn max_backoff(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_backoff", self.base))
    }
    #[doc = "Get a reference to the value of field `max_doublings` after provisioning.\nThe time between retries will double maxDoublings times.\n\nA task's retry interval starts at minBackoff, then doubles maxDoublings times,\nthen increases linearly, and finally retries retries at intervals of maxBackoff\nup to maxAttempts times."]
    pub fn max_doublings(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_doublings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_retry_duration` after provisioning.\nIf positive, maxRetryDuration specifies the time limit for\nretrying a failed task, measured from when the task was first\nattempted. Once maxRetryDuration time has passed and the task has\nbeen attempted maxAttempts times, no further attempts will be\nmade and the task will be deleted.\n\nIf zero, then the task age is unlimited."]
    pub fn max_retry_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_retry_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_backoff` after provisioning.\nA task will be scheduled for retry between minBackoff and\nmaxBackoff duration after it fails, if the queue's RetryConfig\nspecifies that the task should be retried."]
    pub fn min_backoff(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_backoff", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudTasksQueueStackdriverLoggingConfigEl {
    sampling_ratio: PrimField<f64>,
}
impl CloudTasksQueueStackdriverLoggingConfigEl {}
impl ToListMappable for CloudTasksQueueStackdriverLoggingConfigEl {
    type O = BlockAssignable<CloudTasksQueueStackdriverLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueStackdriverLoggingConfigEl {
    #[doc = "Specifies the fraction of operations to write to Stackdriver Logging.\nThis field may contain any value between 0.0 and 1.0, inclusive. 0.0 is the\ndefault and means that no operations are logged."]
    pub sampling_ratio: PrimField<f64>,
}
impl BuildCloudTasksQueueStackdriverLoggingConfigEl {
    pub fn build(self) -> CloudTasksQueueStackdriverLoggingConfigEl {
        CloudTasksQueueStackdriverLoggingConfigEl {
            sampling_ratio: self.sampling_ratio,
        }
    }
}
pub struct CloudTasksQueueStackdriverLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueStackdriverLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudTasksQueueStackdriverLoggingConfigElRef {
        CloudTasksQueueStackdriverLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueStackdriverLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sampling_ratio` after provisioning.\nSpecifies the fraction of operations to write to Stackdriver Logging.\nThis field may contain any value between 0.0 and 1.0, inclusive. 0.0 is the\ndefault and means that no operations are logged."]
    pub fn sampling_ratio(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sampling_ratio", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudTasksQueueTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudTasksQueueTimeoutsEl {
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
impl ToListMappable for CloudTasksQueueTimeoutsEl {
    type O = BlockAssignable<CloudTasksQueueTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudTasksQueueTimeoutsEl {}
impl BuildCloudTasksQueueTimeoutsEl {
    pub fn build(self) -> CloudTasksQueueTimeoutsEl {
        CloudTasksQueueTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudTasksQueueTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudTasksQueueTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudTasksQueueTimeoutsElRef {
        CloudTasksQueueTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudTasksQueueTimeoutsElRef {
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
struct CloudTasksQueueDynamic {
    app_engine_routing_override: Option<DynamicBlock<CloudTasksQueueAppEngineRoutingOverrideEl>>,
    http_target: Option<DynamicBlock<CloudTasksQueueHttpTargetEl>>,
    rate_limits: Option<DynamicBlock<CloudTasksQueueRateLimitsEl>>,
    retry_config: Option<DynamicBlock<CloudTasksQueueRetryConfigEl>>,
    stackdriver_logging_config: Option<DynamicBlock<CloudTasksQueueStackdriverLoggingConfigEl>>,
}
