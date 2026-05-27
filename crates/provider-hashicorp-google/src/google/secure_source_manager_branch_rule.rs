use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SecureSourceManagerBranchRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_stale_reviews: Option<PrimField<bool>>,
    branch_rule_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    include_pattern: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_approvals_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_reviews_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    repository_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_comments_resolved: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_linear_history: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_pull_request: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SecureSourceManagerBranchRuleTimeoutsEl>,
}
struct SecureSourceManagerBranchRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecureSourceManagerBranchRuleData>,
}
#[derive(Clone)]
pub struct SecureSourceManagerBranchRule(Rc<SecureSourceManagerBranchRule_>);
impl SecureSourceManagerBranchRule {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `allow_stale_reviews`.\nDetermines if allow stale reviews or approvals before merging to the branch."]
    pub fn set_allow_stale_reviews(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_stale_reviews = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nDetermines if the branch rule is disabled or not."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum_approvals_count`.\nThe minimum number of approvals required for the branch rule to be matched."]
    pub fn set_minimum_approvals_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().minimum_approvals_count = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum_reviews_count`.\nThe minimum number of reviews required for the branch rule to be matched."]
    pub fn set_minimum_reviews_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().minimum_reviews_count = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `require_comments_resolved`.\nDetermines if require comments resolved before merging to the branch."]
    pub fn set_require_comments_resolved(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_comments_resolved = Some(v.into());
        self
    }
    #[doc = "Set the field `require_linear_history`.\nDetermines if require linear history before merging to the branch."]
    pub fn set_require_linear_history(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_linear_history = Some(v.into());
        self
    }
    #[doc = "Set the field `require_pull_request`.\nDetermines if the branch rule requires a pull request or not."]
    pub fn set_require_pull_request(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_pull_request = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SecureSourceManagerBranchRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `allow_stale_reviews` after provisioning.\nDetermines if allow stale reviews or approvals before merging to the branch."]
    pub fn allow_stale_reviews(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_stale_reviews", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `branch_rule_id` after provisioning.\nThe ID for the BranchRule."]
    pub fn branch_rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.branch_rule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the BranchRule was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nDetermines if the branch rule is disabled or not."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `include_pattern` after provisioning.\nThe BranchRule matches branches based on the specified regular expression. Use .* to match all branches."]
    pub fn include_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_pattern", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the Repository."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_approvals_count` after provisioning.\nThe minimum number of approvals required for the branch rule to be matched."]
    pub fn minimum_approvals_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_approvals_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_reviews_count` after provisioning.\nThe minimum number of reviews required for the branch rule to be matched."]
    pub fn minimum_reviews_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_reviews_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the BranchRule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\nThe ID for the Repository."]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `require_comments_resolved` after provisioning.\nDetermines if require comments resolved before merging to the branch."]
    pub fn require_comments_resolved(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_comments_resolved", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `require_linear_history` after provisioning.\nDetermines if require linear history before merging to the branch."]
    pub fn require_linear_history(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_linear_history", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `require_pull_request` after provisioning.\nDetermines if the branch rule requires a pull request or not."]
    pub fn require_pull_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_pull_request", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier of the BranchRule."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the BranchRule was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecureSourceManagerBranchRuleTimeoutsElRef {
        SecureSourceManagerBranchRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SecureSourceManagerBranchRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SecureSourceManagerBranchRule {}
impl ToListMappable for SecureSourceManagerBranchRule {
    type O = ListRef<SecureSourceManagerBranchRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SecureSourceManagerBranchRule_ {
    fn extract_resource_type(&self) -> String {
        "google_secure_source_manager_branch_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSecureSourceManagerBranchRule {
    pub tf_id: String,
    #[doc = "The ID for the BranchRule."]
    pub branch_rule_id: PrimField<String>,
    #[doc = "The BranchRule matches branches based on the specified regular expression. Use .* to match all branches."]
    pub include_pattern: PrimField<String>,
    #[doc = "The location for the Repository."]
    pub location: PrimField<String>,
    #[doc = "The ID for the Repository."]
    pub repository_id: PrimField<String>,
}
impl BuildSecureSourceManagerBranchRule {
    pub fn build(self, stack: &mut Stack) -> SecureSourceManagerBranchRule {
        let out = SecureSourceManagerBranchRule(Rc::new(SecureSourceManagerBranchRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecureSourceManagerBranchRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_stale_reviews: core::default::Default::default(),
                branch_rule_id: self.branch_rule_id,
                deletion_policy: core::default::Default::default(),
                disabled: core::default::Default::default(),
                id: core::default::Default::default(),
                include_pattern: self.include_pattern,
                location: self.location,
                minimum_approvals_count: core::default::Default::default(),
                minimum_reviews_count: core::default::Default::default(),
                project: core::default::Default::default(),
                repository_id: self.repository_id,
                require_comments_resolved: core::default::Default::default(),
                require_linear_history: core::default::Default::default(),
                require_pull_request: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SecureSourceManagerBranchRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerBranchRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SecureSourceManagerBranchRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_stale_reviews` after provisioning.\nDetermines if allow stale reviews or approvals before merging to the branch."]
    pub fn allow_stale_reviews(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_stale_reviews", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `branch_rule_id` after provisioning.\nThe ID for the BranchRule."]
    pub fn branch_rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.branch_rule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the BranchRule was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nDetermines if the branch rule is disabled or not."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `include_pattern` after provisioning.\nThe BranchRule matches branches based on the specified regular expression. Use .* to match all branches."]
    pub fn include_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_pattern", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the Repository."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_approvals_count` after provisioning.\nThe minimum number of approvals required for the branch rule to be matched."]
    pub fn minimum_approvals_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_approvals_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `minimum_reviews_count` after provisioning.\nThe minimum number of reviews required for the branch rule to be matched."]
    pub fn minimum_reviews_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_reviews_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the BranchRule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\nThe ID for the Repository."]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `require_comments_resolved` after provisioning.\nDetermines if require comments resolved before merging to the branch."]
    pub fn require_comments_resolved(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_comments_resolved", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `require_linear_history` after provisioning.\nDetermines if require linear history before merging to the branch."]
    pub fn require_linear_history(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_linear_history", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `require_pull_request` after provisioning.\nDetermines if the branch rule requires a pull request or not."]
    pub fn require_pull_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_pull_request", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier of the BranchRule."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the BranchRule was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecureSourceManagerBranchRuleTimeoutsElRef {
        SecureSourceManagerBranchRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SecureSourceManagerBranchRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SecureSourceManagerBranchRuleTimeoutsEl {
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
impl ToListMappable for SecureSourceManagerBranchRuleTimeoutsEl {
    type O = BlockAssignable<SecureSourceManagerBranchRuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecureSourceManagerBranchRuleTimeoutsEl {}
impl BuildSecureSourceManagerBranchRuleTimeoutsEl {
    pub fn build(self) -> SecureSourceManagerBranchRuleTimeoutsEl {
        SecureSourceManagerBranchRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SecureSourceManagerBranchRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerBranchRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SecureSourceManagerBranchRuleTimeoutsElRef {
        SecureSourceManagerBranchRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecureSourceManagerBranchRuleTimeoutsElRef {
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
