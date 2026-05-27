use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamOrganizationsPolicyBindingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    organization: PrimField<String>,
    policy: PrimField<String>,
    policy_binding_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_kind: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<IamOrganizationsPolicyBindingConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<IamOrganizationsPolicyBindingTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamOrganizationsPolicyBindingTimeoutsEl>,
    dynamic: IamOrganizationsPolicyBindingDynamic,
}
struct IamOrganizationsPolicyBinding_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamOrganizationsPolicyBindingData>,
}
#[derive(Clone)]
pub struct IamOrganizationsPolicyBinding(Rc<IamOrganizationsPolicyBinding_>);
impl IamOrganizationsPolicyBinding {
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
    #[doc = "Set the field `annotations`.\nOptional. User defined annotations. See https://google.aip.dev/148#annotations for more details such as format and size limitations\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nOptional. The description of the policy binding. Must be less than or equal to 63 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_kind`.\nImmutable. The kind of the policy to attach in this binding. This\nfield must be one of the following:  - Left empty (will be automatically set\nto the policy kind) - The input policy kind   Possible values:  POLICY_KIND_UNSPECIFIED PRINCIPAL_ACCESS_BOUNDARY ACCESS"]
    pub fn set_policy_kind(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_kind = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl Into<BlockAssignable<IamOrganizationsPolicyBindingConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        self,
        v: impl Into<BlockAssignable<IamOrganizationsPolicyBindingTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IamOrganizationsPolicyBindingTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. User defined annotations. See https://google.aip.dev/148#annotations for more details such as format and size limitations\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the policy binding was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. The description of the policy binding. Must be less than or equal to 63 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. The etag for the policy binding. If this is provided on update, it must match the server's etag."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the Policy Binding"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy binding in the format '{binding_parent/locations/{location}/policyBindings/{policy_binding_id}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe parent organization of the Policy Binding."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\nRequired. Immutable. The resource name of the policy to be bound. The binding parent and policy must belong to the same Organization (or Project)."]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_binding_id` after provisioning.\nThe Policy Binding ID."]
    pub fn policy_binding_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_binding_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_kind` after provisioning.\nImmutable. The kind of the policy to attach in this binding. This\nfield must be one of the following:  - Left empty (will be automatically set\nto the policy kind) - The input policy kind   Possible values:  POLICY_KIND_UNSPECIFIED PRINCIPAL_ACCESS_BOUNDARY ACCESS"]
    pub fn policy_kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_uid` after provisioning.\nOutput only. The globally unique ID of the policy to be bound."]
    pub fn policy_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_uid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. The globally unique ID of the policy binding. Assigned when the policy binding is created."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the policy binding was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<IamOrganizationsPolicyBindingConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<IamOrganizationsPolicyBindingTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamOrganizationsPolicyBindingTimeoutsElRef {
        IamOrganizationsPolicyBindingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IamOrganizationsPolicyBinding {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamOrganizationsPolicyBinding {}
impl ToListMappable for IamOrganizationsPolicyBinding {
    type O = ListRef<IamOrganizationsPolicyBindingRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamOrganizationsPolicyBinding_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_organizations_policy_binding".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamOrganizationsPolicyBinding {
    pub tf_id: String,
    #[doc = "The location of the Policy Binding"]
    pub location: PrimField<String>,
    #[doc = "The parent organization of the Policy Binding."]
    pub organization: PrimField<String>,
    #[doc = "Required. Immutable. The resource name of the policy to be bound. The binding parent and policy must belong to the same Organization (or Project)."]
    pub policy: PrimField<String>,
    #[doc = "The Policy Binding ID."]
    pub policy_binding_id: PrimField<String>,
}
impl BuildIamOrganizationsPolicyBinding {
    pub fn build(self, stack: &mut Stack) -> IamOrganizationsPolicyBinding {
        let out = IamOrganizationsPolicyBinding(Rc::new(IamOrganizationsPolicyBinding_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamOrganizationsPolicyBindingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                organization: self.organization,
                policy: self.policy,
                policy_binding_id: self.policy_binding_id,
                policy_kind: core::default::Default::default(),
                condition: core::default::Default::default(),
                target: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IamOrganizationsPolicyBindingRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamOrganizationsPolicyBindingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamOrganizationsPolicyBindingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. User defined annotations. See https://google.aip.dev/148#annotations for more details such as format and size limitations\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the policy binding was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. The description of the policy binding. Must be less than or equal to 63 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. The etag for the policy binding. If this is provided on update, it must match the server's etag."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the Policy Binding"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy binding in the format '{binding_parent/locations/{location}/policyBindings/{policy_binding_id}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe parent organization of the Policy Binding."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\nRequired. Immutable. The resource name of the policy to be bound. The binding parent and policy must belong to the same Organization (or Project)."]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_binding_id` after provisioning.\nThe Policy Binding ID."]
    pub fn policy_binding_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_binding_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_kind` after provisioning.\nImmutable. The kind of the policy to attach in this binding. This\nfield must be one of the following:  - Left empty (will be automatically set\nto the policy kind) - The input policy kind   Possible values:  POLICY_KIND_UNSPECIFIED PRINCIPAL_ACCESS_BOUNDARY ACCESS"]
    pub fn policy_kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_uid` after provisioning.\nOutput only. The globally unique ID of the policy to be bound."]
    pub fn policy_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_uid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. The globally unique ID of the policy binding. Assigned when the policy binding is created."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the policy binding was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<IamOrganizationsPolicyBindingConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<IamOrganizationsPolicyBindingTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamOrganizationsPolicyBindingTimeoutsElRef {
        IamOrganizationsPolicyBindingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamOrganizationsPolicyBindingConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl IamOrganizationsPolicyBindingConditionEl {
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
impl ToListMappable for IamOrganizationsPolicyBindingConditionEl {
    type O = BlockAssignable<IamOrganizationsPolicyBindingConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamOrganizationsPolicyBindingConditionEl {}
impl BuildIamOrganizationsPolicyBindingConditionEl {
    pub fn build(self) -> IamOrganizationsPolicyBindingConditionEl {
        IamOrganizationsPolicyBindingConditionEl {
            description: core::default::Default::default(),
            expression: core::default::Default::default(),
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct IamOrganizationsPolicyBindingConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamOrganizationsPolicyBindingConditionElRef {
    fn new(shared: StackShared, base: String) -> IamOrganizationsPolicyBindingConditionElRef {
        IamOrganizationsPolicyBindingConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamOrganizationsPolicyBindingConditionElRef {
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
pub struct IamOrganizationsPolicyBindingTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_set: Option<PrimField<String>>,
}
impl IamOrganizationsPolicyBindingTargetEl {
    #[doc = "Set the field `principal_set`.\nRequired. Immutable. Full Resource Name of the principal set used for principal access boundary policy bindings.\nExamples for each one of the following supported principal set types:\n* Organization '//cloudresourcemanager.googleapis.com/organizations/ORGANIZATION_ID'\n* Workforce Identity: '//iam.googleapis.com/locations/global/workforcePools/WORKFORCE_POOL_ID'\n* Workspace Identity: '//iam.googleapis.com/locations/global/workspace/WORKSPACE_ID'\nIt must be parent by the policy binding's parent (the organization)."]
    pub fn set_principal_set(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal_set = Some(v.into());
        self
    }
}
impl ToListMappable for IamOrganizationsPolicyBindingTargetEl {
    type O = BlockAssignable<IamOrganizationsPolicyBindingTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamOrganizationsPolicyBindingTargetEl {}
impl BuildIamOrganizationsPolicyBindingTargetEl {
    pub fn build(self) -> IamOrganizationsPolicyBindingTargetEl {
        IamOrganizationsPolicyBindingTargetEl {
            principal_set: core::default::Default::default(),
        }
    }
}
pub struct IamOrganizationsPolicyBindingTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamOrganizationsPolicyBindingTargetElRef {
    fn new(shared: StackShared, base: String) -> IamOrganizationsPolicyBindingTargetElRef {
        IamOrganizationsPolicyBindingTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamOrganizationsPolicyBindingTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `principal_set` after provisioning.\nRequired. Immutable. Full Resource Name of the principal set used for principal access boundary policy bindings.\nExamples for each one of the following supported principal set types:\n* Organization '//cloudresourcemanager.googleapis.com/organizations/ORGANIZATION_ID'\n* Workforce Identity: '//iam.googleapis.com/locations/global/workforcePools/WORKFORCE_POOL_ID'\n* Workspace Identity: '//iam.googleapis.com/locations/global/workspace/WORKSPACE_ID'\nIt must be parent by the policy binding's parent (the organization)."]
    pub fn principal_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_set", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IamOrganizationsPolicyBindingTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IamOrganizationsPolicyBindingTimeoutsEl {
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
impl ToListMappable for IamOrganizationsPolicyBindingTimeoutsEl {
    type O = BlockAssignable<IamOrganizationsPolicyBindingTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamOrganizationsPolicyBindingTimeoutsEl {}
impl BuildIamOrganizationsPolicyBindingTimeoutsEl {
    pub fn build(self) -> IamOrganizationsPolicyBindingTimeoutsEl {
        IamOrganizationsPolicyBindingTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IamOrganizationsPolicyBindingTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamOrganizationsPolicyBindingTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamOrganizationsPolicyBindingTimeoutsElRef {
        IamOrganizationsPolicyBindingTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamOrganizationsPolicyBindingTimeoutsElRef {
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
struct IamOrganizationsPolicyBindingDynamic {
    condition: Option<DynamicBlock<IamOrganizationsPolicyBindingConditionEl>>,
    target: Option<DynamicBlock<IamOrganizationsPolicyBindingTargetEl>>,
}
