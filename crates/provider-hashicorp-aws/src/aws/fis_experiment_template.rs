use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FisExperimentTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<FisExperimentTemplateActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    experiment_options: Option<Vec<FisExperimentTemplateExperimentOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    experiment_report_configuration:
        Option<Vec<FisExperimentTemplateExperimentReportConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_configuration: Option<Vec<FisExperimentTemplateLogConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_condition: Option<Vec<FisExperimentTemplateStopConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<FisExperimentTemplateTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FisExperimentTemplateTimeoutsEl>,
    dynamic: FisExperimentTemplateDynamic,
}
struct FisExperimentTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FisExperimentTemplateData>,
}
#[derive(Clone)]
pub struct FisExperimentTemplate(Rc<FisExperimentTemplate_>);
impl FisExperimentTemplate {
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
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl Into<BlockAssignable<FisExperimentTemplateActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `experiment_options`.\n"]
    pub fn set_experiment_options(
        self,
        v: impl Into<BlockAssignable<FisExperimentTemplateExperimentOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().experiment_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.experiment_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `experiment_report_configuration`.\n"]
    pub fn set_experiment_report_configuration(
        self,
        v: impl Into<BlockAssignable<FisExperimentTemplateExperimentReportConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().experiment_report_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .experiment_report_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        self,
        v: impl Into<BlockAssignable<FisExperimentTemplateLogConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stop_condition`.\n"]
    pub fn set_stop_condition(
        self,
        v: impl Into<BlockAssignable<FisExperimentTemplateStopConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stop_condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stop_condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(self, v: impl Into<BlockAssignable<FisExperimentTemplateTargetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FisExperimentTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `experiment_options` after provisioning.\n"]
    pub fn experiment_options(&self) -> ListRef<FisExperimentTemplateExperimentOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.experiment_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `experiment_report_configuration` after provisioning.\n"]
    pub fn experiment_report_configuration(
        &self,
    ) -> ListRef<FisExperimentTemplateExperimentReportConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.experiment_report_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> ListRef<FisExperimentTemplateLogConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FisExperimentTemplateTimeoutsElRef {
        FisExperimentTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FisExperimentTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FisExperimentTemplate {}
impl ToListMappable for FisExperimentTemplate {
    type O = ListRef<FisExperimentTemplateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FisExperimentTemplate_ {
    fn extract_resource_type(&self) -> String {
        "aws_fis_experiment_template".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFisExperimentTemplate {
    pub tf_id: String,
    #[doc = ""]
    pub description: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildFisExperimentTemplate {
    pub fn build(self, stack: &mut Stack) -> FisExperimentTemplate {
        let out = FisExperimentTemplate(Rc::new(FisExperimentTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FisExperimentTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: self.description,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                action: core::default::Default::default(),
                experiment_options: core::default::Default::default(),
                experiment_report_configuration: core::default::Default::default(),
                log_configuration: core::default::Default::default(),
                stop_condition: core::default::Default::default(),
                target: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FisExperimentTemplateRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FisExperimentTemplateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `experiment_options` after provisioning.\n"]
    pub fn experiment_options(&self) -> ListRef<FisExperimentTemplateExperimentOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.experiment_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `experiment_report_configuration` after provisioning.\n"]
    pub fn experiment_report_configuration(
        &self,
    ) -> ListRef<FisExperimentTemplateExperimentReportConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.experiment_report_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> ListRef<FisExperimentTemplateLogConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FisExperimentTemplateTimeoutsElRef {
        FisExperimentTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FisExperimentTemplateActionElParameterEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl FisExperimentTemplateActionElParameterEl {}
impl ToListMappable for FisExperimentTemplateActionElParameterEl {
    type O = BlockAssignable<FisExperimentTemplateActionElParameterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateActionElParameterEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildFisExperimentTemplateActionElParameterEl {
    pub fn build(self) -> FisExperimentTemplateActionElParameterEl {
        FisExperimentTemplateActionElParameterEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct FisExperimentTemplateActionElParameterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateActionElParameterElRef {
    fn new(shared: StackShared, base: String) -> FisExperimentTemplateActionElParameterElRef {
        FisExperimentTemplateActionElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateActionElParameterElRef {
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
#[derive(Serialize)]
pub struct FisExperimentTemplateActionElTargetEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl FisExperimentTemplateActionElTargetEl {}
impl ToListMappable for FisExperimentTemplateActionElTargetEl {
    type O = BlockAssignable<FisExperimentTemplateActionElTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateActionElTargetEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildFisExperimentTemplateActionElTargetEl {
    pub fn build(self) -> FisExperimentTemplateActionElTargetEl {
        FisExperimentTemplateActionElTargetEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct FisExperimentTemplateActionElTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateActionElTargetElRef {
    fn new(shared: StackShared, base: String) -> FisExperimentTemplateActionElTargetElRef {
        FisExperimentTemplateActionElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateActionElTargetElRef {
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
struct FisExperimentTemplateActionElDynamic {
    parameter: Option<DynamicBlock<FisExperimentTemplateActionElParameterEl>>,
    target: Option<DynamicBlock<FisExperimentTemplateActionElTargetEl>>,
}
#[derive(Serialize)]
pub struct FisExperimentTemplateActionEl {
    action_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_after: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<Vec<FisExperimentTemplateActionElParameterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<FisExperimentTemplateActionElTargetEl>>,
    dynamic: FisExperimentTemplateActionElDynamic,
}
impl FisExperimentTemplateActionEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `start_after`.\n"]
    pub fn set_start_after(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.start_after = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl Into<BlockAssignable<FisExperimentTemplateActionElParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl Into<BlockAssignable<FisExperimentTemplateActionElTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FisExperimentTemplateActionEl {
    type O = BlockAssignable<FisExperimentTemplateActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateActionEl {
    #[doc = ""]
    pub action_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildFisExperimentTemplateActionEl {
    pub fn build(self) -> FisExperimentTemplateActionEl {
        FisExperimentTemplateActionEl {
            action_id: self.action_id,
            description: core::default::Default::default(),
            name: self.name,
            start_after: core::default::Default::default(),
            parameter: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FisExperimentTemplateActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateActionElRef {
    fn new(shared: StackShared, base: String) -> FisExperimentTemplateActionElRef {
        FisExperimentTemplateActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action_id` after provisioning.\n"]
    pub fn action_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_id", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `start_after` after provisioning.\n"]
    pub fn start_after(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.start_after", self.base))
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<FisExperimentTemplateActionElTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
#[derive(Serialize)]
pub struct FisExperimentTemplateExperimentOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_targeting: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    empty_target_resolution_mode: Option<PrimField<String>>,
}
impl FisExperimentTemplateExperimentOptionsEl {
    #[doc = "Set the field `account_targeting`.\n"]
    pub fn set_account_targeting(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_targeting = Some(v.into());
        self
    }
    #[doc = "Set the field `empty_target_resolution_mode`.\n"]
    pub fn set_empty_target_resolution_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.empty_target_resolution_mode = Some(v.into());
        self
    }
}
impl ToListMappable for FisExperimentTemplateExperimentOptionsEl {
    type O = BlockAssignable<FisExperimentTemplateExperimentOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateExperimentOptionsEl {}
impl BuildFisExperimentTemplateExperimentOptionsEl {
    pub fn build(self) -> FisExperimentTemplateExperimentOptionsEl {
        FisExperimentTemplateExperimentOptionsEl {
            account_targeting: core::default::Default::default(),
            empty_target_resolution_mode: core::default::Default::default(),
        }
    }
}
pub struct FisExperimentTemplateExperimentOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateExperimentOptionsElRef {
    fn new(shared: StackShared, base: String) -> FisExperimentTemplateExperimentOptionsElRef {
        FisExperimentTemplateExperimentOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateExperimentOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_targeting` after provisioning.\n"]
    pub fn account_targeting(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_targeting", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `empty_target_resolution_mode` after provisioning.\n"]
    pub fn empty_target_resolution_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.empty_target_resolution_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dashboard_arn: Option<PrimField<String>>,
}
impl FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardEl {
    #[doc = "Set the field `dashboard_arn`.\n"]
    pub fn set_dashboard_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dashboard_arn = Some(v.into());
        self
    }
}
impl ToListMappable
    for FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardEl
{
    type O = BlockAssignable<
        FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardEl
{}
impl BuildFisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardEl {
    pub fn build(
        self,
    ) -> FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardEl
    {
        FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardEl {
            dashboard_arn: core::default::Default::default(),
        }
    }
}
pub struct FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardElRef
    {
        FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dashboard_arn` after provisioning.\n"]
    pub fn dashboard_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dashboard_arn", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct FisExperimentTemplateExperimentReportConfigurationElDataSourcesElDynamic {
    cloudwatch_dashboard: Option<
        DynamicBlock<
            FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct FisExperimentTemplateExperimentReportConfigurationElDataSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_dashboard: Option<
        Vec<FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardEl>,
    >,
    dynamic: FisExperimentTemplateExperimentReportConfigurationElDataSourcesElDynamic,
}
impl FisExperimentTemplateExperimentReportConfigurationElDataSourcesEl {
    #[doc = "Set the field `cloudwatch_dashboard`.\n"]
    pub fn set_cloudwatch_dashboard(
        mut self,
        v : impl Into < BlockAssignable < FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_dashboard = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_dashboard = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FisExperimentTemplateExperimentReportConfigurationElDataSourcesEl {
    type O = BlockAssignable<FisExperimentTemplateExperimentReportConfigurationElDataSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateExperimentReportConfigurationElDataSourcesEl {}
impl BuildFisExperimentTemplateExperimentReportConfigurationElDataSourcesEl {
    pub fn build(self) -> FisExperimentTemplateExperimentReportConfigurationElDataSourcesEl {
        FisExperimentTemplateExperimentReportConfigurationElDataSourcesEl {
            cloudwatch_dashboard: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FisExperimentTemplateExperimentReportConfigurationElDataSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateExperimentReportConfigurationElDataSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FisExperimentTemplateExperimentReportConfigurationElDataSourcesElRef {
        FisExperimentTemplateExperimentReportConfigurationElDataSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateExperimentReportConfigurationElDataSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloudwatch_dashboard` after provisioning.\n"]
    pub fn cloudwatch_dashboard(
        &self,
    ) -> ListRef<
        FisExperimentTemplateExperimentReportConfigurationElDataSourcesElCloudwatchDashboardElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_dashboard", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}
impl FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}
impl ToListMappable
    for FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationEl
{
    type O = BlockAssignable<
        FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
}
impl BuildFisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationEl {
    pub fn build(
        self,
    ) -> FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationEl {
        FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationEl {
            bucket_name: self.bucket_name,
            prefix: core::default::Default::default(),
        }
    }
}
pub struct FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationElRef {
        FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize, Default)]
struct FisExperimentTemplateExperimentReportConfigurationElOutputsElDynamic {
    s3_configuration: Option<
        DynamicBlock<
            FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct FisExperimentTemplateExperimentReportConfigurationElOutputsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_configuration:
        Option<Vec<FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationEl>>,
    dynamic: FisExperimentTemplateExperimentReportConfigurationElOutputsElDynamic,
}
impl FisExperimentTemplateExperimentReportConfigurationElOutputsEl {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FisExperimentTemplateExperimentReportConfigurationElOutputsEl {
    type O = BlockAssignable<FisExperimentTemplateExperimentReportConfigurationElOutputsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateExperimentReportConfigurationElOutputsEl {}
impl BuildFisExperimentTemplateExperimentReportConfigurationElOutputsEl {
    pub fn build(self) -> FisExperimentTemplateExperimentReportConfigurationElOutputsEl {
        FisExperimentTemplateExperimentReportConfigurationElOutputsEl {
            s3_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FisExperimentTemplateExperimentReportConfigurationElOutputsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateExperimentReportConfigurationElOutputsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FisExperimentTemplateExperimentReportConfigurationElOutputsElRef {
        FisExperimentTemplateExperimentReportConfigurationElOutputsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateExperimentReportConfigurationElOutputsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> ListRef<FisExperimentTemplateExperimentReportConfigurationElOutputsElS3ConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_configuration", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct FisExperimentTemplateExperimentReportConfigurationElDynamic {
    data_sources:
        Option<DynamicBlock<FisExperimentTemplateExperimentReportConfigurationElDataSourcesEl>>,
    outputs: Option<DynamicBlock<FisExperimentTemplateExperimentReportConfigurationElOutputsEl>>,
}
#[derive(Serialize)]
pub struct FisExperimentTemplateExperimentReportConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    post_experiment_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_experiment_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_sources: Option<Vec<FisExperimentTemplateExperimentReportConfigurationElDataSourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outputs: Option<Vec<FisExperimentTemplateExperimentReportConfigurationElOutputsEl>>,
    dynamic: FisExperimentTemplateExperimentReportConfigurationElDynamic,
}
impl FisExperimentTemplateExperimentReportConfigurationEl {
    #[doc = "Set the field `post_experiment_duration`.\n"]
    pub fn set_post_experiment_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.post_experiment_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `pre_experiment_duration`.\n"]
    pub fn set_pre_experiment_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pre_experiment_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `data_sources`.\n"]
    pub fn set_data_sources(
        mut self,
        v: impl Into<BlockAssignable<FisExperimentTemplateExperimentReportConfigurationElDataSourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_sources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `outputs`.\n"]
    pub fn set_outputs(
        mut self,
        v: impl Into<BlockAssignable<FisExperimentTemplateExperimentReportConfigurationElOutputsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.outputs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.outputs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FisExperimentTemplateExperimentReportConfigurationEl {
    type O = BlockAssignable<FisExperimentTemplateExperimentReportConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateExperimentReportConfigurationEl {}
impl BuildFisExperimentTemplateExperimentReportConfigurationEl {
    pub fn build(self) -> FisExperimentTemplateExperimentReportConfigurationEl {
        FisExperimentTemplateExperimentReportConfigurationEl {
            post_experiment_duration: core::default::Default::default(),
            pre_experiment_duration: core::default::Default::default(),
            data_sources: core::default::Default::default(),
            outputs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FisExperimentTemplateExperimentReportConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateExperimentReportConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FisExperimentTemplateExperimentReportConfigurationElRef {
        FisExperimentTemplateExperimentReportConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateExperimentReportConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `post_experiment_duration` after provisioning.\n"]
    pub fn post_experiment_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_experiment_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pre_experiment_duration` after provisioning.\n"]
    pub fn pre_experiment_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pre_experiment_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_sources` after provisioning.\n"]
    pub fn data_sources(
        &self,
    ) -> ListRef<FisExperimentTemplateExperimentReportConfigurationElDataSourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_sources", self.base))
    }
    #[doc = "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(
        &self,
    ) -> ListRef<FisExperimentTemplateExperimentReportConfigurationElOutputsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.outputs", self.base))
    }
}
#[derive(Serialize)]
pub struct FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationEl {
    log_group_arn: PrimField<String>,
}
impl FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationEl {}
impl ToListMappable for FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationEl {
    type O = BlockAssignable<FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationEl {
    #[doc = ""]
    pub log_group_arn: PrimField<String>,
}
impl BuildFisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationEl {
    pub fn build(self) -> FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationEl {
        FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationEl {
            log_group_arn: self.log_group_arn,
        }
    }
}
pub struct FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationElRef {
        FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_group_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FisExperimentTemplateLogConfigurationElS3ConfigurationEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}
impl FisExperimentTemplateLogConfigurationElS3ConfigurationEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}
impl ToListMappable for FisExperimentTemplateLogConfigurationElS3ConfigurationEl {
    type O = BlockAssignable<FisExperimentTemplateLogConfigurationElS3ConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateLogConfigurationElS3ConfigurationEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
}
impl BuildFisExperimentTemplateLogConfigurationElS3ConfigurationEl {
    pub fn build(self) -> FisExperimentTemplateLogConfigurationElS3ConfigurationEl {
        FisExperimentTemplateLogConfigurationElS3ConfigurationEl {
            bucket_name: self.bucket_name,
            prefix: core::default::Default::default(),
        }
    }
}
pub struct FisExperimentTemplateLogConfigurationElS3ConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateLogConfigurationElS3ConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FisExperimentTemplateLogConfigurationElS3ConfigurationElRef {
        FisExperimentTemplateLogConfigurationElS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateLogConfigurationElS3ConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize, Default)]
struct FisExperimentTemplateLogConfigurationElDynamic {
    cloudwatch_logs_configuration:
        Option<DynamicBlock<FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationEl>>,
    s3_configuration:
        Option<DynamicBlock<FisExperimentTemplateLogConfigurationElS3ConfigurationEl>>,
}
#[derive(Serialize)]
pub struct FisExperimentTemplateLogConfigurationEl {
    log_schema_version: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logs_configuration:
        Option<Vec<FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_configuration: Option<Vec<FisExperimentTemplateLogConfigurationElS3ConfigurationEl>>,
    dynamic: FisExperimentTemplateLogConfigurationElDynamic,
}
impl FisExperimentTemplateLogConfigurationEl {
    #[doc = "Set the field `cloudwatch_logs_configuration`.\n"]
    pub fn set_cloudwatch_logs_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logs_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v: impl Into<BlockAssignable<FisExperimentTemplateLogConfigurationElS3ConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FisExperimentTemplateLogConfigurationEl {
    type O = BlockAssignable<FisExperimentTemplateLogConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateLogConfigurationEl {
    #[doc = ""]
    pub log_schema_version: PrimField<f64>,
}
impl BuildFisExperimentTemplateLogConfigurationEl {
    pub fn build(self) -> FisExperimentTemplateLogConfigurationEl {
        FisExperimentTemplateLogConfigurationEl {
            log_schema_version: self.log_schema_version,
            cloudwatch_logs_configuration: core::default::Default::default(),
            s3_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FisExperimentTemplateLogConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateLogConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FisExperimentTemplateLogConfigurationElRef {
        FisExperimentTemplateLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateLogConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `log_schema_version` after provisioning.\n"]
    pub fn log_schema_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_schema_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloudwatch_logs_configuration` after provisioning.\n"]
    pub fn cloudwatch_logs_configuration(
        &self,
    ) -> ListRef<FisExperimentTemplateLogConfigurationElCloudwatchLogsConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logs_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> ListRef<FisExperimentTemplateLogConfigurationElS3ConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_configuration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FisExperimentTemplateStopConditionEl {
    source: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl FisExperimentTemplateStopConditionEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for FisExperimentTemplateStopConditionEl {
    type O = BlockAssignable<FisExperimentTemplateStopConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateStopConditionEl {
    #[doc = ""]
    pub source: PrimField<String>,
}
impl BuildFisExperimentTemplateStopConditionEl {
    pub fn build(self) -> FisExperimentTemplateStopConditionEl {
        FisExperimentTemplateStopConditionEl {
            source: self.source,
            value: core::default::Default::default(),
        }
    }
}
pub struct FisExperimentTemplateStopConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateStopConditionElRef {
    fn new(shared: StackShared, base: String) -> FisExperimentTemplateStopConditionElRef {
        FisExperimentTemplateStopConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateStopConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct FisExperimentTemplateTargetElFilterEl {
    path: PrimField<String>,
    values: SetField<PrimField<String>>,
}
impl FisExperimentTemplateTargetElFilterEl {}
impl ToListMappable for FisExperimentTemplateTargetElFilterEl {
    type O = BlockAssignable<FisExperimentTemplateTargetElFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateTargetElFilterEl {
    #[doc = ""]
    pub path: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}
impl BuildFisExperimentTemplateTargetElFilterEl {
    pub fn build(self) -> FisExperimentTemplateTargetElFilterEl {
        FisExperimentTemplateTargetElFilterEl {
            path: self.path,
            values: self.values,
        }
    }
}
pub struct FisExperimentTemplateTargetElFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateTargetElFilterElRef {
    fn new(shared: StackShared, base: String) -> FisExperimentTemplateTargetElFilterElRef {
        FisExperimentTemplateTargetElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateTargetElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct FisExperimentTemplateTargetElResourceTagEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl FisExperimentTemplateTargetElResourceTagEl {}
impl ToListMappable for FisExperimentTemplateTargetElResourceTagEl {
    type O = BlockAssignable<FisExperimentTemplateTargetElResourceTagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateTargetElResourceTagEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildFisExperimentTemplateTargetElResourceTagEl {
    pub fn build(self) -> FisExperimentTemplateTargetElResourceTagEl {
        FisExperimentTemplateTargetElResourceTagEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct FisExperimentTemplateTargetElResourceTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateTargetElResourceTagElRef {
    fn new(shared: StackShared, base: String) -> FisExperimentTemplateTargetElResourceTagElRef {
        FisExperimentTemplateTargetElResourceTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateTargetElResourceTagElRef {
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
struct FisExperimentTemplateTargetElDynamic {
    filter: Option<DynamicBlock<FisExperimentTemplateTargetElFilterEl>>,
    resource_tag: Option<DynamicBlock<FisExperimentTemplateTargetElResourceTagEl>>,
}
#[derive(Serialize)]
pub struct FisExperimentTemplateTargetEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_arns: Option<SetField<PrimField<String>>>,
    resource_type: PrimField<String>,
    selection_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<FisExperimentTemplateTargetElFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tag: Option<Vec<FisExperimentTemplateTargetElResourceTagEl>>,
    dynamic: FisExperimentTemplateTargetElDynamic,
}
impl FisExperimentTemplateTargetEl {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_arns`.\n"]
    pub fn set_resource_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resource_arns = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<BlockAssignable<FisExperimentTemplateTargetElFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_tag`.\n"]
    pub fn set_resource_tag(
        mut self,
        v: impl Into<BlockAssignable<FisExperimentTemplateTargetElResourceTagEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_tag = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_tag = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FisExperimentTemplateTargetEl {
    type O = BlockAssignable<FisExperimentTemplateTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateTargetEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub resource_type: PrimField<String>,
    #[doc = ""]
    pub selection_mode: PrimField<String>,
}
impl BuildFisExperimentTemplateTargetEl {
    pub fn build(self) -> FisExperimentTemplateTargetEl {
        FisExperimentTemplateTargetEl {
            name: self.name,
            parameters: core::default::Default::default(),
            resource_arns: core::default::Default::default(),
            resource_type: self.resource_type,
            selection_mode: self.selection_mode,
            filter: core::default::Default::default(),
            resource_tag: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FisExperimentTemplateTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateTargetElRef {
    fn new(shared: StackShared, base: String) -> FisExperimentTemplateTargetElRef {
        FisExperimentTemplateTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_arns` after provisioning.\n"]
    pub fn resource_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resource_arns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `selection_mode` after provisioning.\n"]
    pub fn selection_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.selection_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<FisExperimentTemplateTargetElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
#[derive(Serialize)]
pub struct FisExperimentTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FisExperimentTemplateTimeoutsEl {
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
impl ToListMappable for FisExperimentTemplateTimeoutsEl {
    type O = BlockAssignable<FisExperimentTemplateTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFisExperimentTemplateTimeoutsEl {}
impl BuildFisExperimentTemplateTimeoutsEl {
    pub fn build(self) -> FisExperimentTemplateTimeoutsEl {
        FisExperimentTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FisExperimentTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FisExperimentTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FisExperimentTemplateTimeoutsElRef {
        FisExperimentTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FisExperimentTemplateTimeoutsElRef {
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
struct FisExperimentTemplateDynamic {
    action: Option<DynamicBlock<FisExperimentTemplateActionEl>>,
    experiment_options: Option<DynamicBlock<FisExperimentTemplateExperimentOptionsEl>>,
    experiment_report_configuration:
        Option<DynamicBlock<FisExperimentTemplateExperimentReportConfigurationEl>>,
    log_configuration: Option<DynamicBlock<FisExperimentTemplateLogConfigurationEl>>,
    stop_condition: Option<DynamicBlock<FisExperimentTemplateStopConditionEl>>,
    target: Option<DynamicBlock<FisExperimentTemplateTargetEl>>,
}
