use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamAccessBoundaryPolicyData {
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
    rules: Option<Vec<IamAccessBoundaryPolicyRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamAccessBoundaryPolicyTimeoutsEl>,
    dynamic: IamAccessBoundaryPolicyDynamic,
}
struct IamAccessBoundaryPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamAccessBoundaryPolicyData>,
}
#[derive(Clone)]
pub struct IamAccessBoundaryPolicy(Rc<IamAccessBoundaryPolicy_>);
impl IamAccessBoundaryPolicy {
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
    pub fn set_rules(self, v: impl Into<BlockAssignable<IamAccessBoundaryPolicyRulesEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<IamAccessBoundaryPolicyTimeoutsEl>) -> Self {
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
    pub fn rules(&self) -> ListRef<IamAccessBoundaryPolicyRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamAccessBoundaryPolicyTimeoutsElRef {
        IamAccessBoundaryPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IamAccessBoundaryPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamAccessBoundaryPolicy {}
impl ToListMappable for IamAccessBoundaryPolicy {
    type O = ListRef<IamAccessBoundaryPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamAccessBoundaryPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_access_boundary_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamAccessBoundaryPolicy {
    pub tf_id: String,
    #[doc = "The name of the policy."]
    pub name: PrimField<String>,
    #[doc = "The attachment point is identified by its URL-encoded full resource name."]
    pub parent: PrimField<String>,
}
impl BuildIamAccessBoundaryPolicy {
    pub fn build(self, stack: &mut Stack) -> IamAccessBoundaryPolicy {
        let out = IamAccessBoundaryPolicy(Rc::new(IamAccessBoundaryPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamAccessBoundaryPolicyData {
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
pub struct IamAccessBoundaryPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamAccessBoundaryPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamAccessBoundaryPolicyRef {
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
    pub fn rules(&self) -> ListRef<IamAccessBoundaryPolicyRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamAccessBoundaryPolicyTimeoutsElRef {
        IamAccessBoundaryPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
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
impl ToListMappable for IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
    type O =
        BlockAssignable<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildIamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
    pub fn build(
        self,
    ) -> IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
        IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionElRef {
        IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionElRef {
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
struct IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElDynamic {
    availability_condition: Option<
        DynamicBlock<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl>,
    >,
}
#[derive(Serialize)]
pub struct IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    available_permissions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_resource: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_condition:
        Option<Vec<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl>>,
    dynamic: IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElDynamic,
}
impl IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {
    #[doc = "Set the field `available_permissions`.\nA list of permissions that may be allowed for use on the specified resource."]
    pub fn set_available_permissions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.available_permissions = Some(v.into());
        self
    }
    #[doc = "Set the field `available_resource`.\nThe full resource name of a Google Cloud resource entity."]
    pub fn set_available_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.available_resource = Some(v.into());
        self
    }
    #[doc = "Set the field `availability_condition`.\n"]
    pub fn set_availability_condition(
        mut self,
        v: impl Into<
            BlockAssignable<
                IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.availability_condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.availability_condition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {
    type O = BlockAssignable<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {}
impl BuildIamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {
    pub fn build(self) -> IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {
        IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {
            available_permissions: core::default::Default::default(),
            available_resource: core::default::Default::default(),
            availability_condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElRef {
        IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `available_permissions` after provisioning.\nA list of permissions that may be allowed for use on the specified resource."]
    pub fn available_permissions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_permissions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `available_resource` after provisioning.\nThe full resource name of a Google Cloud resource entity."]
    pub fn available_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_resource", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `availability_condition` after provisioning.\n"]
    pub fn availability_condition(
        &self,
    ) -> ListRef<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.availability_condition", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IamAccessBoundaryPolicyRulesElDynamic {
    access_boundary_rule: Option<DynamicBlock<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl>>,
}
#[derive(Serialize)]
pub struct IamAccessBoundaryPolicyRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_boundary_rule: Option<Vec<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl>>,
    dynamic: IamAccessBoundaryPolicyRulesElDynamic,
}
impl IamAccessBoundaryPolicyRulesEl {
    #[doc = "Set the field `description`.\nThe description of the rule."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `access_boundary_rule`.\n"]
    pub fn set_access_boundary_rule(
        mut self,
        v: impl Into<BlockAssignable<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_boundary_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_boundary_rule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IamAccessBoundaryPolicyRulesEl {
    type O = BlockAssignable<IamAccessBoundaryPolicyRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamAccessBoundaryPolicyRulesEl {}
impl BuildIamAccessBoundaryPolicyRulesEl {
    pub fn build(self) -> IamAccessBoundaryPolicyRulesEl {
        IamAccessBoundaryPolicyRulesEl {
            description: core::default::Default::default(),
            access_boundary_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IamAccessBoundaryPolicyRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamAccessBoundaryPolicyRulesElRef {
    fn new(shared: StackShared, base: String) -> IamAccessBoundaryPolicyRulesElRef {
        IamAccessBoundaryPolicyRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamAccessBoundaryPolicyRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `access_boundary_rule` after provisioning.\n"]
    pub fn access_boundary_rule(
        &self,
    ) -> ListRef<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_boundary_rule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IamAccessBoundaryPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IamAccessBoundaryPolicyTimeoutsEl {
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
impl ToListMappable for IamAccessBoundaryPolicyTimeoutsEl {
    type O = BlockAssignable<IamAccessBoundaryPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamAccessBoundaryPolicyTimeoutsEl {}
impl BuildIamAccessBoundaryPolicyTimeoutsEl {
    pub fn build(self) -> IamAccessBoundaryPolicyTimeoutsEl {
        IamAccessBoundaryPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IamAccessBoundaryPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamAccessBoundaryPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamAccessBoundaryPolicyTimeoutsElRef {
        IamAccessBoundaryPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamAccessBoundaryPolicyTimeoutsElRef {
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
struct IamAccessBoundaryPolicyDynamic {
    rules: Option<DynamicBlock<IamAccessBoundaryPolicyRulesEl>>,
}
