use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBatchSchedulingPolicyData {
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

struct DataBatchSchedulingPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBatchSchedulingPolicyData>,
}

#[derive(Clone)]
pub struct DataBatchSchedulingPolicy(Rc<DataBatchSchedulingPolicy_>);

impl DataBatchSchedulingPolicy {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fair_share_policy` after provisioning.\n"]
    pub fn fair_share_policy(&self) -> ListRef<DataBatchSchedulingPolicyFairSharePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fair_share_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

impl Referable for DataBatchSchedulingPolicy {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataBatchSchedulingPolicy { }

impl ToListMappable for DataBatchSchedulingPolicy {
    type O = ListRef<DataBatchSchedulingPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBatchSchedulingPolicy_ {
    fn extract_datasource_type(&self) -> String {
        "aws_batch_scheduling_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBatchSchedulingPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildDataBatchSchedulingPolicy {
    pub fn build(self, stack: &mut Stack) -> DataBatchSchedulingPolicy {
        let out = DataBatchSchedulingPolicy(Rc::new(DataBatchSchedulingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBatchSchedulingPolicyData {
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

pub struct DataBatchSchedulingPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBatchSchedulingPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataBatchSchedulingPolicyRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fair_share_policy` after provisioning.\n"]
    pub fn fair_share_policy(&self) -> ListRef<DataBatchSchedulingPolicyFairSharePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fair_share_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
pub struct DataBatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    share_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight_factor: Option<PrimField<f64>>,
}

impl DataBatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    #[doc = "Set the field `share_identifier`.\n"]
    pub fn set_share_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.share_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `weight_factor`.\n"]
    pub fn set_weight_factor(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight_factor = Some(v.into());
        self
    }
}

impl ToListMappable for DataBatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    type O = BlockAssignable<DataBatchSchedulingPolicyFairSharePolicyElShareDistributionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchSchedulingPolicyFairSharePolicyElShareDistributionEl {}

impl BuildDataBatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    pub fn build(self) -> DataBatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
        DataBatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
            share_identifier: core::default::Default::default(),
            weight_factor: core::default::Default::default(),
        }
    }
}

pub struct DataBatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
    fn new(shared: StackShared, base: String) -> DataBatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
        DataBatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `share_identifier` after provisioning.\n"]
    pub fn share_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `weight_factor` after provisioning.\n"]
    pub fn weight_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight_factor", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBatchSchedulingPolicyFairSharePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_reservation: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_decay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_distribution: Option<SetField<DataBatchSchedulingPolicyFairSharePolicyElShareDistributionEl>>,
}

impl DataBatchSchedulingPolicyFairSharePolicyEl {
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
        v: impl Into<SetField<DataBatchSchedulingPolicyFairSharePolicyElShareDistributionEl>>,
    ) -> Self {
        self.share_distribution = Some(v.into());
        self
    }
}

impl ToListMappable for DataBatchSchedulingPolicyFairSharePolicyEl {
    type O = BlockAssignable<DataBatchSchedulingPolicyFairSharePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchSchedulingPolicyFairSharePolicyEl {}

impl BuildDataBatchSchedulingPolicyFairSharePolicyEl {
    pub fn build(self) -> DataBatchSchedulingPolicyFairSharePolicyEl {
        DataBatchSchedulingPolicyFairSharePolicyEl {
            compute_reservation: core::default::Default::default(),
            share_decay_seconds: core::default::Default::default(),
            share_distribution: core::default::Default::default(),
        }
    }
}

pub struct DataBatchSchedulingPolicyFairSharePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBatchSchedulingPolicyFairSharePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataBatchSchedulingPolicyFairSharePolicyElRef {
        DataBatchSchedulingPolicyFairSharePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchSchedulingPolicyFairSharePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compute_reservation` after provisioning.\n"]
    pub fn compute_reservation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_reservation", self.base))
    }

    #[doc = "Get a reference to the value of field `share_decay_seconds` after provisioning.\n"]
    pub fn share_decay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_decay_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `share_distribution` after provisioning.\n"]
    pub fn share_distribution(&self) -> SetRef<DataBatchSchedulingPolicyFairSharePolicyElShareDistributionElRef> {
        SetRef::new(self.shared().clone(), format!("{}.share_distribution", self.base))
    }
}
