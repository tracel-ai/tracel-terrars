use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SsmPatchBaselineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approved_patches: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approved_patches_compliance_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approved_patches_enable_non_security: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_security_updates_compliance_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operating_system: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rejected_patches: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rejected_patches_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approval_rule: Option<Vec<SsmPatchBaselineApprovalRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_filter: Option<Vec<SsmPatchBaselineGlobalFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<SsmPatchBaselineSourceEl>>,
    dynamic: SsmPatchBaselineDynamic,
}

struct SsmPatchBaseline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsmPatchBaselineData>,
}

#[derive(Clone)]
pub struct SsmPatchBaseline(Rc<SsmPatchBaseline_>);

impl SsmPatchBaseline {
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

    #[doc = "Set the field `approved_patches`.\n"]
    pub fn set_approved_patches(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().approved_patches = Some(v.into());
        self
    }

    #[doc = "Set the field `approved_patches_compliance_level`.\n"]
    pub fn set_approved_patches_compliance_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().approved_patches_compliance_level = Some(v.into());
        self
    }

    #[doc = "Set the field `approved_patches_enable_non_security`.\n"]
    pub fn set_approved_patches_enable_non_security(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .approved_patches_enable_non_security = Some(v.into());
        self
    }

    #[doc = "Set the field `available_security_updates_compliance_status`.\n"]
    pub fn set_available_security_updates_compliance_status(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .available_security_updates_compliance_status = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system`.\n"]
    pub fn set_operating_system(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().operating_system = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `rejected_patches`.\n"]
    pub fn set_rejected_patches(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().rejected_patches = Some(v.into());
        self
    }

    #[doc = "Set the field `rejected_patches_action`.\n"]
    pub fn set_rejected_patches_action(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().rejected_patches_action = Some(v.into());
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

    #[doc = "Set the field `approval_rule`.\n"]
    pub fn set_approval_rule(
        self,
        v: impl Into<BlockAssignable<SsmPatchBaselineApprovalRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().approval_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.approval_rule = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `global_filter`.\n"]
    pub fn set_global_filter(
        self,
        v: impl Into<BlockAssignable<SsmPatchBaselineGlobalFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().global_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.global_filter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl Into<BlockAssignable<SsmPatchBaselineSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `approved_patches` after provisioning.\n"]
    pub fn approved_patches(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.approved_patches", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `approved_patches_compliance_level` after provisioning.\n"]
    pub fn approved_patches_compliance_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approved_patches_compliance_level", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `approved_patches_enable_non_security` after provisioning.\n"]
    pub fn approved_patches_enable_non_security(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.approved_patches_enable_non_security",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `available_security_updates_compliance_status` after provisioning.\n"]
    pub fn available_security_updates_compliance_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.available_security_updates_compliance_status",
                self.extract_ref()
            ),
        )
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

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operating_system", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rejected_patches` after provisioning.\n"]
    pub fn rejected_patches(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.rejected_patches", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rejected_patches_action` after provisioning.\n"]
    pub fn rejected_patches_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rejected_patches_action", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `approval_rule` after provisioning.\n"]
    pub fn approval_rule(&self) -> ListRef<SsmPatchBaselineApprovalRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approval_rule", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `global_filter` after provisioning.\n"]
    pub fn global_filter(&self) -> ListRef<SsmPatchBaselineGlobalFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.global_filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<SsmPatchBaselineSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
}

impl Referable for SsmPatchBaseline {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SsmPatchBaseline {}

impl ToListMappable for SsmPatchBaseline {
    type O = ListRef<SsmPatchBaselineRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SsmPatchBaseline_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssm_patch_baseline".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmPatchBaseline {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildSsmPatchBaseline {
    pub fn build(self, stack: &mut Stack) -> SsmPatchBaseline {
        let out = SsmPatchBaseline(Rc::new(SsmPatchBaseline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SsmPatchBaselineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                approved_patches: core::default::Default::default(),
                approved_patches_compliance_level: core::default::Default::default(),
                approved_patches_enable_non_security: core::default::Default::default(),
                available_security_updates_compliance_status: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                operating_system: core::default::Default::default(),
                region: core::default::Default::default(),
                rejected_patches: core::default::Default::default(),
                rejected_patches_action: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                approval_rule: core::default::Default::default(),
                global_filter: core::default::Default::default(),
                source: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmPatchBaselineRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmPatchBaselineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SsmPatchBaselineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approved_patches` after provisioning.\n"]
    pub fn approved_patches(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.approved_patches", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `approved_patches_compliance_level` after provisioning.\n"]
    pub fn approved_patches_compliance_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approved_patches_compliance_level", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `approved_patches_enable_non_security` after provisioning.\n"]
    pub fn approved_patches_enable_non_security(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.approved_patches_enable_non_security",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `available_security_updates_compliance_status` after provisioning.\n"]
    pub fn available_security_updates_compliance_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.available_security_updates_compliance_status",
                self.extract_ref()
            ),
        )
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

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operating_system", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rejected_patches` after provisioning.\n"]
    pub fn rejected_patches(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.rejected_patches", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rejected_patches_action` after provisioning.\n"]
    pub fn rejected_patches_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rejected_patches_action", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `approval_rule` after provisioning.\n"]
    pub fn approval_rule(&self) -> ListRef<SsmPatchBaselineApprovalRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approval_rule", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `global_filter` after provisioning.\n"]
    pub fn global_filter(&self) -> ListRef<SsmPatchBaselineGlobalFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.global_filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<SsmPatchBaselineSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SsmPatchBaselineApprovalRuleElPatchFilterEl {
    key: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl SsmPatchBaselineApprovalRuleElPatchFilterEl {}

impl ToListMappable for SsmPatchBaselineApprovalRuleElPatchFilterEl {
    type O = BlockAssignable<SsmPatchBaselineApprovalRuleElPatchFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmPatchBaselineApprovalRuleElPatchFilterEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildSsmPatchBaselineApprovalRuleElPatchFilterEl {
    pub fn build(self) -> SsmPatchBaselineApprovalRuleElPatchFilterEl {
        SsmPatchBaselineApprovalRuleElPatchFilterEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct SsmPatchBaselineApprovalRuleElPatchFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmPatchBaselineApprovalRuleElPatchFilterElRef {
    fn new(shared: StackShared, base: String) -> SsmPatchBaselineApprovalRuleElPatchFilterElRef {
        SsmPatchBaselineApprovalRuleElPatchFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmPatchBaselineApprovalRuleElPatchFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmPatchBaselineApprovalRuleElDynamic {
    patch_filter: Option<DynamicBlock<SsmPatchBaselineApprovalRuleElPatchFilterEl>>,
}

#[derive(Serialize)]
pub struct SsmPatchBaselineApprovalRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    approve_after_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approve_until_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_non_security: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patch_filter: Option<Vec<SsmPatchBaselineApprovalRuleElPatchFilterEl>>,
    dynamic: SsmPatchBaselineApprovalRuleElDynamic,
}

impl SsmPatchBaselineApprovalRuleEl {
    #[doc = "Set the field `approve_after_days`.\n"]
    pub fn set_approve_after_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.approve_after_days = Some(v.into());
        self
    }

    #[doc = "Set the field `approve_until_date`.\n"]
    pub fn set_approve_until_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.approve_until_date = Some(v.into());
        self
    }

    #[doc = "Set the field `compliance_level`.\n"]
    pub fn set_compliance_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compliance_level = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_non_security`.\n"]
    pub fn set_enable_non_security(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_non_security = Some(v.into());
        self
    }

    #[doc = "Set the field `patch_filter`.\n"]
    pub fn set_patch_filter(
        mut self,
        v: impl Into<BlockAssignable<SsmPatchBaselineApprovalRuleElPatchFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.patch_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.patch_filter = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SsmPatchBaselineApprovalRuleEl {
    type O = BlockAssignable<SsmPatchBaselineApprovalRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmPatchBaselineApprovalRuleEl {}

impl BuildSsmPatchBaselineApprovalRuleEl {
    pub fn build(self) -> SsmPatchBaselineApprovalRuleEl {
        SsmPatchBaselineApprovalRuleEl {
            approve_after_days: core::default::Default::default(),
            approve_until_date: core::default::Default::default(),
            compliance_level: core::default::Default::default(),
            enable_non_security: core::default::Default::default(),
            patch_filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsmPatchBaselineApprovalRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmPatchBaselineApprovalRuleElRef {
    fn new(shared: StackShared, base: String) -> SsmPatchBaselineApprovalRuleElRef {
        SsmPatchBaselineApprovalRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmPatchBaselineApprovalRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approve_after_days` after provisioning.\n"]
    pub fn approve_after_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approve_after_days", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `approve_until_date` after provisioning.\n"]
    pub fn approve_until_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approve_until_date", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `compliance_level` after provisioning.\n"]
    pub fn compliance_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compliance_level", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enable_non_security` after provisioning.\n"]
    pub fn enable_non_security(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_non_security", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `patch_filter` after provisioning.\n"]
    pub fn patch_filter(&self) -> ListRef<SsmPatchBaselineApprovalRuleElPatchFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.patch_filter", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmPatchBaselineGlobalFilterEl {
    key: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl SsmPatchBaselineGlobalFilterEl {}

impl ToListMappable for SsmPatchBaselineGlobalFilterEl {
    type O = BlockAssignable<SsmPatchBaselineGlobalFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmPatchBaselineGlobalFilterEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildSsmPatchBaselineGlobalFilterEl {
    pub fn build(self) -> SsmPatchBaselineGlobalFilterEl {
        SsmPatchBaselineGlobalFilterEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct SsmPatchBaselineGlobalFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmPatchBaselineGlobalFilterElRef {
    fn new(shared: StackShared, base: String) -> SsmPatchBaselineGlobalFilterElRef {
        SsmPatchBaselineGlobalFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmPatchBaselineGlobalFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmPatchBaselineSourceEl {
    configuration: PrimField<String>,
    name: PrimField<String>,
    products: ListField<PrimField<String>>,
}

impl SsmPatchBaselineSourceEl {}

impl ToListMappable for SsmPatchBaselineSourceEl {
    type O = BlockAssignable<SsmPatchBaselineSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmPatchBaselineSourceEl {
    #[doc = ""]
    pub configuration: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub products: ListField<PrimField<String>>,
}

impl BuildSsmPatchBaselineSourceEl {
    pub fn build(self) -> SsmPatchBaselineSourceEl {
        SsmPatchBaselineSourceEl {
            configuration: self.configuration,
            name: self.name,
            products: self.products,
        }
    }
}

pub struct SsmPatchBaselineSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmPatchBaselineSourceElRef {
    fn new(shared: StackShared, base: String) -> SsmPatchBaselineSourceElRef {
        SsmPatchBaselineSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmPatchBaselineSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `products` after provisioning.\n"]
    pub fn products(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.products", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmPatchBaselineDynamic {
    approval_rule: Option<DynamicBlock<SsmPatchBaselineApprovalRuleEl>>,
    global_filter: Option<DynamicBlock<SsmPatchBaselineGlobalFilterEl>>,
    source: Option<DynamicBlock<SsmPatchBaselineSourceEl>>,
}
