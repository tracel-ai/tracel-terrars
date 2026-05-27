use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamProjectsPolicyBindingData {
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
    policy: PrimField<String>,
    policy_binding_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_kind: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<IamProjectsPolicyBindingConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<IamProjectsPolicyBindingTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamProjectsPolicyBindingTimeoutsEl>,
    dynamic: IamProjectsPolicyBindingDynamic,
}
struct IamProjectsPolicyBinding_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamProjectsPolicyBindingData>,
}
#[derive(Clone)]
pub struct IamProjectsPolicyBinding(Rc<IamProjectsPolicyBinding_>);
impl IamProjectsPolicyBinding {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl Into<BlockAssignable<IamProjectsPolicyBindingConditionEl>>,
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
        v: impl Into<BlockAssignable<IamProjectsPolicyBindingTargetEl>>,
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
    pub fn set_timeouts(self, v: impl Into<IamProjectsPolicyBindingTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
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
    pub fn condition(&self) -> ListRef<IamProjectsPolicyBindingConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<IamProjectsPolicyBindingTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamProjectsPolicyBindingTimeoutsElRef {
        IamProjectsPolicyBindingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IamProjectsPolicyBinding {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamProjectsPolicyBinding {}
impl ToListMappable for IamProjectsPolicyBinding {
    type O = ListRef<IamProjectsPolicyBindingRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamProjectsPolicyBinding_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_projects_policy_binding".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamProjectsPolicyBinding {
    pub tf_id: String,
    #[doc = "The location of the Policy Binding"]
    pub location: PrimField<String>,
    #[doc = "Required. Immutable. The resource name of the policy to be bound. The binding parent and policy must belong to the same Organization (or Project)."]
    pub policy: PrimField<String>,
    #[doc = "The Policy Binding ID."]
    pub policy_binding_id: PrimField<String>,
}
impl BuildIamProjectsPolicyBinding {
    pub fn build(self, stack: &mut Stack) -> IamProjectsPolicyBinding {
        let out = IamProjectsPolicyBinding(Rc::new(IamProjectsPolicyBinding_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamProjectsPolicyBindingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                policy: self.policy,
                policy_binding_id: self.policy_binding_id,
                policy_kind: core::default::Default::default(),
                project: core::default::Default::default(),
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
pub struct IamProjectsPolicyBindingRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamProjectsPolicyBindingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamProjectsPolicyBindingRef {
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
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
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
    pub fn condition(&self) -> ListRef<IamProjectsPolicyBindingConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<IamProjectsPolicyBindingTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamProjectsPolicyBindingTimeoutsElRef {
        IamProjectsPolicyBindingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamProjectsPolicyBindingConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl IamProjectsPolicyBindingConditionEl {
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
impl ToListMappable for IamProjectsPolicyBindingConditionEl {
    type O = BlockAssignable<IamProjectsPolicyBindingConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamProjectsPolicyBindingConditionEl {}
impl BuildIamProjectsPolicyBindingConditionEl {
    pub fn build(self) -> IamProjectsPolicyBindingConditionEl {
        IamProjectsPolicyBindingConditionEl {
            description: core::default::Default::default(),
            expression: core::default::Default::default(),
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct IamProjectsPolicyBindingConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamProjectsPolicyBindingConditionElRef {
    fn new(shared: StackShared, base: String) -> IamProjectsPolicyBindingConditionElRef {
        IamProjectsPolicyBindingConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamProjectsPolicyBindingConditionElRef {
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
pub struct IamProjectsPolicyBindingTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_set: Option<PrimField<String>>,
}
impl IamProjectsPolicyBindingTargetEl {
    #[doc = "Set the field `principal_set`.\nRequired. Immutable. Full Resource Name of the principal set used for principal access boundary policy bindings.\nExamples for each one of the following supported principal set types:\n* Project:\n  * '//cloudresourcemanager.googleapis.com/projects/PROJECT_NUMBER'\n  * '//cloudresourcemanager.googleapis.com/projects/PROJECT_ID'\n* Workload Identity Pool: '//iam.googleapis.com/projects/PROJECT_NUMBER/locations/LOCATION/workloadIdentityPools/WORKLOAD_POOL_ID'\nIt must be parent by the policy binding's parent (the project)."]
    pub fn set_principal_set(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal_set = Some(v.into());
        self
    }
}
impl ToListMappable for IamProjectsPolicyBindingTargetEl {
    type O = BlockAssignable<IamProjectsPolicyBindingTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamProjectsPolicyBindingTargetEl {}
impl BuildIamProjectsPolicyBindingTargetEl {
    pub fn build(self) -> IamProjectsPolicyBindingTargetEl {
        IamProjectsPolicyBindingTargetEl {
            principal_set: core::default::Default::default(),
        }
    }
}
pub struct IamProjectsPolicyBindingTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamProjectsPolicyBindingTargetElRef {
    fn new(shared: StackShared, base: String) -> IamProjectsPolicyBindingTargetElRef {
        IamProjectsPolicyBindingTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamProjectsPolicyBindingTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `principal_set` after provisioning.\nRequired. Immutable. Full Resource Name of the principal set used for principal access boundary policy bindings.\nExamples for each one of the following supported principal set types:\n* Project:\n  * '//cloudresourcemanager.googleapis.com/projects/PROJECT_NUMBER'\n  * '//cloudresourcemanager.googleapis.com/projects/PROJECT_ID'\n* Workload Identity Pool: '//iam.googleapis.com/projects/PROJECT_NUMBER/locations/LOCATION/workloadIdentityPools/WORKLOAD_POOL_ID'\nIt must be parent by the policy binding's parent (the project)."]
    pub fn principal_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_set", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IamProjectsPolicyBindingTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IamProjectsPolicyBindingTimeoutsEl {
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
impl ToListMappable for IamProjectsPolicyBindingTimeoutsEl {
    type O = BlockAssignable<IamProjectsPolicyBindingTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamProjectsPolicyBindingTimeoutsEl {}
impl BuildIamProjectsPolicyBindingTimeoutsEl {
    pub fn build(self) -> IamProjectsPolicyBindingTimeoutsEl {
        IamProjectsPolicyBindingTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IamProjectsPolicyBindingTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamProjectsPolicyBindingTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamProjectsPolicyBindingTimeoutsElRef {
        IamProjectsPolicyBindingTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamProjectsPolicyBindingTimeoutsElRef {
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
struct IamProjectsPolicyBindingDynamic {
    condition: Option<DynamicBlock<IamProjectsPolicyBindingConditionEl>>,
    target: Option<DynamicBlock<IamProjectsPolicyBindingTargetEl>>,
}
