use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamDenyPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<IamDenyPolicyRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamDenyPolicyTimeoutsEl>,
    dynamic: IamDenyPolicyDynamic,
}
struct IamDenyPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamDenyPolicyData>,
}
#[derive(Clone)]
pub struct IamDenyPolicy(Rc<IamDenyPolicy_>);
impl IamDenyPolicy {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name of the rule."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<BlockAssignable<IamDenyPolicyRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IamDenyPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the rule."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe hash of the resource. Used internally during updates."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe attachment point is identified by its URL-encoded full resource name."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<IamDenyPolicyRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamDenyPolicyTimeoutsElRef {
        IamDenyPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IamDenyPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamDenyPolicy {}
impl ToListMappable for IamDenyPolicy {
    type O = ListRef<IamDenyPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamDenyPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_deny_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamDenyPolicy {
    pub tf_id: String,
    #[doc = "The name of the policy."]
    pub name: PrimField<String>,
    #[doc = "The attachment point is identified by its URL-encoded full resource name."]
    pub parent: PrimField<String>,
}
impl BuildIamDenyPolicy {
    pub fn build(self, stack: &mut Stack) -> IamDenyPolicy {
        let out = IamDenyPolicy(Rc::new(IamDenyPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamDenyPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
                rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IamDenyPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamDenyPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamDenyPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the rule."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe hash of the resource. Used internally during updates."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe attachment point is identified by its URL-encoded full resource name."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<IamDenyPolicyRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamDenyPolicyTimeoutsElRef {
        IamDenyPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamDenyPolicyRulesElDenyRuleElDenialConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl IamDenyPolicyRulesElDenyRuleElDenialConditionEl {
    #[doc = "Set the field `description`.\nDescription of the expression. This is a longer text which describes the expression,\ne.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nString indicating the location of the expression for error reporting,\ne.g. a file name and a position in the file."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose.\nThis can be used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for IamDenyPolicyRulesElDenyRuleElDenialConditionEl {
    type O = BlockAssignable<IamDenyPolicyRulesElDenyRuleElDenialConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamDenyPolicyRulesElDenyRuleElDenialConditionEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildIamDenyPolicyRulesElDenyRuleElDenialConditionEl {
    pub fn build(self) -> IamDenyPolicyRulesElDenyRuleElDenialConditionEl {
        IamDenyPolicyRulesElDenyRuleElDenialConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct IamDenyPolicyRulesElDenyRuleElDenialConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamDenyPolicyRulesElDenyRuleElDenialConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamDenyPolicyRulesElDenyRuleElDenialConditionElRef {
        IamDenyPolicyRulesElDenyRuleElDenialConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamDenyPolicyRulesElDenyRuleElDenialConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the expression,\ne.g. when hovered over it in a UI."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting,\ne.g. a file name and a position in the file."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose.\nThis can be used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize, Default)]
struct IamDenyPolicyRulesElDenyRuleElDynamic {
    denial_condition: Option<DynamicBlock<IamDenyPolicyRulesElDenyRuleElDenialConditionEl>>,
}
#[derive(Serialize)]
pub struct IamDenyPolicyRulesElDenyRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    denied_permissions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denied_principals: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exception_permissions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exception_principals: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denial_condition: Option<Vec<IamDenyPolicyRulesElDenyRuleElDenialConditionEl>>,
    dynamic: IamDenyPolicyRulesElDenyRuleElDynamic,
}
impl IamDenyPolicyRulesElDenyRuleEl {
    #[doc = "Set the field `denied_permissions`.\nThe permissions that are explicitly denied by this rule. Each permission uses the format '{service-fqdn}/{resource}.{verb}',\nwhere '{service-fqdn}' is the fully qualified domain name for the service. For example, 'iam.googleapis.com/roles.list'."]
    pub fn set_denied_permissions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.denied_permissions = Some(v.into());
        self
    }
    #[doc = "Set the field `denied_principals`.\nThe identities that are prevented from using one or more permissions on Google Cloud resources."]
    pub fn set_denied_principals(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.denied_principals = Some(v.into());
        self
    }
    #[doc = "Set the field `exception_permissions`.\nSpecifies the permissions that this rule excludes from the set of denied permissions given by deniedPermissions.\nIf a permission appears in deniedPermissions and in exceptionPermissions then it will not be denied.\nThe excluded permissions can be specified using the same syntax as deniedPermissions."]
    pub fn set_exception_permissions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exception_permissions = Some(v.into());
        self
    }
    #[doc = "Set the field `exception_principals`.\nThe identities that are excluded from the deny rule, even if they are listed in the deniedPrincipals.\nFor example, you could add a Google group to the deniedPrincipals, then exclude specific users who belong to that group."]
    pub fn set_exception_principals(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exception_principals = Some(v.into());
        self
    }
    #[doc = "Set the field `denial_condition`.\n"]
    pub fn set_denial_condition(
        mut self,
        v: impl Into<BlockAssignable<IamDenyPolicyRulesElDenyRuleElDenialConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.denial_condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.denial_condition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IamDenyPolicyRulesElDenyRuleEl {
    type O = BlockAssignable<IamDenyPolicyRulesElDenyRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamDenyPolicyRulesElDenyRuleEl {}
impl BuildIamDenyPolicyRulesElDenyRuleEl {
    pub fn build(self) -> IamDenyPolicyRulesElDenyRuleEl {
        IamDenyPolicyRulesElDenyRuleEl {
            denied_permissions: core::default::Default::default(),
            denied_principals: core::default::Default::default(),
            exception_permissions: core::default::Default::default(),
            exception_principals: core::default::Default::default(),
            denial_condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IamDenyPolicyRulesElDenyRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamDenyPolicyRulesElDenyRuleElRef {
    fn new(shared: StackShared, base: String) -> IamDenyPolicyRulesElDenyRuleElRef {
        IamDenyPolicyRulesElDenyRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamDenyPolicyRulesElDenyRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `denied_permissions` after provisioning.\nThe permissions that are explicitly denied by this rule. Each permission uses the format '{service-fqdn}/{resource}.{verb}',\nwhere '{service-fqdn}' is the fully qualified domain name for the service. For example, 'iam.googleapis.com/roles.list'."]
    pub fn denied_permissions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.denied_permissions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `denied_principals` after provisioning.\nThe identities that are prevented from using one or more permissions on Google Cloud resources."]
    pub fn denied_principals(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.denied_principals", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exception_permissions` after provisioning.\nSpecifies the permissions that this rule excludes from the set of denied permissions given by deniedPermissions.\nIf a permission appears in deniedPermissions and in exceptionPermissions then it will not be denied.\nThe excluded permissions can be specified using the same syntax as deniedPermissions."]
    pub fn exception_permissions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exception_permissions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exception_principals` after provisioning.\nThe identities that are excluded from the deny rule, even if they are listed in the deniedPrincipals.\nFor example, you could add a Google group to the deniedPrincipals, then exclude specific users who belong to that group."]
    pub fn exception_principals(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exception_principals", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `denial_condition` after provisioning.\n"]
    pub fn denial_condition(&self) -> ListRef<IamDenyPolicyRulesElDenyRuleElDenialConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.denial_condition", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IamDenyPolicyRulesElDynamic {
    deny_rule: Option<DynamicBlock<IamDenyPolicyRulesElDenyRuleEl>>,
}
#[derive(Serialize)]
pub struct IamDenyPolicyRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny_rule: Option<Vec<IamDenyPolicyRulesElDenyRuleEl>>,
    dynamic: IamDenyPolicyRulesElDynamic,
}
impl IamDenyPolicyRulesEl {
    #[doc = "Set the field `description`.\nThe description of the rule."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `deny_rule`.\n"]
    pub fn set_deny_rule(
        mut self,
        v: impl Into<BlockAssignable<IamDenyPolicyRulesElDenyRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deny_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deny_rule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IamDenyPolicyRulesEl {
    type O = BlockAssignable<IamDenyPolicyRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamDenyPolicyRulesEl {}
impl BuildIamDenyPolicyRulesEl {
    pub fn build(self) -> IamDenyPolicyRulesEl {
        IamDenyPolicyRulesEl {
            description: core::default::Default::default(),
            deny_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IamDenyPolicyRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamDenyPolicyRulesElRef {
    fn new(shared: StackShared, base: String) -> IamDenyPolicyRulesElRef {
        IamDenyPolicyRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamDenyPolicyRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `deny_rule` after provisioning.\n"]
    pub fn deny_rule(&self) -> ListRef<IamDenyPolicyRulesElDenyRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deny_rule", self.base))
    }
}
#[derive(Serialize)]
pub struct IamDenyPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IamDenyPolicyTimeoutsEl {
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
impl ToListMappable for IamDenyPolicyTimeoutsEl {
    type O = BlockAssignable<IamDenyPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamDenyPolicyTimeoutsEl {}
impl BuildIamDenyPolicyTimeoutsEl {
    pub fn build(self) -> IamDenyPolicyTimeoutsEl {
        IamDenyPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IamDenyPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamDenyPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamDenyPolicyTimeoutsElRef {
        IamDenyPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamDenyPolicyTimeoutsElRef {
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
struct IamDenyPolicyDynamic {
    rules: Option<DynamicBlock<IamDenyPolicyRulesEl>>,
}
