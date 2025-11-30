use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct RbinRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_resource_tags: Option<Vec<RbinRuleExcludeResourceTagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lock_configuration: Option<Vec<RbinRuleLockConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tags: Option<Vec<RbinRuleResourceTagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_period: Option<Vec<RbinRuleRetentionPeriodEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RbinRuleTimeoutsEl>,
    dynamic: RbinRuleDynamic,
}

struct RbinRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RbinRuleData>,
}

#[derive(Clone)]
pub struct RbinRule(Rc<RbinRule_>);

impl RbinRule {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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

    #[doc = "Set the field `exclude_resource_tags`.\n"]
    pub fn set_exclude_resource_tags(
        self,
        v: impl Into<BlockAssignable<RbinRuleExcludeResourceTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude_resource_tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude_resource_tags = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `lock_configuration`.\n"]
    pub fn set_lock_configuration(
        self,
        v: impl Into<BlockAssignable<RbinRuleLockConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lock_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lock_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(self, v: impl Into<BlockAssignable<RbinRuleResourceTagsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_tags = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(
        self,
        v: impl Into<BlockAssignable<RbinRuleRetentionPeriodEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_period = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_period = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RbinRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_end_time` after provisioning.\n"]
    pub fn lock_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lock_end_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lock_state` after provisioning.\n"]
    pub fn lock_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lock_state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `lock_configuration` after provisioning.\n"]
    pub fn lock_configuration(&self) -> ListRef<RbinRuleLockConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lock_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> ListRef<RbinRuleRetentionPeriodElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RbinRuleTimeoutsElRef {
        RbinRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for RbinRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for RbinRule {}

impl ToListMappable for RbinRule {
    type O = ListRef<RbinRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RbinRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_rbin_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRbinRule {
    pub tf_id: String,
    #[doc = ""]
    pub resource_type: PrimField<String>,
}

impl BuildRbinRule {
    pub fn build(self, stack: &mut Stack) -> RbinRule {
        let out = RbinRule(Rc::new(RbinRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RbinRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_type: self.resource_type,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                exclude_resource_tags: core::default::Default::default(),
                lock_configuration: core::default::Default::default(),
                resource_tags: core::default::Default::default(),
                retention_period: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RbinRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for RbinRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl RbinRuleRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_end_time` after provisioning.\n"]
    pub fn lock_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lock_end_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lock_state` after provisioning.\n"]
    pub fn lock_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lock_state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `lock_configuration` after provisioning.\n"]
    pub fn lock_configuration(&self) -> ListRef<RbinRuleLockConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lock_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> ListRef<RbinRuleRetentionPeriodElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_period", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RbinRuleTimeoutsElRef {
        RbinRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct RbinRuleExcludeResourceTagsEl {
    resource_tag_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tag_value: Option<PrimField<String>>,
}

impl RbinRuleExcludeResourceTagsEl {
    #[doc = "Set the field `resource_tag_value`.\n"]
    pub fn set_resource_tag_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_tag_value = Some(v.into());
        self
    }
}

impl ToListMappable for RbinRuleExcludeResourceTagsEl {
    type O = BlockAssignable<RbinRuleExcludeResourceTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRbinRuleExcludeResourceTagsEl {
    #[doc = ""]
    pub resource_tag_key: PrimField<String>,
}

impl BuildRbinRuleExcludeResourceTagsEl {
    pub fn build(self) -> RbinRuleExcludeResourceTagsEl {
        RbinRuleExcludeResourceTagsEl {
            resource_tag_key: self.resource_tag_key,
            resource_tag_value: core::default::Default::default(),
        }
    }
}

pub struct RbinRuleExcludeResourceTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RbinRuleExcludeResourceTagsElRef {
    fn new(shared: StackShared, base: String) -> RbinRuleExcludeResourceTagsElRef {
        RbinRuleExcludeResourceTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RbinRuleExcludeResourceTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_tag_key` after provisioning.\n"]
    pub fn resource_tag_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_tag_key", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `resource_tag_value` after provisioning.\n"]
    pub fn resource_tag_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_tag_value", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct RbinRuleLockConfigurationElUnlockDelayEl {
    unlock_delay_unit: PrimField<String>,
    unlock_delay_value: PrimField<f64>,
}

impl RbinRuleLockConfigurationElUnlockDelayEl {}

impl ToListMappable for RbinRuleLockConfigurationElUnlockDelayEl {
    type O = BlockAssignable<RbinRuleLockConfigurationElUnlockDelayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRbinRuleLockConfigurationElUnlockDelayEl {
    #[doc = ""]
    pub unlock_delay_unit: PrimField<String>,
    #[doc = ""]
    pub unlock_delay_value: PrimField<f64>,
}

impl BuildRbinRuleLockConfigurationElUnlockDelayEl {
    pub fn build(self) -> RbinRuleLockConfigurationElUnlockDelayEl {
        RbinRuleLockConfigurationElUnlockDelayEl {
            unlock_delay_unit: self.unlock_delay_unit,
            unlock_delay_value: self.unlock_delay_value,
        }
    }
}

pub struct RbinRuleLockConfigurationElUnlockDelayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RbinRuleLockConfigurationElUnlockDelayElRef {
    fn new(shared: StackShared, base: String) -> RbinRuleLockConfigurationElUnlockDelayElRef {
        RbinRuleLockConfigurationElUnlockDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RbinRuleLockConfigurationElUnlockDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unlock_delay_unit` after provisioning.\n"]
    pub fn unlock_delay_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unlock_delay_unit", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `unlock_delay_value` after provisioning.\n"]
    pub fn unlock_delay_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unlock_delay_value", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct RbinRuleLockConfigurationElDynamic {
    unlock_delay: Option<DynamicBlock<RbinRuleLockConfigurationElUnlockDelayEl>>,
}

#[derive(Serialize)]
pub struct RbinRuleLockConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unlock_delay: Option<Vec<RbinRuleLockConfigurationElUnlockDelayEl>>,
    dynamic: RbinRuleLockConfigurationElDynamic,
}

impl RbinRuleLockConfigurationEl {
    #[doc = "Set the field `unlock_delay`.\n"]
    pub fn set_unlock_delay(
        mut self,
        v: impl Into<BlockAssignable<RbinRuleLockConfigurationElUnlockDelayEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.unlock_delay = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.unlock_delay = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for RbinRuleLockConfigurationEl {
    type O = BlockAssignable<RbinRuleLockConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRbinRuleLockConfigurationEl {}

impl BuildRbinRuleLockConfigurationEl {
    pub fn build(self) -> RbinRuleLockConfigurationEl {
        RbinRuleLockConfigurationEl {
            unlock_delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RbinRuleLockConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RbinRuleLockConfigurationElRef {
    fn new(shared: StackShared, base: String) -> RbinRuleLockConfigurationElRef {
        RbinRuleLockConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RbinRuleLockConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unlock_delay` after provisioning.\n"]
    pub fn unlock_delay(&self) -> ListRef<RbinRuleLockConfigurationElUnlockDelayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.unlock_delay", self.base))
    }
}

#[derive(Serialize)]
pub struct RbinRuleResourceTagsEl {
    resource_tag_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tag_value: Option<PrimField<String>>,
}

impl RbinRuleResourceTagsEl {
    #[doc = "Set the field `resource_tag_value`.\n"]
    pub fn set_resource_tag_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_tag_value = Some(v.into());
        self
    }
}

impl ToListMappable for RbinRuleResourceTagsEl {
    type O = BlockAssignable<RbinRuleResourceTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRbinRuleResourceTagsEl {
    #[doc = ""]
    pub resource_tag_key: PrimField<String>,
}

impl BuildRbinRuleResourceTagsEl {
    pub fn build(self) -> RbinRuleResourceTagsEl {
        RbinRuleResourceTagsEl {
            resource_tag_key: self.resource_tag_key,
            resource_tag_value: core::default::Default::default(),
        }
    }
}

pub struct RbinRuleResourceTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RbinRuleResourceTagsElRef {
    fn new(shared: StackShared, base: String) -> RbinRuleResourceTagsElRef {
        RbinRuleResourceTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RbinRuleResourceTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_tag_key` after provisioning.\n"]
    pub fn resource_tag_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_tag_key", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `resource_tag_value` after provisioning.\n"]
    pub fn resource_tag_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_tag_value", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct RbinRuleRetentionPeriodEl {
    retention_period_unit: PrimField<String>,
    retention_period_value: PrimField<f64>,
}

impl RbinRuleRetentionPeriodEl {}

impl ToListMappable for RbinRuleRetentionPeriodEl {
    type O = BlockAssignable<RbinRuleRetentionPeriodEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRbinRuleRetentionPeriodEl {
    #[doc = ""]
    pub retention_period_unit: PrimField<String>,
    #[doc = ""]
    pub retention_period_value: PrimField<f64>,
}

impl BuildRbinRuleRetentionPeriodEl {
    pub fn build(self) -> RbinRuleRetentionPeriodEl {
        RbinRuleRetentionPeriodEl {
            retention_period_unit: self.retention_period_unit,
            retention_period_value: self.retention_period_value,
        }
    }
}

pub struct RbinRuleRetentionPeriodElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RbinRuleRetentionPeriodElRef {
    fn new(shared: StackShared, base: String) -> RbinRuleRetentionPeriodElRef {
        RbinRuleRetentionPeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RbinRuleRetentionPeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retention_period_unit` after provisioning.\n"]
    pub fn retention_period_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_period_unit", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `retention_period_value` after provisioning.\n"]
    pub fn retention_period_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_period_value", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct RbinRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl RbinRuleTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for RbinRuleTimeoutsEl {
    type O = BlockAssignable<RbinRuleTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRbinRuleTimeoutsEl {}

impl BuildRbinRuleTimeoutsEl {
    pub fn build(self) -> RbinRuleTimeoutsEl {
        RbinRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct RbinRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RbinRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RbinRuleTimeoutsElRef {
        RbinRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RbinRuleTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct RbinRuleDynamic {
    exclude_resource_tags: Option<DynamicBlock<RbinRuleExcludeResourceTagsEl>>,
    lock_configuration: Option<DynamicBlock<RbinRuleLockConfigurationEl>>,
    resource_tags: Option<DynamicBlock<RbinRuleResourceTagsEl>>,
    retention_period: Option<DynamicBlock<RbinRuleRetentionPeriodEl>>,
}
