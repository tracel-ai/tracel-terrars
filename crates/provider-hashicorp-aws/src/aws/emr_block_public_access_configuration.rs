use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct EmrBlockPublicAccessConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    block_public_security_group_rules: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permitted_public_security_group_rule_range:
        Option<Vec<EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeEl>>,
    dynamic: EmrBlockPublicAccessConfigurationDynamic,
}

struct EmrBlockPublicAccessConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmrBlockPublicAccessConfigurationData>,
}

#[derive(Clone)]
pub struct EmrBlockPublicAccessConfiguration(Rc<EmrBlockPublicAccessConfiguration_>);

impl EmrBlockPublicAccessConfiguration {
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

    #[doc = "Set the field `permitted_public_security_group_rule_range`.\n"]
    pub fn set_permitted_public_security_group_rule_range(
        self,
        v: impl Into<
            BlockAssignable<
                EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .permitted_public_security_group_rule_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .permitted_public_security_group_rule_range = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `block_public_security_group_rules` after provisioning.\n"]
    pub fn block_public_security_group_rules(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_public_security_group_rules", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `permitted_public_security_group_rule_range` after provisioning.\n"]
    pub fn permitted_public_security_group_rule_range(
        &self,
    ) -> ListRef<EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.permitted_public_security_group_rule_range",
                self.extract_ref()
            ),
        )
    }
}

impl Referable for EmrBlockPublicAccessConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for EmrBlockPublicAccessConfiguration {}

impl ToListMappable for EmrBlockPublicAccessConfiguration {
    type O = ListRef<EmrBlockPublicAccessConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EmrBlockPublicAccessConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_emr_block_public_access_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrBlockPublicAccessConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub block_public_security_group_rules: PrimField<bool>,
}

impl BuildEmrBlockPublicAccessConfiguration {
    pub fn build(self, stack: &mut Stack) -> EmrBlockPublicAccessConfiguration {
        let out = EmrBlockPublicAccessConfiguration(Rc::new(EmrBlockPublicAccessConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmrBlockPublicAccessConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                block_public_security_group_rules: self.block_public_security_group_rules,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                permitted_public_security_group_rule_range: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrBlockPublicAccessConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrBlockPublicAccessConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl EmrBlockPublicAccessConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_public_security_group_rules` after provisioning.\n"]
    pub fn block_public_security_group_rules(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_public_security_group_rules", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `permitted_public_security_group_rule_range` after provisioning.\n"]
    pub fn permitted_public_security_group_rule_range(
        &self,
    ) -> ListRef<EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.permitted_public_security_group_rule_range",
                self.extract_ref()
            ),
        )
    }
}

#[derive(Serialize)]
pub struct EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeEl {
    max_range: PrimField<f64>,
    min_range: PrimField<f64>,
}

impl EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeEl {}

impl ToListMappable for EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeEl {
    type O =
        BlockAssignable<EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeEl {
    #[doc = ""]
    pub max_range: PrimField<f64>,
    #[doc = ""]
    pub min_range: PrimField<f64>,
}

impl BuildEmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeEl {
    pub fn build(self) -> EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeEl {
        EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeEl {
            max_range: self.max_range,
            min_range: self.min_range,
        }
    }
}

pub struct EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeElRef {
        EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_range` after provisioning.\n"]
    pub fn max_range(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_range", self.base))
    }

    #[doc = "Get a reference to the value of field `min_range` after provisioning.\n"]
    pub fn min_range(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrBlockPublicAccessConfigurationDynamic {
    permitted_public_security_group_rule_range: Option<
        DynamicBlock<EmrBlockPublicAccessConfigurationPermittedPublicSecurityGroupRuleRangeEl>,
    >,
}
