use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SagemakerMonitoringScheduleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_schedule_config: Option<Vec<SagemakerMonitoringScheduleMonitoringScheduleConfigEl>>,
    dynamic: SagemakerMonitoringScheduleDynamic,
}
struct SagemakerMonitoringSchedule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerMonitoringScheduleData>,
}
#[derive(Clone)]
pub struct SagemakerMonitoringSchedule(Rc<SagemakerMonitoringSchedule_>);
impl SagemakerMonitoringSchedule {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
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
    #[doc = "Set the field `monitoring_schedule_config`.\n"]
    pub fn set_monitoring_schedule_config(
        self,
        v: impl Into<BlockAssignable<SagemakerMonitoringScheduleMonitoringScheduleConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring_schedule_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring_schedule_config = Some(d);
            }
        }
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
    #[doc = "Get a reference to the value of field `monitoring_schedule_config` after provisioning.\n"]
    pub fn monitoring_schedule_config(
        &self,
    ) -> ListRef<SagemakerMonitoringScheduleMonitoringScheduleConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring_schedule_config", self.extract_ref()),
        )
    }
}
impl Referable for SagemakerMonitoringSchedule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SagemakerMonitoringSchedule {}
impl ToListMappable for SagemakerMonitoringSchedule {
    type O = ListRef<SagemakerMonitoringScheduleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SagemakerMonitoringSchedule_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_monitoring_schedule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSagemakerMonitoringSchedule {
    pub tf_id: String,
}
impl BuildSagemakerMonitoringSchedule {
    pub fn build(self, stack: &mut Stack) -> SagemakerMonitoringSchedule {
        let out = SagemakerMonitoringSchedule(Rc::new(SagemakerMonitoringSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerMonitoringScheduleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                monitoring_schedule_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SagemakerMonitoringScheduleRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerMonitoringScheduleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SagemakerMonitoringScheduleRef {
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
    #[doc = "Get a reference to the value of field `monitoring_schedule_config` after provisioning.\n"]
    pub fn monitoring_schedule_config(
        &self,
    ) -> ListRef<SagemakerMonitoringScheduleMonitoringScheduleConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring_schedule_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigEl {
    schedule_expression: PrimField<String>,
}
impl SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigEl {}
impl ToListMappable for SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigEl {
    type O = BlockAssignable<SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigEl {
    #[doc = ""]
    pub schedule_expression: PrimField<String>,
}
impl BuildSagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigEl {
    pub fn build(self) -> SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigEl {
        SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigEl {
            schedule_expression: self.schedule_expression,
        }
    }
}
pub struct SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigElRef {
        SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_expression", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerMonitoringScheduleMonitoringScheduleConfigElDynamic {
    schedule_config:
        Option<DynamicBlock<SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigEl>>,
}
#[derive(Serialize)]
pub struct SagemakerMonitoringScheduleMonitoringScheduleConfigEl {
    monitoring_job_definition_name: PrimField<String>,
    monitoring_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_config:
        Option<Vec<SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigEl>>,
    dynamic: SagemakerMonitoringScheduleMonitoringScheduleConfigElDynamic,
}
impl SagemakerMonitoringScheduleMonitoringScheduleConfigEl {
    #[doc = "Set the field `schedule_config`.\n"]
    pub fn set_schedule_config(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schedule_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerMonitoringScheduleMonitoringScheduleConfigEl {
    type O = BlockAssignable<SagemakerMonitoringScheduleMonitoringScheduleConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerMonitoringScheduleMonitoringScheduleConfigEl {
    #[doc = ""]
    pub monitoring_job_definition_name: PrimField<String>,
    #[doc = ""]
    pub monitoring_type: PrimField<String>,
}
impl BuildSagemakerMonitoringScheduleMonitoringScheduleConfigEl {
    pub fn build(self) -> SagemakerMonitoringScheduleMonitoringScheduleConfigEl {
        SagemakerMonitoringScheduleMonitoringScheduleConfigEl {
            monitoring_job_definition_name: self.monitoring_job_definition_name,
            monitoring_type: self.monitoring_type,
            schedule_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerMonitoringScheduleMonitoringScheduleConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerMonitoringScheduleMonitoringScheduleConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerMonitoringScheduleMonitoringScheduleConfigElRef {
        SagemakerMonitoringScheduleMonitoringScheduleConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerMonitoringScheduleMonitoringScheduleConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `monitoring_job_definition_name` after provisioning.\n"]
    pub fn monitoring_job_definition_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring_job_definition_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_type` after provisioning.\n"]
    pub fn monitoring_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_config` after provisioning.\n"]
    pub fn schedule_config(
        &self,
    ) -> ListRef<SagemakerMonitoringScheduleMonitoringScheduleConfigElScheduleConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerMonitoringScheduleDynamic {
    monitoring_schedule_config:
        Option<DynamicBlock<SagemakerMonitoringScheduleMonitoringScheduleConfigEl>>,
}
