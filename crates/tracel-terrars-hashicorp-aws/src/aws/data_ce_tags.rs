use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCeTagsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_string: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataCeTagsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by: Option<Vec<DataCeTagsSortByEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_period: Option<Vec<DataCeTagsTimePeriodEl>>,
    dynamic: DataCeTagsDynamic,
}

struct DataCeTags_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCeTagsData>,
}

#[derive(Clone)]
pub struct DataCeTags(Rc<DataCeTags_>);

impl DataCeTags {
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

    #[doc = "Set the field `search_string`.\n"]
    pub fn set_search_string(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().search_string = Some(v.into());
        self
    }

    #[doc = "Set the field `tag_key`.\n"]
    pub fn set_tag_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tag_key = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataCeTagsFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sort_by`.\n"]
    pub fn set_sort_by(self, v: impl Into<BlockAssignable<DataCeTagsSortByEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sort_by = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sort_by = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_period`.\n"]
    pub fn set_time_period(self, v: impl Into<BlockAssignable<DataCeTagsTimePeriodEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().time_period = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.time_period = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search_string` after provisioning.\n"]
    pub fn search_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_key` after provisioning.\n"]
    pub fn tag_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataCeTagsFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort_by` after provisioning.\n"]
    pub fn sort_by(&self) -> ListRef<DataCeTagsSortByElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period` after provisioning.\n"]
    pub fn time_period(&self) -> ListRef<DataCeTagsTimePeriodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_period", self.extract_ref()))
    }
}

impl Referable for DataCeTags {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCeTags { }

impl ToListMappable for DataCeTags {
    type O = ListRef<DataCeTagsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCeTags_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ce_tags".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCeTags {
    pub tf_id: String,
}

impl BuildDataCeTags {
    pub fn build(self, stack: &mut Stack) -> DataCeTags {
        let out = DataCeTags(Rc::new(DataCeTags_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCeTagsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                search_string: core::default::Default::default(),
                tag_key: core::default::Default::default(),
                filter: core::default::Default::default(),
                sort_by: core::default::Default::default(),
                time_period: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCeTagsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataCeTagsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search_string` after provisioning.\n"]
    pub fn search_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_key` after provisioning.\n"]
    pub fn tag_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataCeTagsFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort_by` after provisioning.\n"]
    pub fn sort_by(&self) -> ListRef<DataCeTagsSortByElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period` after provisioning.\n"]
    pub fn time_period(&self) -> ListRef<DataCeTagsTimePeriodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_period", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElAndElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeTagsFilterElAndElCostCategoryEl {
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

impl ToListMappable for DataCeTagsFilterElAndElCostCategoryEl {
    type O = BlockAssignable<DataCeTagsFilterElAndElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElAndElCostCategoryEl {}

impl BuildDataCeTagsFilterElAndElCostCategoryEl {
    pub fn build(self) -> DataCeTagsFilterElAndElCostCategoryEl {
        DataCeTagsFilterElAndElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElAndElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElAndElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElAndElCostCategoryElRef {
        DataCeTagsFilterElAndElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElAndElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElAndElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeTagsFilterElAndElDimensionEl {
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

impl ToListMappable for DataCeTagsFilterElAndElDimensionEl {
    type O = BlockAssignable<DataCeTagsFilterElAndElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElAndElDimensionEl {}

impl BuildDataCeTagsFilterElAndElDimensionEl {
    pub fn build(self) -> DataCeTagsFilterElAndElDimensionEl {
        DataCeTagsFilterElAndElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElAndElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElAndElDimensionElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElAndElDimensionElRef {
        DataCeTagsFilterElAndElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElAndElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElAndElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeTagsFilterElAndElTagsEl {
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

impl ToListMappable for DataCeTagsFilterElAndElTagsEl {
    type O = BlockAssignable<DataCeTagsFilterElAndElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElAndElTagsEl {}

impl BuildDataCeTagsFilterElAndElTagsEl {
    pub fn build(self) -> DataCeTagsFilterElAndElTagsEl {
        DataCeTagsFilterElAndElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElAndElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElAndElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElAndElTagsElRef {
        DataCeTagsFilterElAndElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElAndElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCeTagsFilterElAndElDynamic {
    cost_category: Option<DynamicBlock<DataCeTagsFilterElAndElCostCategoryEl>>,
    dimension: Option<DynamicBlock<DataCeTagsFilterElAndElDimensionEl>>,
    tags: Option<DynamicBlock<DataCeTagsFilterElAndElTagsEl>>,
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElAndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<Vec<DataCeTagsFilterElAndElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<DataCeTagsFilterElAndElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DataCeTagsFilterElAndElTagsEl>>,
    dynamic: DataCeTagsFilterElAndElDynamic,
}

impl DataCeTagsFilterElAndEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElAndElCostCategoryEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cost_category = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElAndElDimensionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElAndElTagsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataCeTagsFilterElAndEl {
    type O = BlockAssignable<DataCeTagsFilterElAndEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElAndEl {}

impl BuildDataCeTagsFilterElAndEl {
    pub fn build(self) -> DataCeTagsFilterElAndEl {
        DataCeTagsFilterElAndEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElAndElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElAndElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElAndElRef {
        DataCeTagsFilterElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<DataCeTagsFilterElAndElCostCategoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeTagsFilterElAndElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeTagsFilterElAndElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeTagsFilterElCostCategoryEl {
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

impl ToListMappable for DataCeTagsFilterElCostCategoryEl {
    type O = BlockAssignable<DataCeTagsFilterElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElCostCategoryEl {}

impl BuildDataCeTagsFilterElCostCategoryEl {
    pub fn build(self) -> DataCeTagsFilterElCostCategoryEl {
        DataCeTagsFilterElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElCostCategoryElRef {
        DataCeTagsFilterElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeTagsFilterElDimensionEl {
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

impl ToListMappable for DataCeTagsFilterElDimensionEl {
    type O = BlockAssignable<DataCeTagsFilterElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElDimensionEl {}

impl BuildDataCeTagsFilterElDimensionEl {
    pub fn build(self) -> DataCeTagsFilterElDimensionEl {
        DataCeTagsFilterElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElDimensionElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElDimensionElRef {
        DataCeTagsFilterElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElNotElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeTagsFilterElNotElCostCategoryEl {
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

impl ToListMappable for DataCeTagsFilterElNotElCostCategoryEl {
    type O = BlockAssignable<DataCeTagsFilterElNotElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElNotElCostCategoryEl {}

impl BuildDataCeTagsFilterElNotElCostCategoryEl {
    pub fn build(self) -> DataCeTagsFilterElNotElCostCategoryEl {
        DataCeTagsFilterElNotElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElNotElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElNotElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElNotElCostCategoryElRef {
        DataCeTagsFilterElNotElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElNotElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElNotElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeTagsFilterElNotElDimensionEl {
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

impl ToListMappable for DataCeTagsFilterElNotElDimensionEl {
    type O = BlockAssignable<DataCeTagsFilterElNotElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElNotElDimensionEl {}

impl BuildDataCeTagsFilterElNotElDimensionEl {
    pub fn build(self) -> DataCeTagsFilterElNotElDimensionEl {
        DataCeTagsFilterElNotElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElNotElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElNotElDimensionElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElNotElDimensionElRef {
        DataCeTagsFilterElNotElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElNotElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElNotElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeTagsFilterElNotElTagsEl {
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

impl ToListMappable for DataCeTagsFilterElNotElTagsEl {
    type O = BlockAssignable<DataCeTagsFilterElNotElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElNotElTagsEl {}

impl BuildDataCeTagsFilterElNotElTagsEl {
    pub fn build(self) -> DataCeTagsFilterElNotElTagsEl {
        DataCeTagsFilterElNotElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElNotElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElNotElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElNotElTagsElRef {
        DataCeTagsFilterElNotElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElNotElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCeTagsFilterElNotElDynamic {
    cost_category: Option<DynamicBlock<DataCeTagsFilterElNotElCostCategoryEl>>,
    dimension: Option<DynamicBlock<DataCeTagsFilterElNotElDimensionEl>>,
    tags: Option<DynamicBlock<DataCeTagsFilterElNotElTagsEl>>,
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElNotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<Vec<DataCeTagsFilterElNotElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<DataCeTagsFilterElNotElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DataCeTagsFilterElNotElTagsEl>>,
    dynamic: DataCeTagsFilterElNotElDynamic,
}

impl DataCeTagsFilterElNotEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElNotElCostCategoryEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cost_category = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElNotElDimensionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElNotElTagsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataCeTagsFilterElNotEl {
    type O = BlockAssignable<DataCeTagsFilterElNotEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElNotEl {}

impl BuildDataCeTagsFilterElNotEl {
    pub fn build(self) -> DataCeTagsFilterElNotEl {
        DataCeTagsFilterElNotEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElNotElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElNotElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElNotElRef {
        DataCeTagsFilterElNotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElNotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<DataCeTagsFilterElNotElCostCategoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeTagsFilterElNotElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeTagsFilterElNotElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElOrElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeTagsFilterElOrElCostCategoryEl {
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

impl ToListMappable for DataCeTagsFilterElOrElCostCategoryEl {
    type O = BlockAssignable<DataCeTagsFilterElOrElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElOrElCostCategoryEl {}

impl BuildDataCeTagsFilterElOrElCostCategoryEl {
    pub fn build(self) -> DataCeTagsFilterElOrElCostCategoryEl {
        DataCeTagsFilterElOrElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElOrElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElOrElCostCategoryElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElOrElCostCategoryElRef {
        DataCeTagsFilterElOrElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElOrElCostCategoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElOrElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeTagsFilterElOrElDimensionEl {
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

impl ToListMappable for DataCeTagsFilterElOrElDimensionEl {
    type O = BlockAssignable<DataCeTagsFilterElOrElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElOrElDimensionEl {}

impl BuildDataCeTagsFilterElOrElDimensionEl {
    pub fn build(self) -> DataCeTagsFilterElOrElDimensionEl {
        DataCeTagsFilterElOrElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElOrElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElOrElDimensionElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElOrElDimensionElRef {
        DataCeTagsFilterElOrElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElOrElDimensionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElOrElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeTagsFilterElOrElTagsEl {
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

impl ToListMappable for DataCeTagsFilterElOrElTagsEl {
    type O = BlockAssignable<DataCeTagsFilterElOrElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElOrElTagsEl {}

impl BuildDataCeTagsFilterElOrElTagsEl {
    pub fn build(self) -> DataCeTagsFilterElOrElTagsEl {
        DataCeTagsFilterElOrElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElOrElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElOrElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElOrElTagsElRef {
        DataCeTagsFilterElOrElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElOrElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCeTagsFilterElOrElDynamic {
    cost_category: Option<DynamicBlock<DataCeTagsFilterElOrElCostCategoryEl>>,
    dimension: Option<DynamicBlock<DataCeTagsFilterElOrElDimensionEl>>,
    tags: Option<DynamicBlock<DataCeTagsFilterElOrElTagsEl>>,
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElOrEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<Vec<DataCeTagsFilterElOrElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<DataCeTagsFilterElOrElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DataCeTagsFilterElOrElTagsEl>>,
    dynamic: DataCeTagsFilterElOrElDynamic,
}

impl DataCeTagsFilterElOrEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElOrElCostCategoryEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cost_category = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElOrElDimensionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElOrElTagsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataCeTagsFilterElOrEl {
    type O = BlockAssignable<DataCeTagsFilterElOrEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElOrEl {}

impl BuildDataCeTagsFilterElOrEl {
    pub fn build(self) -> DataCeTagsFilterElOrEl {
        DataCeTagsFilterElOrEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElOrElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElOrElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElOrElRef {
        DataCeTagsFilterElOrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElOrElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<DataCeTagsFilterElOrElCostCategoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeTagsFilterElOrElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeTagsFilterElOrElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsFilterElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataCeTagsFilterElTagsEl {
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

impl ToListMappable for DataCeTagsFilterElTagsEl {
    type O = BlockAssignable<DataCeTagsFilterElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterElTagsEl {}

impl BuildDataCeTagsFilterElTagsEl {
    pub fn build(self) -> DataCeTagsFilterElTagsEl {
        DataCeTagsFilterElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElTagsElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElTagsElRef {
        DataCeTagsFilterElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCeTagsFilterElDynamic {
    and: Option<DynamicBlock<DataCeTagsFilterElAndEl>>,
    cost_category: Option<DynamicBlock<DataCeTagsFilterElCostCategoryEl>>,
    dimension: Option<DynamicBlock<DataCeTagsFilterElDimensionEl>>,
    not: Option<DynamicBlock<DataCeTagsFilterElNotEl>>,
    or: Option<DynamicBlock<DataCeTagsFilterElOrEl>>,
    tags: Option<DynamicBlock<DataCeTagsFilterElTagsEl>>,
}

#[derive(Serialize)]
pub struct DataCeTagsFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<DataCeTagsFilterElAndEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<Vec<DataCeTagsFilterElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<DataCeTagsFilterElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not: Option<Vec<DataCeTagsFilterElNotEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<Vec<DataCeTagsFilterElOrEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DataCeTagsFilterElTagsEl>>,
    dynamic: DataCeTagsFilterElDynamic,
}

impl DataCeTagsFilterEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElAndEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.and = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.and = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElCostCategoryEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cost_category = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElDimensionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElNotEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.not = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.not = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElOrEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.or = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.or = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<BlockAssignable<DataCeTagsFilterElTagsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataCeTagsFilterEl {
    type O = BlockAssignable<DataCeTagsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsFilterEl {}

impl BuildDataCeTagsFilterEl {
    pub fn build(self) -> DataCeTagsFilterEl {
        DataCeTagsFilterEl {
            and: core::default::Default::default(),
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataCeTagsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsFilterElRef {
        DataCeTagsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> ListRef<DataCeTagsFilterElCostCategoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<DataCeTagsFilterElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> ListRef<DataCeTagsFilterElNotElRef> {
        ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataCeTagsFilterElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsSortByEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<PrimField<String>>,
}

impl DataCeTagsSortByEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `sort_order`.\n"]
    pub fn set_sort_order(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
}

impl ToListMappable for DataCeTagsSortByEl {
    type O = BlockAssignable<DataCeTagsSortByEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsSortByEl {}

impl BuildDataCeTagsSortByEl {
    pub fn build(self) -> DataCeTagsSortByEl {
        DataCeTagsSortByEl {
            key: core::default::Default::default(),
            sort_order: core::default::Default::default(),
        }
    }
}

pub struct DataCeTagsSortByElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsSortByElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsSortByElRef {
        DataCeTagsSortByElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsSortByElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_order` after provisioning.\n"]
    pub fn sort_order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort_order", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCeTagsTimePeriodEl {
    end: PrimField<String>,
    start: PrimField<String>,
}

impl DataCeTagsTimePeriodEl { }

impl ToListMappable for DataCeTagsTimePeriodEl {
    type O = BlockAssignable<DataCeTagsTimePeriodEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCeTagsTimePeriodEl {
    #[doc = ""]
    pub end: PrimField<String>,
    #[doc = ""]
    pub start: PrimField<String>,
}

impl BuildDataCeTagsTimePeriodEl {
    pub fn build(self) -> DataCeTagsTimePeriodEl {
        DataCeTagsTimePeriodEl {
            end: self.end,
            start: self.start,
        }
    }
}

pub struct DataCeTagsTimePeriodElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCeTagsTimePeriodElRef {
    fn new(shared: StackShared, base: String) -> DataCeTagsTimePeriodElRef {
        DataCeTagsTimePeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCeTagsTimePeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCeTagsDynamic {
    filter: Option<DynamicBlock<DataCeTagsFilterEl>>,
    sort_by: Option<DynamicBlock<DataCeTagsSortByEl>>,
    time_period: Option<DynamicBlock<DataCeTagsTimePeriodEl>>,
}
