use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsmincidentsResponsePlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataSsmincidentsResponsePlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSsmincidentsResponsePlanData>,
}

#[derive(Clone)]
pub struct DataSsmincidentsResponsePlan(Rc<DataSsmincidentsResponsePlan_>);

impl DataSsmincidentsResponsePlan {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataSsmincidentsResponsePlanActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chat_channel` after provisioning.\n"]
    pub fn chat_channel(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.chat_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engagements` after provisioning.\n"]
    pub fn engagements(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.engagements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `incident_template` after provisioning.\n"]
    pub fn incident_template(&self) -> ListRef<DataSsmincidentsResponsePlanIncidentTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.incident_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> ListRef<DataSsmincidentsResponsePlanIntegrationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.integration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataSsmincidentsResponsePlan {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSsmincidentsResponsePlan { }

impl ToListMappable for DataSsmincidentsResponsePlan {
    type O = ListRef<DataSsmincidentsResponsePlanRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSsmincidentsResponsePlan_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ssmincidents_response_plan".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsmincidentsResponsePlan {
    pub tf_id: String,
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildDataSsmincidentsResponsePlan {
    pub fn build(self, stack: &mut Stack) -> DataSsmincidentsResponsePlan {
        let out = DataSsmincidentsResponsePlan(Rc::new(DataSsmincidentsResponsePlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSsmincidentsResponsePlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: self.arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsmincidentsResponsePlanRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmincidentsResponsePlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataSsmincidentsResponsePlanRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataSsmincidentsResponsePlanActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chat_channel` after provisioning.\n"]
    pub fn chat_channel(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.chat_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engagements` after provisioning.\n"]
    pub fn engagements(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.engagements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `incident_template` after provisioning.\n"]
    pub fn incident_template(&self) -> ListRef<DataSsmincidentsResponsePlanIncidentTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.incident_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> ListRef<DataSsmincidentsResponsePlanIntegrationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.integration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlanActionElSsmAutomationElParameterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataSsmincidentsResponsePlanActionElSsmAutomationElParameterEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmincidentsResponsePlanActionElSsmAutomationElParameterEl {
    type O = BlockAssignable<DataSsmincidentsResponsePlanActionElSsmAutomationElParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlanActionElSsmAutomationElParameterEl {}

impl BuildDataSsmincidentsResponsePlanActionElSsmAutomationElParameterEl {
    pub fn build(self) -> DataSsmincidentsResponsePlanActionElSsmAutomationElParameterEl {
        DataSsmincidentsResponsePlanActionElSsmAutomationElParameterEl {
            name: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataSsmincidentsResponsePlanActionElSsmAutomationElParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmincidentsResponsePlanActionElSsmAutomationElParameterElRef {
    fn new(shared: StackShared, base: String) -> DataSsmincidentsResponsePlanActionElSsmAutomationElParameterElRef {
        DataSsmincidentsResponsePlanActionElSsmAutomationElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlanActionElSsmAutomationElParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlanActionElSsmAutomationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    document_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamic_parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<SetField<DataSsmincidentsResponsePlanActionElSsmAutomationElParameterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_account: Option<PrimField<String>>,
}

impl DataSsmincidentsResponsePlanActionElSsmAutomationEl {
    #[doc = "Set the field `document_name`.\n"]
    pub fn set_document_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.document_name = Some(v.into());
        self
    }

    #[doc = "Set the field `document_version`.\n"]
    pub fn set_document_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.document_version = Some(v.into());
        self
    }

    #[doc = "Set the field `dynamic_parameters`.\n"]
    pub fn set_dynamic_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.dynamic_parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl Into<SetField<DataSsmincidentsResponsePlanActionElSsmAutomationElParameterEl>>,
    ) -> Self {
        self.parameter = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `target_account`.\n"]
    pub fn set_target_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_account = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmincidentsResponsePlanActionElSsmAutomationEl {
    type O = BlockAssignable<DataSsmincidentsResponsePlanActionElSsmAutomationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlanActionElSsmAutomationEl {}

impl BuildDataSsmincidentsResponsePlanActionElSsmAutomationEl {
    pub fn build(self) -> DataSsmincidentsResponsePlanActionElSsmAutomationEl {
        DataSsmincidentsResponsePlanActionElSsmAutomationEl {
            document_name: core::default::Default::default(),
            document_version: core::default::Default::default(),
            dynamic_parameters: core::default::Default::default(),
            parameter: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            target_account: core::default::Default::default(),
        }
    }
}

pub struct DataSsmincidentsResponsePlanActionElSsmAutomationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmincidentsResponsePlanActionElSsmAutomationElRef {
    fn new(shared: StackShared, base: String) -> DataSsmincidentsResponsePlanActionElSsmAutomationElRef {
        DataSsmincidentsResponsePlanActionElSsmAutomationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlanActionElSsmAutomationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `document_name` after provisioning.\n"]
    pub fn document_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_name", self.base))
    }

    #[doc = "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_version", self.base))
    }

    #[doc = "Get a reference to the value of field `dynamic_parameters` after provisioning.\n"]
    pub fn dynamic_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.dynamic_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter` after provisioning.\n"]
    pub fn parameter(&self) -> SetRef<DataSsmincidentsResponsePlanActionElSsmAutomationElParameterElRef> {
        SetRef::new(self.shared().clone(), format!("{}.parameter", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `target_account` after provisioning.\n"]
    pub fn target_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_account", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlanActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ssm_automation: Option<ListField<DataSsmincidentsResponsePlanActionElSsmAutomationEl>>,
}

impl DataSsmincidentsResponsePlanActionEl {
    #[doc = "Set the field `ssm_automation`.\n"]
    pub fn set_ssm_automation(
        mut self,
        v: impl Into<ListField<DataSsmincidentsResponsePlanActionElSsmAutomationEl>>,
    ) -> Self {
        self.ssm_automation = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmincidentsResponsePlanActionEl {
    type O = BlockAssignable<DataSsmincidentsResponsePlanActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlanActionEl {}

impl BuildDataSsmincidentsResponsePlanActionEl {
    pub fn build(self) -> DataSsmincidentsResponsePlanActionEl {
        DataSsmincidentsResponsePlanActionEl { ssm_automation: core::default::Default::default() }
    }
}

pub struct DataSsmincidentsResponsePlanActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmincidentsResponsePlanActionElRef {
    fn new(shared: StackShared, base: String) -> DataSsmincidentsResponsePlanActionElRef {
        DataSsmincidentsResponsePlanActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlanActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ssm_automation` after provisioning.\n"]
    pub fn ssm_automation(&self) -> ListRef<DataSsmincidentsResponsePlanActionElSsmAutomationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssm_automation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sns_topic_arn: Option<PrimField<String>>,
}

impl DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {
    #[doc = "Set the field `sns_topic_arn`.\n"]
    pub fn set_sns_topic_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sns_topic_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {
    type O = BlockAssignable<DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {}

impl BuildDataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {
    pub fn build(self) -> DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {
        DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {
            sns_topic_arn: core::default::Default::default(),
        }
    }
}

pub struct DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetElRef {
        DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlanIncidentTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dedupe_string: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    impact: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    incident_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_target: Option<SetField<DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}

impl DataSsmincidentsResponsePlanIncidentTemplateEl {
    #[doc = "Set the field `dedupe_string`.\n"]
    pub fn set_dedupe_string(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dedupe_string = Some(v.into());
        self
    }

    #[doc = "Set the field `impact`.\n"]
    pub fn set_impact(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.impact = Some(v.into());
        self
    }

    #[doc = "Set the field `incident_tags`.\n"]
    pub fn set_incident_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.incident_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `notification_target`.\n"]
    pub fn set_notification_target(
        mut self,
        v: impl Into<SetField<DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl>>,
    ) -> Self {
        self.notification_target = Some(v.into());
        self
    }

    #[doc = "Set the field `summary`.\n"]
    pub fn set_summary(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.summary = Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmincidentsResponsePlanIncidentTemplateEl {
    type O = BlockAssignable<DataSsmincidentsResponsePlanIncidentTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlanIncidentTemplateEl {}

impl BuildDataSsmincidentsResponsePlanIncidentTemplateEl {
    pub fn build(self) -> DataSsmincidentsResponsePlanIncidentTemplateEl {
        DataSsmincidentsResponsePlanIncidentTemplateEl {
            dedupe_string: core::default::Default::default(),
            impact: core::default::Default::default(),
            incident_tags: core::default::Default::default(),
            notification_target: core::default::Default::default(),
            summary: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct DataSsmincidentsResponsePlanIncidentTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmincidentsResponsePlanIncidentTemplateElRef {
    fn new(shared: StackShared, base: String) -> DataSsmincidentsResponsePlanIncidentTemplateElRef {
        DataSsmincidentsResponsePlanIncidentTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlanIncidentTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dedupe_string` after provisioning.\n"]
    pub fn dedupe_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dedupe_string", self.base))
    }

    #[doc = "Get a reference to the value of field `impact` after provisioning.\n"]
    pub fn impact(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.impact", self.base))
    }

    #[doc = "Get a reference to the value of field `incident_tags` after provisioning.\n"]
    pub fn incident_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.incident_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_target` after provisioning.\n"]
    pub fn notification_target(&self) -> SetRef<DataSsmincidentsResponsePlanIncidentTemplateElNotificationTargetElRef> {
        SetRef::new(self.shared().clone(), format!("{}.notification_target", self.base))
    }

    #[doc = "Get a reference to the value of field `summary` after provisioning.\n"]
    pub fn summary(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.summary", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlanIntegrationElPagerdutyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_id: Option<PrimField<String>>,
}

impl DataSsmincidentsResponsePlanIntegrationElPagerdutyEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `secret_id`.\n"]
    pub fn set_secret_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_id = Some(v.into());
        self
    }

    #[doc = "Set the field `service_id`.\n"]
    pub fn set_service_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmincidentsResponsePlanIntegrationElPagerdutyEl {
    type O = BlockAssignable<DataSsmincidentsResponsePlanIntegrationElPagerdutyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlanIntegrationElPagerdutyEl {}

impl BuildDataSsmincidentsResponsePlanIntegrationElPagerdutyEl {
    pub fn build(self) -> DataSsmincidentsResponsePlanIntegrationElPagerdutyEl {
        DataSsmincidentsResponsePlanIntegrationElPagerdutyEl {
            name: core::default::Default::default(),
            secret_id: core::default::Default::default(),
            service_id: core::default::Default::default(),
        }
    }
}

pub struct DataSsmincidentsResponsePlanIntegrationElPagerdutyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmincidentsResponsePlanIntegrationElPagerdutyElRef {
    fn new(shared: StackShared, base: String) -> DataSsmincidentsResponsePlanIntegrationElPagerdutyElRef {
        DataSsmincidentsResponsePlanIntegrationElPagerdutyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlanIntegrationElPagerdutyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlanIntegrationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pagerduty: Option<ListField<DataSsmincidentsResponsePlanIntegrationElPagerdutyEl>>,
}

impl DataSsmincidentsResponsePlanIntegrationEl {
    #[doc = "Set the field `pagerduty`.\n"]
    pub fn set_pagerduty(
        mut self,
        v: impl Into<ListField<DataSsmincidentsResponsePlanIntegrationElPagerdutyEl>>,
    ) -> Self {
        self.pagerduty = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmincidentsResponsePlanIntegrationEl {
    type O = BlockAssignable<DataSsmincidentsResponsePlanIntegrationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlanIntegrationEl {}

impl BuildDataSsmincidentsResponsePlanIntegrationEl {
    pub fn build(self) -> DataSsmincidentsResponsePlanIntegrationEl {
        DataSsmincidentsResponsePlanIntegrationEl { pagerduty: core::default::Default::default() }
    }
}

pub struct DataSsmincidentsResponsePlanIntegrationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmincidentsResponsePlanIntegrationElRef {
    fn new(shared: StackShared, base: String) -> DataSsmincidentsResponsePlanIntegrationElRef {
        DataSsmincidentsResponsePlanIntegrationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlanIntegrationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pagerduty` after provisioning.\n"]
    pub fn pagerduty(&self) -> ListRef<DataSsmincidentsResponsePlanIntegrationElPagerdutyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pagerduty", self.base))
    }
}
