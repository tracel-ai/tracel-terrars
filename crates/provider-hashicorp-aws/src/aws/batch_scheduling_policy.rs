use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BatchSchedulingPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
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
    fair_share_policy: Option<Vec<BatchSchedulingPolicyFairSharePolicyEl>>,
    dynamic: BatchSchedulingPolicyDynamic,
}
struct BatchSchedulingPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BatchSchedulingPolicyData>,
}
#[derive(Clone)]
pub struct BatchSchedulingPolicy(Rc<BatchSchedulingPolicy_>);
impl BatchSchedulingPolicy {
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
    #[doc = "Set the field `fair_share_policy`.\n"]
    pub fn set_fair_share_policy(
        self,
        v: impl Into<BlockAssignable<BatchSchedulingPolicyFairSharePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fair_share_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fair_share_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `fair_share_policy` after provisioning.\n"]
    pub fn fair_share_policy(&self) -> ListRef<BatchSchedulingPolicyFairSharePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fair_share_policy", self.extract_ref()),
        )
    }
}
impl Referable for BatchSchedulingPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BatchSchedulingPolicy {}
impl ToListMappable for BatchSchedulingPolicy {
    type O = ListRef<BatchSchedulingPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BatchSchedulingPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_batch_scheduling_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBatchSchedulingPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildBatchSchedulingPolicy {
    pub fn build(self, stack: &mut Stack) -> BatchSchedulingPolicy {
        let out = BatchSchedulingPolicy(Rc::new(BatchSchedulingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BatchSchedulingPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                fair_share_policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BatchSchedulingPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for BatchSchedulingPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BatchSchedulingPolicyRef {
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
    #[doc = "Get a reference to the value of field `fair_share_policy` after provisioning.\n"]
    pub fn fair_share_policy(&self) -> ListRef<BatchSchedulingPolicyFairSharePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fair_share_policy", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    share_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight_factor: Option<PrimField<f64>>,
}
impl BatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    #[doc = "Set the field `weight_factor`.\n"]
    pub fn set_weight_factor(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight_factor = Some(v.into());
        self
    }
}
impl ToListMappable for BatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    type O = BlockAssignable<BatchSchedulingPolicyFairSharePolicyElShareDistributionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    #[doc = ""]
    pub share_identifier: PrimField<String>,
}
impl BuildBatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    pub fn build(self) -> BatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
        BatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
            share_identifier: self.share_identifier,
            weight_factor: core::default::Default::default(),
        }
    }
}
pub struct BatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
        BatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `share_identifier` after provisioning.\n"]
    pub fn share_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.share_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weight_factor` after provisioning.\n"]
    pub fn weight_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.weight_factor", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BatchSchedulingPolicyFairSharePolicyElDynamic {
    share_distribution:
        Option<DynamicBlock<BatchSchedulingPolicyFairSharePolicyElShareDistributionEl>>,
}
#[derive(Serialize)]
pub struct BatchSchedulingPolicyFairSharePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_reservation: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_decay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_distribution: Option<Vec<BatchSchedulingPolicyFairSharePolicyElShareDistributionEl>>,
    dynamic: BatchSchedulingPolicyFairSharePolicyElDynamic,
}
impl BatchSchedulingPolicyFairSharePolicyEl {
    #[doc = "Set the field `compute_reservation`.\n"]
    pub fn set_compute_reservation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.compute_reservation = Some(v.into());
        self
    }
    #[doc = "Set the field `share_decay_seconds`.\n"]
    pub fn set_share_decay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.share_decay_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `share_distribution`.\n"]
    pub fn set_share_distribution(
        mut self,
        v: impl Into<BlockAssignable<BatchSchedulingPolicyFairSharePolicyElShareDistributionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.share_distribution = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.share_distribution = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BatchSchedulingPolicyFairSharePolicyEl {
    type O = BlockAssignable<BatchSchedulingPolicyFairSharePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBatchSchedulingPolicyFairSharePolicyEl {}
impl BuildBatchSchedulingPolicyFairSharePolicyEl {
    pub fn build(self) -> BatchSchedulingPolicyFairSharePolicyEl {
        BatchSchedulingPolicyFairSharePolicyEl {
            compute_reservation: core::default::Default::default(),
            share_decay_seconds: core::default::Default::default(),
            share_distribution: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BatchSchedulingPolicyFairSharePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BatchSchedulingPolicyFairSharePolicyElRef {
    fn new(shared: StackShared, base: String) -> BatchSchedulingPolicyFairSharePolicyElRef {
        BatchSchedulingPolicyFairSharePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BatchSchedulingPolicyFairSharePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compute_reservation` after provisioning.\n"]
    pub fn compute_reservation(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_reservation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `share_decay_seconds` after provisioning.\n"]
    pub fn share_decay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.share_decay_seconds", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BatchSchedulingPolicyDynamic {
    fair_share_policy: Option<DynamicBlock<BatchSchedulingPolicyFairSharePolicyEl>>,
}
