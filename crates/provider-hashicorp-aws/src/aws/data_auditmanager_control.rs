use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataAuditmanagerControlData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

struct DataAuditmanagerControl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAuditmanagerControlData>,
}

#[derive(Clone)]
pub struct DataAuditmanagerControl(Rc<DataAuditmanagerControl_>);

impl DataAuditmanagerControl {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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

    #[doc = "Get a reference to the value of field `control_mapping_sources` after provisioning.\n"]
    pub fn control_mapping_sources(
        &self,
    ) -> ListRef<DataAuditmanagerControlControlMappingSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_mapping_sources", self.extract_ref()),
        )
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

impl Referable for DataAuditmanagerControl {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataAuditmanagerControl {}

impl ToListMappable for DataAuditmanagerControl {
    type O = ListRef<DataAuditmanagerControlRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAuditmanagerControl_ {
    fn extract_datasource_type(&self) -> String {
        "aws_auditmanager_control".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAuditmanagerControl {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildDataAuditmanagerControl {
    pub fn build(self, stack: &mut Stack) -> DataAuditmanagerControl {
        let out = DataAuditmanagerControl(Rc::new(DataAuditmanagerControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAuditmanagerControlData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                name: self.name,
                region: core::default::Default::default(),
                type_: self.type_,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAuditmanagerControlRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAuditmanagerControlRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataAuditmanagerControlRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `control_mapping_sources` after provisioning.\n"]
    pub fn control_mapping_sources(
        &self,
    ) -> ListRef<DataAuditmanagerControlControlMappingSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_mapping_sources", self.extract_ref()),
        )
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
pub struct DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    keyword_input_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keyword_value: Option<PrimField<String>>,
}

impl DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
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

impl ToListMappable for DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    type O = BlockAssignable<DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {}

impl BuildDataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    pub fn build(self) -> DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
        DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
            keyword_input_type: core::default::Default::default(),
            keyword_value: core::default::Default::default(),
        }
    }
}

pub struct DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
        DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
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
pub struct DataAuditmanagerControlControlMappingSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_frequency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_keyword:
        Option<ListField<DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_set_up_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    troubleshooting_text: Option<PrimField<String>>,
}

impl DataAuditmanagerControlControlMappingSourcesEl {
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

    #[doc = "Set the field `source_id`.\n"]
    pub fn set_source_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_id = Some(v.into());
        self
    }

    #[doc = "Set the field `source_keyword`.\n"]
    pub fn set_source_keyword(
        mut self,
        v: impl Into<ListField<DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
    ) -> Self {
        self.source_keyword = Some(v.into());
        self
    }

    #[doc = "Set the field `source_name`.\n"]
    pub fn set_source_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_name = Some(v.into());
        self
    }

    #[doc = "Set the field `source_set_up_option`.\n"]
    pub fn set_source_set_up_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_set_up_option = Some(v.into());
        self
    }

    #[doc = "Set the field `source_type`.\n"]
    pub fn set_source_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_type = Some(v.into());
        self
    }

    #[doc = "Set the field `troubleshooting_text`.\n"]
    pub fn set_troubleshooting_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.troubleshooting_text = Some(v.into());
        self
    }
}

impl ToListMappable for DataAuditmanagerControlControlMappingSourcesEl {
    type O = BlockAssignable<DataAuditmanagerControlControlMappingSourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAuditmanagerControlControlMappingSourcesEl {}

impl BuildDataAuditmanagerControlControlMappingSourcesEl {
    pub fn build(self) -> DataAuditmanagerControlControlMappingSourcesEl {
        DataAuditmanagerControlControlMappingSourcesEl {
            source_description: core::default::Default::default(),
            source_frequency: core::default::Default::default(),
            source_id: core::default::Default::default(),
            source_keyword: core::default::Default::default(),
            source_name: core::default::Default::default(),
            source_set_up_option: core::default::Default::default(),
            source_type: core::default::Default::default(),
            troubleshooting_text: core::default::Default::default(),
        }
    }
}

pub struct DataAuditmanagerControlControlMappingSourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAuditmanagerControlControlMappingSourcesElRef {
    fn new(shared: StackShared, base: String) -> DataAuditmanagerControlControlMappingSourcesElRef {
        DataAuditmanagerControlControlMappingSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAuditmanagerControlControlMappingSourcesElRef {
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
    ) -> ListRef<DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef> {
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
