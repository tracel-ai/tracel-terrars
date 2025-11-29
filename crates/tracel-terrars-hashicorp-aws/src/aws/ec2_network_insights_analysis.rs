use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2NetworkInsightsAnalysisData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_in_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    network_insights_path_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_completion: Option<PrimField<bool>>,
}

struct Ec2NetworkInsightsAnalysis_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Ec2NetworkInsightsAnalysisData>,
}

#[derive(Clone)]
pub struct Ec2NetworkInsightsAnalysis(Rc<Ec2NetworkInsightsAnalysis_>);

impl Ec2NetworkInsightsAnalysis {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `filter_in_arns`.\n"]
    pub fn set_filter_in_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().filter_in_arns = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for_completion`.\n"]
    pub fn set_wait_for_completion(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_completion = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `alternate_path_hints` after provisioning.\n"]
    pub fn alternate_path_hints(&self) -> ListRef<Ec2NetworkInsightsAnalysisAlternatePathHintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alternate_path_hints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `explanations` after provisioning.\n"]
    pub fn explanations(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.explanations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_in_arns` after provisioning.\n"]
    pub fn filter_in_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.filter_in_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forward_path_components` after provisioning.\n"]
    pub fn forward_path_components(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forward_path_components", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_insights_path_id` after provisioning.\n"]
    pub fn network_insights_path_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_insights_path_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_found` after provisioning.\n"]
    pub fn path_found(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_found", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `return_path_components` after provisioning.\n"]
    pub fn return_path_components(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.return_path_components", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_completion` after provisioning.\n"]
    pub fn wait_for_completion(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_for_completion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warning_message` after provisioning.\n"]
    pub fn warning_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.warning_message", self.extract_ref()))
    }
}

impl Referable for Ec2NetworkInsightsAnalysis {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Ec2NetworkInsightsAnalysis { }

impl ToListMappable for Ec2NetworkInsightsAnalysis {
    type O = ListRef<Ec2NetworkInsightsAnalysisRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Ec2NetworkInsightsAnalysis_ {
    fn extract_resource_type(&self) -> String {
        "aws_ec2_network_insights_analysis".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2NetworkInsightsAnalysis {
    pub tf_id: String,
    #[doc = ""]
    pub network_insights_path_id: PrimField<String>,
}

impl BuildEc2NetworkInsightsAnalysis {
    pub fn build(self, stack: &mut Stack) -> Ec2NetworkInsightsAnalysis {
        let out = Ec2NetworkInsightsAnalysis(Rc::new(Ec2NetworkInsightsAnalysis_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Ec2NetworkInsightsAnalysisData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                filter_in_arns: core::default::Default::default(),
                id: core::default::Default::default(),
                network_insights_path_id: self.network_insights_path_id,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                wait_for_completion: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2NetworkInsightsAnalysisRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Ec2NetworkInsightsAnalysisRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alternate_path_hints` after provisioning.\n"]
    pub fn alternate_path_hints(&self) -> ListRef<Ec2NetworkInsightsAnalysisAlternatePathHintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alternate_path_hints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `explanations` after provisioning.\n"]
    pub fn explanations(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.explanations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_in_arns` after provisioning.\n"]
    pub fn filter_in_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.filter_in_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forward_path_components` after provisioning.\n"]
    pub fn forward_path_components(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forward_path_components", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_insights_path_id` after provisioning.\n"]
    pub fn network_insights_path_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_insights_path_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_found` after provisioning.\n"]
    pub fn path_found(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_found", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `return_path_components` after provisioning.\n"]
    pub fn return_path_components(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.return_path_components", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_completion` after provisioning.\n"]
    pub fn wait_for_completion(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_for_completion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warning_message` after provisioning.\n"]
    pub fn warning_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.warning_message", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisAlternatePathHintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    component_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    component_id: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisAlternatePathHintsEl {
    #[doc = "Set the field `component_arn`.\n"]
    pub fn set_component_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.component_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `component_id`.\n"]
    pub fn set_component_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.component_id = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisAlternatePathHintsEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisAlternatePathHintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisAlternatePathHintsEl {}

impl BuildEc2NetworkInsightsAnalysisAlternatePathHintsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisAlternatePathHintsEl {
        Ec2NetworkInsightsAnalysisAlternatePathHintsEl {
            component_arn: core::default::Default::default(),
            component_id: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisAlternatePathHintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisAlternatePathHintsElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisAlternatePathHintsElRef {
        Ec2NetworkInsightsAnalysisAlternatePathHintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisAlternatePathHintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_arn` after provisioning.\n"]
    pub fn component_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.component_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `component_id` after provisioning.\n"]
    pub fn component_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.component_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElAclEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElAclEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElAclEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElAclEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElAclEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElAclEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElAclEl {
        Ec2NetworkInsightsAnalysisExplanationsElAclEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElAclElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElAclElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElAclElRef {
        Ec2NetworkInsightsAnalysisExplanationsElAclElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElAclElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
        Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
        Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_number: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl {
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.egress = Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl>>,
    ) -> Self {
        self.port_range = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_action`.\n"]
    pub fn set_rule_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule_action = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_number`.\n"]
    pub fn set_rule_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rule_number = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElAclRuleEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElAclRuleEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl {
        Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl {
            cidr: core::default::Default::default(),
            egress: core::default::Default::default(),
            port_range: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rule_action: core::default::Default::default(),
            rule_number: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
        Ec2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_action", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElAttachedToEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElAttachedToEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl {
        Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
        Ec2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_port: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
    #[doc = "Set the field `instance_port`.\n"]
    pub fn set_instance_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_port = Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_port`.\n"]
    pub fn set_load_balancer_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.load_balancer_port = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
        Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
            instance_port: core::default::Default::default(),
            load_balancer_port: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
        Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_port` after provisioning.\n"]
    pub fn instance_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_port", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_port` after provisioning.\n"]
    pub fn load_balancer_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_port", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElComponentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElComponentEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElComponentEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElComponentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElComponentEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElComponentEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElComponentEl {
        Ec2NetworkInsightsAnalysisExplanationsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElComponentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElComponentElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElComponentElRef {
        Ec2NetworkInsightsAnalysisExplanationsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElComponentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
        Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
        Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElDestinationEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElDestinationEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElDestinationEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElDestinationEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElDestinationEl {
        Ec2NetworkInsightsAnalysisExplanationsElDestinationEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElDestinationElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElDestinationElRef {
        Ec2NetworkInsightsAnalysisExplanationsElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
        Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
        Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
        Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
        Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
        Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
        Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
        Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
        Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
        Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
        Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
        Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
        Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
        Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
        Ec2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
        Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
        Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElPortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElPortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl {
        Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
        Ec2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElPrefixListEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElPrefixListEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl {
        Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
        Ec2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElRouteTableEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElRouteTableEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl {
        Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
        Ec2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_prefix_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_only_internet_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_peering_connection_id: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
    #[doc = "Set the field `destination_cidr`.\n"]
    pub fn set_destination_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_prefix_list_id`.\n"]
    pub fn set_destination_prefix_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_prefix_list_id = Some(v.into());
        self
    }

    #[doc = "Set the field `egress_only_internet_gateway_id`.\n"]
    pub fn set_egress_only_internet_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.egress_only_internet_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_id`.\n"]
    pub fn set_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_id`.\n"]
    pub fn set_instance_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_id = Some(v.into());
        self
    }

    #[doc = "Set the field `nat_gateway_id`.\n"]
    pub fn set_nat_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nat_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }

    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transit_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering_connection_id`.\n"]
    pub fn set_vpc_peering_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_peering_connection_id = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
        Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
            destination_cidr: core::default::Default::default(),
            destination_prefix_list_id: core::default::Default::default(),
            egress_only_internet_gateway_id: core::default::Default::default(),
            gateway_id: core::default::Default::default(),
            instance_id: core::default::Default::default(),
            nat_gateway_id: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            origin: core::default::Default::default(),
            transit_gateway_id: core::default::Default::default(),
            vpc_peering_connection_id: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
        Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_cidr` after provisioning.\n"]
    pub fn destination_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_prefix_list_id` after provisioning.\n"]
    pub fn destination_prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_only_internet_gateway_id` after provisioning.\n"]
    pub fn egress_only_internet_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress_only_internet_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_gateway_id` after provisioning.\n"]
    pub fn nat_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nat_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_connection_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_id: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl>>,
    ) -> Self {
        self.port_range = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list_id`.\n"]
    pub fn set_prefix_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_list_id = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_id`.\n"]
    pub fn set_security_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_group_id = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
            cidr: core::default::Default::default(),
            direction: core::default::Default::default(),
            port_range: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            protocol: core::default::Default::default(),
            security_group_id: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list_id` after provisioning.\n"]
    pub fn prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
        Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElSubnetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSubnetEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSubnetEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSubnetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSubnetEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSubnetEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSubnetEl {
        Ec2NetworkInsightsAnalysisExplanationsElSubnetEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSubnetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElSubnetElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElSubnetElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSubnetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSubnetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
        Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_origin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
    #[doc = "Set the field `attachment_id`.\n"]
    pub fn set_attachment_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attachment_id = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_cidr`.\n"]
    pub fn set_destination_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list_id`.\n"]
    pub fn set_prefix_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_list_id = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_id = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }

    #[doc = "Set the field `route_origin`.\n"]
    pub fn set_route_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.route_origin = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
            attachment_id: core::default::Default::default(),
            destination_cidr: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            route_origin: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_cidr` after provisioning.\n"]
    pub fn destination_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list_id` after provisioning.\n"]
    pub fn prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `route_origin` after provisioning.\n"]
    pub fn route_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_origin", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElVpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpcEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElVpcEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElVpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElVpcEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElVpcEl {
        Ec2NetworkInsightsAnalysisExplanationsElVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElVpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElVpcElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElVpcElRef {
        Ec2NetworkInsightsAnalysisExplanationsElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
        Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
        Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
        Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
        Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
        Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
        Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
        Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
        Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisExplanationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acl: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElAclEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acl_rule: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attached_to: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zones: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidrs: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classic_load_balancer_listener: Option<
        ListField<Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    component: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElComponentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_gateway: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_vpc: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elastic_load_balancer_listener: Option<
        ListField<Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_route_table: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internet_gateway: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_listener_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_target_group: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_target_groups: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_target_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    missing_component: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_gateway: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packet_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_ranges: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_list: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocols: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_table: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_table_route: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_rule: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_vpc: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElSubnetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_route_table: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_attachment: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_route_table: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_route_table_route: Option<
        ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElVpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_peering_connection: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpn_connection: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpn_gateway: Option<ListField<Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsEl {
    #[doc = "Set the field `acl`.\n"]
    pub fn set_acl(mut self, v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElAclEl>>) -> Self {
        self.acl = Some(v.into());
        self
    }

    #[doc = "Set the field `acl_rule`.\n"]
    pub fn set_acl_rule(mut self, v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl>>) -> Self {
        self.acl_rule = Some(v.into());
        self
    }

    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }

    #[doc = "Set the field `addresses`.\n"]
    pub fn set_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `attached_to`.\n"]
    pub fn set_attached_to(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl>>,
    ) -> Self {
        self.attached_to = Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.availability_zones = Some(v.into());
        self
    }

    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.cidrs = Some(v.into());
        self
    }

    #[doc = "Set the field `classic_load_balancer_listener`.\n"]
    pub fn set_classic_load_balancer_listener(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl>>,
    ) -> Self {
        self.classic_load_balancer_listener = Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElComponentEl>>,
    ) -> Self {
        self.component = Some(v.into());
        self
    }

    #[doc = "Set the field `customer_gateway`.\n"]
    pub fn set_customer_gateway(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl>>,
    ) -> Self {
        self.customer_gateway = Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElDestinationEl>>,
    ) -> Self {
        self.destination = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_vpc`.\n"]
    pub fn set_destination_vpc(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl>>,
    ) -> Self {
        self.destination_vpc = Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }

    #[doc = "Set the field `elastic_load_balancer_listener`.\n"]
    pub fn set_elastic_load_balancer_listener(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl>>,
    ) -> Self {
        self.elastic_load_balancer_listener = Some(v.into());
        self
    }

    #[doc = "Set the field `explanation_code`.\n"]
    pub fn set_explanation_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.explanation_code = Some(v.into());
        self
    }

    #[doc = "Set the field `ingress_route_table`.\n"]
    pub fn set_ingress_route_table(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl>>,
    ) -> Self {
        self.ingress_route_table = Some(v.into());
        self
    }

    #[doc = "Set the field `internet_gateway`.\n"]
    pub fn set_internet_gateway(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl>>,
    ) -> Self {
        self.internet_gateway = Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_arn`.\n"]
    pub fn set_load_balancer_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.load_balancer_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_listener_port`.\n"]
    pub fn set_load_balancer_listener_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.load_balancer_listener_port = Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_target_group`.\n"]
    pub fn set_load_balancer_target_group(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl>>,
    ) -> Self {
        self.load_balancer_target_group = Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_target_groups`.\n"]
    pub fn set_load_balancer_target_groups(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl>>,
    ) -> Self {
        self.load_balancer_target_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_target_port`.\n"]
    pub fn set_load_balancer_target_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.load_balancer_target_port = Some(v.into());
        self
    }

    #[doc = "Set the field `missing_component`.\n"]
    pub fn set_missing_component(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.missing_component = Some(v.into());
        self
    }

    #[doc = "Set the field `nat_gateway`.\n"]
    pub fn set_nat_gateway(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl>>,
    ) -> Self {
        self.nat_gateway = Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl>>,
    ) -> Self {
        self.network_interface = Some(v.into());
        self
    }

    #[doc = "Set the field `packet_field`.\n"]
    pub fn set_packet_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.packet_field = Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc = "Set the field `port_ranges`.\n"]
    pub fn set_port_ranges(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl>>,
    ) -> Self {
        self.port_ranges = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list`.\n"]
    pub fn set_prefix_list(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl>>,
    ) -> Self {
        self.prefix_list = Some(v.into());
        self
    }

    #[doc = "Set the field `protocols`.\n"]
    pub fn set_protocols(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.protocols = Some(v.into());
        self
    }

    #[doc = "Set the field `route_table`.\n"]
    pub fn set_route_table(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl>>,
    ) -> Self {
        self.route_table = Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_route`.\n"]
    pub fn set_route_table_route(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl>>,
    ) -> Self {
        self.route_table_route = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group`.\n"]
    pub fn set_security_group(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl>>,
    ) -> Self {
        self.security_group = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_rule`.\n"]
    pub fn set_security_group_rule(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl>>,
    ) -> Self {
        self.security_group_rule = Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl>>,
    ) -> Self {
        self.security_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `source_vpc`.\n"]
    pub fn set_source_vpc(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl>>,
    ) -> Self {
        self.source_vpc = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(mut self, v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElSubnetEl>>) -> Self {
        self.subnet = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_route_table`.\n"]
    pub fn set_subnet_route_table(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl>>,
    ) -> Self {
        self.subnet_route_table = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway`.\n"]
    pub fn set_transit_gateway(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl>>,
    ) -> Self {
        self.transit_gateway = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_attachment`.\n"]
    pub fn set_transit_gateway_attachment(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl>>,
    ) -> Self {
        self.transit_gateway_attachment = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_route_table`.\n"]
    pub fn set_transit_gateway_route_table(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl>>,
    ) -> Self {
        self.transit_gateway_route_table = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_route_table_route`.\n"]
    pub fn set_transit_gateway_route_table_route(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl>>,
    ) -> Self {
        self.transit_gateway_route_table_route = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(mut self, v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElVpcEl>>) -> Self {
        self.vpc = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint`.\n"]
    pub fn set_vpc_endpoint(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl>>,
    ) -> Self {
        self.vpc_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering_connection`.\n"]
    pub fn set_vpc_peering_connection(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl>>,
    ) -> Self {
        self.vpc_peering_connection = Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_connection`.\n"]
    pub fn set_vpn_connection(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl>>,
    ) -> Self {
        self.vpn_connection = Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_gateway`.\n"]
    pub fn set_vpn_gateway(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl>>,
    ) -> Self {
        self.vpn_gateway = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisExplanationsEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsEl {
        Ec2NetworkInsightsAnalysisExplanationsEl {
            acl: core::default::Default::default(),
            acl_rule: core::default::Default::default(),
            address: core::default::Default::default(),
            addresses: core::default::Default::default(),
            attached_to: core::default::Default::default(),
            availability_zones: core::default::Default::default(),
            cidrs: core::default::Default::default(),
            classic_load_balancer_listener: core::default::Default::default(),
            component: core::default::Default::default(),
            customer_gateway: core::default::Default::default(),
            destination: core::default::Default::default(),
            destination_vpc: core::default::Default::default(),
            direction: core::default::Default::default(),
            elastic_load_balancer_listener: core::default::Default::default(),
            explanation_code: core::default::Default::default(),
            ingress_route_table: core::default::Default::default(),
            internet_gateway: core::default::Default::default(),
            load_balancer_arn: core::default::Default::default(),
            load_balancer_listener_port: core::default::Default::default(),
            load_balancer_target_group: core::default::Default::default(),
            load_balancer_target_groups: core::default::Default::default(),
            load_balancer_target_port: core::default::Default::default(),
            missing_component: core::default::Default::default(),
            nat_gateway: core::default::Default::default(),
            network_interface: core::default::Default::default(),
            packet_field: core::default::Default::default(),
            port: core::default::Default::default(),
            port_ranges: core::default::Default::default(),
            prefix_list: core::default::Default::default(),
            protocols: core::default::Default::default(),
            route_table: core::default::Default::default(),
            route_table_route: core::default::Default::default(),
            security_group: core::default::Default::default(),
            security_group_rule: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            source_vpc: core::default::Default::default(),
            state: core::default::Default::default(),
            subnet: core::default::Default::default(),
            subnet_route_table: core::default::Default::default(),
            transit_gateway: core::default::Default::default(),
            transit_gateway_attachment: core::default::Default::default(),
            transit_gateway_route_table: core::default::Default::default(),
            transit_gateway_route_table_route: core::default::Default::default(),
            vpc: core::default::Default::default(),
            vpc_endpoint: core::default::Default::default(),
            vpc_peering_connection: core::default::Default::default(),
            vpn_connection: core::default::Default::default(),
            vpn_gateway: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisExplanationsElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisExplanationsElRef {
        Ec2NetworkInsightsAnalysisExplanationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElAclElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acl", self.base))
    }

    #[doc = "Get a reference to the value of field `acl_rule` after provisioning.\n"]
    pub fn acl_rule(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElAclRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acl_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_to` after provisioning.\n"]
    pub fn attached_to(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElAttachedToElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attached_to", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }

    #[doc = "Get a reference to the value of field `classic_load_balancer_listener` after provisioning.\n"]
    pub fn classic_load_balancer_listener(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.classic_load_balancer_listener", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElComponentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_gateway` after provisioning.\n"]
    pub fn customer_gateway(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.customer_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_vpc` after provisioning.\n"]
    pub fn destination_vpc(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `elastic_load_balancer_listener` after provisioning.\n"]
    pub fn elastic_load_balancer_listener(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.elastic_load_balancer_listener", self.base))
    }

    #[doc = "Get a reference to the value of field `explanation_code` after provisioning.\n"]
    pub fn explanation_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.explanation_code", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_route_table` after provisioning.\n"]
    pub fn ingress_route_table(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ingress_route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `internet_gateway` after provisioning.\n"]
    pub fn internet_gateway(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.internet_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_listener_port` after provisioning.\n"]
    pub fn load_balancer_listener_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_listener_port", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_target_group` after provisioning.\n"]
    pub fn load_balancer_target_group(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef> {
        ListRef::new(self.shared().clone(), format!("{}.load_balancer_target_group", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_target_groups` after provisioning.\n"]
    pub fn load_balancer_target_groups(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.load_balancer_target_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_target_port` after provisioning.\n"]
    pub fn load_balancer_target_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_target_port", self.base))
    }

    #[doc = "Get a reference to the value of field `missing_component` after provisioning.\n"]
    pub fn missing_component(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.missing_component", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_gateway` after provisioning.\n"]
    pub fn nat_gateway(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElNatGatewayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nat_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_interface", self.base))
    }

    #[doc = "Get a reference to the value of field `packet_field` after provisioning.\n"]
    pub fn packet_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.packet_field", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `port_ranges` after provisioning.\n"]
    pub fn port_ranges(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElPortRangesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list` after provisioning.\n"]
    pub fn prefix_list(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElPrefixListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.prefix_list", self.base))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table` after provisioning.\n"]
    pub fn route_table(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElRouteTableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_route` after provisioning.\n"]
    pub fn route_table_route(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group` after provisioning.\n"]
    pub fn security_group(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_group", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_rule` after provisioning.\n"]
    pub fn security_group_rule(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_group_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `source_vpc` after provisioning.\n"]
    pub fn source_vpc(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElSourceVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElSubnetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_route_table` after provisioning.\n"]
    pub fn subnet_route_table(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subnet_route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway` after provisioning.\n"]
    pub fn transit_gateway(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transit_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment` after provisioning.\n"]
    pub fn transit_gateway_attachment(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transit_gateway_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table` after provisioning.\n"]
    pub fn transit_gateway_route_table(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_route` after provisioning.\n"]
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint` after provisioning.\n"]
    pub fn vpc_endpoint(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection` after provisioning.\n"]
    pub fn vpc_peering_connection(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_peering_connection", self.base))
    }

    #[doc = "Get a reference to the value of field `vpn_connection` after provisioning.\n"]
    pub fn vpn_connection(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpn_connection", self.base))
    }

    #[doc = "Get a reference to the value of field `vpn_gateway` after provisioning.\n"]
    pub fn vpn_gateway(&self) -> ListRef<Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpn_gateway", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_number: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.egress = Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl>>,
    ) -> Self {
        self.port_range = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_action`.\n"]
    pub fn set_rule_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule_action = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_number`.\n"]
    pub fn set_rule_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rule_number = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
            cidr: core::default::Default::default(),
            egress: core::default::Default::default(),
            port_range: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rule_action: core::default::Default::default(),
            rule_number: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_action", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_detail_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    component: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
    #[doc = "Set the field `additional_detail_type`.\n"]
    pub fn set_additional_detail_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_detail_type = Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl>>,
    ) -> Self {
        self.component = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
            additional_detail_type: core::default::Default::default(),
            component: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_detail_type` after provisioning.\n"]
    pub fn additional_detail_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.additional_detail_type", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
    type O =
        BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_port_ranges: Option<
        ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_port_ranges: Option<
        ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl>,
    >,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
    #[doc = "Set the field `destination_addresses`.\n"]
    pub fn set_destination_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.destination_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_ranges`.\n"]
    pub fn set_destination_port_ranges(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl,
                        >,
                    >,
    ) -> Self {
        self.destination_port_ranges = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `source_addresses`.\n"]
    pub fn set_source_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.source_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `source_port_ranges`.\n"]
    pub fn set_source_port_ranges(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl>>,
    ) -> Self {
        self.source_port_ranges = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_addresses` after provisioning.\n"]
    pub fn destination_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.destination_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_ranges` after provisioning.\n"]
    pub fn destination_port_ranges(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_port_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `source_addresses` after provisioning.\n"]
    pub fn source_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.source_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_ranges` after provisioning.\n"]
    pub fn source_port_ranges(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    type O =
        BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_port_ranges: Option<
        ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_port_ranges: Option<
        ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl>,
    >,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
    #[doc = "Set the field `destination_addresses`.\n"]
    pub fn set_destination_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.destination_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_ranges`.\n"]
    pub fn set_destination_port_ranges(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl,
                        >,
                    >,
    ) -> Self {
        self.destination_port_ranges = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `source_addresses`.\n"]
    pub fn set_source_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.source_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `source_port_ranges`.\n"]
    pub fn set_source_port_ranges(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl>>,
    ) -> Self {
        self.source_port_ranges = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_addresses` after provisioning.\n"]
    pub fn destination_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.destination_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_ranges` after provisioning.\n"]
    pub fn destination_port_ranges(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_port_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `source_addresses` after provisioning.\n"]
    pub fn source_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.source_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_ranges` after provisioning.\n"]
    pub fn source_port_ranges(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_prefix_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_only_internet_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_peering_connection_id: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
    #[doc = "Set the field `destination_cidr`.\n"]
    pub fn set_destination_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_prefix_list_id`.\n"]
    pub fn set_destination_prefix_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_prefix_list_id = Some(v.into());
        self
    }

    #[doc = "Set the field `egress_only_internet_gateway_id`.\n"]
    pub fn set_egress_only_internet_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.egress_only_internet_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_id`.\n"]
    pub fn set_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_id`.\n"]
    pub fn set_instance_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_id = Some(v.into());
        self
    }

    #[doc = "Set the field `nat_gateway_id`.\n"]
    pub fn set_nat_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nat_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }

    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transit_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering_connection_id`.\n"]
    pub fn set_vpc_peering_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_peering_connection_id = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
            destination_cidr: core::default::Default::default(),
            destination_prefix_list_id: core::default::Default::default(),
            egress_only_internet_gateway_id: core::default::Default::default(),
            gateway_id: core::default::Default::default(),
            instance_id: core::default::Default::default(),
            nat_gateway_id: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            origin: core::default::Default::default(),
            transit_gateway_id: core::default::Default::default(),
            vpc_peering_connection_id: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_cidr` after provisioning.\n"]
    pub fn destination_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_prefix_list_id` after provisioning.\n"]
    pub fn destination_prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_only_internet_gateway_id` after provisioning.\n"]
    pub fn egress_only_internet_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress_only_internet_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_gateway_id` after provisioning.\n"]
    pub fn nat_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nat_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_connection_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_id: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl>>,
    ) -> Self {
        self.port_range = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list_id`.\n"]
    pub fn set_prefix_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_list_id = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_id`.\n"]
    pub fn set_security_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_group_id = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
            cidr: core::default::Default::default(),
            direction: core::default::Default::default(),
            port_range: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            protocol: core::default::Default::default(),
            security_group_id: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list_id` after provisioning.\n"]
    pub fn prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_origin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
    #[doc = "Set the field `attachment_id`.\n"]
    pub fn set_attachment_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attachment_id = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_cidr`.\n"]
    pub fn set_destination_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list_id`.\n"]
    pub fn set_prefix_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_list_id = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_id = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }

    #[doc = "Set the field `route_origin`.\n"]
    pub fn set_route_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.route_origin = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
            attachment_id: core::default::Default::default(),
            destination_cidr: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            route_origin: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_cidr` after provisioning.\n"]
    pub fn destination_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list_id` after provisioning.\n"]
    pub fn prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `route_origin` after provisioning.\n"]
    pub fn route_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_origin", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acl_rule: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_details: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attached_to: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    component: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_vpc: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inbound_header: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outbound_header: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_table_route: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_rule: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sequence_number: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_vpc: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_route_table_route: Option<
        ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc: Option<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsEl {
    #[doc = "Set the field `acl_rule`.\n"]
    pub fn set_acl_rule(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl>>,
    ) -> Self {
        self.acl_rule = Some(v.into());
        self
    }

    #[doc = "Set the field `additional_details`.\n"]
    pub fn set_additional_details(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl>>,
    ) -> Self {
        self.additional_details = Some(v.into());
        self
    }

    #[doc = "Set the field `attached_to`.\n"]
    pub fn set_attached_to(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl>>,
    ) -> Self {
        self.attached_to = Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl>>,
    ) -> Self {
        self.component = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_vpc`.\n"]
    pub fn set_destination_vpc(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl>>,
    ) -> Self {
        self.destination_vpc = Some(v.into());
        self
    }

    #[doc = "Set the field `inbound_header`.\n"]
    pub fn set_inbound_header(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl>>,
    ) -> Self {
        self.inbound_header = Some(v.into());
        self
    }

    #[doc = "Set the field `outbound_header`.\n"]
    pub fn set_outbound_header(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl>>,
    ) -> Self {
        self.outbound_header = Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_route`.\n"]
    pub fn set_route_table_route(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl>>,
    ) -> Self {
        self.route_table_route = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_rule`.\n"]
    pub fn set_security_group_rule(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl>>,
    ) -> Self {
        self.security_group_rule = Some(v.into());
        self
    }

    #[doc = "Set the field `sequence_number`.\n"]
    pub fn set_sequence_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sequence_number = Some(v.into());
        self
    }

    #[doc = "Set the field `source_vpc`.\n"]
    pub fn set_source_vpc(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl>>,
    ) -> Self {
        self.source_vpc = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl>>,
    ) -> Self {
        self.subnet = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway`.\n"]
    pub fn set_transit_gateway(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl>>,
    ) -> Self {
        self.transit_gateway = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_route_table_route`.\n"]
    pub fn set_transit_gateway_route_table_route(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl>>,
    ) -> Self {
        self.transit_gateway_route_table_route = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(mut self, v: impl Into<ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl>>) -> Self {
        self.vpc = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsEl {
            acl_rule: core::default::Default::default(),
            additional_details: core::default::Default::default(),
            attached_to: core::default::Default::default(),
            component: core::default::Default::default(),
            destination_vpc: core::default::Default::default(),
            inbound_header: core::default::Default::default(),
            outbound_header: core::default::Default::default(),
            route_table_route: core::default::Default::default(),
            security_group_rule: core::default::Default::default(),
            sequence_number: core::default::Default::default(),
            source_vpc: core::default::Default::default(),
            subnet: core::default::Default::default(),
            transit_gateway: core::default::Default::default(),
            transit_gateway_route_table_route: core::default::Default::default(),
            vpc: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl_rule` after provisioning.\n"]
    pub fn acl_rule(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acl_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_details` after provisioning.\n"]
    pub fn additional_details(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.additional_details", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_to` after provisioning.\n"]
    pub fn attached_to(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attached_to", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_vpc` after provisioning.\n"]
    pub fn destination_vpc(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_header` after provisioning.\n"]
    pub fn inbound_header(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.inbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `outbound_header` after provisioning.\n"]
    pub fn outbound_header(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.outbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_route` after provisioning.\n"]
    pub fn route_table_route(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_rule` after provisioning.\n"]
    pub fn security_group_rule(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_group_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `sequence_number` after provisioning.\n"]
    pub fn sequence_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sequence_number", self.base))
    }

    #[doc = "Get a reference to the value of field `source_vpc` after provisioning.\n"]
    pub fn source_vpc(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway` after provisioning.\n"]
    pub fn transit_gateway(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transit_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_route` after provisioning.\n"]
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_number: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.egress = Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl>>,
    ) -> Self {
        self.port_range = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_action`.\n"]
    pub fn set_rule_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule_action = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_number`.\n"]
    pub fn set_rule_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rule_number = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
            cidr: core::default::Default::default(),
            egress: core::default::Default::default(),
            port_range: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rule_action: core::default::Default::default(),
            rule_number: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_action", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_detail_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    component: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
    #[doc = "Set the field `additional_detail_type`.\n"]
    pub fn set_additional_detail_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_detail_type = Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl>>,
    ) -> Self {
        self.component = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
            additional_detail_type: core::default::Default::default(),
            component: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_detail_type` after provisioning.\n"]
    pub fn additional_detail_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.additional_detail_type", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_port_ranges: Option<
        ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_port_ranges: Option<
        ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl>,
    >,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
    #[doc = "Set the field `destination_addresses`.\n"]
    pub fn set_destination_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.destination_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_ranges`.\n"]
    pub fn set_destination_port_ranges(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl,
                        >,
                    >,
    ) -> Self {
        self.destination_port_ranges = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `source_addresses`.\n"]
    pub fn set_source_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.source_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `source_port_ranges`.\n"]
    pub fn set_source_port_ranges(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl>>,
    ) -> Self {
        self.source_port_ranges = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_addresses` after provisioning.\n"]
    pub fn destination_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.destination_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_ranges` after provisioning.\n"]
    pub fn destination_port_ranges(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_port_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `source_addresses` after provisioning.\n"]
    pub fn source_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.source_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_ranges` after provisioning.\n"]
    pub fn source_port_ranges(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    type O =
        BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_port_ranges: Option<
        ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_port_ranges: Option<
        ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl>,
    >,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
    #[doc = "Set the field `destination_addresses`.\n"]
    pub fn set_destination_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.destination_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_ranges`.\n"]
    pub fn set_destination_port_ranges(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl,
                        >,
                    >,
    ) -> Self {
        self.destination_port_ranges = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `source_addresses`.\n"]
    pub fn set_source_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.source_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `source_port_ranges`.\n"]
    pub fn set_source_port_ranges(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl>>,
    ) -> Self {
        self.source_port_ranges = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_addresses` after provisioning.\n"]
    pub fn destination_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.destination_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_ranges` after provisioning.\n"]
    pub fn destination_port_ranges(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_port_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `source_addresses` after provisioning.\n"]
    pub fn source_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.source_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_ranges` after provisioning.\n"]
    pub fn source_port_ranges(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_prefix_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_only_internet_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_peering_connection_id: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
    #[doc = "Set the field `destination_cidr`.\n"]
    pub fn set_destination_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_prefix_list_id`.\n"]
    pub fn set_destination_prefix_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_prefix_list_id = Some(v.into());
        self
    }

    #[doc = "Set the field `egress_only_internet_gateway_id`.\n"]
    pub fn set_egress_only_internet_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.egress_only_internet_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_id`.\n"]
    pub fn set_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_id`.\n"]
    pub fn set_instance_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_id = Some(v.into());
        self
    }

    #[doc = "Set the field `nat_gateway_id`.\n"]
    pub fn set_nat_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nat_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }

    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transit_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering_connection_id`.\n"]
    pub fn set_vpc_peering_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_peering_connection_id = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
            destination_cidr: core::default::Default::default(),
            destination_prefix_list_id: core::default::Default::default(),
            egress_only_internet_gateway_id: core::default::Default::default(),
            gateway_id: core::default::Default::default(),
            instance_id: core::default::Default::default(),
            nat_gateway_id: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            origin: core::default::Default::default(),
            transit_gateway_id: core::default::Default::default(),
            vpc_peering_connection_id: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_cidr` after provisioning.\n"]
    pub fn destination_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_prefix_list_id` after provisioning.\n"]
    pub fn destination_prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_only_internet_gateway_id` after provisioning.\n"]
    pub fn egress_only_internet_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress_only_internet_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_gateway_id` after provisioning.\n"]
    pub fn nat_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nat_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_connection_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_id: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl>>,
    ) -> Self {
        self.port_range = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list_id`.\n"]
    pub fn set_prefix_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_list_id = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_id`.\n"]
    pub fn set_security_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_group_id = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
            cidr: core::default::Default::default(),
            direction: core::default::Default::default(),
            port_range: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            protocol: core::default::Default::default(),
            security_group_id: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list_id` after provisioning.\n"]
    pub fn prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_origin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
    #[doc = "Set the field `attachment_id`.\n"]
    pub fn set_attachment_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attachment_id = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_cidr`.\n"]
    pub fn set_destination_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list_id`.\n"]
    pub fn set_prefix_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_list_id = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_id = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }

    #[doc = "Set the field `route_origin`.\n"]
    pub fn set_route_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.route_origin = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
            attachment_id: core::default::Default::default(),
            destination_cidr: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            route_origin: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_cidr` after provisioning.\n"]
    pub fn destination_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list_id` after provisioning.\n"]
    pub fn prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `route_origin` after provisioning.\n"]
    pub fn route_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_origin", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acl_rule: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_details: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attached_to: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    component: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_vpc: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inbound_header: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outbound_header: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_table_route: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_rule: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sequence_number: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_vpc: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_route_table_route: Option<
        ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc: Option<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsEl {
    #[doc = "Set the field `acl_rule`.\n"]
    pub fn set_acl_rule(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl>>,
    ) -> Self {
        self.acl_rule = Some(v.into());
        self
    }

    #[doc = "Set the field `additional_details`.\n"]
    pub fn set_additional_details(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl>>,
    ) -> Self {
        self.additional_details = Some(v.into());
        self
    }

    #[doc = "Set the field `attached_to`.\n"]
    pub fn set_attached_to(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl>>,
    ) -> Self {
        self.attached_to = Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl>>,
    ) -> Self {
        self.component = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_vpc`.\n"]
    pub fn set_destination_vpc(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl>>,
    ) -> Self {
        self.destination_vpc = Some(v.into());
        self
    }

    #[doc = "Set the field `inbound_header`.\n"]
    pub fn set_inbound_header(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl>>,
    ) -> Self {
        self.inbound_header = Some(v.into());
        self
    }

    #[doc = "Set the field `outbound_header`.\n"]
    pub fn set_outbound_header(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl>>,
    ) -> Self {
        self.outbound_header = Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_route`.\n"]
    pub fn set_route_table_route(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl>>,
    ) -> Self {
        self.route_table_route = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_rule`.\n"]
    pub fn set_security_group_rule(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl>>,
    ) -> Self {
        self.security_group_rule = Some(v.into());
        self
    }

    #[doc = "Set the field `sequence_number`.\n"]
    pub fn set_sequence_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sequence_number = Some(v.into());
        self
    }

    #[doc = "Set the field `source_vpc`.\n"]
    pub fn set_source_vpc(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl>>,
    ) -> Self {
        self.source_vpc = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl>>,
    ) -> Self {
        self.subnet = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway`.\n"]
    pub fn set_transit_gateway(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl>>,
    ) -> Self {
        self.transit_gateway = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_route_table_route`.\n"]
    pub fn set_transit_gateway_route_table_route(
        mut self,
        v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl>>,
    ) -> Self {
        self.transit_gateway_route_table_route = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(mut self, v: impl Into<ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl>>) -> Self {
        self.vpc = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsEl {
    type O = BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsEl {
            acl_rule: core::default::Default::default(),
            additional_details: core::default::Default::default(),
            attached_to: core::default::Default::default(),
            component: core::default::Default::default(),
            destination_vpc: core::default::Default::default(),
            inbound_header: core::default::Default::default(),
            outbound_header: core::default::Default::default(),
            route_table_route: core::default::Default::default(),
            security_group_rule: core::default::Default::default(),
            sequence_number: core::default::Default::default(),
            source_vpc: core::default::Default::default(),
            subnet: core::default::Default::default(),
            transit_gateway: core::default::Default::default(),
            transit_gateway_route_table_route: core::default::Default::default(),
            vpc: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl_rule` after provisioning.\n"]
    pub fn acl_rule(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acl_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_details` after provisioning.\n"]
    pub fn additional_details(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.additional_details", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_to` after provisioning.\n"]
    pub fn attached_to(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attached_to", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_vpc` after provisioning.\n"]
    pub fn destination_vpc(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_header` after provisioning.\n"]
    pub fn inbound_header(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.inbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `outbound_header` after provisioning.\n"]
    pub fn outbound_header(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.outbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_route` after provisioning.\n"]
    pub fn route_table_route(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_rule` after provisioning.\n"]
    pub fn security_group_rule(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_group_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `sequence_number` after provisioning.\n"]
    pub fn sequence_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sequence_number", self.base))
    }

    #[doc = "Get a reference to the value of field `source_vpc` after provisioning.\n"]
    pub fn source_vpc(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway` after provisioning.\n"]
    pub fn transit_gateway(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transit_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_route` after provisioning.\n"]
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}
