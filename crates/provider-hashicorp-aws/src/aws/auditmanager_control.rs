use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AuditmanagerControlData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_plan_instructions: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_plan_title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    testing_information: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_mapping_sources: Option<Vec<AuditmanagerControlControlMappingSourcesEl>>,
    dynamic: AuditmanagerControlDynamic,
}
struct AuditmanagerControl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AuditmanagerControlData>,
}
#[derive(Clone)]
pub struct AuditmanagerControl(Rc<AuditmanagerControl_>);
impl AuditmanagerControl {
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
    #[doc = "Set the field `action_plan_instructions`.\n"]
    pub fn set_action_plan_instructions(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().action_plan_instructions = Some(v.into());
        self
    }
    #[doc = "Set the field `action_plan_title`.\n"]
    pub fn set_action_plan_title(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().action_plan_title = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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
    #[doc = "Set the field `testing_information`.\n"]
    pub fn set_testing_information(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().testing_information = Some(v.into());
        self
    }
    #[doc = "Set the field `control_mapping_sources`.\n"]
    pub fn set_control_mapping_sources(
        self,
        v: impl Into<BlockAssignable<AuditmanagerControlControlMappingSourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_mapping_sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_mapping_sources = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `action_plan_instructions` after provisioning.\n"]
    pub fn action_plan_instructions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_plan_instructions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `action_plan_title` after provisioning.\n"]
    pub fn action_plan_title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_plan_title", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `testing_information` after provisioning.\n"]
    pub fn testing_information(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.testing_information", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
impl Referable for AuditmanagerControl {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AuditmanagerControl {}
impl ToListMappable for AuditmanagerControl {
    type O = ListRef<AuditmanagerControlRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AuditmanagerControl_ {
    fn extract_resource_type(&self) -> String {
        "aws_auditmanager_control".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAuditmanagerControl {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildAuditmanagerControl {
    pub fn build(self, stack: &mut Stack) -> AuditmanagerControl {
        let out = AuditmanagerControl(Rc::new(AuditmanagerControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AuditmanagerControlData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action_plan_instructions: core::default::Default::default(),
                action_plan_title: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                testing_information: core::default::Default::default(),
                control_mapping_sources: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AuditmanagerControlRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerControlRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AuditmanagerControlRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action_plan_instructions` after provisioning.\n"]
    pub fn action_plan_instructions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_plan_instructions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `action_plan_title` after provisioning.\n"]
    pub fn action_plan_title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_plan_title", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `testing_information` after provisioning.\n"]
    pub fn testing_information(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.testing_information", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    keyword_input_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keyword_value: Option<PrimField<String>>,
}
impl AuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    #[doc = "Set the field `keyword_input_type`.\n"]
    pub fn set_keyword_input_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.keyword_input_type = Some(v.into());
        self
    }
    #[doc = "Set the field `keyword_value`.\n"]
    pub fn set_keyword_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.keyword_value = Some(v.into());
        self
    }
}
impl ToListMappable for AuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    type O = BlockAssignable<AuditmanagerControlControlMappingSourcesElSourceKeywordEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAuditmanagerControlControlMappingSourcesElSourceKeywordEl {}
impl BuildAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    pub fn build(self) -> AuditmanagerControlControlMappingSourcesElSourceKeywordEl {
        AuditmanagerControlControlMappingSourcesElSourceKeywordEl {
            keyword_input_type: core::default::Default::default(),
            keyword_value: core::default::Default::default(),
        }
    }
}
pub struct AuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
        AuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `keyword_input_type` after provisioning.\n"]
    pub fn keyword_input_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keyword_input_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `keyword_value` after provisioning.\n"]
    pub fn keyword_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keyword_value", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AuditmanagerControlControlMappingSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_frequency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_keyword: Option<ListField<AuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
    source_name: PrimField<String>,
    source_set_up_option: PrimField<String>,
    source_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    troubleshooting_text: Option<PrimField<String>>,
}
impl AuditmanagerControlControlMappingSourcesEl {
    #[doc = "Set the field `source_description`.\n"]
    pub fn set_source_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_description = Some(v.into());
        self
    }
    #[doc = "Set the field `source_frequency`.\n"]
    pub fn set_source_frequency(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `source_keyword`.\n"]
    pub fn set_source_keyword(
        mut self,
        v: impl Into<ListField<AuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
    ) -> Self {
        self.source_keyword = Some(v.into());
        self
    }
    #[doc = "Set the field `troubleshooting_text`.\n"]
    pub fn set_troubleshooting_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.troubleshooting_text = Some(v.into());
        self
    }
}
impl ToListMappable for AuditmanagerControlControlMappingSourcesEl {
    type O = BlockAssignable<AuditmanagerControlControlMappingSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAuditmanagerControlControlMappingSourcesEl {
    #[doc = ""]
    pub source_name: PrimField<String>,
    #[doc = ""]
    pub source_set_up_option: PrimField<String>,
    #[doc = ""]
    pub source_type: PrimField<String>,
}
impl BuildAuditmanagerControlControlMappingSourcesEl {
    pub fn build(self) -> AuditmanagerControlControlMappingSourcesEl {
        AuditmanagerControlControlMappingSourcesEl {
            source_description: core::default::Default::default(),
            source_frequency: core::default::Default::default(),
            source_keyword: core::default::Default::default(),
            source_name: self.source_name,
            source_set_up_option: self.source_set_up_option,
            source_type: self.source_type,
            troubleshooting_text: core::default::Default::default(),
        }
    }
}
pub struct AuditmanagerControlControlMappingSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerControlControlMappingSourcesElRef {
    fn new(shared: StackShared, base: String) -> AuditmanagerControlControlMappingSourcesElRef {
        AuditmanagerControlControlMappingSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AuditmanagerControlControlMappingSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_description` after provisioning.\n"]
    pub fn source_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_description", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_frequency` after provisioning.\n"]
    pub fn source_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_frequency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_id` after provisioning.\n"]
    pub fn source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.base))
    }
    #[doc = "Get a reference to the value of field `source_keyword` after provisioning.\n"]
    pub fn source_keyword(
        &self,
    ) -> ListRef<AuditmanagerControlControlMappingSourcesElSourceKeywordElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_keyword", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.base))
    }
    #[doc = "Get a reference to the value of field `source_set_up_option` after provisioning.\n"]
    pub fn source_set_up_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_set_up_option", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_type` after provisioning.\n"]
    pub fn source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }
    #[doc = "Get a reference to the value of field `troubleshooting_text` after provisioning.\n"]
    pub fn troubleshooting_text(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.troubleshooting_text", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AuditmanagerControlDynamic {
    control_mapping_sources: Option<DynamicBlock<AuditmanagerControlControlMappingSourcesEl>>,
}
