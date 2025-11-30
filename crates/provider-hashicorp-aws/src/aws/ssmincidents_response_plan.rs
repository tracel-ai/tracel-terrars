use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SsmincidentsResponsePlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_channel: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engagements: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<SsmincidentsResponsePlanActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    incident_template: Option<Vec<SsmincidentsResponsePlanIncidentTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration: Option<Vec<SsmincidentsResponsePlanIntegrationEl>>,
    dynamic: SsmincidentsResponsePlanDynamic,
}

struct SsmincidentsResponsePlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsmincidentsResponsePlanData>,
}

#[derive(Clone)]
pub struct SsmincidentsResponsePlan(Rc<SsmincidentsResponsePlan_>);

impl SsmincidentsResponsePlan {
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

    #[doc = "Set the field `chat_channel`.\n"]
    pub fn set_chat_channel(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().chat_channel = Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }

    #[doc = "Set the field `engagements`.\n"]
    pub fn set_engagements(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().engagements = Some(v.into());
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
    pub fn set_action(
        self,
        v: impl Into<BlockAssignable<SsmincidentsResponsePlanActionEl>>,
    ) -> Self {
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

    #[doc = "Set the field `incident_template`.\n"]
    pub fn set_incident_template(
        self,
        v: impl Into<BlockAssignable<SsmincidentsResponsePlanIncidentTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().incident_template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.incident_template = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `integration`.\n"]
    pub fn set_integration(
        self,
        v: impl Into<BlockAssignable<SsmincidentsResponsePlanIntegrationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().integration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.integration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chat_channel` after provisioning.\n"]
    pub fn chat_channel(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.chat_channel", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engagements` after provisioning.\n"]
    pub fn engagements(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.engagements", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<SsmincidentsResponsePlanActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `incident_template` after provisioning.\n"]
    pub fn incident_template(&self) -> ListRef<SsmincidentsResponsePlanIncidentTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.incident_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> ListRef<SsmincidentsResponsePlanIntegrationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.integration", self.extract_ref()),
        )
    }
}

impl Referable for SsmincidentsResponsePlan {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SsmincidentsResponsePlan {}

impl ToListMappable for SsmincidentsResponsePlan {
    type O = ListRef<SsmincidentsResponsePlanRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SsmincidentsResponsePlan_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssmincidents_response_plan".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmincidentsResponsePlan {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildSsmincidentsResponsePlan {
    pub fn build(self, stack: &mut Stack) -> SsmincidentsResponsePlan {
        let out = SsmincidentsResponsePlan(Rc::new(SsmincidentsResponsePlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SsmincidentsResponsePlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                chat_channel: core::default::Default::default(),
                display_name: core::default::Default::default(),
                engagements: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                action: core::default::Default::default(),
                incident_template: core::default::Default::default(),
                integration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmincidentsResponsePlanRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmincidentsResponsePlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SsmincidentsResponsePlanRef {
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

    #[doc = "Get a reference to the value of field `chat_channel` after provisioning.\n"]
    pub fn chat_channel(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.chat_channel", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engagements` after provisioning.\n"]
    pub fn engagements(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.engagements", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<SsmincidentsResponsePlanActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `incident_template` after provisioning.\n"]
    pub fn incident_template(&self) -> ListRef<SsmincidentsResponsePlanIncidentTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.incident_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> ListRef<SsmincidentsResponsePlanIntegrationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.integration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SsmincidentsResponsePlanActionElSsmAutomationElParameterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl SsmincidentsResponsePlanActionElSsmAutomationElParameterEl {}

impl ToListMappable for SsmincidentsResponsePlanActionElSsmAutomationElParameterEl {
    type O = BlockAssignable<SsmincidentsResponsePlanActionElSsmAutomationElParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlanActionElSsmAutomationElParameterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildSsmincidentsResponsePlanActionElSsmAutomationElParameterEl {
    pub fn build(self) -> SsmincidentsResponsePlanActionElSsmAutomationElParameterEl {
        SsmincidentsResponsePlanActionElSsmAutomationElParameterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct SsmincidentsResponsePlanActionElSsmAutomationElParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmincidentsResponsePlanActionElSsmAutomationElParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmincidentsResponsePlanActionElSsmAutomationElParameterElRef {
        SsmincidentsResponsePlanActionElSsmAutomationElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlanActionElSsmAutomationElParameterElRef {
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

#[derive(Serialize, Default)]
struct SsmincidentsResponsePlanActionElSsmAutomationElDynamic {
    parameter: Option<DynamicBlock<SsmincidentsResponsePlanActionElSsmAutomationElParameterEl>>,
}

#[derive(Serialize)]
pub struct SsmincidentsResponsePlanActionElSsmAutomationEl {
    document_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamic_parameters: Option<RecField<PrimField<String>>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<Vec<SsmincidentsResponsePlanActionElSsmAutomationElParameterEl>>,
    dynamic: SsmincidentsResponsePlanActionElSsmAutomationElDynamic,
}

impl SsmincidentsResponsePlanActionElSsmAutomationEl {
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

    #[doc = "Set the field `target_account`.\n"]
    pub fn set_target_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_account = Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl Into<BlockAssignable<SsmincidentsResponsePlanActionElSsmAutomationElParameterEl>>,
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
}

impl ToListMappable for SsmincidentsResponsePlanActionElSsmAutomationEl {
    type O = BlockAssignable<SsmincidentsResponsePlanActionElSsmAutomationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlanActionElSsmAutomationEl {
    #[doc = ""]
    pub document_name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildSsmincidentsResponsePlanActionElSsmAutomationEl {
    pub fn build(self) -> SsmincidentsResponsePlanActionElSsmAutomationEl {
        SsmincidentsResponsePlanActionElSsmAutomationEl {
            document_name: self.document_name,
            document_version: core::default::Default::default(),
            dynamic_parameters: core::default::Default::default(),
            role_arn: self.role_arn,
            target_account: core::default::Default::default(),
            parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmincidentsResponsePlanActionElSsmAutomationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmincidentsResponsePlanActionElSsmAutomationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmincidentsResponsePlanActionElSsmAutomationElRef {
        SsmincidentsResponsePlanActionElSsmAutomationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlanActionElSsmAutomationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `document_name` after provisioning.\n"]
    pub fn document_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_version", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dynamic_parameters` after provisioning.\n"]
    pub fn dynamic_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.dynamic_parameters", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `target_account` after provisioning.\n"]
    pub fn target_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_account", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SsmincidentsResponsePlanActionElDynamic {
    ssm_automation: Option<DynamicBlock<SsmincidentsResponsePlanActionElSsmAutomationEl>>,
}

#[derive(Serialize)]
pub struct SsmincidentsResponsePlanActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ssm_automation: Option<Vec<SsmincidentsResponsePlanActionElSsmAutomationEl>>,
    dynamic: SsmincidentsResponsePlanActionElDynamic,
}

impl SsmincidentsResponsePlanActionEl {
    #[doc = "Set the field `ssm_automation`.\n"]
    pub fn set_ssm_automation(
        mut self,
        v: impl Into<BlockAssignable<SsmincidentsResponsePlanActionElSsmAutomationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssm_automation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssm_automation = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SsmincidentsResponsePlanActionEl {
    type O = BlockAssignable<SsmincidentsResponsePlanActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlanActionEl {}

impl BuildSsmincidentsResponsePlanActionEl {
    pub fn build(self) -> SsmincidentsResponsePlanActionEl {
        SsmincidentsResponsePlanActionEl {
            ssm_automation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmincidentsResponsePlanActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmincidentsResponsePlanActionElRef {
    fn new(shared: StackShared, base: String) -> SsmincidentsResponsePlanActionElRef {
        SsmincidentsResponsePlanActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlanActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ssm_automation` after provisioning.\n"]
    pub fn ssm_automation(&self) -> ListRef<SsmincidentsResponsePlanActionElSsmAutomationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssm_automation", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {
    sns_topic_arn: PrimField<String>,
}

impl SsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {}

impl ToListMappable for SsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {
    type O = BlockAssignable<SsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {
    #[doc = ""]
    pub sns_topic_arn: PrimField<String>,
}

impl BuildSsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {
    pub fn build(self) -> SsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {
        SsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl {
            sns_topic_arn: self.sns_topic_arn,
        }
    }
}

pub struct SsmincidentsResponsePlanIncidentTemplateElNotificationTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmincidentsResponsePlanIncidentTemplateElNotificationTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmincidentsResponsePlanIncidentTemplateElNotificationTargetElRef {
        SsmincidentsResponsePlanIncidentTemplateElNotificationTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlanIncidentTemplateElNotificationTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sns_topic_arn", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SsmincidentsResponsePlanIncidentTemplateElDynamic {
    notification_target:
        Option<DynamicBlock<SsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl>>,
}

#[derive(Serialize)]
pub struct SsmincidentsResponsePlanIncidentTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dedupe_string: Option<PrimField<String>>,
    impact: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    incident_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<PrimField<String>>,
    title: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_target:
        Option<Vec<SsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl>>,
    dynamic: SsmincidentsResponsePlanIncidentTemplateElDynamic,
}

impl SsmincidentsResponsePlanIncidentTemplateEl {
    #[doc = "Set the field `dedupe_string`.\n"]
    pub fn set_dedupe_string(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dedupe_string = Some(v.into());
        self
    }

    #[doc = "Set the field `incident_tags`.\n"]
    pub fn set_incident_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.incident_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `summary`.\n"]
    pub fn set_summary(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.summary = Some(v.into());
        self
    }

    #[doc = "Set the field `notification_target`.\n"]
    pub fn set_notification_target(
        mut self,
        v: impl Into<BlockAssignable<SsmincidentsResponsePlanIncidentTemplateElNotificationTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.notification_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_target = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SsmincidentsResponsePlanIncidentTemplateEl {
    type O = BlockAssignable<SsmincidentsResponsePlanIncidentTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlanIncidentTemplateEl {
    #[doc = ""]
    pub impact: PrimField<f64>,
    #[doc = ""]
    pub title: PrimField<String>,
}

impl BuildSsmincidentsResponsePlanIncidentTemplateEl {
    pub fn build(self) -> SsmincidentsResponsePlanIncidentTemplateEl {
        SsmincidentsResponsePlanIncidentTemplateEl {
            dedupe_string: core::default::Default::default(),
            impact: self.impact,
            incident_tags: core::default::Default::default(),
            summary: core::default::Default::default(),
            title: self.title,
            notification_target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmincidentsResponsePlanIncidentTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmincidentsResponsePlanIncidentTemplateElRef {
    fn new(shared: StackShared, base: String) -> SsmincidentsResponsePlanIncidentTemplateElRef {
        SsmincidentsResponsePlanIncidentTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlanIncidentTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dedupe_string` after provisioning.\n"]
    pub fn dedupe_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dedupe_string", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `impact` after provisioning.\n"]
    pub fn impact(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.impact", self.base))
    }

    #[doc = "Get a reference to the value of field `incident_tags` after provisioning.\n"]
    pub fn incident_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.incident_tags", self.base),
        )
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
pub struct SsmincidentsResponsePlanIntegrationElPagerdutyEl {
    name: PrimField<String>,
    secret_id: PrimField<String>,
    service_id: PrimField<String>,
}

impl SsmincidentsResponsePlanIntegrationElPagerdutyEl {}

impl ToListMappable for SsmincidentsResponsePlanIntegrationElPagerdutyEl {
    type O = BlockAssignable<SsmincidentsResponsePlanIntegrationElPagerdutyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlanIntegrationElPagerdutyEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub secret_id: PrimField<String>,
    #[doc = ""]
    pub service_id: PrimField<String>,
}

impl BuildSsmincidentsResponsePlanIntegrationElPagerdutyEl {
    pub fn build(self) -> SsmincidentsResponsePlanIntegrationElPagerdutyEl {
        SsmincidentsResponsePlanIntegrationElPagerdutyEl {
            name: self.name,
            secret_id: self.secret_id,
            service_id: self.service_id,
        }
    }
}

pub struct SsmincidentsResponsePlanIntegrationElPagerdutyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmincidentsResponsePlanIntegrationElPagerdutyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmincidentsResponsePlanIntegrationElPagerdutyElRef {
        SsmincidentsResponsePlanIntegrationElPagerdutyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlanIntegrationElPagerdutyElRef {
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

#[derive(Serialize, Default)]
struct SsmincidentsResponsePlanIntegrationElDynamic {
    pagerduty: Option<DynamicBlock<SsmincidentsResponsePlanIntegrationElPagerdutyEl>>,
}

#[derive(Serialize)]
pub struct SsmincidentsResponsePlanIntegrationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pagerduty: Option<Vec<SsmincidentsResponsePlanIntegrationElPagerdutyEl>>,
    dynamic: SsmincidentsResponsePlanIntegrationElDynamic,
}

impl SsmincidentsResponsePlanIntegrationEl {
    #[doc = "Set the field `pagerduty`.\n"]
    pub fn set_pagerduty(
        mut self,
        v: impl Into<BlockAssignable<SsmincidentsResponsePlanIntegrationElPagerdutyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pagerduty = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pagerduty = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SsmincidentsResponsePlanIntegrationEl {
    type O = BlockAssignable<SsmincidentsResponsePlanIntegrationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlanIntegrationEl {}

impl BuildSsmincidentsResponsePlanIntegrationEl {
    pub fn build(self) -> SsmincidentsResponsePlanIntegrationEl {
        SsmincidentsResponsePlanIntegrationEl {
            pagerduty: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmincidentsResponsePlanIntegrationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmincidentsResponsePlanIntegrationElRef {
    fn new(shared: StackShared, base: String) -> SsmincidentsResponsePlanIntegrationElRef {
        SsmincidentsResponsePlanIntegrationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlanIntegrationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pagerduty` after provisioning.\n"]
    pub fn pagerduty(&self) -> ListRef<SsmincidentsResponsePlanIntegrationElPagerdutyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pagerduty", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmincidentsResponsePlanDynamic {
    action: Option<DynamicBlock<SsmincidentsResponsePlanActionEl>>,
    incident_template: Option<DynamicBlock<SsmincidentsResponsePlanIncidentTemplateEl>>,
    integration: Option<DynamicBlock<SsmincidentsResponsePlanIntegrationEl>>,
}
