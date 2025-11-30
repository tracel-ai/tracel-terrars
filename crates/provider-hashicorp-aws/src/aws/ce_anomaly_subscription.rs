use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CeAnomalySubscriptionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    frequency: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    monitor_arn_list: ListField<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscriber: Option<Vec<CeAnomalySubscriptionSubscriberEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold_expression: Option<Vec<CeAnomalySubscriptionThresholdExpressionEl>>,
    dynamic: CeAnomalySubscriptionDynamic,
}

struct CeAnomalySubscription_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CeAnomalySubscriptionData>,
}

#[derive(Clone)]
pub struct CeAnomalySubscription(Rc<CeAnomalySubscription_>);

impl CeAnomalySubscription {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `subscriber`.\n"]
    pub fn set_subscriber(
        self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionSubscriberEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subscriber = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subscriber = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `threshold_expression`.\n"]
    pub fn set_threshold_expression(
        self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().threshold_expression = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.threshold_expression = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `frequency` after provisioning.\n"]
    pub fn frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.frequency", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitor_arn_list` after provisioning.\n"]
    pub fn monitor_arn_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitor_arn_list", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `threshold_expression` after provisioning.\n"]
    pub fn threshold_expression(&self) -> ListRef<CeAnomalySubscriptionThresholdExpressionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.threshold_expression", self.extract_ref()),
        )
    }
}

impl Referable for CeAnomalySubscription {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CeAnomalySubscription {}

impl ToListMappable for CeAnomalySubscription {
    type O = ListRef<CeAnomalySubscriptionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CeAnomalySubscription_ {
    fn extract_resource_type(&self) -> String {
        "aws_ce_anomaly_subscription".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCeAnomalySubscription {
    pub tf_id: String,
    #[doc = ""]
    pub frequency: PrimField<String>,
    #[doc = ""]
    pub monitor_arn_list: ListField<PrimField<String>>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCeAnomalySubscription {
    pub fn build(self, stack: &mut Stack) -> CeAnomalySubscription {
        let out = CeAnomalySubscription(Rc::new(CeAnomalySubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CeAnomalySubscriptionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                frequency: self.frequency,
                id: core::default::Default::default(),
                monitor_arn_list: self.monitor_arn_list,
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                subscriber: core::default::Default::default(),
                threshold_expression: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CeAnomalySubscriptionRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CeAnomalySubscriptionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `frequency` after provisioning.\n"]
    pub fn frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.frequency", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitor_arn_list` after provisioning.\n"]
    pub fn monitor_arn_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitor_arn_list", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `threshold_expression` after provisioning.\n"]
    pub fn threshold_expression(&self) -> ListRef<CeAnomalySubscriptionThresholdExpressionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.threshold_expression", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionSubscriberEl {
    address: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl CeAnomalySubscriptionSubscriberEl {}

impl ToListMappable for CeAnomalySubscriptionSubscriberEl {
    type O = BlockAssignable<CeAnomalySubscriptionSubscriberEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionSubscriberEl {
    #[doc = ""]
    pub address: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildCeAnomalySubscriptionSubscriberEl {
    pub fn build(self) -> CeAnomalySubscriptionSubscriberEl {
        CeAnomalySubscriptionSubscriberEl {
            address: self.address,
            type_: self.type_,
        }
    }
}

pub struct CeAnomalySubscriptionSubscriberElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionSubscriberElRef {
    fn new(shared: StackShared, base: String) -> CeAnomalySubscriptionSubscriberElRef {
        CeAnomalySubscriptionSubscriberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionSubscriberElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {
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

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {
        CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryElRef {
        CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {
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

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {
        CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElAndElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElAndElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElAndElDimensionElRef {
        CeAnomalySubscriptionThresholdExpressionElAndElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElAndElDimensionElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElAndElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElAndElTagsEl {
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

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElAndElTagsEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElAndElTagsEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElAndElTagsEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElAndElTagsEl {
        CeAnomalySubscriptionThresholdExpressionElAndElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElAndElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElAndElTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElAndElTagsElRef {
        CeAnomalySubscriptionThresholdExpressionElAndElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElAndElTagsElRef {
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

#[derive(Serialize, Default)]
struct CeAnomalySubscriptionThresholdExpressionElAndElDynamic {
    cost_category:
        Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl>>,
    dimension: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl>>,
    tags: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElAndElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElAndEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<Vec<CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CeAnomalySubscriptionThresholdExpressionElAndElTagsEl>>,
    dynamic: CeAnomalySubscriptionThresholdExpressionElAndElDynamic,
}

impl CeAnomalySubscriptionThresholdExpressionElAndEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cost_category = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndElTagsEl>>,
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
}

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElAndEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElAndEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElAndEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElAndEl {
        CeAnomalySubscriptionThresholdExpressionElAndEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElAndElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElAndElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElAndElRef {
        CeAnomalySubscriptionThresholdExpressionElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElAndElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(
        &self,
    ) -> ListRef<CeAnomalySubscriptionThresholdExpressionElAndElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<CeAnomalySubscriptionThresholdExpressionElAndElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElCostCategoryEl {
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

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElCostCategoryEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElCostCategoryEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElCostCategoryEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElCostCategoryEl {
        CeAnomalySubscriptionThresholdExpressionElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElCostCategoryElRef {
        CeAnomalySubscriptionThresholdExpressionElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElCostCategoryElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElDimensionEl {
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

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElDimensionEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElDimensionEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElDimensionEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElDimensionEl {
        CeAnomalySubscriptionThresholdExpressionElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElDimensionElRef {
        CeAnomalySubscriptionThresholdExpressionElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElDimensionElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {
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

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {
        CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryElRef {
        CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {
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

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {
        CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElNotElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElNotElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElNotElDimensionElRef {
        CeAnomalySubscriptionThresholdExpressionElNotElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElNotElDimensionElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElNotElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElNotElTagsEl {
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

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElNotElTagsEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElNotElTagsEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElNotElTagsEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElNotElTagsEl {
        CeAnomalySubscriptionThresholdExpressionElNotElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElNotElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElNotElTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElNotElTagsElRef {
        CeAnomalySubscriptionThresholdExpressionElNotElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElNotElTagsElRef {
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

#[derive(Serialize, Default)]
struct CeAnomalySubscriptionThresholdExpressionElNotElDynamic {
    cost_category:
        Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl>>,
    dimension: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl>>,
    tags: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElNotElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElNotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<Vec<CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CeAnomalySubscriptionThresholdExpressionElNotElTagsEl>>,
    dynamic: CeAnomalySubscriptionThresholdExpressionElNotElDynamic,
}

impl CeAnomalySubscriptionThresholdExpressionElNotEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cost_category = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotElTagsEl>>,
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
}

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElNotEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElNotEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElNotEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElNotEl {
        CeAnomalySubscriptionThresholdExpressionElNotEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElNotElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElNotElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElNotElRef {
        CeAnomalySubscriptionThresholdExpressionElNotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElNotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(
        &self,
    ) -> ListRef<CeAnomalySubscriptionThresholdExpressionElNotElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<CeAnomalySubscriptionThresholdExpressionElNotElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {
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

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {
        CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryElRef {
        CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {
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

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {
        CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElOrElDimensionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElOrElDimensionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElOrElDimensionElRef {
        CeAnomalySubscriptionThresholdExpressionElOrElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElOrElDimensionElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElOrElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElOrElTagsEl {
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

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElOrElTagsEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElOrElTagsEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElOrElTagsEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElOrElTagsEl {
        CeAnomalySubscriptionThresholdExpressionElOrElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElOrElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElOrElTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElOrElTagsElRef {
        CeAnomalySubscriptionThresholdExpressionElOrElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElOrElTagsElRef {
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

#[derive(Serialize, Default)]
struct CeAnomalySubscriptionThresholdExpressionElOrElDynamic {
    cost_category:
        Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl>>,
    dimension: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl>>,
    tags: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElOrElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElOrEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<Vec<CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CeAnomalySubscriptionThresholdExpressionElOrElTagsEl>>,
    dynamic: CeAnomalySubscriptionThresholdExpressionElOrElDynamic,
}

impl CeAnomalySubscriptionThresholdExpressionElOrEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cost_category = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrElTagsEl>>,
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
}

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElOrEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElOrEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElOrEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElOrEl {
        CeAnomalySubscriptionThresholdExpressionElOrEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElOrElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElOrElRef {
    fn new(shared: StackShared, base: String) -> CeAnomalySubscriptionThresholdExpressionElOrElRef {
        CeAnomalySubscriptionThresholdExpressionElOrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElOrElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(
        &self,
    ) -> ListRef<CeAnomalySubscriptionThresholdExpressionElOrElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<CeAnomalySubscriptionThresholdExpressionElOrElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_options: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElTagsEl {
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

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionElTagsEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionElTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElTagsEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElTagsEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElTagsEl {
        CeAnomalySubscriptionThresholdExpressionElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CeAnomalySubscriptionThresholdExpressionElTagsElRef {
        CeAnomalySubscriptionThresholdExpressionElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElTagsElRef {
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

#[derive(Serialize, Default)]
struct CeAnomalySubscriptionThresholdExpressionElDynamic {
    and: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElAndEl>>,
    cost_category: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElCostCategoryEl>>,
    dimension: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElDimensionEl>>,
    not: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElNotEl>>,
    or: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElOrEl>>,
    tags: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<CeAnomalySubscriptionThresholdExpressionElAndEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost_category: Option<Vec<CeAnomalySubscriptionThresholdExpressionElCostCategoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<Vec<CeAnomalySubscriptionThresholdExpressionElDimensionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not: Option<Vec<CeAnomalySubscriptionThresholdExpressionElNotEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<Vec<CeAnomalySubscriptionThresholdExpressionElOrEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<CeAnomalySubscriptionThresholdExpressionElTagsEl>>,
    dynamic: CeAnomalySubscriptionThresholdExpressionElDynamic,
}

impl CeAnomalySubscriptionThresholdExpressionEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.and = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.and = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElCostCategoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cost_category = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElDimensionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dimension = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.not = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.not = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.or = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.or = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<BlockAssignable<CeAnomalySubscriptionThresholdExpressionElTagsEl>>,
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
}

impl ToListMappable for CeAnomalySubscriptionThresholdExpressionEl {
    type O = BlockAssignable<CeAnomalySubscriptionThresholdExpressionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionEl {
        CeAnomalySubscriptionThresholdExpressionEl {
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

pub struct CeAnomalySubscriptionThresholdExpressionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CeAnomalySubscriptionThresholdExpressionElRef {
    fn new(shared: StackShared, base: String) -> CeAnomalySubscriptionThresholdExpressionElRef {
        CeAnomalySubscriptionThresholdExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(
        &self,
    ) -> ListRef<CeAnomalySubscriptionThresholdExpressionElCostCategoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_category", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> ListRef<CeAnomalySubscriptionThresholdExpressionElDimensionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> ListRef<CeAnomalySubscriptionThresholdExpressionElNotElRef> {
        ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<CeAnomalySubscriptionThresholdExpressionElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeAnomalySubscriptionDynamic {
    subscriber: Option<DynamicBlock<CeAnomalySubscriptionSubscriberEl>>,
    threshold_expression: Option<DynamicBlock<CeAnomalySubscriptionThresholdExpressionEl>>,
}
