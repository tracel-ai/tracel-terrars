use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OrgPolicyPolicyData {
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
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dry_run_spec: Option<Vec<OrgPolicyPolicyDryRunSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<OrgPolicyPolicySpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OrgPolicyPolicyTimeoutsEl>,
    dynamic: OrgPolicyPolicyDynamic,
}
struct OrgPolicyPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OrgPolicyPolicyData>,
}
#[derive(Clone)]
pub struct OrgPolicyPolicy(Rc<OrgPolicyPolicy_>);
impl OrgPolicyPolicy {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `dry_run_spec`.\n"]
    pub fn set_dry_run_spec(
        self,
        v: impl Into<BlockAssignable<OrgPolicyPolicyDryRunSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dry_run_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dry_run_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl Into<BlockAssignable<OrgPolicyPolicySpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OrgPolicyPolicyTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. An opaque tag indicating the current state of the policy, used for concurrency control. This 'etag' is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. The resource name of the Policy. Must be one of the following forms, where constraint_name is the name of the constraint which this Policy configures: * 'projects/{project_number}/policies/{constraint_name}' * 'folders/{folder_id}/policies/{constraint_name}' * 'organizations/{organization_id}/policies/{constraint_name}' For example, \"projects/123/policies/compute.disableSerialPortAccess\". Note: 'projects/{project_id}/policies/{constraint_name}' is also an acceptable name for API requests, but responses will return the name using the equivalent project number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the resource."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dry_run_spec` after provisioning.\n"]
    pub fn dry_run_spec(&self) -> ListRef<OrgPolicyPolicyDryRunSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dry_run_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<OrgPolicyPolicySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OrgPolicyPolicyTimeoutsElRef {
        OrgPolicyPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for OrgPolicyPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OrgPolicyPolicy {}
impl ToListMappable for OrgPolicyPolicy {
    type O = ListRef<OrgPolicyPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OrgPolicyPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_org_policy_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOrgPolicyPolicy {
    pub tf_id: String,
    #[doc = "Immutable. The resource name of the Policy. Must be one of the following forms, where constraint_name is the name of the constraint which this Policy configures: * 'projects/{project_number}/policies/{constraint_name}' * 'folders/{folder_id}/policies/{constraint_name}' * 'organizations/{organization_id}/policies/{constraint_name}' For example, \"projects/123/policies/compute.disableSerialPortAccess\". Note: 'projects/{project_id}/policies/{constraint_name}' is also an acceptable name for API requests, but responses will return the name using the equivalent project number."]
    pub name: PrimField<String>,
    #[doc = "The parent of the resource."]
    pub parent: PrimField<String>,
}
impl BuildOrgPolicyPolicy {
    pub fn build(self, stack: &mut Stack) -> OrgPolicyPolicy {
        let out = OrgPolicyPolicy(Rc::new(OrgPolicyPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OrgPolicyPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
                dry_run_spec: core::default::Default::default(),
                spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct OrgPolicyPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrgPolicyPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OrgPolicyPolicyRef {
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. An opaque tag indicating the current state of the policy, used for concurrency control. This 'etag' is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. The resource name of the Policy. Must be one of the following forms, where constraint_name is the name of the constraint which this Policy configures: * 'projects/{project_number}/policies/{constraint_name}' * 'folders/{folder_id}/policies/{constraint_name}' * 'organizations/{organization_id}/policies/{constraint_name}' For example, \"projects/123/policies/compute.disableSerialPortAccess\". Note: 'projects/{project_id}/policies/{constraint_name}' is also an acceptable name for API requests, but responses will return the name using the equivalent project number."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the resource."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dry_run_spec` after provisioning.\n"]
    pub fn dry_run_spec(&self) -> ListRef<OrgPolicyPolicyDryRunSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dry_run_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<OrgPolicyPolicySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OrgPolicyPolicyTimeoutsElRef {
        OrgPolicyPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OrgPolicyPolicyDryRunSpecElRulesElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl OrgPolicyPolicyDryRunSpecElRulesElConditionEl {
    #[doc = "Set the field `description`.\nOptional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `expression`.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nOptional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nOptional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for OrgPolicyPolicyDryRunSpecElRulesElConditionEl {
    type O = BlockAssignable<OrgPolicyPolicyDryRunSpecElRulesElConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrgPolicyPolicyDryRunSpecElRulesElConditionEl {}
impl BuildOrgPolicyPolicyDryRunSpecElRulesElConditionEl {
    pub fn build(self) -> OrgPolicyPolicyDryRunSpecElRulesElConditionEl {
        OrgPolicyPolicyDryRunSpecElRulesElConditionEl {
            description: core::default::Default::default(),
            expression: core::default::Default::default(),
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct OrgPolicyPolicyDryRunSpecElRulesElConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrgPolicyPolicyDryRunSpecElRulesElConditionElRef {
    fn new(shared: StackShared, base: String) -> OrgPolicyPolicyDryRunSpecElRulesElConditionElRef {
        OrgPolicyPolicyDryRunSpecElRulesElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrgPolicyPolicyDryRunSpecElRulesElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nOptional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nOptional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize)]
pub struct OrgPolicyPolicyDryRunSpecElRulesElValuesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_values: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denied_values: Option<ListField<PrimField<String>>>,
}
impl OrgPolicyPolicyDryRunSpecElRulesElValuesEl {
    #[doc = "Set the field `allowed_values`.\nList of values allowed at this resource."]
    pub fn set_allowed_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_values = Some(v.into());
        self
    }
    #[doc = "Set the field `denied_values`.\nList of values denied at this resource."]
    pub fn set_denied_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.denied_values = Some(v.into());
        self
    }
}
impl ToListMappable for OrgPolicyPolicyDryRunSpecElRulesElValuesEl {
    type O = BlockAssignable<OrgPolicyPolicyDryRunSpecElRulesElValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrgPolicyPolicyDryRunSpecElRulesElValuesEl {}
impl BuildOrgPolicyPolicyDryRunSpecElRulesElValuesEl {
    pub fn build(self) -> OrgPolicyPolicyDryRunSpecElRulesElValuesEl {
        OrgPolicyPolicyDryRunSpecElRulesElValuesEl {
            allowed_values: core::default::Default::default(),
            denied_values: core::default::Default::default(),
        }
    }
}
pub struct OrgPolicyPolicyDryRunSpecElRulesElValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrgPolicyPolicyDryRunSpecElRulesElValuesElRef {
    fn new(shared: StackShared, base: String) -> OrgPolicyPolicyDryRunSpecElRulesElValuesElRef {
        OrgPolicyPolicyDryRunSpecElRulesElValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrgPolicyPolicyDryRunSpecElRulesElValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\nList of values allowed at this resource."]
    pub fn allowed_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `denied_values` after provisioning.\nList of values denied at this resource."]
    pub fn denied_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.denied_values", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct OrgPolicyPolicyDryRunSpecElRulesElDynamic {
    condition: Option<DynamicBlock<OrgPolicyPolicyDryRunSpecElRulesElConditionEl>>,
    values: Option<DynamicBlock<OrgPolicyPolicyDryRunSpecElRulesElValuesEl>>,
}
#[derive(Serialize)]
pub struct OrgPolicyPolicyDryRunSpecElRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_all: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny_all: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<OrgPolicyPolicyDryRunSpecElRulesElConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<OrgPolicyPolicyDryRunSpecElRulesElValuesEl>>,
    dynamic: OrgPolicyPolicyDryRunSpecElRulesElDynamic,
}
impl OrgPolicyPolicyDryRunSpecElRulesEl {
    #[doc = "Set the field `allow_all`.\nSetting this to '\"TRUE\"' means that all values are allowed. This field can be set only in Policies for list constraints."]
    pub fn set_allow_all(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allow_all = Some(v.into());
        self
    }
    #[doc = "Set the field `deny_all`.\nSetting this to '\"TRUE\"' means that all values are denied. This field can be set only in Policies for list constraints."]
    pub fn set_deny_all(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deny_all = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce`.\nIf '\"TRUE\"', then the 'Policy' is enforced. If '\"FALSE\"', then any configuration is acceptable. This field can be set only in Policies for boolean constraints."]
    pub fn set_enforce(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\nOptional. Required for Managed Constraints if parameters defined in constraints. Pass parameter values when policy enforcement is enabled. Ensure that parameter value types match those defined in the constraint definition. For example: { \\\"allowedLocations\\\" : [\\\"us-east1\\\", \\\"us-west1\\\"], \\\"allowAll\\\" : true }"]
    pub fn set_parameters(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<OrgPolicyPolicyDryRunSpecElRulesElConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl Into<BlockAssignable<OrgPolicyPolicyDryRunSpecElRulesElValuesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OrgPolicyPolicyDryRunSpecElRulesEl {
    type O = BlockAssignable<OrgPolicyPolicyDryRunSpecElRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrgPolicyPolicyDryRunSpecElRulesEl {}
impl BuildOrgPolicyPolicyDryRunSpecElRulesEl {
    pub fn build(self) -> OrgPolicyPolicyDryRunSpecElRulesEl {
        OrgPolicyPolicyDryRunSpecElRulesEl {
            allow_all: core::default::Default::default(),
            deny_all: core::default::Default::default(),
            enforce: core::default::Default::default(),
            parameters: core::default::Default::default(),
            condition: core::default::Default::default(),
            values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OrgPolicyPolicyDryRunSpecElRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrgPolicyPolicyDryRunSpecElRulesElRef {
    fn new(shared: StackShared, base: String) -> OrgPolicyPolicyDryRunSpecElRulesElRef {
        OrgPolicyPolicyDryRunSpecElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrgPolicyPolicyDryRunSpecElRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_all` after provisioning.\nSetting this to '\"TRUE\"' means that all values are allowed. This field can be set only in Policies for list constraints."]
    pub fn allow_all(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_all", self.base))
    }
    #[doc = "Get a reference to the value of field `deny_all` after provisioning.\nSetting this to '\"TRUE\"' means that all values are denied. This field can be set only in Policies for list constraints."]
    pub fn deny_all(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deny_all", self.base))
    }
    #[doc = "Get a reference to the value of field `enforce` after provisioning.\nIf '\"TRUE\"', then the 'Policy' is enforced. If '\"FALSE\"', then any configuration is acceptable. This field can be set only in Policies for boolean constraints."]
    pub fn enforce(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\nOptional. Required for Managed Constraints if parameters defined in constraints. Pass parameter values when policy enforcement is enabled. Ensure that parameter value types match those defined in the constraint definition. For example: { \\\"allowedLocations\\\" : [\\\"us-east1\\\", \\\"us-west1\\\"], \\\"allowAll\\\" : true }"]
    pub fn parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<OrgPolicyPolicyDryRunSpecElRulesElConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<OrgPolicyPolicyDryRunSpecElRulesElValuesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct OrgPolicyPolicyDryRunSpecElDynamic {
    rules: Option<DynamicBlock<OrgPolicyPolicyDryRunSpecElRulesEl>>,
}
#[derive(Serialize)]
pub struct OrgPolicyPolicyDryRunSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inherit_from_parent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reset: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<OrgPolicyPolicyDryRunSpecElRulesEl>>,
    dynamic: OrgPolicyPolicyDryRunSpecElDynamic,
}
impl OrgPolicyPolicyDryRunSpecEl {
    #[doc = "Set the field `inherit_from_parent`.\nDetermines the inheritance behavior for this policy. If 'inherit_from_parent' is true, policy rules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this policy becomes the new root for evaluation. This field can be set only for policies which configure list constraints."]
    pub fn set_inherit_from_parent(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.inherit_from_parent = Some(v.into());
        self
    }
    #[doc = "Set the field `reset`.\nIgnores policies set above this resource and restores the 'constraint_default' enforcement behavior of the specific constraint at this resource. This field can be set in policies for either list or boolean constraints. If set, 'rules' must be empty and 'inherit_from_parent' must be set to false."]
    pub fn set_reset(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.reset = Some(v.into());
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl Into<BlockAssignable<OrgPolicyPolicyDryRunSpecElRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OrgPolicyPolicyDryRunSpecEl {
    type O = BlockAssignable<OrgPolicyPolicyDryRunSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrgPolicyPolicyDryRunSpecEl {}
impl BuildOrgPolicyPolicyDryRunSpecEl {
    pub fn build(self) -> OrgPolicyPolicyDryRunSpecEl {
        OrgPolicyPolicyDryRunSpecEl {
            inherit_from_parent: core::default::Default::default(),
            reset: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OrgPolicyPolicyDryRunSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrgPolicyPolicyDryRunSpecElRef {
    fn new(shared: StackShared, base: String) -> OrgPolicyPolicyDryRunSpecElRef {
        OrgPolicyPolicyDryRunSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrgPolicyPolicyDryRunSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nAn opaque tag indicating the current version of the policy, used for concurrency control. This field is ignored if used in a 'CreatePolicy' request. When the policy' is returned from either a 'GetPolicy' or a 'ListPolicies' request, this 'etag' indicates the version of the current policy to use when executing a read-modify-write loop. When the policy is returned from a 'GetEffectivePolicy' request, the 'etag' will be unset."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }
    #[doc = "Get a reference to the value of field `inherit_from_parent` after provisioning.\nDetermines the inheritance behavior for this policy. If 'inherit_from_parent' is true, policy rules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this policy becomes the new root for evaluation. This field can be set only for policies which configure list constraints."]
    pub fn inherit_from_parent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inherit_from_parent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reset` after provisioning.\nIgnores policies set above this resource and restores the 'constraint_default' enforcement behavior of the specific constraint at this resource. This field can be set in policies for either list or boolean constraints. If set, 'rules' must be empty and 'inherit_from_parent' must be set to false."]
    pub fn reset(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reset", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time stamp this was previously updated. This represents the last time a call to 'CreatePolicy' or 'UpdatePolicy' was made for that policy."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<OrgPolicyPolicyDryRunSpecElRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}
#[derive(Serialize)]
pub struct OrgPolicyPolicySpecElRulesElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl OrgPolicyPolicySpecElRulesElConditionEl {
    #[doc = "Set the field `description`.\nOptional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `expression`.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nOptional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nOptional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for OrgPolicyPolicySpecElRulesElConditionEl {
    type O = BlockAssignable<OrgPolicyPolicySpecElRulesElConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrgPolicyPolicySpecElRulesElConditionEl {}
impl BuildOrgPolicyPolicySpecElRulesElConditionEl {
    pub fn build(self) -> OrgPolicyPolicySpecElRulesElConditionEl {
        OrgPolicyPolicySpecElRulesElConditionEl {
            description: core::default::Default::default(),
            expression: core::default::Default::default(),
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct OrgPolicyPolicySpecElRulesElConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrgPolicyPolicySpecElRulesElConditionElRef {
    fn new(shared: StackShared, base: String) -> OrgPolicyPolicySpecElRulesElConditionElRef {
        OrgPolicyPolicySpecElRulesElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrgPolicyPolicySpecElRulesElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nOptional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nOptional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize)]
pub struct OrgPolicyPolicySpecElRulesElValuesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_values: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denied_values: Option<ListField<PrimField<String>>>,
}
impl OrgPolicyPolicySpecElRulesElValuesEl {
    #[doc = "Set the field `allowed_values`.\nList of values allowed at this resource."]
    pub fn set_allowed_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_values = Some(v.into());
        self
    }
    #[doc = "Set the field `denied_values`.\nList of values denied at this resource."]
    pub fn set_denied_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.denied_values = Some(v.into());
        self
    }
}
impl ToListMappable for OrgPolicyPolicySpecElRulesElValuesEl {
    type O = BlockAssignable<OrgPolicyPolicySpecElRulesElValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrgPolicyPolicySpecElRulesElValuesEl {}
impl BuildOrgPolicyPolicySpecElRulesElValuesEl {
    pub fn build(self) -> OrgPolicyPolicySpecElRulesElValuesEl {
        OrgPolicyPolicySpecElRulesElValuesEl {
            allowed_values: core::default::Default::default(),
            denied_values: core::default::Default::default(),
        }
    }
}
pub struct OrgPolicyPolicySpecElRulesElValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrgPolicyPolicySpecElRulesElValuesElRef {
    fn new(shared: StackShared, base: String) -> OrgPolicyPolicySpecElRulesElValuesElRef {
        OrgPolicyPolicySpecElRulesElValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrgPolicyPolicySpecElRulesElValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\nList of values allowed at this resource."]
    pub fn allowed_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `denied_values` after provisioning.\nList of values denied at this resource."]
    pub fn denied_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.denied_values", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct OrgPolicyPolicySpecElRulesElDynamic {
    condition: Option<DynamicBlock<OrgPolicyPolicySpecElRulesElConditionEl>>,
    values: Option<DynamicBlock<OrgPolicyPolicySpecElRulesElValuesEl>>,
}
#[derive(Serialize)]
pub struct OrgPolicyPolicySpecElRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_all: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny_all: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<OrgPolicyPolicySpecElRulesElConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<OrgPolicyPolicySpecElRulesElValuesEl>>,
    dynamic: OrgPolicyPolicySpecElRulesElDynamic,
}
impl OrgPolicyPolicySpecElRulesEl {
    #[doc = "Set the field `allow_all`.\nSetting this to '\"TRUE\"' means that all values are allowed. This field can be set only in Policies for list constraints."]
    pub fn set_allow_all(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allow_all = Some(v.into());
        self
    }
    #[doc = "Set the field `deny_all`.\nSetting this to '\"TRUE\"' means that all values are denied. This field can be set only in Policies for list constraints."]
    pub fn set_deny_all(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deny_all = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce`.\nIf '\"TRUE\"', then the 'Policy' is enforced. If '\"FALSE\"', then any configuration is acceptable. This field can be set only in Policies for boolean constraints."]
    pub fn set_enforce(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\nOptional. Required for Managed Constraints if parameters defined in constraints. Pass parameter values when policy enforcement is enabled. Ensure that parameter value types match those defined in the constraint definition. For example: { \\\"allowedLocations\\\" : [\\\"us-east1\\\", \\\"us-west1\\\"], \\\"allowAll\\\" : true }"]
    pub fn set_parameters(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<OrgPolicyPolicySpecElRulesElConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl Into<BlockAssignable<OrgPolicyPolicySpecElRulesElValuesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OrgPolicyPolicySpecElRulesEl {
    type O = BlockAssignable<OrgPolicyPolicySpecElRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrgPolicyPolicySpecElRulesEl {}
impl BuildOrgPolicyPolicySpecElRulesEl {
    pub fn build(self) -> OrgPolicyPolicySpecElRulesEl {
        OrgPolicyPolicySpecElRulesEl {
            allow_all: core::default::Default::default(),
            deny_all: core::default::Default::default(),
            enforce: core::default::Default::default(),
            parameters: core::default::Default::default(),
            condition: core::default::Default::default(),
            values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OrgPolicyPolicySpecElRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrgPolicyPolicySpecElRulesElRef {
    fn new(shared: StackShared, base: String) -> OrgPolicyPolicySpecElRulesElRef {
        OrgPolicyPolicySpecElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrgPolicyPolicySpecElRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_all` after provisioning.\nSetting this to '\"TRUE\"' means that all values are allowed. This field can be set only in Policies for list constraints."]
    pub fn allow_all(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_all", self.base))
    }
    #[doc = "Get a reference to the value of field `deny_all` after provisioning.\nSetting this to '\"TRUE\"' means that all values are denied. This field can be set only in Policies for list constraints."]
    pub fn deny_all(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deny_all", self.base))
    }
    #[doc = "Get a reference to the value of field `enforce` after provisioning.\nIf '\"TRUE\"', then the 'Policy' is enforced. If '\"FALSE\"', then any configuration is acceptable. This field can be set only in Policies for boolean constraints."]
    pub fn enforce(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\nOptional. Required for Managed Constraints if parameters defined in constraints. Pass parameter values when policy enforcement is enabled. Ensure that parameter value types match those defined in the constraint definition. For example: { \\\"allowedLocations\\\" : [\\\"us-east1\\\", \\\"us-west1\\\"], \\\"allowAll\\\" : true }"]
    pub fn parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<OrgPolicyPolicySpecElRulesElConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<OrgPolicyPolicySpecElRulesElValuesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct OrgPolicyPolicySpecElDynamic {
    rules: Option<DynamicBlock<OrgPolicyPolicySpecElRulesEl>>,
}
#[derive(Serialize)]
pub struct OrgPolicyPolicySpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inherit_from_parent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reset: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<OrgPolicyPolicySpecElRulesEl>>,
    dynamic: OrgPolicyPolicySpecElDynamic,
}
impl OrgPolicyPolicySpecEl {
    #[doc = "Set the field `inherit_from_parent`.\nDetermines the inheritance behavior for this 'Policy'. If 'inherit_from_parent' is true, PolicyRules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this Policy becomes the new root for evaluation. This field can be set only for Policies which configure list constraints."]
    pub fn set_inherit_from_parent(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.inherit_from_parent = Some(v.into());
        self
    }
    #[doc = "Set the field `reset`.\nIgnores policies set above this resource and restores the 'constraint_default' enforcement behavior of the specific 'Constraint' at this resource. This field can be set in policies for either list or boolean constraints. If set, 'rules' must be empty and 'inherit_from_parent' must be set to false."]
    pub fn set_reset(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.reset = Some(v.into());
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl Into<BlockAssignable<OrgPolicyPolicySpecElRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OrgPolicyPolicySpecEl {
    type O = BlockAssignable<OrgPolicyPolicySpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrgPolicyPolicySpecEl {}
impl BuildOrgPolicyPolicySpecEl {
    pub fn build(self) -> OrgPolicyPolicySpecEl {
        OrgPolicyPolicySpecEl {
            inherit_from_parent: core::default::Default::default(),
            reset: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OrgPolicyPolicySpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrgPolicyPolicySpecElRef {
    fn new(shared: StackShared, base: String) -> OrgPolicyPolicySpecElRef {
        OrgPolicyPolicySpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrgPolicyPolicySpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nAn opaque tag indicating the current version of the 'Policy', used for concurrency control. This field is ignored if used in a 'CreatePolicy' request. When the 'Policy' is returned from either a 'GetPolicy' or a 'ListPolicies' request, this 'etag' indicates the version of the current 'Policy' to use when executing a read-modify-write loop. When the 'Policy' is returned from a 'GetEffectivePolicy' request, the 'etag' will be unset."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }
    #[doc = "Get a reference to the value of field `inherit_from_parent` after provisioning.\nDetermines the inheritance behavior for this 'Policy'. If 'inherit_from_parent' is true, PolicyRules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this Policy becomes the new root for evaluation. This field can be set only for Policies which configure list constraints."]
    pub fn inherit_from_parent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inherit_from_parent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reset` after provisioning.\nIgnores policies set above this resource and restores the 'constraint_default' enforcement behavior of the specific 'Constraint' at this resource. This field can be set in policies for either list or boolean constraints. If set, 'rules' must be empty and 'inherit_from_parent' must be set to false."]
    pub fn reset(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reset", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time stamp this was previously updated. This represents the last time a call to 'CreatePolicy' or 'UpdatePolicy' was made for that 'Policy'."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<OrgPolicyPolicySpecElRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}
#[derive(Serialize)]
pub struct OrgPolicyPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl OrgPolicyPolicyTimeoutsEl {
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
impl ToListMappable for OrgPolicyPolicyTimeoutsEl {
    type O = BlockAssignable<OrgPolicyPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOrgPolicyPolicyTimeoutsEl {}
impl BuildOrgPolicyPolicyTimeoutsEl {
    pub fn build(self) -> OrgPolicyPolicyTimeoutsEl {
        OrgPolicyPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct OrgPolicyPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OrgPolicyPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OrgPolicyPolicyTimeoutsElRef {
        OrgPolicyPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OrgPolicyPolicyTimeoutsElRef {
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
struct OrgPolicyPolicyDynamic {
    dry_run_spec: Option<DynamicBlock<OrgPolicyPolicyDryRunSpecEl>>,
    spec: Option<DynamicBlock<OrgPolicyPolicySpecEl>>,
}
