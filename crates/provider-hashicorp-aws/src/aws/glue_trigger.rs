use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GlueTriggerData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_on_creation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<GlueTriggerActionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_batching_condition: Option<Vec<GlueTriggerEventBatchingConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predicate: Option<Vec<GlueTriggerPredicateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GlueTriggerTimeoutsEl>,
    dynamic: GlueTriggerDynamic,
}
struct GlueTrigger_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlueTriggerData>,
}
#[derive(Clone)]
pub struct GlueTrigger(Rc<GlueTrigger_>);
impl GlueTrigger {
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
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
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
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `start_on_creation`.\n"]
    pub fn set_start_on_creation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_on_creation = Some(v.into());
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
    #[doc = "Set the field `workflow_name`.\n"]
    pub fn set_workflow_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workflow_name = Some(v.into());
        self
    }
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(self, v: impl Into<BlockAssignable<GlueTriggerActionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `event_batching_condition`.\n"]
    pub fn set_event_batching_condition(
        self,
        v: impl Into<BlockAssignable<GlueTriggerEventBatchingConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_batching_condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_batching_condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `predicate`.\n"]
    pub fn set_predicate(self, v: impl Into<BlockAssignable<GlueTriggerPredicateEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().predicate = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.predicate = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GlueTriggerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_on_creation` after provisioning.\n"]
    pub fn start_on_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_on_creation", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workflow_name` after provisioning.\n"]
    pub fn workflow_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workflow_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> ListRef<GlueTriggerActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_batching_condition` after provisioning.\n"]
    pub fn event_batching_condition(&self) -> ListRef<GlueTriggerEventBatchingConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_batching_condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `predicate` after provisioning.\n"]
    pub fn predicate(&self) -> ListRef<GlueTriggerPredicateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predicate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlueTriggerTimeoutsElRef {
        GlueTriggerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for GlueTrigger {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GlueTrigger {}
impl ToListMappable for GlueTrigger {
    type O = ListRef<GlueTriggerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GlueTrigger_ {
    fn extract_resource_type(&self) -> String {
        "aws_glue_trigger".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGlueTrigger {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildGlueTrigger {
    pub fn build(self, stack: &mut Stack) -> GlueTrigger {
        let out = GlueTrigger(Rc::new(GlueTrigger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlueTriggerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                schedule: core::default::Default::default(),
                start_on_creation: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                workflow_name: core::default::Default::default(),
                actions: core::default::Default::default(),
                event_batching_condition: core::default::Default::default(),
                predicate: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GlueTriggerRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueTriggerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GlueTriggerRef {
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
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `start_on_creation` after provisioning.\n"]
    pub fn start_on_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_on_creation", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workflow_name` after provisioning.\n"]
    pub fn workflow_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workflow_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> ListRef<GlueTriggerActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_batching_condition` after provisioning.\n"]
    pub fn event_batching_condition(&self) -> ListRef<GlueTriggerEventBatchingConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_batching_condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `predicate` after provisioning.\n"]
    pub fn predicate(&self) -> ListRef<GlueTriggerPredicateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predicate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlueTriggerTimeoutsElRef {
        GlueTriggerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GlueTriggerActionsElNotificationPropertyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_delay_after: Option<PrimField<f64>>,
}
impl GlueTriggerActionsElNotificationPropertyEl {
    #[doc = "Set the field `notify_delay_after`.\n"]
    pub fn set_notify_delay_after(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.notify_delay_after = Some(v.into());
        self
    }
}
impl ToListMappable for GlueTriggerActionsElNotificationPropertyEl {
    type O = BlockAssignable<GlueTriggerActionsElNotificationPropertyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueTriggerActionsElNotificationPropertyEl {}
impl BuildGlueTriggerActionsElNotificationPropertyEl {
    pub fn build(self) -> GlueTriggerActionsElNotificationPropertyEl {
        GlueTriggerActionsElNotificationPropertyEl {
            notify_delay_after: core::default::Default::default(),
        }
    }
}
pub struct GlueTriggerActionsElNotificationPropertyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueTriggerActionsElNotificationPropertyElRef {
    fn new(shared: StackShared, base: String) -> GlueTriggerActionsElNotificationPropertyElRef {
        GlueTriggerActionsElNotificationPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueTriggerActionsElNotificationPropertyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `notify_delay_after` after provisioning.\n"]
    pub fn notify_delay_after(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notify_delay_after", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GlueTriggerActionsElDynamic {
    notification_property: Option<DynamicBlock<GlueTriggerActionsElNotificationPropertyEl>>,
}
#[derive(Serialize)]
pub struct GlueTriggerActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crawler_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_configuration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_property: Option<Vec<GlueTriggerActionsElNotificationPropertyEl>>,
    dynamic: GlueTriggerActionsElDynamic,
}
impl GlueTriggerActionsEl {
    #[doc = "Set the field `arguments`.\n"]
    pub fn set_arguments(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.arguments = Some(v.into());
        self
    }
    #[doc = "Set the field `crawler_name`.\n"]
    pub fn set_crawler_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.crawler_name = Some(v.into());
        self
    }
    #[doc = "Set the field `job_name`.\n"]
    pub fn set_job_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.job_name = Some(v.into());
        self
    }
    #[doc = "Set the field `security_configuration`.\n"]
    pub fn set_security_configuration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `notification_property`.\n"]
    pub fn set_notification_property(
        mut self,
        v: impl Into<BlockAssignable<GlueTriggerActionsElNotificationPropertyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.notification_property = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_property = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GlueTriggerActionsEl {
    type O = BlockAssignable<GlueTriggerActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueTriggerActionsEl {}
impl BuildGlueTriggerActionsEl {
    pub fn build(self) -> GlueTriggerActionsEl {
        GlueTriggerActionsEl {
            arguments: core::default::Default::default(),
            crawler_name: core::default::Default::default(),
            job_name: core::default::Default::default(),
            security_configuration: core::default::Default::default(),
            timeout: core::default::Default::default(),
            notification_property: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GlueTriggerActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueTriggerActionsElRef {
    fn new(shared: StackShared, base: String) -> GlueTriggerActionsElRef {
        GlueTriggerActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueTriggerActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arguments` after provisioning.\n"]
    pub fn arguments(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.arguments", self.base))
    }
    #[doc = "Get a reference to the value of field `crawler_name` after provisioning.\n"]
    pub fn crawler_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.crawler_name", self.base))
    }
    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.base))
    }
    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `notification_property` after provisioning.\n"]
    pub fn notification_property(&self) -> ListRef<GlueTriggerActionsElNotificationPropertyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_property", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GlueTriggerEventBatchingConditionEl {
    batch_size: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_window: Option<PrimField<f64>>,
}
impl GlueTriggerEventBatchingConditionEl {
    #[doc = "Set the field `batch_window`.\n"]
    pub fn set_batch_window(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_window = Some(v.into());
        self
    }
}
impl ToListMappable for GlueTriggerEventBatchingConditionEl {
    type O = BlockAssignable<GlueTriggerEventBatchingConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueTriggerEventBatchingConditionEl {
    #[doc = ""]
    pub batch_size: PrimField<f64>,
}
impl BuildGlueTriggerEventBatchingConditionEl {
    pub fn build(self) -> GlueTriggerEventBatchingConditionEl {
        GlueTriggerEventBatchingConditionEl {
            batch_size: self.batch_size,
            batch_window: core::default::Default::default(),
        }
    }
}
pub struct GlueTriggerEventBatchingConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueTriggerEventBatchingConditionElRef {
    fn new(shared: StackShared, base: String) -> GlueTriggerEventBatchingConditionElRef {
        GlueTriggerEventBatchingConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueTriggerEventBatchingConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }
    #[doc = "Get a reference to the value of field `batch_window` after provisioning.\n"]
    pub fn batch_window(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_window", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueTriggerPredicateElConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crawl_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crawler_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logical_operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl GlueTriggerPredicateElConditionsEl {
    #[doc = "Set the field `crawl_state`.\n"]
    pub fn set_crawl_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.crawl_state = Some(v.into());
        self
    }
    #[doc = "Set the field `crawler_name`.\n"]
    pub fn set_crawler_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.crawler_name = Some(v.into());
        self
    }
    #[doc = "Set the field `job_name`.\n"]
    pub fn set_job_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.job_name = Some(v.into());
        self
    }
    #[doc = "Set the field `logical_operator`.\n"]
    pub fn set_logical_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logical_operator = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for GlueTriggerPredicateElConditionsEl {
    type O = BlockAssignable<GlueTriggerPredicateElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueTriggerPredicateElConditionsEl {}
impl BuildGlueTriggerPredicateElConditionsEl {
    pub fn build(self) -> GlueTriggerPredicateElConditionsEl {
        GlueTriggerPredicateElConditionsEl {
            crawl_state: core::default::Default::default(),
            crawler_name: core::default::Default::default(),
            job_name: core::default::Default::default(),
            logical_operator: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct GlueTriggerPredicateElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueTriggerPredicateElConditionsElRef {
    fn new(shared: StackShared, base: String) -> GlueTriggerPredicateElConditionsElRef {
        GlueTriggerPredicateElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueTriggerPredicateElConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `crawl_state` after provisioning.\n"]
    pub fn crawl_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.crawl_state", self.base))
    }
    #[doc = "Get a reference to the value of field `crawler_name` after provisioning.\n"]
    pub fn crawler_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.crawler_name", self.base))
    }
    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.base))
    }
    #[doc = "Get a reference to the value of field `logical_operator` after provisioning.\n"]
    pub fn logical_operator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logical_operator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize, Default)]
struct GlueTriggerPredicateElDynamic {
    conditions: Option<DynamicBlock<GlueTriggerPredicateElConditionsEl>>,
}
#[derive(Serialize)]
pub struct GlueTriggerPredicateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    logical: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<GlueTriggerPredicateElConditionsEl>>,
    dynamic: GlueTriggerPredicateElDynamic,
}
impl GlueTriggerPredicateEl {
    #[doc = "Set the field `logical`.\n"]
    pub fn set_logical(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logical = Some(v.into());
        self
    }
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<BlockAssignable<GlueTriggerPredicateElConditionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GlueTriggerPredicateEl {
    type O = BlockAssignable<GlueTriggerPredicateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueTriggerPredicateEl {}
impl BuildGlueTriggerPredicateEl {
    pub fn build(self) -> GlueTriggerPredicateEl {
        GlueTriggerPredicateEl {
            logical: core::default::Default::default(),
            conditions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GlueTriggerPredicateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueTriggerPredicateElRef {
    fn new(shared: StackShared, base: String) -> GlueTriggerPredicateElRef {
        GlueTriggerPredicateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueTriggerPredicateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `logical` after provisioning.\n"]
    pub fn logical(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logical", self.base))
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<GlueTriggerPredicateElConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueTriggerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl GlueTriggerTimeoutsEl {
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
impl ToListMappable for GlueTriggerTimeoutsEl {
    type O = BlockAssignable<GlueTriggerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueTriggerTimeoutsEl {}
impl BuildGlueTriggerTimeoutsEl {
    pub fn build(self) -> GlueTriggerTimeoutsEl {
        GlueTriggerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct GlueTriggerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueTriggerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GlueTriggerTimeoutsElRef {
        GlueTriggerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueTriggerTimeoutsElRef {
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
struct GlueTriggerDynamic {
    actions: Option<DynamicBlock<GlueTriggerActionsEl>>,
    event_batching_condition: Option<DynamicBlock<GlueTriggerEventBatchingConditionEl>>,
    predicate: Option<DynamicBlock<GlueTriggerPredicateEl>>,
}
