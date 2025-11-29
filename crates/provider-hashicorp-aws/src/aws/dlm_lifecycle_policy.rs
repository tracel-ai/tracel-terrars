use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DlmLifecyclePolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_policy: Option<PrimField<String>>,
    description: PrimField<String>,
    execution_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_details: Option<Vec<DlmLifecyclePolicyPolicyDetailsEl>>,
    dynamic: DlmLifecyclePolicyDynamic,
}

struct DlmLifecyclePolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DlmLifecyclePolicyData>,
}

#[derive(Clone)]
pub struct DlmLifecyclePolicy(Rc<DlmLifecyclePolicy_>);

impl DlmLifecyclePolicy {
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

    #[doc = "Set the field `default_policy`.\n"]
    pub fn set_default_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_policy = Some(v.into());
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

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
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

    #[doc = "Set the field `policy_details`.\n"]
    pub fn set_policy_details(self, v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy_details = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy_details = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_policy` after provisioning.\n"]
    pub fn default_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_details` after provisioning.\n"]
    pub fn policy_details(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policy_details", self.extract_ref()))
    }
}

impl Referable for DlmLifecyclePolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DlmLifecyclePolicy { }

impl ToListMappable for DlmLifecyclePolicy {
    type O = ListRef<DlmLifecyclePolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DlmLifecyclePolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_dlm_lifecycle_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDlmLifecyclePolicy {
    pub tf_id: String,
    #[doc = ""]
    pub description: PrimField<String>,
    #[doc = ""]
    pub execution_role_arn: PrimField<String>,
}

impl BuildDlmLifecyclePolicy {
    pub fn build(self, stack: &mut Stack) -> DlmLifecyclePolicy {
        let out = DlmLifecyclePolicy(Rc::new(DlmLifecyclePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DlmLifecyclePolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_policy: core::default::Default::default(),
                description: self.description,
                execution_role_arn: self.execution_role_arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                state: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                policy_details: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DlmLifecyclePolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DlmLifecyclePolicyRef {
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

    #[doc = "Get a reference to the value of field `default_policy` after provisioning.\n"]
    pub fn default_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_details` after provisioning.\n"]
    pub fn policy_details(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policy_details", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cmk_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<bool>>,
}

impl DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {
    #[doc = "Set the field `cmk_arn`.\n"]
    pub fn set_cmk_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cmk_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encrypted = Some(v.into());
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {
        DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {
            cmk_arn: core::default::Default::default(),
            encrypted: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationElRef {
        DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cmk_arn` after provisioning.\n"]
    pub fn cmk_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cmk_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl {
    interval: PrimField<f64>,
    interval_unit: PrimField<String>,
}

impl DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl { }

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl {
    #[doc = ""]
    pub interval: PrimField<f64>,
    #[doc = ""]
    pub interval_unit: PrimField<String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl {
        DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl {
            interval: self.interval,
            interval_unit: self.interval_unit,
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElDynamic {
    encryption_configuration: Option<
        DynamicBlock<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl>,
    >,
    retain_rule: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
    target: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<
        Vec<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_rule: Option<Vec<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retain_rule`.\n"]
    pub fn set_retain_rule(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retain_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retain_rule = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
    #[doc = ""]
    pub target: PrimField<String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
        DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
            target: self.target,
            encryption_configuration: core::default::Default::default(),
            retain_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRef {
        DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_rule` after provisioning.\n"]
    pub fn retain_rule(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retain_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElActionElDynamic {
    cross_region_copy: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElActionEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_region_copy: Option<Vec<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElActionElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElActionEl {
    #[doc = "Set the field `cross_region_copy`.\n"]
    pub fn set_cross_region_copy(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cross_region_copy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cross_region_copy = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElActionEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElActionEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElActionEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElActionEl {
        DlmLifecyclePolicyPolicyDetailsElActionEl {
            name: self.name,
            cross_region_copy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElActionElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElActionElRef {
        DlmLifecyclePolicyPolicyDetailsElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl {
    description_regex: PrimField<String>,
    event_type: PrimField<String>,
    snapshot_owner: SetField<PrimField<String>>,
}

impl DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl { }

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl {
    #[doc = ""]
    pub description_regex: PrimField<String>,
    #[doc = ""]
    pub event_type: PrimField<String>,
    #[doc = ""]
    pub snapshot_owner: SetField<PrimField<String>>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl {
        DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl {
            description_regex: self.description_regex,
            event_type: self.event_type,
            snapshot_owner: self.snapshot_owner,
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersElRef {
        DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description_regex` after provisioning.\n"]
    pub fn description_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_owner` after provisioning.\n"]
    pub fn snapshot_owner(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.snapshot_owner", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElEventSourceElDynamic {
    parameters: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElEventSourceEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElEventSourceElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElEventSourceEl {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElEventSourceEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElEventSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElEventSourceEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElEventSourceEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElEventSourceEl {
        DlmLifecyclePolicyPolicyDetailsElEventSourceEl {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElEventSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElEventSourceElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElEventSourceElRef {
        DlmLifecyclePolicyPolicyDetailsElEventSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElEventSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElExclusionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_boot_volumes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_volume_types: Option<ListField<PrimField<String>>>,
}

impl DlmLifecyclePolicyPolicyDetailsElExclusionsEl {
    #[doc = "Set the field `exclude_boot_volumes`.\n"]
    pub fn set_exclude_boot_volumes(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.exclude_boot_volumes = Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_tags`.\n"]
    pub fn set_exclude_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.exclude_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_volume_types`.\n"]
    pub fn set_exclude_volume_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_volume_types = Some(v.into());
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElExclusionsEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElExclusionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElExclusionsEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElExclusionsEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElExclusionsEl {
        DlmLifecyclePolicyPolicyDetailsElExclusionsEl {
            exclude_boot_volumes: core::default::Default::default(),
            exclude_tags: core::default::Default::default(),
            exclude_volume_types: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElExclusionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElExclusionsElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElExclusionsElRef {
        DlmLifecyclePolicyPolicyDetailsElExclusionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElExclusionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude_boot_volumes` after provisioning.\n"]
    pub fn exclude_boot_volumes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_boot_volumes", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_tags` after provisioning.\n"]
    pub fn exclude_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.exclude_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_volume_types` after provisioning.\n"]
    pub fn exclude_volume_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exclude_volume_types", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_boot_volume: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_reboot: Option<PrimField<bool>>,
}

impl DlmLifecyclePolicyPolicyDetailsElParametersEl {
    #[doc = "Set the field `exclude_boot_volume`.\n"]
    pub fn set_exclude_boot_volume(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.exclude_boot_volume = Some(v.into());
        self
    }

    #[doc = "Set the field `no_reboot`.\n"]
    pub fn set_no_reboot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_reboot = Some(v.into());
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElParametersEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElParametersEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElParametersEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElParametersEl {
        DlmLifecyclePolicyPolicyDetailsElParametersEl {
            exclude_boot_volume: core::default::Default::default(),
            no_reboot: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElParametersElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElParametersElRef {
        DlmLifecyclePolicyPolicyDetailsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude_boot_volume` after provisioning.\n"]
    pub fn exclude_boot_volume(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_boot_volume", self.base))
    }

    #[doc = "Get a reference to the value of field `no_reboot` after provisioning.\n"]
    pub fn no_reboot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_reboot", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_unit: Option<PrimField<String>>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval = Some(v.into());
        self
    }

    #[doc = "Set the field `interval_unit`.\n"]
    pub fn set_interval_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interval_unit = Some(v.into());
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierEl {
    type O =
        BlockAssignable<
            DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierEl {
    pub fn build(
        self,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierEl {
            count: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElDynamic {
    retention_archive_tier: Option<
        DynamicBlock<
            DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_archive_tier: Option<
        Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierEl>,
    >,
    dynamic: DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleEl {
    #[doc = "Set the field `retention_archive_tier`.\n"]
    pub fn set_retention_archive_tier(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retention_archive_tier = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retention_archive_tier = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleEl {
            retention_archive_tier: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retention_archive_tier` after provisioning.\n"]
    pub fn retention_archive_tier(
        &self,
    ) -> ListRef<
        DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRetentionArchiveTierElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.retention_archive_tier", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElDynamic {
    archive_retain_rule: Option<
        DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleEl>,
    >,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_retain_rule: Option<Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleEl {
    #[doc = "Set the field `archive_retain_rule`.\n"]
    pub fn set_archive_retain_rule(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.archive_retain_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.archive_retain_rule = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleEl {
            archive_retain_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `archive_retain_rule` after provisioning.\n"]
    pub fn archive_retain_rule(
        &self,
    ) -> ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElArchiveRetainRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.archive_retain_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execute_operation_on_script_failure: Option<PrimField<bool>>,
    execution_handler: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_handler_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_retry_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stages: Option<ListField<PrimField<String>>>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsEl {
    #[doc = "Set the field `execute_operation_on_script_failure`.\n"]
    pub fn set_execute_operation_on_script_failure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.execute_operation_on_script_failure = Some(v.into());
        self
    }

    #[doc = "Set the field `execution_handler_service`.\n"]
    pub fn set_execution_handler_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_handler_service = Some(v.into());
        self
    }

    #[doc = "Set the field `execution_timeout`.\n"]
    pub fn set_execution_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.execution_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_retry_count`.\n"]
    pub fn set_maximum_retry_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_retry_count = Some(v.into());
        self
    }

    #[doc = "Set the field `stages`.\n"]
    pub fn set_stages(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.stages = Some(v.into());
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsEl {
    #[doc = ""]
    pub execution_handler: PrimField<String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsEl {
            execute_operation_on_script_failure: core::default::Default::default(),
            execution_handler: self.execution_handler,
            execution_handler_service: core::default::Default::default(),
            execution_timeout: core::default::Default::default(),
            maximum_retry_count: core::default::Default::default(),
            stages: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execute_operation_on_script_failure` after provisioning.\n"]
    pub fn execute_operation_on_script_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.execute_operation_on_script_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_handler` after provisioning.\n"]
    pub fn execution_handler(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_handler", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_handler_service` after provisioning.\n"]
    pub fn execution_handler_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_handler_service", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_timeout` after provisioning.\n"]
    pub fn execution_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_retry_count` after provisioning.\n"]
    pub fn maximum_retry_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_count", self.base))
    }

    #[doc = "Get a reference to the value of field `stages` after provisioning.\n"]
    pub fn stages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.stages", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElDynamic {
    scripts: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cron_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    times: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scripts: Option<Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {
    #[doc = "Set the field `cron_expression`.\n"]
    pub fn set_cron_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cron_expression = Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval = Some(v.into());
        self
    }

    #[doc = "Set the field `interval_unit`.\n"]
    pub fn set_interval_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interval_unit = Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }

    #[doc = "Set the field `times`.\n"]
    pub fn set_times(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.times = Some(v.into());
        self
    }

    #[doc = "Set the field `scripts`.\n"]
    pub fn set_scripts(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scripts = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scripts = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {
            cron_expression: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
            location: core::default::Default::default(),
            times: core::default::Default::default(),
            scripts: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cron_expression` after provisioning.\n"]
    pub fn cron_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cron_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `times` after provisioning.\n"]
    pub fn times(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.times", self.base))
    }

    #[doc = "Get a reference to the value of field `scripts` after provisioning.\n"]
    pub fn scripts(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElScriptsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scripts", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl {
    interval: PrimField<f64>,
    interval_unit: PrimField<String>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl { }

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl {
    #[doc = ""]
    pub interval: PrimField<f64>,
    #[doc = ""]
    pub interval_unit: PrimField<String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl {
            interval: self.interval,
            interval_unit: self.interval_unit,
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl {
    interval: PrimField<f64>,
    interval_unit: PrimField<String>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl { }

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl {
    #[doc = ""]
    pub interval: PrimField<f64>,
    #[doc = ""]
    pub interval_unit: PrimField<String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl {
            interval: self.interval,
            interval_unit: self.interval_unit,
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDynamic {
    deprecate_rule: Option<
        DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl>,
    >,
    retain_rule: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cmk_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags: Option<PrimField<bool>>,
    encrypted: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deprecate_rule: Option<Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_rule: Option<Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
    #[doc = "Set the field `cmk_arn`.\n"]
    pub fn set_cmk_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cmk_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `copy_tags`.\n"]
    pub fn set_copy_tags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.copy_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }

    #[doc = "Set the field `target_region`.\n"]
    pub fn set_target_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_region = Some(v.into());
        self
    }

    #[doc = "Set the field `deprecate_rule`.\n"]
    pub fn set_deprecate_rule(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deprecate_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deprecate_rule = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retain_rule`.\n"]
    pub fn set_retain_rule(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retain_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retain_rule = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
    #[doc = ""]
    pub encrypted: PrimField<bool>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
            cmk_arn: core::default::Default::default(),
            copy_tags: core::default::Default::default(),
            encrypted: self.encrypted,
            target: core::default::Default::default(),
            target_region: core::default::Default::default(),
            deprecate_rule: core::default::Default::default(),
            retain_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cmk_arn` after provisioning.\n"]
    pub fn cmk_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cmk_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_tags` after provisioning.\n"]
    pub fn copy_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.copy_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `target_region` after provisioning.\n"]
    pub fn target_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_region", self.base))
    }

    #[doc = "Get a reference to the value of field `deprecate_rule` after provisioning.\n"]
    pub fn deprecate_rule(
        &self,
    ) -> ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deprecate_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_rule` after provisioning.\n"]
    pub fn retain_rule(
        &self,
    ) -> ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retain_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_unit: Option<PrimField<String>>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval = Some(v.into());
        self
    }

    #[doc = "Set the field `interval_unit`.\n"]
    pub fn set_interval_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interval_unit = Some(v.into());
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {
            count: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
    availability_zones: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_unit: Option<PrimField<String>>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval = Some(v.into());
        self
    }

    #[doc = "Set the field `interval_unit`.\n"]
    pub fn set_interval_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interval_unit = Some(v.into());
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
    #[doc = ""]
    pub availability_zones: SetField<PrimField<String>>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
            availability_zones: self.availability_zones,
            count: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_unit: Option<PrimField<String>>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval = Some(v.into());
        self
    }

    #[doc = "Set the field `interval_unit`.\n"]
    pub fn set_interval_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interval_unit = Some(v.into());
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {
            count: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
    target_accounts: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unshare_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unshare_interval_unit: Option<PrimField<String>>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
    #[doc = "Set the field `unshare_interval`.\n"]
    pub fn set_unshare_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unshare_interval = Some(v.into());
        self
    }

    #[doc = "Set the field `unshare_interval_unit`.\n"]
    pub fn set_unshare_interval_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unshare_interval_unit = Some(v.into());
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
    #[doc = ""]
    pub target_accounts: SetField<PrimField<String>>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
            target_accounts: self.target_accounts,
            unshare_interval: core::default::Default::default(),
            unshare_interval_unit: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_accounts` after provisioning.\n"]
    pub fn target_accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.target_accounts", self.base))
    }

    #[doc = "Get a reference to the value of field `unshare_interval` after provisioning.\n"]
    pub fn unshare_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unshare_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `unshare_interval_unit` after provisioning.\n"]
    pub fn unshare_interval_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unshare_interval_unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElScheduleElDynamic {
    archive_rule: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleEl>>,
    create_rule: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl>>,
    cross_region_copy_rule: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl>>,
    deprecate_rule: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl>>,
    fast_restore_rule: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl>>,
    retain_rule: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl>>,
    share_rule: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_to_add: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_rule: Option<Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_rule: Option<Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_region_copy_rule: Option<Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deprecate_rule: Option<Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_restore_rule: Option<Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_rule: Option<Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_rule: Option<Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElScheduleElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleEl {
    #[doc = "Set the field `copy_tags`.\n"]
    pub fn set_copy_tags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.copy_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_to_add`.\n"]
    pub fn set_tags_to_add(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags_to_add = Some(v.into());
        self
    }

    #[doc = "Set the field `variable_tags`.\n"]
    pub fn set_variable_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.variable_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `archive_rule`.\n"]
    pub fn set_archive_rule(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.archive_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.archive_rule = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_rule`.\n"]
    pub fn set_create_rule(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.create_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.create_rule = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cross_region_copy_rule`.\n"]
    pub fn set_cross_region_copy_rule(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cross_region_copy_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cross_region_copy_rule = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deprecate_rule`.\n"]
    pub fn set_deprecate_rule(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deprecate_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deprecate_rule = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fast_restore_rule`.\n"]
    pub fn set_fast_restore_rule(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fast_restore_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fast_restore_rule = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retain_rule`.\n"]
    pub fn set_retain_rule(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retain_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retain_rule = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `share_rule`.\n"]
    pub fn set_share_rule(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.share_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.share_rule = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleEl {
            copy_tags: core::default::Default::default(),
            name: self.name,
            tags_to_add: core::default::Default::default(),
            variable_tags: core::default::Default::default(),
            archive_rule: core::default::Default::default(),
            create_rule: core::default::Default::default(),
            cross_region_copy_rule: core::default::Default::default(),
            deprecate_rule: core::default::Default::default(),
            fast_restore_rule: core::default::Default::default(),
            retain_rule: core::default::Default::default(),
            share_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElScheduleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `copy_tags` after provisioning.\n"]
    pub fn copy_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.copy_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `tags_to_add` after provisioning.\n"]
    pub fn tags_to_add(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `variable_tags` after provisioning.\n"]
    pub fn variable_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.variable_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `archive_rule` after provisioning.\n"]
    pub fn archive_rule(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElArchiveRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.archive_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `create_rule` after provisioning.\n"]
    pub fn create_rule(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.create_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `deprecate_rule` after provisioning.\n"]
    pub fn deprecate_rule(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deprecate_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `fast_restore_rule` after provisioning.\n"]
    pub fn fast_restore_rule(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fast_restore_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_rule` after provisioning.\n"]
    pub fn retain_rule(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retain_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `share_rule` after provisioning.\n"]
    pub fn share_rule(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.share_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElDynamic {
    action: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElActionEl>>,
    event_source: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElEventSourceEl>>,
    exclusions: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElExclusionsEl>>,
    parameters: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElParametersEl>>,
    schedule: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_tags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extend_deletion: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_locations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<DlmLifecyclePolicyPolicyDetailsElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_source: Option<Vec<DlmLifecyclePolicyPolicyDetailsElEventSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusions: Option<Vec<DlmLifecyclePolicyPolicyDetailsElExclusionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<DlmLifecyclePolicyPolicyDetailsElParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<Vec<DlmLifecyclePolicyPolicyDetailsElScheduleEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsEl {
    #[doc = "Set the field `copy_tags`.\n"]
    pub fn set_copy_tags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.copy_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `create_interval`.\n"]
    pub fn set_create_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.create_interval = Some(v.into());
        self
    }

    #[doc = "Set the field `extend_deletion`.\n"]
    pub fn set_extend_deletion(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.extend_deletion = Some(v.into());
        self
    }

    #[doc = "Set the field `policy_language`.\n"]
    pub fn set_policy_language(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_language = Some(v.into());
        self
    }

    #[doc = "Set the field `policy_type`.\n"]
    pub fn set_policy_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_type = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_locations`.\n"]
    pub fn set_resource_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resource_locations = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_types`.\n"]
    pub fn set_resource_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resource_types = Some(v.into());
        self
    }

    #[doc = "Set the field `retain_interval`.\n"]
    pub fn set_retain_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retain_interval = Some(v.into());
        self
    }

    #[doc = "Set the field `target_tags`.\n"]
    pub fn set_target_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.target_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_source`.\n"]
    pub fn set_event_source(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElEventSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.event_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.event_source = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclusions`.\n"]
    pub fn set_exclusions(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElExclusionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclusions = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusions = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl Into<BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schedule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DlmLifecyclePolicyPolicyDetailsEl {
    type O = BlockAssignable<DlmLifecyclePolicyPolicyDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsEl {
        DlmLifecyclePolicyPolicyDetailsEl {
            copy_tags: core::default::Default::default(),
            create_interval: core::default::Default::default(),
            extend_deletion: core::default::Default::default(),
            policy_language: core::default::Default::default(),
            policy_type: core::default::Default::default(),
            resource_locations: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            resource_types: core::default::Default::default(),
            retain_interval: core::default::Default::default(),
            target_tags: core::default::Default::default(),
            action: core::default::Default::default(),
            event_source: core::default::Default::default(),
            exclusions: core::default::Default::default(),
            parameters: core::default::Default::default(),
            schedule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlmLifecyclePolicyPolicyDetailsElRef {
    fn new(shared: StackShared, base: String) -> DlmLifecyclePolicyPolicyDetailsElRef {
        DlmLifecyclePolicyPolicyDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `copy_tags` after provisioning.\n"]
    pub fn copy_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.copy_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `create_interval` after provisioning.\n"]
    pub fn create_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `extend_deletion` after provisioning.\n"]
    pub fn extend_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.extend_deletion", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_language` after provisioning.\n"]
    pub fn policy_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_language", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_locations` after provisioning.\n"]
    pub fn resource_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resource_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_interval` after provisioning.\n"]
    pub fn retain_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retain_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `target_tags` after provisioning.\n"]
    pub fn target_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.target_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `event_source` after provisioning.\n"]
    pub fn event_source(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElEventSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.event_source", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElExclusionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclusions", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyDynamic {
    policy_details: Option<DynamicBlock<DlmLifecyclePolicyPolicyDetailsEl>>,
}
