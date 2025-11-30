use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataEcrLifecyclePolicyDocumentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<DataEcrLifecyclePolicyDocumentRuleEl>>,
    dynamic: DataEcrLifecyclePolicyDocumentDynamic,
}

struct DataEcrLifecyclePolicyDocument_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEcrLifecyclePolicyDocumentData>,
}

#[derive(Clone)]
pub struct DataEcrLifecyclePolicyDocument(Rc<DataEcrLifecyclePolicyDocument_>);

impl DataEcrLifecyclePolicyDocument {
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

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl Into<BlockAssignable<DataEcrLifecyclePolicyDocumentRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<DataEcrLifecyclePolicyDocumentRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
}

impl Referable for DataEcrLifecyclePolicyDocument {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataEcrLifecyclePolicyDocument {}

impl ToListMappable for DataEcrLifecyclePolicyDocument {
    type O = ListRef<DataEcrLifecyclePolicyDocumentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEcrLifecyclePolicyDocument_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ecr_lifecycle_policy_document".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcrLifecyclePolicyDocument {
    pub tf_id: String,
}

impl BuildDataEcrLifecyclePolicyDocument {
    pub fn build(self, stack: &mut Stack) -> DataEcrLifecyclePolicyDocument {
        let out = DataEcrLifecyclePolicyDocument(Rc::new(DataEcrLifecyclePolicyDocument_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEcrLifecyclePolicyDocumentData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcrLifecyclePolicyDocumentRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcrLifecyclePolicyDocumentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataEcrLifecyclePolicyDocumentRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<DataEcrLifecyclePolicyDocumentRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEcrLifecyclePolicyDocumentRuleElActionEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl DataEcrLifecyclePolicyDocumentRuleElActionEl {}

impl ToListMappable for DataEcrLifecyclePolicyDocumentRuleElActionEl {
    type O = BlockAssignable<DataEcrLifecyclePolicyDocumentRuleElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrLifecyclePolicyDocumentRuleElActionEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildDataEcrLifecyclePolicyDocumentRuleElActionEl {
    pub fn build(self) -> DataEcrLifecyclePolicyDocumentRuleElActionEl {
        DataEcrLifecyclePolicyDocumentRuleElActionEl { type_: self.type_ }
    }
}

pub struct DataEcrLifecyclePolicyDocumentRuleElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcrLifecyclePolicyDocumentRuleElActionElRef {
    fn new(shared: StackShared, base: String) -> DataEcrLifecyclePolicyDocumentRuleElActionElRef {
        DataEcrLifecyclePolicyDocumentRuleElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrLifecyclePolicyDocumentRuleElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcrLifecyclePolicyDocumentRuleElSelectionEl {
    count_number: PrimField<f64>,
    count_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_pattern_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_prefix_list: Option<ListField<PrimField<String>>>,
    tag_status: PrimField<String>,
}

impl DataEcrLifecyclePolicyDocumentRuleElSelectionEl {
    #[doc = "Set the field `count_unit`.\n"]
    pub fn set_count_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.count_unit = Some(v.into());
        self
    }

    #[doc = "Set the field `tag_pattern_list`.\n"]
    pub fn set_tag_pattern_list(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tag_pattern_list = Some(v.into());
        self
    }

    #[doc = "Set the field `tag_prefix_list`.\n"]
    pub fn set_tag_prefix_list(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tag_prefix_list = Some(v.into());
        self
    }
}

impl ToListMappable for DataEcrLifecyclePolicyDocumentRuleElSelectionEl {
    type O = BlockAssignable<DataEcrLifecyclePolicyDocumentRuleElSelectionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrLifecyclePolicyDocumentRuleElSelectionEl {
    #[doc = ""]
    pub count_number: PrimField<f64>,
    #[doc = ""]
    pub count_type: PrimField<String>,
    #[doc = ""]
    pub tag_status: PrimField<String>,
}

impl BuildDataEcrLifecyclePolicyDocumentRuleElSelectionEl {
    pub fn build(self) -> DataEcrLifecyclePolicyDocumentRuleElSelectionEl {
        DataEcrLifecyclePolicyDocumentRuleElSelectionEl {
            count_number: self.count_number,
            count_type: self.count_type,
            count_unit: core::default::Default::default(),
            tag_pattern_list: core::default::Default::default(),
            tag_prefix_list: core::default::Default::default(),
            tag_status: self.tag_status,
        }
    }
}

pub struct DataEcrLifecyclePolicyDocumentRuleElSelectionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcrLifecyclePolicyDocumentRuleElSelectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEcrLifecyclePolicyDocumentRuleElSelectionElRef {
        DataEcrLifecyclePolicyDocumentRuleElSelectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrLifecyclePolicyDocumentRuleElSelectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count_number` after provisioning.\n"]
    pub fn count_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count_number", self.base))
    }

    #[doc = "Get a reference to the value of field `count_type` after provisioning.\n"]
    pub fn count_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.count_type", self.base))
    }

    #[doc = "Get a reference to the value of field `count_unit` after provisioning.\n"]
    pub fn count_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.count_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_pattern_list` after provisioning.\n"]
    pub fn tag_pattern_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_pattern_list", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `tag_prefix_list` after provisioning.\n"]
    pub fn tag_prefix_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_prefix_list", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `tag_status` after provisioning.\n"]
    pub fn tag_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_status", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEcrLifecyclePolicyDocumentRuleElDynamic {
    action: Option<DynamicBlock<DataEcrLifecyclePolicyDocumentRuleElActionEl>>,
    selection: Option<DynamicBlock<DataEcrLifecyclePolicyDocumentRuleElSelectionEl>>,
}

#[derive(Serialize)]
pub struct DataEcrLifecyclePolicyDocumentRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<DataEcrLifecyclePolicyDocumentRuleElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selection: Option<Vec<DataEcrLifecyclePolicyDocumentRuleElSelectionEl>>,
    dynamic: DataEcrLifecyclePolicyDocumentRuleElDynamic,
}

impl DataEcrLifecyclePolicyDocumentRuleEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<DataEcrLifecyclePolicyDocumentRuleElActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `selection`.\n"]
    pub fn set_selection(
        mut self,
        v: impl Into<BlockAssignable<DataEcrLifecyclePolicyDocumentRuleElSelectionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.selection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.selection = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for DataEcrLifecyclePolicyDocumentRuleEl {
    type O = BlockAssignable<DataEcrLifecyclePolicyDocumentRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcrLifecyclePolicyDocumentRuleEl {
    #[doc = ""]
    pub priority: PrimField<f64>,
}

impl BuildDataEcrLifecyclePolicyDocumentRuleEl {
    pub fn build(self) -> DataEcrLifecyclePolicyDocumentRuleEl {
        DataEcrLifecyclePolicyDocumentRuleEl {
            description: core::default::Default::default(),
            priority: self.priority,
            action: core::default::Default::default(),
            selection: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataEcrLifecyclePolicyDocumentRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEcrLifecyclePolicyDocumentRuleElRef {
    fn new(shared: StackShared, base: String) -> DataEcrLifecyclePolicyDocumentRuleElRef {
        DataEcrLifecyclePolicyDocumentRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcrLifecyclePolicyDocumentRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataEcrLifecyclePolicyDocumentRuleElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `selection` after provisioning.\n"]
    pub fn selection(&self) -> ListRef<DataEcrLifecyclePolicyDocumentRuleElSelectionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.selection", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEcrLifecyclePolicyDocumentDynamic {
    rule: Option<DynamicBlock<DataEcrLifecyclePolicyDocumentRuleEl>>,
}
