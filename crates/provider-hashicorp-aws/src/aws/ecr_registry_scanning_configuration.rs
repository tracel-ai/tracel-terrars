use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcrRegistryScanningConfigurationData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    scan_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<EcrRegistryScanningConfigurationRuleEl>>,
    dynamic: EcrRegistryScanningConfigurationDynamic,
}

struct EcrRegistryScanningConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EcrRegistryScanningConfigurationData>,
}

#[derive(Clone)]
pub struct EcrRegistryScanningConfiguration(Rc<EcrRegistryScanningConfiguration_>);

impl EcrRegistryScanningConfiguration {
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

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl Into<BlockAssignable<EcrRegistryScanningConfigurationRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scan_type` after provisioning.\n"]
    pub fn scan_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_type", self.extract_ref()))
    }
}

impl Referable for EcrRegistryScanningConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EcrRegistryScanningConfiguration { }

impl ToListMappable for EcrRegistryScanningConfiguration {
    type O = ListRef<EcrRegistryScanningConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EcrRegistryScanningConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_ecr_registry_scanning_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcrRegistryScanningConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub scan_type: PrimField<String>,
}

impl BuildEcrRegistryScanningConfiguration {
    pub fn build(self, stack: &mut Stack) -> EcrRegistryScanningConfiguration {
        let out = EcrRegistryScanningConfiguration(Rc::new(EcrRegistryScanningConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EcrRegistryScanningConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                scan_type: self.scan_type,
                rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcrRegistryScanningConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcrRegistryScanningConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EcrRegistryScanningConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scan_type` after provisioning.\n"]
    pub fn scan_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcrRegistryScanningConfigurationRuleElRepositoryFilterEl {
    filter: PrimField<String>,
    filter_type: PrimField<String>,
}

impl EcrRegistryScanningConfigurationRuleElRepositoryFilterEl { }

impl ToListMappable for EcrRegistryScanningConfigurationRuleElRepositoryFilterEl {
    type O = BlockAssignable<EcrRegistryScanningConfigurationRuleElRepositoryFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrRegistryScanningConfigurationRuleElRepositoryFilterEl {
    #[doc = ""]
    pub filter: PrimField<String>,
    #[doc = ""]
    pub filter_type: PrimField<String>,
}

impl BuildEcrRegistryScanningConfigurationRuleElRepositoryFilterEl {
    pub fn build(self) -> EcrRegistryScanningConfigurationRuleElRepositoryFilterEl {
        EcrRegistryScanningConfigurationRuleElRepositoryFilterEl {
            filter: self.filter,
            filter_type: self.filter_type,
        }
    }
}

pub struct EcrRegistryScanningConfigurationRuleElRepositoryFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcrRegistryScanningConfigurationRuleElRepositoryFilterElRef {
    fn new(shared: StackShared, base: String) -> EcrRegistryScanningConfigurationRuleElRepositoryFilterElRef {
        EcrRegistryScanningConfigurationRuleElRepositoryFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrRegistryScanningConfigurationRuleElRepositoryFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_type` after provisioning.\n"]
    pub fn filter_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcrRegistryScanningConfigurationRuleElDynamic {
    repository_filter: Option<DynamicBlock<EcrRegistryScanningConfigurationRuleElRepositoryFilterEl>>,
}

#[derive(Serialize)]
pub struct EcrRegistryScanningConfigurationRuleEl {
    scan_frequency: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_filter: Option<Vec<EcrRegistryScanningConfigurationRuleElRepositoryFilterEl>>,
    dynamic: EcrRegistryScanningConfigurationRuleElDynamic,
}

impl EcrRegistryScanningConfigurationRuleEl {
    #[doc = "Set the field `repository_filter`.\n"]
    pub fn set_repository_filter(
        mut self,
        v: impl Into<BlockAssignable<EcrRegistryScanningConfigurationRuleElRepositoryFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.repository_filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.repository_filter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EcrRegistryScanningConfigurationRuleEl {
    type O = BlockAssignable<EcrRegistryScanningConfigurationRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrRegistryScanningConfigurationRuleEl {
    #[doc = ""]
    pub scan_frequency: PrimField<String>,
}

impl BuildEcrRegistryScanningConfigurationRuleEl {
    pub fn build(self) -> EcrRegistryScanningConfigurationRuleEl {
        EcrRegistryScanningConfigurationRuleEl {
            scan_frequency: self.scan_frequency,
            repository_filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EcrRegistryScanningConfigurationRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcrRegistryScanningConfigurationRuleElRef {
    fn new(shared: StackShared, base: String) -> EcrRegistryScanningConfigurationRuleElRef {
        EcrRegistryScanningConfigurationRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrRegistryScanningConfigurationRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scan_frequency` after provisioning.\n"]
    pub fn scan_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_frequency", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcrRegistryScanningConfigurationDynamic {
    rule: Option<DynamicBlock<EcrRegistryScanningConfigurationRuleEl>>,
}
