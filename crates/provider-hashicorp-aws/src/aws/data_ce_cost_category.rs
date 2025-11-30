use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataCeCostCategoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cost_category_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataCeCostCategory_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCeCostCategoryData>,
}

#[derive(Clone)]
pub struct DataCeCostCategory(Rc<DataCeCostCategory_>);

impl DataCeCostCategory {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cost_category_arn` after provisioning.\n"]
    pub fn cost_category_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cost_category_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_value", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `effective_end` after provisioning.\n"]
    pub fn effective_end(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_end", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `effective_start` after provisioning.\n"]
    pub fn effective_start(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_start", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> SetRef<DataCeCostCategoryRuleElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_version` after provisioning.\n"]
    pub fn rule_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `split_charge_rule` after provisioning.\n"]
    pub fn split_charge_rule(&self) -> SetRef<DataCeCostCategorySplitChargeRuleElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.split_charge_rule", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

impl Referable for DataCeCostCategory {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataCeCostCategory {}

impl ToListMappable for DataCeCostCategory {
    type O = ListRef<DataCeCostCategoryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCeCostCategory_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ce_cost_category".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCeCostCategory {
    pub tf_id: String,
    #[doc = ""]
    pub cost_category_arn: PrimField<String>,
}

impl BuildDataCeCostCategory {
    pub fn build(self, stack: &mut Stack) -> DataCeCostCategory {
        let out = DataCeCostCategory(Rc::new(DataCeCostCategory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCeCostCategoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cost_category_arn: self.cost_category_arn,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCeCostCategoryRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataCeCostCategoryRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cost_category_arn` after provisioning.\n"]
    pub fn cost_category_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cost_category_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_value", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `effective_end` after provisioning.\n"]
    pub fn effective_end(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_end", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `effective_start` after provisioning.\n"]
    pub fn effective_start(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_start", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> SetRef<DataCeCostCategoryRuleElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_version` after provisioning.\n"]
    pub fn rule_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `split_charge_rule` after provisioning.\n"]
    pub fn split_charge_rule(&self) -> SetRef<DataCeCostCategorySplitChargeRuleElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.split_charge_rule", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElInheritedValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension_name: Option<PrimField<String>>,
}

impl DataCeCostCategoryRuleElInheritedValueEl {
    #[doc = "Set the field `dimension_key`.\n"]
    pub fn set_dimension_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dimension_key = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension_name`.\n"]
    pub fn set_dimension_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dimension_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElInheritedValueEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElInheritedValueEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElInheritedValueEl {}

impl BuildDataCeCostCategoryRuleElInheritedValueEl {
    pub fn build(self) -> DataCeCostCategoryRuleElInheritedValueEl {
        DataCeCostCategoryRuleElInheritedValueEl {
            dimension_key: core::default::Default::default(),
            dimension_name: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElInheritedValueElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElInheritedValueElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElInheritedValueElRef {
        DataCeCostCategoryRuleElInheritedValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElInheritedValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimension_key` after provisioning.\n"]
    pub fn dimension_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dimension_key", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension_name` after provisioning.\n"]
    pub fn dimension_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dimension_name", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElAndElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElAndElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElAndElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElAndElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElAndElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElAndElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElAndElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElAndElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElAndElDimensionEl {
        DataCeCostCategoryRuleElRuleElAndElAndElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElAndElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElAndElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElAndElAndElDimensionElRef {
        DataCeCostCategoryRuleElRuleElAndElAndElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElAndElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElAndElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElAndElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElAndElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElAndElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElAndElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElAndElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElAndElTagsEl {
        DataCeCostCategoryRuleElRuleElAndElAndElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElAndElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElAndElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElAndElTagsElRef {
        DataCeCostCategoryRuleElRuleElAndElAndElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElAndElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElAndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElAndElAndElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElAndElAndElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElAndElAndEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElAndElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElAndElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElAndEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElAndEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElAndEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElAndEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElAndEl {
        DataCeCostCategoryRuleElRuleElAndElAndEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElAndElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElAndElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElAndElRef {
        DataCeCostCategoryRuleElRuleElAndElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<DataCeCostCategoryRuleElRuleElAndElAndElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElAndElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElAndElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElAndElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElDimensionEl {
        DataCeCostCategoryRuleElRuleElAndElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
        DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElNotElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElNotElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElNotElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElNotElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElNotElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElNotElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElNotElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElNotElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElNotElDimensionEl {
        DataCeCostCategoryRuleElRuleElAndElNotElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElNotElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElNotElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElAndElNotElDimensionElRef {
        DataCeCostCategoryRuleElRuleElAndElNotElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElNotElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElNotElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElNotElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElNotElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElNotElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElNotElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElNotElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElNotElTagsEl {
        DataCeCostCategoryRuleElRuleElAndElNotElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElNotElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElNotElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElNotElTagsElRef {
        DataCeCostCategoryRuleElRuleElAndElNotElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElNotElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElNotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElAndElNotElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElAndElNotElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElAndElNotEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElNotElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElNotElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElNotEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElNotEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElNotEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElNotEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElNotEl {
        DataCeCostCategoryRuleElRuleElAndElNotEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElNotElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElNotElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElNotElRef {
        DataCeCostCategoryRuleElRuleElAndElNotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElNotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<DataCeCostCategoryRuleElRuleElAndElNotElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElNotElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElNotElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElOrElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElOrElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElOrElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElOrElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElOrElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElOrElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElOrElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElOrElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElOrElDimensionEl {
        DataCeCostCategoryRuleElRuleElAndElOrElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElOrElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElOrElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElAndElOrElDimensionElRef {
        DataCeCostCategoryRuleElRuleElAndElOrElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElOrElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElOrElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElOrElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElOrElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElOrElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElOrElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElOrElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElOrElTagsEl {
        DataCeCostCategoryRuleElRuleElAndElOrElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElOrElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElOrElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElOrElTagsElRef {
        DataCeCostCategoryRuleElRuleElAndElOrElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElOrElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElOrEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElAndElOrElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElAndElOrElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElAndElOrEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElOrElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElOrElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElOrEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElOrEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElOrEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElOrEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElOrEl {
        DataCeCostCategoryRuleElRuleElAndElOrEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElOrElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElOrElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElOrElRef {
        DataCeCostCategoryRuleElRuleElAndElOrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElOrElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<DataCeCostCategoryRuleElRuleElAndElOrElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElOrElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElOrElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElAndElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndElTagsEl {
        DataCeCostCategoryRuleElRuleElAndElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElTagsElRef {
        DataCeCostCategoryRuleElRuleElAndElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElAndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<SetField<DataCeCostCategoryRuleElRuleElAndElAndEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElAndElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElAndElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not: Option<ListField<DataCeCostCategoryRuleElRuleElAndElNotEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<SetField<DataCeCostCategoryRuleElRuleElAndElOrEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElAndElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElAndEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl Into<SetField<DataCeCostCategoryRuleElRuleElAndElAndEl>>,
    ) -> Self {
        self.and = Some(v.into());
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElNotEl>>,
    ) -> Self {
        self.not = Some(v.into());
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(
        mut self,
        v: impl Into<SetField<DataCeCostCategoryRuleElRuleElAndElOrEl>>,
    ) -> Self {
        self.or = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElAndElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElAndEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElAndEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElAndEl {}

impl BuildDataCeCostCategoryRuleElRuleElAndEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElAndEl {
        DataCeCostCategoryRuleElRuleElAndEl {
            and: core::default::Default::default(),
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElAndElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElAndElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElAndElRef {
        DataCeCostCategoryRuleElRuleElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> SetRef<DataCeCostCategoryRuleElRuleElAndElAndElRef> {
        SetRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElNotElRef> {
        ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> SetRef<DataCeCostCategoryRuleElRuleElAndElOrElRef> {
        SetRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElAndElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElDimensionEl {
        DataCeCostCategoryRuleElRuleElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElDimensionElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElDimensionElRef {
        DataCeCostCategoryRuleElRuleElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElAndElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElAndElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElAndElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElAndElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElAndElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElAndElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElAndElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElAndElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElAndElDimensionEl {
        DataCeCostCategoryRuleElRuleElNotElAndElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElAndElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElAndElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElNotElAndElDimensionElRef {
        DataCeCostCategoryRuleElRuleElNotElAndElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElAndElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElAndElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElAndElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElAndElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElAndElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElAndElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElAndElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElAndElTagsEl {
        DataCeCostCategoryRuleElRuleElNotElAndElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElAndElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElAndElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElAndElTagsElRef {
        DataCeCostCategoryRuleElRuleElNotElAndElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElAndElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElAndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElNotElAndElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElNotElAndElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElNotElAndEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElAndElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElAndElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElAndEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElAndEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElAndEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElAndEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElAndEl {
        DataCeCostCategoryRuleElRuleElNotElAndEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElAndElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElAndElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElAndElRef {
        DataCeCostCategoryRuleElRuleElNotElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<DataCeCostCategoryRuleElRuleElNotElAndElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElAndElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElAndElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElNotElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElDimensionEl {
        DataCeCostCategoryRuleElRuleElNotElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
        DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElNotElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElNotElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElNotElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElNotElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElNotElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElNotElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElNotElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElNotElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElNotElDimensionEl {
        DataCeCostCategoryRuleElRuleElNotElNotElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElNotElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElNotElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElNotElNotElDimensionElRef {
        DataCeCostCategoryRuleElRuleElNotElNotElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElNotElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElNotElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElNotElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElNotElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElNotElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElNotElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElNotElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElNotElTagsEl {
        DataCeCostCategoryRuleElRuleElNotElNotElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElNotElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElNotElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElNotElTagsElRef {
        DataCeCostCategoryRuleElRuleElNotElNotElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElNotElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElNotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElNotElNotElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElNotElNotElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElNotElNotEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElNotElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElNotElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElNotEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElNotEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElNotEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElNotEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElNotEl {
        DataCeCostCategoryRuleElRuleElNotElNotEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElNotElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElNotElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElNotElRef {
        DataCeCostCategoryRuleElRuleElNotElNotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElNotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<DataCeCostCategoryRuleElRuleElNotElNotElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElNotElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElNotElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElOrElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElOrElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElOrElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElOrElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElOrElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElOrElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElOrElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElOrElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElOrElDimensionEl {
        DataCeCostCategoryRuleElRuleElNotElOrElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElOrElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElOrElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElNotElOrElDimensionElRef {
        DataCeCostCategoryRuleElRuleElNotElOrElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElOrElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElOrElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElOrElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElOrElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElOrElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElOrElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElOrElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElOrElTagsEl {
        DataCeCostCategoryRuleElRuleElNotElOrElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElOrElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElOrElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElOrElTagsElRef {
        DataCeCostCategoryRuleElRuleElNotElOrElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElOrElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElOrEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElNotElOrElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElNotElOrElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElNotElOrEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElOrElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElOrElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElOrEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElOrEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElOrEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElOrEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElOrEl {
        DataCeCostCategoryRuleElRuleElNotElOrEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElOrElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElOrElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElOrElRef {
        DataCeCostCategoryRuleElRuleElNotElOrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElOrElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<DataCeCostCategoryRuleElRuleElNotElOrElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElOrElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElOrElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElNotElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotElTagsEl {
        DataCeCostCategoryRuleElRuleElNotElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElTagsElRef {
        DataCeCostCategoryRuleElRuleElNotElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElNotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<SetField<DataCeCostCategoryRuleElRuleElNotElAndEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElNotElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElNotElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not: Option<ListField<DataCeCostCategoryRuleElRuleElNotElNotEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<SetField<DataCeCostCategoryRuleElRuleElNotElOrEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElNotElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElNotEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl Into<SetField<DataCeCostCategoryRuleElRuleElNotElAndEl>>,
    ) -> Self {
        self.and = Some(v.into());
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElNotEl>>,
    ) -> Self {
        self.not = Some(v.into());
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(
        mut self,
        v: impl Into<SetField<DataCeCostCategoryRuleElRuleElNotElOrEl>>,
    ) -> Self {
        self.or = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElNotEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElNotEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElNotEl {}

impl BuildDataCeCostCategoryRuleElRuleElNotEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElNotEl {
        DataCeCostCategoryRuleElRuleElNotEl {
            and: core::default::Default::default(),
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElNotElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElNotElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElNotElRef {
        DataCeCostCategoryRuleElRuleElNotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElNotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> SetRef<DataCeCostCategoryRuleElRuleElNotElAndElRef> {
        SetRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElNotElRef> {
        ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> SetRef<DataCeCostCategoryRuleElRuleElNotElOrElRef> {
        SetRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElAndElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElAndElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElAndElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElAndElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElAndElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElAndElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElAndElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElAndElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElAndElDimensionEl {
        DataCeCostCategoryRuleElRuleElOrElAndElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElAndElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElAndElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElOrElAndElDimensionElRef {
        DataCeCostCategoryRuleElRuleElOrElAndElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElAndElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElAndElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElAndElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElAndElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElAndElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElAndElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElAndElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElAndElTagsEl {
        DataCeCostCategoryRuleElRuleElOrElAndElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElAndElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElAndElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElAndElTagsElRef {
        DataCeCostCategoryRuleElRuleElOrElAndElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElAndElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElAndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElOrElAndElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElOrElAndElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElOrElAndEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElAndElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElAndElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElAndEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElAndEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElAndEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElAndEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElAndEl {
        DataCeCostCategoryRuleElRuleElOrElAndEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElAndElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElAndElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElAndElRef {
        DataCeCostCategoryRuleElRuleElOrElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<DataCeCostCategoryRuleElRuleElOrElAndElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElAndElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElAndElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElOrElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElDimensionEl {
        DataCeCostCategoryRuleElRuleElOrElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
        DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElNotElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElNotElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElNotElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElNotElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElNotElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElNotElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElNotElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElNotElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElNotElDimensionEl {
        DataCeCostCategoryRuleElRuleElOrElNotElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElNotElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElNotElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElOrElNotElDimensionElRef {
        DataCeCostCategoryRuleElRuleElOrElNotElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElNotElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElNotElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElNotElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElNotElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElNotElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElNotElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElNotElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElNotElTagsEl {
        DataCeCostCategoryRuleElRuleElOrElNotElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElNotElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElNotElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElNotElTagsElRef {
        DataCeCostCategoryRuleElRuleElOrElNotElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElNotElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElNotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElOrElNotElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElOrElNotElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElOrElNotEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElNotElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElNotElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElNotEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElNotEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElNotEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElNotEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElNotEl {
        DataCeCostCategoryRuleElRuleElOrElNotEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElNotElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElNotElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElNotElRef {
        DataCeCostCategoryRuleElRuleElOrElNotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElNotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<DataCeCostCategoryRuleElRuleElOrElNotElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElNotElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElNotElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElOrElCostCategoryEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElOrElCostCategoryEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryEl {
        DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryElRef {
        DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElOrElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElOrElDimensionEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElOrElDimensionEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElOrElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElOrElDimensionEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElOrElDimensionEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElOrElDimensionEl {
        DataCeCostCategoryRuleElRuleElOrElOrElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElOrElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElOrElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCeCostCategoryRuleElRuleElOrElOrElDimensionElRef {
        DataCeCostCategoryRuleElRuleElOrElOrElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElOrElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElOrElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElOrElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElOrElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElOrElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElOrElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElOrElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElOrElTagsEl {
        DataCeCostCategoryRuleElRuleElOrElOrElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElOrElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElOrElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElOrElTagsElRef {
        DataCeCostCategoryRuleElRuleElOrElOrElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElOrElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElOrEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElOrElOrElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElOrElOrElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElOrElOrEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElOrElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElOrElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElOrEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElOrEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElOrEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElOrEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElOrEl {
        DataCeCostCategoryRuleElRuleElOrElOrEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElOrElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElOrElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElOrElRef {
        DataCeCostCategoryRuleElRuleElOrElOrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElOrElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<DataCeCostCategoryRuleElRuleElOrElOrElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElOrElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElOrElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElOrElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrElTagsEl {
        DataCeCostCategoryRuleElRuleElOrElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElTagsElRef {
        DataCeCostCategoryRuleElRuleElOrElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElOrEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<SetField<DataCeCostCategoryRuleElRuleElOrElAndEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElOrElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElOrElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not: Option<ListField<DataCeCostCategoryRuleElRuleElOrElNotEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<SetField<DataCeCostCategoryRuleElRuleElOrElOrEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElOrElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleElOrEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl Into<SetField<DataCeCostCategoryRuleElRuleElOrElAndEl>>,
    ) -> Self {
        self.and = Some(v.into());
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElNotEl>>,
    ) -> Self {
        self.not = Some(v.into());
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(
        mut self,
        v: impl Into<SetField<DataCeCostCategoryRuleElRuleElOrElOrEl>>,
    ) -> Self {
        self.or = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElOrElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElOrEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElOrEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElOrEl {}

impl BuildDataCeCostCategoryRuleElRuleElOrEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElOrEl {
        DataCeCostCategoryRuleElRuleElOrEl {
            and: core::default::Default::default(),
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElOrElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElOrElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElOrElRef {
        DataCeCostCategoryRuleElRuleElOrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElOrElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> SetRef<DataCeCostCategoryRuleElRuleElOrElAndElRef> {
        SetRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElNotElRef> {
        ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> SetRef<DataCeCostCategoryRuleElRuleElOrElOrElRef> {
        SetRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElOrElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategoryRuleElRuleElTagsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.match_options = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleElTagsEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleElTagsEl {}

impl BuildDataCeCostCategoryRuleElRuleElTagsEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleElTagsEl {
        DataCeCostCategoryRuleElRuleElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElTagsElRef {
        DataCeCostCategoryRuleElRuleElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.match_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleElRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<SetField<DataCeCostCategoryRuleElRuleElAndEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<ListField<DataCeCostCategoryRuleElRuleElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<ListField<DataCeCostCategoryRuleElRuleElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not: Option<ListField<DataCeCostCategoryRuleElRuleElNotEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<SetField<DataCeCostCategoryRuleElRuleElOrEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<DataCeCostCategoryRuleElRuleElTagsEl>>,
}

impl DataCeCostCategoryRuleElRuleEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(mut self, v: impl Into<SetField<DataCeCostCategoryRuleElRuleElAndEl>>) -> Self {
        self.and = Some(v.into());
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElCostCategoryEl>>,
    ) -> Self {
        self.cost_category = Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElDimensionEl>>,
    ) -> Self {
        self.dimension = Some(v.into());
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleElNotEl>>) -> Self {
        self.not = Some(v.into());
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(mut self, v: impl Into<SetField<DataCeCostCategoryRuleElRuleElOrEl>>) -> Self {
        self.or = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElRuleElTagsEl>>,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleElRuleEl {
    type O = BlockAssignable<DataCeCostCategoryRuleElRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleElRuleEl {}

impl BuildDataCeCostCategoryRuleElRuleEl {
    pub fn build(self) -> DataCeCostCategoryRuleElRuleEl {
        DataCeCostCategoryRuleElRuleEl {
            and: core::default::Default::default(),
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRuleElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRuleElRef {
        DataCeCostCategoryRuleElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> SetRef<DataCeCostCategoryRuleElRuleElAndElRef> {
        SetRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<DataCeCostCategoryRuleElRuleElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeCostCategoryRuleElRuleElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> ListRef<DataCeCostCategoryRuleElRuleElNotElRef> {
        ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> SetRef<DataCeCostCategoryRuleElRuleElOrElRef> {
        SetRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeCostCategoryRuleElRuleElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategoryRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inherited_value: Option<ListField<DataCeCostCategoryRuleElInheritedValueEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<ListField<DataCeCostCategoryRuleElRuleEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataCeCostCategoryRuleEl {
    #[doc = "Set the field `inherited_value`.\n"]
    pub fn set_inherited_value(
        mut self,
        v: impl Into<ListField<DataCeCostCategoryRuleElInheritedValueEl>>,
    ) -> Self {
        self.inherited_value = Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl Into<ListField<DataCeCostCategoryRuleElRuleEl>>) -> Self {
        self.rule = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategoryRuleEl {
    type O = BlockAssignable<DataCeCostCategoryRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategoryRuleEl {}

impl BuildDataCeCostCategoryRuleEl {
    pub fn build(self) -> DataCeCostCategoryRuleEl {
        DataCeCostCategoryRuleEl {
            inherited_value: core::default::Default::default(),
            rule: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategoryRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategoryRuleElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategoryRuleElRef {
        DataCeCostCategoryRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategoryRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inherited_value` after provisioning.\n"]
    pub fn inherited_value(&self) -> ListRef<DataCeCostCategoryRuleElInheritedValueElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inherited_value", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<DataCeCostCategoryRuleElRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategorySplitChargeRuleElParameterEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategorySplitChargeRuleElParameterEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategorySplitChargeRuleElParameterEl {
    type O = BlockAssignable<DataCeCostCategorySplitChargeRuleElParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategorySplitChargeRuleElParameterEl {}

impl BuildDataCeCostCategorySplitChargeRuleElParameterEl {
    pub fn build(self) -> DataCeCostCategorySplitChargeRuleElParameterEl {
        DataCeCostCategorySplitChargeRuleElParameterEl {
            type_: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategorySplitChargeRuleElParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategorySplitChargeRuleElParameterElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategorySplitChargeRuleElParameterElRef {
        DataCeCostCategorySplitChargeRuleElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategorySplitChargeRuleElParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeCostCategorySplitChargeRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<SetField<DataCeCostCategorySplitChargeRuleElParameterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<SetField<PrimField<String>>>,
}

impl DataCeCostCategorySplitChargeRuleEl {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method = Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl Into<SetField<DataCeCostCategorySplitChargeRuleElParameterEl>>,
    ) -> Self {
        self.parameter = Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }

    #[doc = "Set the field `targets`.\n"]
    pub fn set_targets(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.targets = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeCostCategorySplitChargeRuleEl {
    type O = BlockAssignable<DataCeCostCategorySplitChargeRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeCostCategorySplitChargeRuleEl {}

impl BuildDataCeCostCategorySplitChargeRuleEl {
    pub fn build(self) -> DataCeCostCategorySplitChargeRuleEl {
        DataCeCostCategorySplitChargeRuleEl {
            method: core::default::Default::default(),
            parameter: core::default::Default::default(),
            source: core::default::Default::default(),
            targets: core::default::Default::default(),
        }
    }
}

pub struct DataCeCostCategorySplitChargeRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeCostCategorySplitChargeRuleElRef {
    fn new(shared: StackShared, base: String) -> DataCeCostCategorySplitChargeRuleElRef {
        DataCeCostCategorySplitChargeRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeCostCategorySplitChargeRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter` after provisioning.\n"]
    pub fn parameter(&self) -> SetRef<DataCeCostCategorySplitChargeRuleElParameterElRef> {
        SetRef::new(self.shared().clone(), format!("{}.parameter", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.targets", self.base))
    }
}
