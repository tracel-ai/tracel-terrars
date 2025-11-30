use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataCloudwatchContributorManagedInsightRulesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_arn: PrimField<String>,
}

struct DataCloudwatchContributorManagedInsightRules_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudwatchContributorManagedInsightRulesData>,
}

#[derive(Clone)]
pub struct DataCloudwatchContributorManagedInsightRules(
    Rc<DataCloudwatchContributorManagedInsightRules_>,
);

impl DataCloudwatchContributorManagedInsightRules {
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

    #[doc = "Get a reference to the value of field `managed_rules` after provisioning.\n"]
    pub fn managed_rules(
        &self,
    ) -> ListRef<DataCloudwatchContributorManagedInsightRulesManagedRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_rules", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_arn", self.extract_ref()),
        )
    }
}

impl Referable for DataCloudwatchContributorManagedInsightRules {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataCloudwatchContributorManagedInsightRules {}

impl ToListMappable for DataCloudwatchContributorManagedInsightRules {
    type O = ListRef<DataCloudwatchContributorManagedInsightRulesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCloudwatchContributorManagedInsightRules_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudwatch_contributor_managed_insight_rules".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudwatchContributorManagedInsightRules {
    pub tf_id: String,
    #[doc = ""]
    pub resource_arn: PrimField<String>,
}

impl BuildDataCloudwatchContributorManagedInsightRules {
    pub fn build(self, stack: &mut Stack) -> DataCloudwatchContributorManagedInsightRules {
        let out = DataCloudwatchContributorManagedInsightRules(Rc::new(
            DataCloudwatchContributorManagedInsightRules_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataCloudwatchContributorManagedInsightRulesData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    region: core::default::Default::default(),
                    resource_arn: self.resource_arn,
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudwatchContributorManagedInsightRulesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudwatchContributorManagedInsightRulesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataCloudwatchContributorManagedInsightRulesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `managed_rules` after provisioning.\n"]
    pub fn managed_rules(
        &self,
    ) -> ListRef<DataCloudwatchContributorManagedInsightRulesManagedRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_rules", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_arn", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateEl {
    #[doc = "Set the field `rule_name`.\n"]
    pub fn set_rule_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule_name = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateEl {
    type O = BlockAssignable<DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateEl {}

impl BuildDataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateEl {
    pub fn build(self) -> DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateEl {
        DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateEl {
            rule_name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateElRef {
        DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchContributorManagedInsightRulesManagedRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_state:
        Option<ListField<DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_name: Option<PrimField<String>>,
}

impl DataCloudwatchContributorManagedInsightRulesManagedRulesEl {
    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_state`.\n"]
    pub fn set_rule_state(
        mut self,
        v: impl Into<ListField<DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateEl>>,
    ) -> Self {
        self.rule_state = Some(v.into());
        self
    }

    #[doc = "Set the field `template_name`.\n"]
    pub fn set_template_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.template_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudwatchContributorManagedInsightRulesManagedRulesEl {
    type O = BlockAssignable<DataCloudwatchContributorManagedInsightRulesManagedRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchContributorManagedInsightRulesManagedRulesEl {}

impl BuildDataCloudwatchContributorManagedInsightRulesManagedRulesEl {
    pub fn build(self) -> DataCloudwatchContributorManagedInsightRulesManagedRulesEl {
        DataCloudwatchContributorManagedInsightRulesManagedRulesEl {
            resource_arn: core::default::Default::default(),
            rule_state: core::default::Default::default(),
            template_name: core::default::Default::default(),
        }
    }
}

pub struct DataCloudwatchContributorManagedInsightRulesManagedRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudwatchContributorManagedInsightRulesManagedRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudwatchContributorManagedInsightRulesManagedRulesElRef {
        DataCloudwatchContributorManagedInsightRulesManagedRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchContributorManagedInsightRulesManagedRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_state` after provisioning.\n"]
    pub fn rule_state(
        &self,
    ) -> ListRef<DataCloudwatchContributorManagedInsightRulesManagedRulesElRuleStateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule_state", self.base))
    }

    #[doc = "Get a reference to the value of field `template_name` after provisioning.\n"]
    pub fn template_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_name", self.base),
        )
    }
}
