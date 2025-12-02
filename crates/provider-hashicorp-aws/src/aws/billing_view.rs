use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BillingViewData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_views: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_filter_expression: Option<Vec<BillingViewDataFilterExpressionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BillingViewTimeoutsEl>,
    dynamic: BillingViewDynamic,
}
struct BillingView_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BillingViewData>,
}
#[derive(Clone)]
pub struct BillingView(Rc<BillingView_>);
impl BillingView {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `source_views`.\n"]
    pub fn set_source_views(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().source_views = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `data_filter_expression`.\n"]
    pub fn set_data_filter_expression(
        self,
        v: impl Into<BlockAssignable<BillingViewDataFilterExpressionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_filter_expression = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_filter_expression = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BillingViewTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `billing_view_type` after provisioning.\n"]
    pub fn billing_view_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_view_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `derived_view_count` after provisioning.\n"]
    pub fn derived_view_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.derived_view_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_account_id` after provisioning.\n"]
    pub fn source_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_view_count` after provisioning.\n"]
    pub fn source_view_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_view_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_views` after provisioning.\n"]
    pub fn source_views(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_views", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view_definition_last_updated_at` after provisioning.\n"]
    pub fn view_definition_last_updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view_definition_last_updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_filter_expression` after provisioning.\n"]
    pub fn data_filter_expression(&self) -> ListRef<BillingViewDataFilterExpressionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_filter_expression", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BillingViewTimeoutsElRef {
        BillingViewTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BillingView {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BillingView {}
impl ToListMappable for BillingView {
    type O = ListRef<BillingViewRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BillingView_ {
    fn extract_resource_type(&self) -> String {
        "aws_billing_view".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBillingView {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildBillingView {
    pub fn build(self, stack: &mut Stack) -> BillingView {
        let out = BillingView(Rc::new(BillingView_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BillingViewData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                name: self.name,
                source_views: core::default::Default::default(),
                tags: core::default::Default::default(),
                data_filter_expression: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BillingViewRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingViewRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BillingViewRef {
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
    #[doc = "Get a reference to the value of field `billing_view_type` after provisioning.\n"]
    pub fn billing_view_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_view_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `derived_view_count` after provisioning.\n"]
    pub fn derived_view_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.derived_view_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_account_id` after provisioning.\n"]
    pub fn source_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_view_count` after provisioning.\n"]
    pub fn source_view_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_view_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_views` after provisioning.\n"]
    pub fn source_views(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_views", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view_definition_last_updated_at` after provisioning.\n"]
    pub fn view_definition_last_updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view_definition_last_updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_filter_expression` after provisioning.\n"]
    pub fn data_filter_expression(&self) -> ListRef<BillingViewDataFilterExpressionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_filter_expression", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BillingViewTimeoutsElRef {
        BillingViewTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BillingViewDataFilterExpressionElDimensionsEl {
    key: PrimField<String>,
    values: ListField<PrimField<String>>,
}
impl BillingViewDataFilterExpressionElDimensionsEl {}
impl ToListMappable for BillingViewDataFilterExpressionElDimensionsEl {
    type O = BlockAssignable<BillingViewDataFilterExpressionElDimensionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingViewDataFilterExpressionElDimensionsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}
impl BuildBillingViewDataFilterExpressionElDimensionsEl {
    pub fn build(self) -> BillingViewDataFilterExpressionElDimensionsEl {
        BillingViewDataFilterExpressionElDimensionsEl {
            key: self.key,
            values: self.values,
        }
    }
}
pub struct BillingViewDataFilterExpressionElDimensionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingViewDataFilterExpressionElDimensionsElRef {
    fn new(shared: StackShared, base: String) -> BillingViewDataFilterExpressionElDimensionsElRef {
        BillingViewDataFilterExpressionElDimensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingViewDataFilterExpressionElDimensionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct BillingViewDataFilterExpressionElTagsEl {
    key: PrimField<String>,
    values: ListField<PrimField<String>>,
}
impl BillingViewDataFilterExpressionElTagsEl {}
impl ToListMappable for BillingViewDataFilterExpressionElTagsEl {
    type O = BlockAssignable<BillingViewDataFilterExpressionElTagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingViewDataFilterExpressionElTagsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}
impl BuildBillingViewDataFilterExpressionElTagsEl {
    pub fn build(self) -> BillingViewDataFilterExpressionElTagsEl {
        BillingViewDataFilterExpressionElTagsEl {
            key: self.key,
            values: self.values,
        }
    }
}
pub struct BillingViewDataFilterExpressionElTagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingViewDataFilterExpressionElTagsElRef {
    fn new(shared: StackShared, base: String) -> BillingViewDataFilterExpressionElTagsElRef {
        BillingViewDataFilterExpressionElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingViewDataFilterExpressionElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct BillingViewDataFilterExpressionElTimeRangeEl {
    begin_date_inclusive: PrimField<String>,
    end_date_inclusive: PrimField<String>,
}
impl BillingViewDataFilterExpressionElTimeRangeEl {}
impl ToListMappable for BillingViewDataFilterExpressionElTimeRangeEl {
    type O = BlockAssignable<BillingViewDataFilterExpressionElTimeRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingViewDataFilterExpressionElTimeRangeEl {
    #[doc = ""]
    pub begin_date_inclusive: PrimField<String>,
    #[doc = ""]
    pub end_date_inclusive: PrimField<String>,
}
impl BuildBillingViewDataFilterExpressionElTimeRangeEl {
    pub fn build(self) -> BillingViewDataFilterExpressionElTimeRangeEl {
        BillingViewDataFilterExpressionElTimeRangeEl {
            begin_date_inclusive: self.begin_date_inclusive,
            end_date_inclusive: self.end_date_inclusive,
        }
    }
}
pub struct BillingViewDataFilterExpressionElTimeRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingViewDataFilterExpressionElTimeRangeElRef {
    fn new(shared: StackShared, base: String) -> BillingViewDataFilterExpressionElTimeRangeElRef {
        BillingViewDataFilterExpressionElTimeRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingViewDataFilterExpressionElTimeRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `begin_date_inclusive` after provisioning.\n"]
    pub fn begin_date_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.begin_date_inclusive", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `end_date_inclusive` after provisioning.\n"]
    pub fn end_date_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_date_inclusive", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BillingViewDataFilterExpressionElDynamic {
    dimensions: Option<DynamicBlock<BillingViewDataFilterExpressionElDimensionsEl>>,
    tags: Option<DynamicBlock<BillingViewDataFilterExpressionElTagsEl>>,
    time_range: Option<DynamicBlock<BillingViewDataFilterExpressionElTimeRangeEl>>,
}
#[derive(Serialize)]
pub struct BillingViewDataFilterExpressionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<Vec<BillingViewDataFilterExpressionElDimensionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<BillingViewDataFilterExpressionElTagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_range: Option<Vec<BillingViewDataFilterExpressionElTimeRangeEl>>,
    dynamic: BillingViewDataFilterExpressionElDynamic,
}
impl BillingViewDataFilterExpressionEl {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl Into<BlockAssignable<BillingViewDataFilterExpressionElDimensionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimensions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<BlockAssignable<BillingViewDataFilterExpressionElTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `time_range`.\n"]
    pub fn set_time_range(
        mut self,
        v: impl Into<BlockAssignable<BillingViewDataFilterExpressionElTimeRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BillingViewDataFilterExpressionEl {
    type O = BlockAssignable<BillingViewDataFilterExpressionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingViewDataFilterExpressionEl {}
impl BuildBillingViewDataFilterExpressionEl {
    pub fn build(self) -> BillingViewDataFilterExpressionEl {
        BillingViewDataFilterExpressionEl {
            dimensions: core::default::Default::default(),
            tags: core::default::Default::default(),
            time_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BillingViewDataFilterExpressionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingViewDataFilterExpressionElRef {
    fn new(shared: StackShared, base: String) -> BillingViewDataFilterExpressionElRef {
        BillingViewDataFilterExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingViewDataFilterExpressionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> ListRef<BillingViewDataFilterExpressionElDimensionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<BillingViewDataFilterExpressionElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `time_range` after provisioning.\n"]
    pub fn time_range(&self) -> ListRef<BillingViewDataFilterExpressionElTimeRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_range", self.base))
    }
}
#[derive(Serialize)]
pub struct BillingViewTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BillingViewTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for BillingViewTimeoutsEl {
    type O = BlockAssignable<BillingViewTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBillingViewTimeoutsEl {}
impl BuildBillingViewTimeoutsEl {
    pub fn build(self) -> BillingViewTimeoutsEl {
        BillingViewTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BillingViewTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BillingViewTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BillingViewTimeoutsElRef {
        BillingViewTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BillingViewTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct BillingViewDynamic {
    data_filter_expression: Option<DynamicBlock<BillingViewDataFilterExpressionEl>>,
}
