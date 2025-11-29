use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SchedulerScheduleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_after_completion: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    schedule_expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_expression_timezone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flexible_time_window: Option<Vec<SchedulerScheduleFlexibleTimeWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<SchedulerScheduleTargetEl>>,
    dynamic: SchedulerScheduleDynamic,
}

struct SchedulerSchedule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SchedulerScheduleData>,
}

#[derive(Clone)]
pub struct SchedulerSchedule(Rc<SchedulerSchedule_>);

impl SchedulerSchedule {
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

    #[doc = "Set the field `action_after_completion`.\n"]
    pub fn set_action_after_completion(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().action_after_completion = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().end_date = Some(v.into());
        self
    }

    #[doc = "Set the field `group_name`.\n"]
    pub fn set_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = Some(v.into());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_expression_timezone`.\n"]
    pub fn set_schedule_expression_timezone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().schedule_expression_timezone = Some(v.into());
        self
    }

    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().start_date = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }

    #[doc = "Set the field `flexible_time_window`.\n"]
    pub fn set_flexible_time_window(self, v: impl Into<BlockAssignable<SchedulerScheduleFlexibleTimeWindowEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().flexible_time_window = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.flexible_time_window = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(self, v: impl Into<BlockAssignable<SchedulerScheduleTargetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `action_after_completion` after provisioning.\n"]
    pub fn action_after_completion(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_after_completion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression_timezone` after provisioning.\n"]
    pub fn schedule_expression_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression_timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flexible_time_window` after provisioning.\n"]
    pub fn flexible_time_window(&self) -> ListRef<SchedulerScheduleFlexibleTimeWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.flexible_time_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<SchedulerScheduleTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }
}

impl Referable for SchedulerSchedule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SchedulerSchedule { }

impl ToListMappable for SchedulerSchedule {
    type O = ListRef<SchedulerScheduleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SchedulerSchedule_ {
    fn extract_resource_type(&self) -> String {
        "aws_scheduler_schedule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSchedulerSchedule {
    pub tf_id: String,
    #[doc = ""]
    pub schedule_expression: PrimField<String>,
}

impl BuildSchedulerSchedule {
    pub fn build(self, stack: &mut Stack) -> SchedulerSchedule {
        let out = SchedulerSchedule(Rc::new(SchedulerSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SchedulerScheduleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action_after_completion: core::default::Default::default(),
                description: core::default::Default::default(),
                end_date: core::default::Default::default(),
                group_name: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                region: core::default::Default::default(),
                schedule_expression: self.schedule_expression,
                schedule_expression_timezone: core::default::Default::default(),
                start_date: core::default::Default::default(),
                state: core::default::Default::default(),
                flexible_time_window: core::default::Default::default(),
                target: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SchedulerScheduleRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SchedulerScheduleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_after_completion` after provisioning.\n"]
    pub fn action_after_completion(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_after_completion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression_timezone` after provisioning.\n"]
    pub fn schedule_expression_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression_timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flexible_time_window` after provisioning.\n"]
    pub fn flexible_time_window(&self) -> ListRef<SchedulerScheduleFlexibleTimeWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.flexible_time_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<SchedulerScheduleTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SchedulerScheduleFlexibleTimeWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_window_in_minutes: Option<PrimField<f64>>,
    mode: PrimField<String>,
}

impl SchedulerScheduleFlexibleTimeWindowEl {
    #[doc = "Set the field `maximum_window_in_minutes`.\n"]
    pub fn set_maximum_window_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_window_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for SchedulerScheduleFlexibleTimeWindowEl {
    type O = BlockAssignable<SchedulerScheduleFlexibleTimeWindowEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleFlexibleTimeWindowEl {
    #[doc = ""]
    pub mode: PrimField<String>,
}

impl BuildSchedulerScheduleFlexibleTimeWindowEl {
    pub fn build(self) -> SchedulerScheduleFlexibleTimeWindowEl {
        SchedulerScheduleFlexibleTimeWindowEl {
            maximum_window_in_minutes: core::default::Default::default(),
            mode: self.mode,
        }
    }
}

pub struct SchedulerScheduleFlexibleTimeWindowElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleFlexibleTimeWindowElRef {
    fn new(shared: StackShared, base: String) -> SchedulerScheduleFlexibleTimeWindowElRef {
        SchedulerScheduleFlexibleTimeWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleFlexibleTimeWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_window_in_minutes` after provisioning.\n"]
    pub fn maximum_window_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_window_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct SchedulerScheduleTargetElDeadLetterConfigEl {
    arn: PrimField<String>,
}

impl SchedulerScheduleTargetElDeadLetterConfigEl { }

impl ToListMappable for SchedulerScheduleTargetElDeadLetterConfigEl {
    type O = BlockAssignable<SchedulerScheduleTargetElDeadLetterConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetElDeadLetterConfigEl {
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildSchedulerScheduleTargetElDeadLetterConfigEl {
    pub fn build(self) -> SchedulerScheduleTargetElDeadLetterConfigEl {
        SchedulerScheduleTargetElDeadLetterConfigEl { arn: self.arn }
    }
}

pub struct SchedulerScheduleTargetElDeadLetterConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElDeadLetterConfigElRef {
    fn new(shared: StackShared, base: String) -> SchedulerScheduleTargetElDeadLetterConfigElRef {
        SchedulerScheduleTargetElDeadLetterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElDeadLetterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<PrimField<f64>>,
    capacity_provider: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyEl {
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

impl ToListMappable for SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyEl {
    type O = BlockAssignable<SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyEl {
    #[doc = ""]
    pub capacity_provider: PrimField<String>,
}

impl BuildSchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyEl {
    pub fn build(self) -> SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyEl {
        SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyEl {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

pub struct SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyElRef {
        SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct SchedulerScheduleTargetElEcsParametersElNetworkConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    subnets: SetField<PrimField<String>>,
}

impl SchedulerScheduleTargetElEcsParametersElNetworkConfigurationEl {
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

impl ToListMappable for SchedulerScheduleTargetElEcsParametersElNetworkConfigurationEl {
    type O = BlockAssignable<SchedulerScheduleTargetElEcsParametersElNetworkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetElEcsParametersElNetworkConfigurationEl {
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
}

impl BuildSchedulerScheduleTargetElEcsParametersElNetworkConfigurationEl {
    pub fn build(self) -> SchedulerScheduleTargetElEcsParametersElNetworkConfigurationEl {
        SchedulerScheduleTargetElEcsParametersElNetworkConfigurationEl {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: self.subnets,
        }
    }
}

pub struct SchedulerScheduleTargetElEcsParametersElNetworkConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElEcsParametersElNetworkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SchedulerScheduleTargetElEcsParametersElNetworkConfigurationElRef {
        SchedulerScheduleTargetElEcsParametersElNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElEcsParametersElNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.assign_public_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct SchedulerScheduleTargetElEcsParametersElPlacementConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl SchedulerScheduleTargetElEcsParametersElPlacementConstraintsEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
}

impl ToListMappable for SchedulerScheduleTargetElEcsParametersElPlacementConstraintsEl {
    type O = BlockAssignable<SchedulerScheduleTargetElEcsParametersElPlacementConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetElEcsParametersElPlacementConstraintsEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildSchedulerScheduleTargetElEcsParametersElPlacementConstraintsEl {
    pub fn build(self) -> SchedulerScheduleTargetElEcsParametersElPlacementConstraintsEl {
        SchedulerScheduleTargetElEcsParametersElPlacementConstraintsEl {
            expression: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct SchedulerScheduleTargetElEcsParametersElPlacementConstraintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElEcsParametersElPlacementConstraintsElRef {
    fn new(shared: StackShared, base: String) -> SchedulerScheduleTargetElEcsParametersElPlacementConstraintsElRef {
        SchedulerScheduleTargetElEcsParametersElPlacementConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElEcsParametersElPlacementConstraintsElRef {
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

#[derive(Serialize)]
pub struct SchedulerScheduleTargetElEcsParametersElPlacementStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl SchedulerScheduleTargetElEcsParametersElPlacementStrategyEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
}

impl ToListMappable for SchedulerScheduleTargetElEcsParametersElPlacementStrategyEl {
    type O = BlockAssignable<SchedulerScheduleTargetElEcsParametersElPlacementStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetElEcsParametersElPlacementStrategyEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildSchedulerScheduleTargetElEcsParametersElPlacementStrategyEl {
    pub fn build(self) -> SchedulerScheduleTargetElEcsParametersElPlacementStrategyEl {
        SchedulerScheduleTargetElEcsParametersElPlacementStrategyEl {
            field: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct SchedulerScheduleTargetElEcsParametersElPlacementStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElEcsParametersElPlacementStrategyElRef {
    fn new(shared: StackShared, base: String) -> SchedulerScheduleTargetElEcsParametersElPlacementStrategyElRef {
        SchedulerScheduleTargetElEcsParametersElPlacementStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElEcsParametersElPlacementStrategyElRef {
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

#[derive(Serialize, Default)]
struct SchedulerScheduleTargetElEcsParametersElDynamic {
    capacity_provider_strategy: Option<
        DynamicBlock<SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyEl>,
    >,
    network_configuration: Option<DynamicBlock<SchedulerScheduleTargetElEcsParametersElNetworkConfigurationEl>>,
    placement_constraints: Option<DynamicBlock<SchedulerScheduleTargetElEcsParametersElPlacementConstraintsEl>>,
    placement_strategy: Option<DynamicBlock<SchedulerScheduleTargetElEcsParametersElPlacementStrategyEl>>,
}

#[derive(Serialize)]
pub struct SchedulerScheduleTargetElEcsParametersEl {
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
    reference_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_count: Option<PrimField<f64>>,
    task_definition_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_provider_strategy: Option<Vec<SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configuration: Option<Vec<SchedulerScheduleTargetElEcsParametersElNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_constraints: Option<Vec<SchedulerScheduleTargetElEcsParametersElPlacementConstraintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_strategy: Option<Vec<SchedulerScheduleTargetElEcsParametersElPlacementStrategyEl>>,
    dynamic: SchedulerScheduleTargetElEcsParametersElDynamic,
}

impl SchedulerScheduleTargetElEcsParametersEl {
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

    #[doc = "Set the field `reference_id`.\n"]
    pub fn set_reference_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reference_id = Some(v.into());
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
        v: impl Into<BlockAssignable<SchedulerScheduleTargetElEcsParametersElCapacityProviderStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.capacity_provider_strategy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_provider_strategy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        mut self,
        v: impl Into<BlockAssignable<SchedulerScheduleTargetElEcsParametersElNetworkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_constraints`.\n"]
    pub fn set_placement_constraints(
        mut self,
        v: impl Into<BlockAssignable<SchedulerScheduleTargetElEcsParametersElPlacementConstraintsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.placement_constraints = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_constraints = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_strategy`.\n"]
    pub fn set_placement_strategy(
        mut self,
        v: impl Into<BlockAssignable<SchedulerScheduleTargetElEcsParametersElPlacementStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.placement_strategy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_strategy = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SchedulerScheduleTargetElEcsParametersEl {
    type O = BlockAssignable<SchedulerScheduleTargetElEcsParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetElEcsParametersEl {
    #[doc = ""]
    pub task_definition_arn: PrimField<String>,
}

impl BuildSchedulerScheduleTargetElEcsParametersEl {
    pub fn build(self) -> SchedulerScheduleTargetElEcsParametersEl {
        SchedulerScheduleTargetElEcsParametersEl {
            enable_ecs_managed_tags: core::default::Default::default(),
            enable_execute_command: core::default::Default::default(),
            group: core::default::Default::default(),
            launch_type: core::default::Default::default(),
            platform_version: core::default::Default::default(),
            propagate_tags: core::default::Default::default(),
            reference_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            task_count: core::default::Default::default(),
            task_definition_arn: self.task_definition_arn,
            capacity_provider_strategy: core::default::Default::default(),
            network_configuration: core::default::Default::default(),
            placement_constraints: core::default::Default::default(),
            placement_strategy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SchedulerScheduleTargetElEcsParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElEcsParametersElRef {
    fn new(shared: StackShared, base: String) -> SchedulerScheduleTargetElEcsParametersElRef {
        SchedulerScheduleTargetElEcsParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElEcsParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ecs_managed_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_execute_command", self.base))
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
        PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.base))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `reference_id` after provisioning.\n"]
    pub fn reference_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.base))
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
        PrimExpr::new(self.shared().clone(), format!("{}.task_definition_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<SchedulerScheduleTargetElEcsParametersElNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct SchedulerScheduleTargetElEventbridgeParametersEl {
    detail_type: PrimField<String>,
    source: PrimField<String>,
}

impl SchedulerScheduleTargetElEventbridgeParametersEl { }

impl ToListMappable for SchedulerScheduleTargetElEventbridgeParametersEl {
    type O = BlockAssignable<SchedulerScheduleTargetElEventbridgeParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetElEventbridgeParametersEl {
    #[doc = ""]
    pub detail_type: PrimField<String>,
    #[doc = ""]
    pub source: PrimField<String>,
}

impl BuildSchedulerScheduleTargetElEventbridgeParametersEl {
    pub fn build(self) -> SchedulerScheduleTargetElEventbridgeParametersEl {
        SchedulerScheduleTargetElEventbridgeParametersEl {
            detail_type: self.detail_type,
            source: self.source,
        }
    }
}

pub struct SchedulerScheduleTargetElEventbridgeParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElEventbridgeParametersElRef {
    fn new(shared: StackShared, base: String) -> SchedulerScheduleTargetElEventbridgeParametersElRef {
        SchedulerScheduleTargetElEventbridgeParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElEventbridgeParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `detail_type` after provisioning.\n"]
    pub fn detail_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detail_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct SchedulerScheduleTargetElKinesisParametersEl {
    partition_key: PrimField<String>,
}

impl SchedulerScheduleTargetElKinesisParametersEl { }

impl ToListMappable for SchedulerScheduleTargetElKinesisParametersEl {
    type O = BlockAssignable<SchedulerScheduleTargetElKinesisParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetElKinesisParametersEl {
    #[doc = ""]
    pub partition_key: PrimField<String>,
}

impl BuildSchedulerScheduleTargetElKinesisParametersEl {
    pub fn build(self) -> SchedulerScheduleTargetElKinesisParametersEl {
        SchedulerScheduleTargetElKinesisParametersEl { partition_key: self.partition_key }
    }
}

pub struct SchedulerScheduleTargetElKinesisParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElKinesisParametersElRef {
    fn new(shared: StackShared, base: String) -> SchedulerScheduleTargetElKinesisParametersElRef {
        SchedulerScheduleTargetElKinesisParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElKinesisParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.partition_key", self.base))
    }
}

#[derive(Serialize)]
pub struct SchedulerScheduleTargetElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_event_age_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_retry_attempts: Option<PrimField<f64>>,
}

impl SchedulerScheduleTargetElRetryPolicyEl {
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

impl ToListMappable for SchedulerScheduleTargetElRetryPolicyEl {
    type O = BlockAssignable<SchedulerScheduleTargetElRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetElRetryPolicyEl {}

impl BuildSchedulerScheduleTargetElRetryPolicyEl {
    pub fn build(self) -> SchedulerScheduleTargetElRetryPolicyEl {
        SchedulerScheduleTargetElRetryPolicyEl {
            maximum_event_age_in_seconds: core::default::Default::default(),
            maximum_retry_attempts: core::default::Default::default(),
        }
    }
}

pub struct SchedulerScheduleTargetElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> SchedulerScheduleTargetElRetryPolicyElRef {
        SchedulerScheduleTargetElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_event_age_in_seconds` after provisioning.\n"]
    pub fn maximum_event_age_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_event_age_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_attempts", self.base))
    }
}

#[derive(Serialize)]
pub struct SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterEl { }

impl ToListMappable for SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterEl {
    type O = BlockAssignable<SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildSchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterEl {
    pub fn build(self) -> SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterEl {
        SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterElRef {
        SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterElRef {
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
struct SchedulerScheduleTargetElSagemakerPipelineParametersElDynamic {
    pipeline_parameter: Option<
        DynamicBlock<SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterEl>,
    >,
}

#[derive(Serialize)]
pub struct SchedulerScheduleTargetElSagemakerPipelineParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_parameter: Option<Vec<SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterEl>>,
    dynamic: SchedulerScheduleTargetElSagemakerPipelineParametersElDynamic,
}

impl SchedulerScheduleTargetElSagemakerPipelineParametersEl {
    #[doc = "Set the field `pipeline_parameter`.\n"]
    pub fn set_pipeline_parameter(
        mut self,
        v: impl Into<BlockAssignable<SchedulerScheduleTargetElSagemakerPipelineParametersElPipelineParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pipeline_parameter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pipeline_parameter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SchedulerScheduleTargetElSagemakerPipelineParametersEl {
    type O = BlockAssignable<SchedulerScheduleTargetElSagemakerPipelineParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetElSagemakerPipelineParametersEl {}

impl BuildSchedulerScheduleTargetElSagemakerPipelineParametersEl {
    pub fn build(self) -> SchedulerScheduleTargetElSagemakerPipelineParametersEl {
        SchedulerScheduleTargetElSagemakerPipelineParametersEl {
            pipeline_parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SchedulerScheduleTargetElSagemakerPipelineParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElSagemakerPipelineParametersElRef {
    fn new(shared: StackShared, base: String) -> SchedulerScheduleTargetElSagemakerPipelineParametersElRef {
        SchedulerScheduleTargetElSagemakerPipelineParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElSagemakerPipelineParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct SchedulerScheduleTargetElSqsParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_group_id: Option<PrimField<String>>,
}

impl SchedulerScheduleTargetElSqsParametersEl {
    #[doc = "Set the field `message_group_id`.\n"]
    pub fn set_message_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_group_id = Some(v.into());
        self
    }
}

impl ToListMappable for SchedulerScheduleTargetElSqsParametersEl {
    type O = BlockAssignable<SchedulerScheduleTargetElSqsParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetElSqsParametersEl {}

impl BuildSchedulerScheduleTargetElSqsParametersEl {
    pub fn build(self) -> SchedulerScheduleTargetElSqsParametersEl {
        SchedulerScheduleTargetElSqsParametersEl { message_group_id: core::default::Default::default() }
    }
}

pub struct SchedulerScheduleTargetElSqsParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElSqsParametersElRef {
    fn new(shared: StackShared, base: String) -> SchedulerScheduleTargetElSqsParametersElRef {
        SchedulerScheduleTargetElSqsParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElSqsParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_group_id` after provisioning.\n"]
    pub fn message_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_group_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SchedulerScheduleTargetElDynamic {
    dead_letter_config: Option<DynamicBlock<SchedulerScheduleTargetElDeadLetterConfigEl>>,
    ecs_parameters: Option<DynamicBlock<SchedulerScheduleTargetElEcsParametersEl>>,
    eventbridge_parameters: Option<DynamicBlock<SchedulerScheduleTargetElEventbridgeParametersEl>>,
    kinesis_parameters: Option<DynamicBlock<SchedulerScheduleTargetElKinesisParametersEl>>,
    retry_policy: Option<DynamicBlock<SchedulerScheduleTargetElRetryPolicyEl>>,
    sagemaker_pipeline_parameters: Option<DynamicBlock<SchedulerScheduleTargetElSagemakerPipelineParametersEl>>,
    sqs_parameters: Option<DynamicBlock<SchedulerScheduleTargetElSqsParametersEl>>,
}

#[derive(Serialize)]
pub struct SchedulerScheduleTargetEl {
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_letter_config: Option<Vec<SchedulerScheduleTargetElDeadLetterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecs_parameters: Option<Vec<SchedulerScheduleTargetElEcsParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eventbridge_parameters: Option<Vec<SchedulerScheduleTargetElEventbridgeParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_parameters: Option<Vec<SchedulerScheduleTargetElKinesisParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<SchedulerScheduleTargetElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_pipeline_parameters: Option<Vec<SchedulerScheduleTargetElSagemakerPipelineParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs_parameters: Option<Vec<SchedulerScheduleTargetElSqsParametersEl>>,
    dynamic: SchedulerScheduleTargetElDynamic,
}

impl SchedulerScheduleTargetEl {
    #[doc = "Set the field `input`.\n"]
    pub fn set_input(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input = Some(v.into());
        self
    }

    #[doc = "Set the field `dead_letter_config`.\n"]
    pub fn set_dead_letter_config(
        mut self,
        v: impl Into<BlockAssignable<SchedulerScheduleTargetElDeadLetterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dead_letter_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dead_letter_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecs_parameters`.\n"]
    pub fn set_ecs_parameters(
        mut self,
        v: impl Into<BlockAssignable<SchedulerScheduleTargetElEcsParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecs_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecs_parameters = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eventbridge_parameters`.\n"]
    pub fn set_eventbridge_parameters(
        mut self,
        v: impl Into<BlockAssignable<SchedulerScheduleTargetElEventbridgeParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.eventbridge_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.eventbridge_parameters = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_parameters`.\n"]
    pub fn set_kinesis_parameters(
        mut self,
        v: impl Into<BlockAssignable<SchedulerScheduleTargetElKinesisParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_parameters = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(mut self, v: impl Into<BlockAssignable<SchedulerScheduleTargetElRetryPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sagemaker_pipeline_parameters`.\n"]
    pub fn set_sagemaker_pipeline_parameters(
        mut self,
        v: impl Into<BlockAssignable<SchedulerScheduleTargetElSagemakerPipelineParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sagemaker_pipeline_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sagemaker_pipeline_parameters = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqs_parameters`.\n"]
    pub fn set_sqs_parameters(
        mut self,
        v: impl Into<BlockAssignable<SchedulerScheduleTargetElSqsParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sqs_parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs_parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SchedulerScheduleTargetEl {
    type O = BlockAssignable<SchedulerScheduleTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSchedulerScheduleTargetEl {
    #[doc = ""]
    pub arn: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildSchedulerScheduleTargetEl {
    pub fn build(self) -> SchedulerScheduleTargetEl {
        SchedulerScheduleTargetEl {
            arn: self.arn,
            input: core::default::Default::default(),
            role_arn: self.role_arn,
            dead_letter_config: core::default::Default::default(),
            ecs_parameters: core::default::Default::default(),
            eventbridge_parameters: core::default::Default::default(),
            kinesis_parameters: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            sagemaker_pipeline_parameters: core::default::Default::default(),
            sqs_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SchedulerScheduleTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SchedulerScheduleTargetElRef {
    fn new(shared: StackShared, base: String) -> SchedulerScheduleTargetElRef {
        SchedulerScheduleTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SchedulerScheduleTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> ListRef<SchedulerScheduleTargetElDeadLetterConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ecs_parameters` after provisioning.\n"]
    pub fn ecs_parameters(&self) -> ListRef<SchedulerScheduleTargetElEcsParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ecs_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `eventbridge_parameters` after provisioning.\n"]
    pub fn eventbridge_parameters(&self) -> ListRef<SchedulerScheduleTargetElEventbridgeParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.eventbridge_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_parameters` after provisioning.\n"]
    pub fn kinesis_parameters(&self) -> ListRef<SchedulerScheduleTargetElKinesisParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kinesis_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<SchedulerScheduleTargetElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_pipeline_parameters` after provisioning.\n"]
    pub fn sagemaker_pipeline_parameters(&self) -> ListRef<SchedulerScheduleTargetElSagemakerPipelineParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sagemaker_pipeline_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `sqs_parameters` after provisioning.\n"]
    pub fn sqs_parameters(&self) -> ListRef<SchedulerScheduleTargetElSqsParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sqs_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct SchedulerScheduleDynamic {
    flexible_time_window: Option<DynamicBlock<SchedulerScheduleFlexibleTimeWindowEl>>,
    target: Option<DynamicBlock<SchedulerScheduleTargetEl>>,
}
