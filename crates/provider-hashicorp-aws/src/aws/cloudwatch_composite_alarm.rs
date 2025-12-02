use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudwatchCompositeAlarmData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alarm_description: Option<PrimField<String>>,
    alarm_name: PrimField<String>,
    alarm_rule: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insufficient_data_actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ok_actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions_suppressor: Option<Vec<CloudwatchCompositeAlarmActionsSuppressorEl>>,
    dynamic: CloudwatchCompositeAlarmDynamic,
}
struct CloudwatchCompositeAlarm_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudwatchCompositeAlarmData>,
}
#[derive(Clone)]
pub struct CloudwatchCompositeAlarm(Rc<CloudwatchCompositeAlarm_>);
impl CloudwatchCompositeAlarm {
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
    #[doc = "Set the field `actions_enabled`.\n"]
    pub fn set_actions_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().actions_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `alarm_actions`.\n"]
    pub fn set_alarm_actions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().alarm_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `alarm_description`.\n"]
    pub fn set_alarm_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().alarm_description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `insufficient_data_actions`.\n"]
    pub fn set_insufficient_data_actions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().insufficient_data_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `ok_actions`.\n"]
    pub fn set_ok_actions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ok_actions = Some(v.into());
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
    #[doc = "Set the field `actions_suppressor`.\n"]
    pub fn set_actions_suppressor(
        self,
        v: impl Into<BlockAssignable<CloudwatchCompositeAlarmActionsSuppressorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions_suppressor = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions_suppressor = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `actions_enabled` after provisioning.\n"]
    pub fn actions_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.actions_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_actions` after provisioning.\n"]
    pub fn alarm_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.alarm_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_description` after provisioning.\n"]
    pub fn alarm_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alarm_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_name` after provisioning.\n"]
    pub fn alarm_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alarm_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_rule` after provisioning.\n"]
    pub fn alarm_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alarm_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `insufficient_data_actions` after provisioning.\n"]
    pub fn insufficient_data_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.insufficient_data_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ok_actions` after provisioning.\n"]
    pub fn ok_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ok_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `actions_suppressor` after provisioning.\n"]
    pub fn actions_suppressor(&self) -> ListRef<CloudwatchCompositeAlarmActionsSuppressorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.actions_suppressor", self.extract_ref()),
        )
    }
}
impl Referable for CloudwatchCompositeAlarm {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudwatchCompositeAlarm {}
impl ToListMappable for CloudwatchCompositeAlarm {
    type O = ListRef<CloudwatchCompositeAlarmRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudwatchCompositeAlarm_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudwatch_composite_alarm".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudwatchCompositeAlarm {
    pub tf_id: String,
    #[doc = ""]
    pub alarm_name: PrimField<String>,
    #[doc = ""]
    pub alarm_rule: PrimField<String>,
}
impl BuildCloudwatchCompositeAlarm {
    pub fn build(self, stack: &mut Stack) -> CloudwatchCompositeAlarm {
        let out = CloudwatchCompositeAlarm(Rc::new(CloudwatchCompositeAlarm_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudwatchCompositeAlarmData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                actions_enabled: core::default::Default::default(),
                alarm_actions: core::default::Default::default(),
                alarm_description: core::default::Default::default(),
                alarm_name: self.alarm_name,
                alarm_rule: self.alarm_rule,
                id: core::default::Default::default(),
                insufficient_data_actions: core::default::Default::default(),
                ok_actions: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                actions_suppressor: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudwatchCompositeAlarmRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchCompositeAlarmRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudwatchCompositeAlarmRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `actions_enabled` after provisioning.\n"]
    pub fn actions_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.actions_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_actions` after provisioning.\n"]
    pub fn alarm_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.alarm_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_description` after provisioning.\n"]
    pub fn alarm_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alarm_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_name` after provisioning.\n"]
    pub fn alarm_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alarm_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alarm_rule` after provisioning.\n"]
    pub fn alarm_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alarm_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `insufficient_data_actions` after provisioning.\n"]
    pub fn insufficient_data_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.insufficient_data_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ok_actions` after provisioning.\n"]
    pub fn ok_actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ok_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `actions_suppressor` after provisioning.\n"]
    pub fn actions_suppressor(&self) -> ListRef<CloudwatchCompositeAlarmActionsSuppressorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.actions_suppressor", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudwatchCompositeAlarmActionsSuppressorEl {
    alarm: PrimField<String>,
    extension_period: PrimField<f64>,
    wait_period: PrimField<f64>,
}
impl CloudwatchCompositeAlarmActionsSuppressorEl {}
impl ToListMappable for CloudwatchCompositeAlarmActionsSuppressorEl {
    type O = BlockAssignable<CloudwatchCompositeAlarmActionsSuppressorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchCompositeAlarmActionsSuppressorEl {
    #[doc = ""]
    pub alarm: PrimField<String>,
    #[doc = ""]
    pub extension_period: PrimField<f64>,
    #[doc = ""]
    pub wait_period: PrimField<f64>,
}
impl BuildCloudwatchCompositeAlarmActionsSuppressorEl {
    pub fn build(self) -> CloudwatchCompositeAlarmActionsSuppressorEl {
        CloudwatchCompositeAlarmActionsSuppressorEl {
            alarm: self.alarm,
            extension_period: self.extension_period,
            wait_period: self.wait_period,
        }
    }
}
pub struct CloudwatchCompositeAlarmActionsSuppressorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchCompositeAlarmActionsSuppressorElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchCompositeAlarmActionsSuppressorElRef {
        CloudwatchCompositeAlarmActionsSuppressorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchCompositeAlarmActionsSuppressorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alarm` after provisioning.\n"]
    pub fn alarm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alarm", self.base))
    }
    #[doc = "Get a reference to the value of field `extension_period` after provisioning.\n"]
    pub fn extension_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.extension_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `wait_period` after provisioning.\n"]
    pub fn wait_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_period", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudwatchCompositeAlarmDynamic {
    actions_suppressor: Option<DynamicBlock<CloudwatchCompositeAlarmActionsSuppressorEl>>,
}
