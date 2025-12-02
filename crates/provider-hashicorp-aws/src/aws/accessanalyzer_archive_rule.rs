use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AccessanalyzerArchiveRuleData {
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
    rule_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<AccessanalyzerArchiveRuleFilterEl>>,
    dynamic: AccessanalyzerArchiveRuleDynamic,
}
struct AccessanalyzerArchiveRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessanalyzerArchiveRuleData>,
}
#[derive(Clone)]
pub struct AccessanalyzerArchiveRule(Rc<AccessanalyzerArchiveRule_>);
impl AccessanalyzerArchiveRule {
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
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl Into<BlockAssignable<AccessanalyzerArchiveRuleFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `analyzer_name` after provisioning.\n"]
    pub fn analyzer_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.analyzer_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_name", self.extract_ref()),
        )
    }
}
impl Referable for AccessanalyzerArchiveRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AccessanalyzerArchiveRule {}
impl ToListMappable for AccessanalyzerArchiveRule {
    type O = ListRef<AccessanalyzerArchiveRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AccessanalyzerArchiveRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_accessanalyzer_archive_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAccessanalyzerArchiveRule {
    pub tf_id: String,
    #[doc = ""]
    pub analyzer_name: PrimField<String>,
    #[doc = ""]
    pub rule_name: PrimField<String>,
}
impl BuildAccessanalyzerArchiveRule {
    pub fn build(self, stack: &mut Stack) -> AccessanalyzerArchiveRule {
        let out = AccessanalyzerArchiveRule(Rc::new(AccessanalyzerArchiveRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AccessanalyzerArchiveRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                analyzer_name: self.analyzer_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                rule_name: self.rule_name,
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AccessanalyzerArchiveRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessanalyzerArchiveRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AccessanalyzerArchiveRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `analyzer_name` after provisioning.\n"]
    pub fn analyzer_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.analyzer_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_name", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AccessanalyzerArchiveRuleFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<ListField<PrimField<String>>>,
    criteria: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exists: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    neq: Option<ListField<PrimField<String>>>,
}
impl AccessanalyzerArchiveRuleFilterEl {
    #[doc = "Set the field `contains`.\n"]
    pub fn set_contains(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.contains = Some(v.into());
        self
    }
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.eq = Some(v.into());
        self
    }
    #[doc = "Set the field `exists`.\n"]
    pub fn set_exists(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exists = Some(v.into());
        self
    }
    #[doc = "Set the field `neq`.\n"]
    pub fn set_neq(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.neq = Some(v.into());
        self
    }
}
impl ToListMappable for AccessanalyzerArchiveRuleFilterEl {
    type O = BlockAssignable<AccessanalyzerArchiveRuleFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessanalyzerArchiveRuleFilterEl {
    #[doc = ""]
    pub criteria: PrimField<String>,
}
impl BuildAccessanalyzerArchiveRuleFilterEl {
    pub fn build(self) -> AccessanalyzerArchiveRuleFilterEl {
        AccessanalyzerArchiveRuleFilterEl {
            contains: core::default::Default::default(),
            criteria: self.criteria,
            eq: core::default::Default::default(),
            exists: core::default::Default::default(),
            neq: core::default::Default::default(),
        }
    }
}
pub struct AccessanalyzerArchiveRuleFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessanalyzerArchiveRuleFilterElRef {
    fn new(shared: StackShared, base: String) -> AccessanalyzerArchiveRuleFilterElRef {
        AccessanalyzerArchiveRuleFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessanalyzerArchiveRuleFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains` after provisioning.\n"]
    pub fn contains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.contains", self.base))
    }
    #[doc = "Get a reference to the value of field `criteria` after provisioning.\n"]
    pub fn criteria(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.criteria", self.base))
    }
    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.eq", self.base))
    }
    #[doc = "Get a reference to the value of field `exists` after provisioning.\n"]
    pub fn exists(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exists", self.base))
    }
    #[doc = "Get a reference to the value of field `neq` after provisioning.\n"]
    pub fn neq(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.neq", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessanalyzerArchiveRuleDynamic {
    filter: Option<DynamicBlock<AccessanalyzerArchiveRuleFilterEl>>,
}
