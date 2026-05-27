use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeSecurityFeedbackData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    feedback_id: PrimField<String>,
    feedback_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    org_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feedback_contexts: Option<Vec<ApigeeSecurityFeedbackFeedbackContextsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeSecurityFeedbackTimeoutsEl>,
    dynamic: ApigeeSecurityFeedbackDynamic,
}
struct ApigeeSecurityFeedback_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeSecurityFeedbackData>,
}
#[derive(Clone)]
pub struct ApigeeSecurityFeedback(Rc<ApigeeSecurityFeedback_>);
impl ApigeeSecurityFeedback {
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
    #[doc = "Set the field `comment`.\nOptional text the user can provide for additional, unstructured context."]
    pub fn set_comment(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().comment = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name of the feedback."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\nThe reason for the feedback. Possible values: [\"INTERNAL_SYSTEM\", \"NON_RISK_CLIENT\", \"NAT\", \"PENETRATION_TEST\", \"OTHER\"]"]
    pub fn set_reason(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().reason = Some(v.into());
        self
    }
    #[doc = "Set the field `feedback_contexts`.\n"]
    pub fn set_feedback_contexts(
        self,
        v: impl Into<BlockAssignable<ApigeeSecurityFeedbackFeedbackContextsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().feedback_contexts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.feedback_contexts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeSecurityFeedbackTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `comment` after provisioning.\nOptional text the user can provide for additional, unstructured context."]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when this specific feedback id was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the feedback."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feedback_id` after provisioning.\nResource ID of the security feedback."]
    pub fn feedback_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feedback_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feedback_type` after provisioning.\nThe type of feedback being submitted. Possible values: [\"EXCLUDED_DETECTION\"]"]
    pub fn feedback_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feedback_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the security feedback resource,\nin the format 'organizations/{{org_name}}/securityFeedback/{{feedback_id}}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee Security Feedback,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\nThe reason for the feedback. Possible values: [\"INTERNAL_SYSTEM\", \"NON_RISK_CLIENT\", \"NAT\", \"PENETRATION_TEST\", \"OTHER\"]"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when this specific feedback id was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feedback_contexts` after provisioning.\n"]
    pub fn feedback_contexts(&self) -> ListRef<ApigeeSecurityFeedbackFeedbackContextsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.feedback_contexts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeSecurityFeedbackTimeoutsElRef {
        ApigeeSecurityFeedbackTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeSecurityFeedback {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeSecurityFeedback {}
impl ToListMappable for ApigeeSecurityFeedback {
    type O = ListRef<ApigeeSecurityFeedbackRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeSecurityFeedback_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_security_feedback".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeSecurityFeedback {
    pub tf_id: String,
    #[doc = "Resource ID of the security feedback."]
    pub feedback_id: PrimField<String>,
    #[doc = "The type of feedback being submitted. Possible values: [\"EXCLUDED_DETECTION\"]"]
    pub feedback_type: PrimField<String>,
    #[doc = "The Apigee Organization associated with the Apigee Security Feedback,\nin the format 'organizations/{{org_name}}'."]
    pub org_id: PrimField<String>,
}
impl BuildApigeeSecurityFeedback {
    pub fn build(self, stack: &mut Stack) -> ApigeeSecurityFeedback {
        let out = ApigeeSecurityFeedback(Rc::new(ApigeeSecurityFeedback_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeSecurityFeedbackData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                comment: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                feedback_id: self.feedback_id,
                feedback_type: self.feedback_type,
                id: core::default::Default::default(),
                org_id: self.org_id,
                reason: core::default::Default::default(),
                feedback_contexts: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeSecurityFeedbackRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityFeedbackRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeSecurityFeedbackRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `comment` after provisioning.\nOptional text the user can provide for additional, unstructured context."]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when this specific feedback id was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the feedback."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feedback_id` after provisioning.\nResource ID of the security feedback."]
    pub fn feedback_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feedback_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feedback_type` after provisioning.\nThe type of feedback being submitted. Possible values: [\"EXCLUDED_DETECTION\"]"]
    pub fn feedback_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feedback_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the security feedback resource,\nin the format 'organizations/{{org_name}}/securityFeedback/{{feedback_id}}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee Security Feedback,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\nThe reason for the feedback. Possible values: [\"INTERNAL_SYSTEM\", \"NON_RISK_CLIENT\", \"NAT\", \"PENETRATION_TEST\", \"OTHER\"]"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when this specific feedback id was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feedback_contexts` after provisioning.\n"]
    pub fn feedback_contexts(&self) -> ListRef<ApigeeSecurityFeedbackFeedbackContextsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.feedback_contexts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeSecurityFeedbackTimeoutsElRef {
        ApigeeSecurityFeedbackTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeSecurityFeedbackFeedbackContextsEl {
    attribute: PrimField<String>,
    values: ListField<PrimField<String>>,
}
impl ApigeeSecurityFeedbackFeedbackContextsEl {}
impl ToListMappable for ApigeeSecurityFeedbackFeedbackContextsEl {
    type O = BlockAssignable<ApigeeSecurityFeedbackFeedbackContextsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeSecurityFeedbackFeedbackContextsEl {
    #[doc = "The attribute the user is providing feedback about. Possible values: [\"ATTRIBUTE_ENVIRONMENTS\", \"ATTRIBUTE_IP_ADDRESS_RANGES\"]"]
    pub attribute: PrimField<String>,
    #[doc = "The values of the attribute the user is providing feedback about, separated by commas."]
    pub values: ListField<PrimField<String>>,
}
impl BuildApigeeSecurityFeedbackFeedbackContextsEl {
    pub fn build(self) -> ApigeeSecurityFeedbackFeedbackContextsEl {
        ApigeeSecurityFeedbackFeedbackContextsEl {
            attribute: self.attribute,
            values: self.values,
        }
    }
}
pub struct ApigeeSecurityFeedbackFeedbackContextsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityFeedbackFeedbackContextsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeSecurityFeedbackFeedbackContextsElRef {
        ApigeeSecurityFeedbackFeedbackContextsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeSecurityFeedbackFeedbackContextsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attribute` after provisioning.\nThe attribute the user is providing feedback about. Possible values: [\"ATTRIBUTE_ENVIRONMENTS\", \"ATTRIBUTE_IP_ADDRESS_RANGES\"]"]
    pub fn attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nThe values of the attribute the user is providing feedback about, separated by commas."]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeSecurityFeedbackTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApigeeSecurityFeedbackTimeoutsEl {
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
impl ToListMappable for ApigeeSecurityFeedbackTimeoutsEl {
    type O = BlockAssignable<ApigeeSecurityFeedbackTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeSecurityFeedbackTimeoutsEl {}
impl BuildApigeeSecurityFeedbackTimeoutsEl {
    pub fn build(self) -> ApigeeSecurityFeedbackTimeoutsEl {
        ApigeeSecurityFeedbackTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApigeeSecurityFeedbackTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityFeedbackTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeSecurityFeedbackTimeoutsElRef {
        ApigeeSecurityFeedbackTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeSecurityFeedbackTimeoutsElRef {
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
struct ApigeeSecurityFeedbackDynamic {
    feedback_contexts: Option<DynamicBlock<ApigeeSecurityFeedbackFeedbackContextsEl>>,
}
