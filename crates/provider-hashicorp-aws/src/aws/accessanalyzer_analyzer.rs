use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AccessanalyzerAnalyzerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    analyzer_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<AccessanalyzerAnalyzerConfigurationEl>>,
    dynamic: AccessanalyzerAnalyzerDynamic,
}

struct AccessanalyzerAnalyzer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessanalyzerAnalyzerData>,
}

#[derive(Clone)]
pub struct AccessanalyzerAnalyzer(Rc<AccessanalyzerAnalyzer_>);

impl AccessanalyzerAnalyzer {
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(self, v: impl Into<BlockAssignable<AccessanalyzerAnalyzerConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `analyzer_name` after provisioning.\n"]
    pub fn analyzer_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.analyzer_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<AccessanalyzerAnalyzerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

impl Referable for AccessanalyzerAnalyzer {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AccessanalyzerAnalyzer { }

impl ToListMappable for AccessanalyzerAnalyzer {
    type O = ListRef<AccessanalyzerAnalyzerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AccessanalyzerAnalyzer_ {
    fn extract_resource_type(&self) -> String {
        "aws_accessanalyzer_analyzer".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessanalyzerAnalyzer {
    pub tf_id: String,
    #[doc = ""]
    pub analyzer_name: PrimField<String>,
}

impl BuildAccessanalyzerAnalyzer {
    pub fn build(self, stack: &mut Stack) -> AccessanalyzerAnalyzer {
        let out = AccessanalyzerAnalyzer(Rc::new(AccessanalyzerAnalyzer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AccessanalyzerAnalyzerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                analyzer_name: self.analyzer_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: core::default::Default::default(),
                configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessanalyzerAnalyzerRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessanalyzerAnalyzerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AccessanalyzerAnalyzerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analyzer_name` after provisioning.\n"]
    pub fn analyzer_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.analyzer_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<AccessanalyzerAnalyzerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_arns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_types: Option<ListField<PrimField<String>>>,
}

impl AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionEl {
    #[doc = "Set the field `account_ids`.\n"]
    pub fn set_account_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.account_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arns`.\n"]
    pub fn set_resource_arns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resource_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_types`.\n"]
    pub fn set_resource_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resource_types = Some(v.into());
        self
    }
}

impl ToListMappable for AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionEl {
    type O = BlockAssignable<AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionEl {}

impl BuildAccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionEl {
    pub fn build(self) -> AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionEl {
        AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionEl {
            account_ids: core::default::Default::default(),
            resource_arns: core::default::Default::default(),
            resource_types: core::default::Default::default(),
        }
    }
}

pub struct AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionElRef {
        AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_ids` after provisioning.\n"]
    pub fn account_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.account_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arns` after provisioning.\n"]
    pub fn resource_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resource_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElDynamic {
    inclusion: Option<DynamicBlock<AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionEl>>,
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusion: Option<Vec<AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionEl>>,
    dynamic: AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElDynamic,
}

impl AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleEl {
    #[doc = "Set the field `inclusion`.\n"]
    pub fn set_inclusion(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inclusion = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inclusion = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleEl {
    type O = BlockAssignable<AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleEl {}

impl BuildAccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleEl {
    pub fn build(self) -> AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleEl {
        AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleEl {
            inclusion: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElRef {
        AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inclusion` after provisioning.\n"]
    pub fn inclusion(
        &self,
    ) -> ListRef<AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElInclusionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.inclusion", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessanalyzerAnalyzerConfigurationElInternalAccessElDynamic {
    analysis_rule: Option<DynamicBlock<AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleEl>>,
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzerConfigurationElInternalAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    analysis_rule: Option<Vec<AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleEl>>,
    dynamic: AccessanalyzerAnalyzerConfigurationElInternalAccessElDynamic,
}

impl AccessanalyzerAnalyzerConfigurationElInternalAccessEl {
    #[doc = "Set the field `analysis_rule`.\n"]
    pub fn set_analysis_rule(
        mut self,
        v: impl Into<BlockAssignable<AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.analysis_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis_rule = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessanalyzerAnalyzerConfigurationElInternalAccessEl {
    type O = BlockAssignable<AccessanalyzerAnalyzerConfigurationElInternalAccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzerConfigurationElInternalAccessEl {}

impl BuildAccessanalyzerAnalyzerConfigurationElInternalAccessEl {
    pub fn build(self) -> AccessanalyzerAnalyzerConfigurationElInternalAccessEl {
        AccessanalyzerAnalyzerConfigurationElInternalAccessEl {
            analysis_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AccessanalyzerAnalyzerConfigurationElInternalAccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessanalyzerAnalyzerConfigurationElInternalAccessElRef {
    fn new(shared: StackShared, base: String) -> AccessanalyzerAnalyzerConfigurationElInternalAccessElRef {
        AccessanalyzerAnalyzerConfigurationElInternalAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzerConfigurationElInternalAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analysis_rule` after provisioning.\n"]
    pub fn analysis_rule(&self) -> ListRef<AccessanalyzerAnalyzerConfigurationElInternalAccessElAnalysisRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.analysis_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tags: Option<ListField<RecField<PrimField<String>>>>,
}

impl AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionEl {
    #[doc = "Set the field `account_ids`.\n"]
    pub fn set_account_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.account_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(mut self, v: impl Into<ListField<RecField<PrimField<String>>>>) -> Self {
        self.resource_tags = Some(v.into());
        self
    }
}

impl ToListMappable for AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionEl {
    type O = BlockAssignable<AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionEl {}

impl BuildAccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionEl {
    pub fn build(self) -> AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionEl {
        AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionEl {
            account_ids: core::default::Default::default(),
            resource_tags: core::default::Default::default(),
        }
    }
}

pub struct AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionElRef {
        AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_ids` after provisioning.\n"]
    pub fn account_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.account_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> ListRef<RecRef<PrimExpr<String>>> {
        ListRef::new(self.shared().clone(), format!("{}.resource_tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElDynamic {
    exclusion: Option<DynamicBlock<AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionEl>>,
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion: Option<Vec<AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionEl>>,
    dynamic: AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElDynamic,
}

impl AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleEl {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v: impl Into<BlockAssignable<AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclusion = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleEl {
    type O = BlockAssignable<AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleEl {}

impl BuildAccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleEl {
    pub fn build(self) -> AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleEl {
        AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleEl {
            exclusion: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElRef {
        AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(&self) -> ListRef<AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElExclusionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessanalyzerAnalyzerConfigurationElUnusedAccessElDynamic {
    analysis_rule: Option<DynamicBlock<AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleEl>>,
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzerConfigurationElUnusedAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unused_access_age: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analysis_rule: Option<Vec<AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleEl>>,
    dynamic: AccessanalyzerAnalyzerConfigurationElUnusedAccessElDynamic,
}

impl AccessanalyzerAnalyzerConfigurationElUnusedAccessEl {
    #[doc = "Set the field `unused_access_age`.\n"]
    pub fn set_unused_access_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unused_access_age = Some(v.into());
        self
    }

    #[doc = "Set the field `analysis_rule`.\n"]
    pub fn set_analysis_rule(
        mut self,
        v: impl Into<BlockAssignable<AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.analysis_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis_rule = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessanalyzerAnalyzerConfigurationElUnusedAccessEl {
    type O = BlockAssignable<AccessanalyzerAnalyzerConfigurationElUnusedAccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzerConfigurationElUnusedAccessEl {}

impl BuildAccessanalyzerAnalyzerConfigurationElUnusedAccessEl {
    pub fn build(self) -> AccessanalyzerAnalyzerConfigurationElUnusedAccessEl {
        AccessanalyzerAnalyzerConfigurationElUnusedAccessEl {
            unused_access_age: core::default::Default::default(),
            analysis_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AccessanalyzerAnalyzerConfigurationElUnusedAccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessanalyzerAnalyzerConfigurationElUnusedAccessElRef {
    fn new(shared: StackShared, base: String) -> AccessanalyzerAnalyzerConfigurationElUnusedAccessElRef {
        AccessanalyzerAnalyzerConfigurationElUnusedAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzerConfigurationElUnusedAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unused_access_age` after provisioning.\n"]
    pub fn unused_access_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unused_access_age", self.base))
    }

    #[doc = "Get a reference to the value of field `analysis_rule` after provisioning.\n"]
    pub fn analysis_rule(&self) -> ListRef<AccessanalyzerAnalyzerConfigurationElUnusedAccessElAnalysisRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.analysis_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessanalyzerAnalyzerConfigurationElDynamic {
    internal_access: Option<DynamicBlock<AccessanalyzerAnalyzerConfigurationElInternalAccessEl>>,
    unused_access: Option<DynamicBlock<AccessanalyzerAnalyzerConfigurationElUnusedAccessEl>>,
}

#[derive(Serialize)]
pub struct AccessanalyzerAnalyzerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_access: Option<Vec<AccessanalyzerAnalyzerConfigurationElInternalAccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unused_access: Option<Vec<AccessanalyzerAnalyzerConfigurationElUnusedAccessEl>>,
    dynamic: AccessanalyzerAnalyzerConfigurationElDynamic,
}

impl AccessanalyzerAnalyzerConfigurationEl {
    #[doc = "Set the field `internal_access`.\n"]
    pub fn set_internal_access(
        mut self,
        v: impl Into<BlockAssignable<AccessanalyzerAnalyzerConfigurationElInternalAccessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.internal_access = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.internal_access = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `unused_access`.\n"]
    pub fn set_unused_access(
        mut self,
        v: impl Into<BlockAssignable<AccessanalyzerAnalyzerConfigurationElUnusedAccessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.unused_access = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.unused_access = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessanalyzerAnalyzerConfigurationEl {
    type O = BlockAssignable<AccessanalyzerAnalyzerConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessanalyzerAnalyzerConfigurationEl {}

impl BuildAccessanalyzerAnalyzerConfigurationEl {
    pub fn build(self) -> AccessanalyzerAnalyzerConfigurationEl {
        AccessanalyzerAnalyzerConfigurationEl {
            internal_access: core::default::Default::default(),
            unused_access: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AccessanalyzerAnalyzerConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessanalyzerAnalyzerConfigurationElRef {
    fn new(shared: StackShared, base: String) -> AccessanalyzerAnalyzerConfigurationElRef {
        AccessanalyzerAnalyzerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessanalyzerAnalyzerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `internal_access` after provisioning.\n"]
    pub fn internal_access(&self) -> ListRef<AccessanalyzerAnalyzerConfigurationElInternalAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.internal_access", self.base))
    }

    #[doc = "Get a reference to the value of field `unused_access` after provisioning.\n"]
    pub fn unused_access(&self) -> ListRef<AccessanalyzerAnalyzerConfigurationElUnusedAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.unused_access", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessanalyzerAnalyzerDynamic {
    configuration: Option<DynamicBlock<AccessanalyzerAnalyzerConfigurationEl>>,
}
