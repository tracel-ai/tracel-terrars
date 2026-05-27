use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct PrivatecaCertificateTemplateData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_lifetime: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_constraints: Option<Vec<PrivatecaCertificateTemplateIdentityConstraintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    passthrough_extensions: Option<Vec<PrivatecaCertificateTemplatePassthroughExtensionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_values: Option<Vec<PrivatecaCertificateTemplatePredefinedValuesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PrivatecaCertificateTemplateTimeoutsEl>,
    dynamic: PrivatecaCertificateTemplateDynamic,
}
struct PrivatecaCertificateTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PrivatecaCertificateTemplateData>,
}
#[derive(Clone)]
pub struct PrivatecaCertificateTemplate(Rc<PrivatecaCertificateTemplate_>);
impl PrivatecaCertificateTemplate {
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
    #[doc = "Set the field `description`.\nOptional. A human-readable description of scenarios this template is intended for."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Labels with user-defined metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_lifetime`.\nOptional. The maximum lifetime allowed for all issued certificates that use this template. If the issuing CaPool's IssuancePolicy specifies a maximum lifetime the minimum of the two durations will be the maximum lifetime for issued. Note that if the issuing CertificateAuthority expires before a Certificate's requested maximum_lifetime, the effective lifetime will be explicitly truncated to match it."]
    pub fn set_maximum_lifetime(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maximum_lifetime = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_constraints`.\n"]
    pub fn set_identity_constraints(
        self,
        v: impl Into<BlockAssignable<PrivatecaCertificateTemplateIdentityConstraintsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity_constraints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity_constraints = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `passthrough_extensions`.\n"]
    pub fn set_passthrough_extensions(
        self,
        v: impl Into<BlockAssignable<PrivatecaCertificateTemplatePassthroughExtensionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().passthrough_extensions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.passthrough_extensions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `predefined_values`.\n"]
    pub fn set_predefined_values(
        self,
        v: impl Into<BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().predefined_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.predefined_values = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PrivatecaCertificateTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this CertificateTemplate was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. A human-readable description of scenarios this template is intended for."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels with user-defined metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_lifetime` after provisioning.\nOptional. The maximum lifetime allowed for all issued certificates that use this template. If the issuing CaPool's IssuancePolicy specifies a maximum lifetime the minimum of the two durations will be the maximum lifetime for issued. Note that if the issuing CertificateAuthority expires before a Certificate's requested maximum_lifetime, the effective lifetime will be explicitly truncated to match it."]
    pub fn maximum_lifetime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_lifetime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CertificateTemplate in the format 'projects/*/locations/*/certificateTemplates/*'."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this CertificateTemplate was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity_constraints` after provisioning.\n"]
    pub fn identity_constraints(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplateIdentityConstraintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_constraints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `passthrough_extensions` after provisioning.\n"]
    pub fn passthrough_extensions(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplatePassthroughExtensionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.passthrough_extensions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `predefined_values` after provisioning.\n"]
    pub fn predefined_values(&self) -> ListRef<PrivatecaCertificateTemplatePredefinedValuesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_values", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCertificateTemplateTimeoutsElRef {
        PrivatecaCertificateTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for PrivatecaCertificateTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for PrivatecaCertificateTemplate {}
impl ToListMappable for PrivatecaCertificateTemplate {
    type O = ListRef<PrivatecaCertificateTemplateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for PrivatecaCertificateTemplate_ {
    fn extract_resource_type(&self) -> String {
        "google_privateca_certificate_template".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildPrivatecaCertificateTemplate {
    pub tf_id: String,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "The resource name for this CertificateTemplate in the format 'projects/*/locations/*/certificateTemplates/*'."]
    pub name: PrimField<String>,
}
impl BuildPrivatecaCertificateTemplate {
    pub fn build(self, stack: &mut Stack) -> PrivatecaCertificateTemplate {
        let out = PrivatecaCertificateTemplate(Rc::new(PrivatecaCertificateTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PrivatecaCertificateTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                maximum_lifetime: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                identity_constraints: core::default::Default::default(),
                passthrough_extensions: core::default::Default::default(),
                predefined_values: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct PrivatecaCertificateTemplateRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl PrivatecaCertificateTemplateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this CertificateTemplate was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. A human-readable description of scenarios this template is intended for."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels with user-defined metadata.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_lifetime` after provisioning.\nOptional. The maximum lifetime allowed for all issued certificates that use this template. If the issuing CaPool's IssuancePolicy specifies a maximum lifetime the minimum of the two durations will be the maximum lifetime for issued. Note that if the issuing CertificateAuthority expires before a Certificate's requested maximum_lifetime, the effective lifetime will be explicitly truncated to match it."]
    pub fn maximum_lifetime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_lifetime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CertificateTemplate in the format 'projects/*/locations/*/certificateTemplates/*'."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this CertificateTemplate was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity_constraints` after provisioning.\n"]
    pub fn identity_constraints(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplateIdentityConstraintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_constraints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `passthrough_extensions` after provisioning.\n"]
    pub fn passthrough_extensions(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplatePassthroughExtensionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.passthrough_extensions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `predefined_values` after provisioning.\n"]
    pub fn predefined_values(&self) -> ListRef<PrivatecaCertificateTemplatePredefinedValuesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.predefined_values", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCertificateTemplateTimeoutsElRef {
        PrivatecaCertificateTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {
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
impl ToListMappable for PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {
    type O = BlockAssignable<PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {}
impl BuildPrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {
    pub fn build(self) -> PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {
        PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl {
            description: core::default::Default::default(),
            expression: core::default::Default::default(),
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionElRef {
        PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionElRef {
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
#[derive(Serialize, Default)]
struct PrivatecaCertificateTemplateIdentityConstraintsElDynamic {
    cel_expression:
        Option<DynamicBlock<PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl>>,
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplateIdentityConstraintsEl {
    allow_subject_alt_names_passthrough: PrimField<bool>,
    allow_subject_passthrough: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cel_expression: Option<Vec<PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl>>,
    dynamic: PrivatecaCertificateTemplateIdentityConstraintsElDynamic,
}
impl PrivatecaCertificateTemplateIdentityConstraintsEl {
    #[doc = "Set the field `cel_expression`.\n"]
    pub fn set_cel_expression(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cel_expression = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cel_expression = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCertificateTemplateIdentityConstraintsEl {
    type O = BlockAssignable<PrivatecaCertificateTemplateIdentityConstraintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplateIdentityConstraintsEl {
    #[doc = "Required. If this is true, the SubjectAltNames extension may be copied from a certificate request into the signed certificate. Otherwise, the requested SubjectAltNames will be discarded."]
    pub allow_subject_alt_names_passthrough: PrimField<bool>,
    #[doc = "Required. If this is true, the Subject field may be copied from a certificate request into the signed certificate. Otherwise, the requested Subject will be discarded."]
    pub allow_subject_passthrough: PrimField<bool>,
}
impl BuildPrivatecaCertificateTemplateIdentityConstraintsEl {
    pub fn build(self) -> PrivatecaCertificateTemplateIdentityConstraintsEl {
        PrivatecaCertificateTemplateIdentityConstraintsEl {
            allow_subject_alt_names_passthrough: self.allow_subject_alt_names_passthrough,
            allow_subject_passthrough: self.allow_subject_passthrough,
            cel_expression: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCertificateTemplateIdentityConstraintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplateIdentityConstraintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplateIdentityConstraintsElRef {
        PrivatecaCertificateTemplateIdentityConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplateIdentityConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_subject_alt_names_passthrough` after provisioning.\nRequired. If this is true, the SubjectAltNames extension may be copied from a certificate request into the signed certificate. Otherwise, the requested SubjectAltNames will be discarded."]
    pub fn allow_subject_alt_names_passthrough(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_subject_alt_names_passthrough", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_subject_passthrough` after provisioning.\nRequired. If this is true, the Subject field may be copied from a certificate request into the signed certificate. Otherwise, the requested Subject will be discarded."]
    pub fn allow_subject_passthrough(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_subject_passthrough", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cel_expression` after provisioning.\n"]
    pub fn cel_expression(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplateIdentityConstraintsElCelExpressionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cel_expression", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {
    object_id_path: ListField<PrimField<f64>>,
}
impl PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {}
impl ToListMappable for PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {
    type O =
        BlockAssignable<PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {
    #[doc = "Required. The parts of an OID path. The most significant parts of the path come first."]
    pub object_id_path: ListField<PrimField<f64>>,
}
impl BuildPrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {
    pub fn build(
        self,
    ) -> PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {
        PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl {
            object_id_path: self.object_id_path,
        }
    }
}
pub struct PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsElRef {
        PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\nRequired. The parts of an OID path. The most significant parts of the path come first."]
    pub fn object_id_path(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_id_path", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCertificateTemplatePassthroughExtensionsElDynamic {
    additional_extensions: Option<
        DynamicBlock<PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl>,
    >,
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePassthroughExtensionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    known_extensions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_extensions:
        Option<Vec<PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl>>,
    dynamic: PrivatecaCertificateTemplatePassthroughExtensionsElDynamic,
}
impl PrivatecaCertificateTemplatePassthroughExtensionsEl {
    #[doc = "Set the field `known_extensions`.\nOptional. A set of named X.509 extensions. Will be combined with additional_extensions to determine the full set of X.509 extensions."]
    pub fn set_known_extensions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.known_extensions = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_extensions`.\n"]
    pub fn set_additional_extensions(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.additional_extensions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_extensions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCertificateTemplatePassthroughExtensionsEl {
    type O = BlockAssignable<PrivatecaCertificateTemplatePassthroughExtensionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplatePassthroughExtensionsEl {}
impl BuildPrivatecaCertificateTemplatePassthroughExtensionsEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePassthroughExtensionsEl {
        PrivatecaCertificateTemplatePassthroughExtensionsEl {
            known_extensions: core::default::Default::default(),
            additional_extensions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCertificateTemplatePassthroughExtensionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplatePassthroughExtensionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplatePassthroughExtensionsElRef {
        PrivatecaCertificateTemplatePassthroughExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplatePassthroughExtensionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `known_extensions` after provisioning.\nOptional. A set of named X.509 extensions. Will be combined with additional_extensions to determine the full set of X.509 extensions."]
    pub fn known_extensions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.known_extensions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_extensions` after provisioning.\n"]
    pub fn additional_extensions(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplatePassthroughExtensionsElAdditionalExtensionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_extensions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl {
    object_id_path: ListField<PrimField<f64>>,
}
impl PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl {}
impl ToListMappable
    for PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl
{
    type O = BlockAssignable<
        PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl {
    #[doc = "Required. The parts of an OID path. The most significant parts of the path come first."]
    pub object_id_path: ListField<PrimField<f64>>,
}
impl BuildPrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl {
    pub fn build(
        self,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl {
        PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl {
            object_id_path: self.object_id_path,
        }
    }
}
pub struct PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdElRef {
        PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\nRequired. The parts of an OID path. The most significant parts of the path come first."]
    pub fn object_id_path(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_id_path", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElDynamic {
    object_id: Option<
        DynamicBlock<
            PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    critical: Option<PrimField<bool>>,
    value: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_id:
        Option<Vec<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl>>,
    dynamic: PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElDynamic,
}
impl PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
    #[doc = "Set the field `critical`.\nOptional. Indicates whether or not this extension is critical (i.e., if the client does not know how to handle this extension, the client should consider this to be an error)."]
    pub fn set_critical(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.critical = Some(v.into());
        self
    }
    #[doc = "Set the field `object_id`.\n"]
    pub fn set_object_id(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.object_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.object_id = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
    type O = BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
    #[doc = "Required. The value of this X.509 extension."]
    pub value: PrimField<String>,
}
impl BuildPrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
        PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl {
            critical: core::default::Default::default(),
            value: self.value,
            object_id: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElRef {
        PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `critical` after provisioning.\nOptional. Indicates whether or not this extension is critical (i.e., if the client does not know how to handle this extension, the client should consider this to be an error)."]
    pub fn critical(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.critical", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nRequired. The value of this X.509 extension."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
    #[doc = "Get a reference to the value of field `object_id` after provisioning.\n"]
    pub fn object_id(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElObjectIdElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.object_id", self.base))
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_ca: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_issuer_path_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    null_ca: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zero_max_issuer_path_length: Option<PrimField<bool>>,
}
impl PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {
    #[doc = "Set the field `is_ca`.\nOptional. Refers to the \"CA\" X.509 extension, which is a boolean value. When this value is true, the \"CA\" in Basic Constraints extension will be set to true."]
    pub fn set_is_ca(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `max_issuer_path_length`.\nOptional. Refers to the \"path length constraint\" in Basic Constraints extension. For a CA certificate, this value describes the depth of\nsubordinate CA certificates that are allowed. If this value is less than 0, the request will fail."]
    pub fn set_max_issuer_path_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_issuer_path_length = Some(v.into());
        self
    }
    #[doc = "Set the field `null_ca`.\nOptional. When true, the \"CA\" in Basic Constraints extension will be set to null and omitted from the CA certificate.\nIf both 'is_ca' and 'null_ca' are unset, the \"CA\" in Basic Constraints extension will be set to false.\nNote that the behavior when 'is_ca = false' for this resource is different from the behavior in the Certificate Authority, Certificate and CaPool resources."]
    pub fn set_null_ca(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.null_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `zero_max_issuer_path_length`.\nOptional. When true, the \"path length constraint\" in Basic Constraints extension will be set to 0.\nif both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,\nthe max path length will be omitted from the CA certificate."]
    pub fn set_zero_max_issuer_path_length(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.zero_max_issuer_path_length = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {
    type O = BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {}
impl BuildPrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {
        PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl {
            is_ca: core::default::Default::default(),
            max_issuer_path_length: core::default::Default::default(),
            null_ca: core::default::Default::default(),
            zero_max_issuer_path_length: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateTemplatePredefinedValuesElCaOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplatePredefinedValuesElCaOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElCaOptionsElRef {
        PrivatecaCertificateTemplatePredefinedValuesElCaOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplatePredefinedValuesElCaOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_ca` after provisioning.\nOptional. Refers to the \"CA\" X.509 extension, which is a boolean value. When this value is true, the \"CA\" in Basic Constraints extension will be set to true."]
    pub fn is_ca(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_ca", self.base))
    }
    #[doc = "Get a reference to the value of field `max_issuer_path_length` after provisioning.\nOptional. Refers to the \"path length constraint\" in Basic Constraints extension. For a CA certificate, this value describes the depth of\nsubordinate CA certificates that are allowed. If this value is less than 0, the request will fail."]
    pub fn max_issuer_path_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_issuer_path_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `null_ca` after provisioning.\nOptional. When true, the \"CA\" in Basic Constraints extension will be set to null and omitted from the CA certificate.\nIf both 'is_ca' and 'null_ca' are unset, the \"CA\" in Basic Constraints extension will be set to false.\nNote that the behavior when 'is_ca = false' for this resource is different from the behavior in the Certificate Authority, Certificate and CaPool resources."]
    pub fn null_ca(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.null_ca", self.base))
    }
    #[doc = "Get a reference to the value of field `zero_max_issuer_path_length` after provisioning.\nOptional. When true, the \"path length constraint\" in Basic Constraints extension will be set to 0.\nif both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,\nthe max path length will be omitted from the CA certificate."]
    pub fn zero_max_issuer_path_length(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zero_max_issuer_path_length", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert_sign: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_commitment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crl_sign: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_encipherment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decipher_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    digital_signature: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encipher_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_agreement: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_encipherment: Option<PrimField<bool>>,
}
impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {
    #[doc = "Set the field `cert_sign`.\nThe key may be used to sign certificates."]
    pub fn set_cert_sign(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cert_sign = Some(v.into());
        self
    }
    #[doc = "Set the field `content_commitment`.\nThe key may be used for cryptographic commitments. Note that this may also be referred to as \"non-repudiation\"."]
    pub fn set_content_commitment(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.content_commitment = Some(v.into());
        self
    }
    #[doc = "Set the field `crl_sign`.\nThe key may be used sign certificate revocation lists."]
    pub fn set_crl_sign(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.crl_sign = Some(v.into());
        self
    }
    #[doc = "Set the field `data_encipherment`.\nThe key may be used to encipher data."]
    pub fn set_data_encipherment(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.data_encipherment = Some(v.into());
        self
    }
    #[doc = "Set the field `decipher_only`.\nThe key may be used to decipher only."]
    pub fn set_decipher_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.decipher_only = Some(v.into());
        self
    }
    #[doc = "Set the field `digital_signature`.\nThe key may be used for digital signatures."]
    pub fn set_digital_signature(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.digital_signature = Some(v.into());
        self
    }
    #[doc = "Set the field `encipher_only`.\nThe key may be used to encipher only."]
    pub fn set_encipher_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encipher_only = Some(v.into());
        self
    }
    #[doc = "Set the field `key_agreement`.\nThe key may be used in a key agreement protocol."]
    pub fn set_key_agreement(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.key_agreement = Some(v.into());
        self
    }
    #[doc = "Set the field `key_encipherment`.\nThe key may be used to encipher other keys."]
    pub fn set_key_encipherment(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.key_encipherment = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {
    type O =
        BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {}
impl BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl {
            cert_sign: core::default::Default::default(),
            content_commitment: core::default::Default::default(),
            crl_sign: core::default::Default::default(),
            data_encipherment: core::default::Default::default(),
            decipher_only: core::default::Default::default(),
            digital_signature: core::default::Default::default(),
            encipher_only: core::default::Default::default(),
            key_agreement: core::default::Default::default(),
            key_encipherment: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageElRef {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert_sign` after provisioning.\nThe key may be used to sign certificates."]
    pub fn cert_sign(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert_sign", self.base))
    }
    #[doc = "Get a reference to the value of field `content_commitment` after provisioning.\nThe key may be used for cryptographic commitments. Note that this may also be referred to as \"non-repudiation\"."]
    pub fn content_commitment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_commitment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `crl_sign` after provisioning.\nThe key may be used sign certificate revocation lists."]
    pub fn crl_sign(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.crl_sign", self.base))
    }
    #[doc = "Get a reference to the value of field `data_encipherment` after provisioning.\nThe key may be used to encipher data."]
    pub fn data_encipherment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_encipherment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `decipher_only` after provisioning.\nThe key may be used to decipher only."]
    pub fn decipher_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.decipher_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `digital_signature` after provisioning.\nThe key may be used for digital signatures."]
    pub fn digital_signature(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.digital_signature", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encipher_only` after provisioning.\nThe key may be used to encipher only."]
    pub fn encipher_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encipher_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_agreement` after provisioning.\nThe key may be used in a key agreement protocol."]
    pub fn key_agreement(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_agreement", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_encipherment` after provisioning.\nThe key may be used to encipher other keys."]
    pub fn key_encipherment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_encipherment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_auth: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_signing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocsp_signing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_auth: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_stamping: Option<PrimField<bool>>,
}
impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {
    #[doc = "Set the field `client_auth`.\nCorresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as \"TLS WWW client authentication\", though regularly used for non-WWW TLS."]
    pub fn set_client_auth(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.client_auth = Some(v.into());
        self
    }
    #[doc = "Set the field `code_signing`.\nCorresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as \"Signing of downloadable executable code client authentication\"."]
    pub fn set_code_signing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.code_signing = Some(v.into());
        self
    }
    #[doc = "Set the field `email_protection`.\nCorresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as \"Email protection\"."]
    pub fn set_email_protection(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.email_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `ocsp_signing`.\nCorresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as \"Signing OCSP responses\"."]
    pub fn set_ocsp_signing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ocsp_signing = Some(v.into());
        self
    }
    #[doc = "Set the field `server_auth`.\nCorresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as \"TLS WWW server authentication\", though regularly used for non-WWW TLS."]
    pub fn set_server_auth(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.server_auth = Some(v.into());
        self
    }
    #[doc = "Set the field `time_stamping`.\nCorresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as \"Binding the hash of an object to a time\"."]
    pub fn set_time_stamping(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.time_stamping = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {
    type O =
        BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {}
impl BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {
    pub fn build(
        self,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl {
            client_auth: core::default::Default::default(),
            code_signing: core::default::Default::default(),
            email_protection: core::default::Default::default(),
            ocsp_signing: core::default::Default::default(),
            server_auth: core::default::Default::default(),
            time_stamping: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageElRef {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_auth` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as \"TLS WWW client authentication\", though regularly used for non-WWW TLS."]
    pub fn client_auth(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_auth", self.base))
    }
    #[doc = "Get a reference to the value of field `code_signing` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as \"Signing of downloadable executable code client authentication\"."]
    pub fn code_signing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.code_signing", self.base))
    }
    #[doc = "Get a reference to the value of field `email_protection` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as \"Email protection\"."]
    pub fn email_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_protection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ocsp_signing` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as \"Signing OCSP responses\"."]
    pub fn ocsp_signing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocsp_signing", self.base))
    }
    #[doc = "Get a reference to the value of field `server_auth` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as \"TLS WWW server authentication\", though regularly used for non-WWW TLS."]
    pub fn server_auth(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_auth", self.base))
    }
    #[doc = "Get a reference to the value of field `time_stamping` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as \"Binding the hash of an object to a time\"."]
    pub fn time_stamping(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_stamping", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    object_id_path: ListField<PrimField<f64>>,
}
impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl {}
impl ToListMappable
    for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl
{
    type O = BlockAssignable<
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    #[doc = "Required. The parts of an OID path. The most significant parts of the path come first."]
    pub object_id_path: ListField<PrimField<f64>>,
}
impl BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
    pub fn build(
        self,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl {
            object_id_path: self.object_id_path,
        }
    }
}
pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\nRequired. The parts of an OID path. The most significant parts of the path come first."]
    pub fn object_id_path(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_id_path", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElDynamic {
    base_key_usage: Option<
        DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl>,
    >,
    extended_key_usage: Option<
        DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl>,
    >,
    unknown_extended_key_usages: Option<
        DynamicBlock<
            PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_key_usage:
        Option<Vec<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extended_key_usage:
        Option<Vec<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown_extended_key_usages: Option<
        Vec<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl>,
    >,
    dynamic: PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElDynamic,
}
impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {
    #[doc = "Set the field `base_key_usage`.\n"]
    pub fn set_base_key_usage(
        mut self,
        v: impl Into<
            BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.base_key_usage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.base_key_usage = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `extended_key_usage`.\n"]
    pub fn set_extended_key_usage(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.extended_key_usage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.extended_key_usage = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `unknown_extended_key_usages`.\n"]
    pub fn set_unknown_extended_key_usages(
        mut self,
        v: impl Into<
            BlockAssignable<
                PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.unknown_extended_key_usages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.unknown_extended_key_usages = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {
    type O = BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {}
impl BuildPrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl {
            base_key_usage: core::default::Default::default(),
            extended_key_usage: core::default::Default::default(),
            unknown_extended_key_usages: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElRef {
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base_key_usage` after provisioning.\n"]
    pub fn base_key_usage(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElBaseKeyUsageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.base_key_usage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `extended_key_usage` after provisioning.\n"]
    pub fn extended_key_usage(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElExtendedKeyUsageElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.extended_key_usage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `unknown_extended_key_usages` after provisioning.\n"]
    pub fn unknown_extended_key_usages(
        &self,
    ) -> ListRef<
        PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElUnknownExtendedKeyUsagesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.unknown_extended_key_usages", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsEl {
    critical: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_dns_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_email_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permitted_dns_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permitted_email_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permitted_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permitted_uris: Option<ListField<PrimField<String>>>,
}
impl PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsEl {
    #[doc = "Set the field `excluded_dns_names`.\nContains excluded DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn set_excluded_dns_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excluded_dns_names = Some(v.into());
        self
    }
    #[doc = "Set the field `excluded_email_addresses`.\nContains the excluded email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn set_excluded_email_addresses(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.excluded_email_addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `excluded_ip_ranges`.\nContains the excluded IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn set_excluded_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excluded_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `excluded_uris`.\nContains the excluded URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn set_excluded_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excluded_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `permitted_dns_names`.\nContains permitted DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn set_permitted_dns_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.permitted_dns_names = Some(v.into());
        self
    }
    #[doc = "Set the field `permitted_email_addresses`.\nContains the permitted email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn set_permitted_email_addresses(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.permitted_email_addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `permitted_ip_ranges`.\nContains the permitted IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn set_permitted_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.permitted_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `permitted_uris`.\nContains the permitted URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn set_permitted_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.permitted_uris = Some(v.into());
        self
    }
}
impl ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsEl {
    type O = BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElNameConstraintsEl {
    #[doc = "Indicates whether or not the name constraints are marked critical."]
    pub critical: PrimField<bool>,
}
impl BuildPrivatecaCertificateTemplatePredefinedValuesElNameConstraintsEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsEl {
        PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsEl {
            critical: self.critical,
            excluded_dns_names: core::default::Default::default(),
            excluded_email_addresses: core::default::Default::default(),
            excluded_ip_ranges: core::default::Default::default(),
            excluded_uris: core::default::Default::default(),
            permitted_dns_names: core::default::Default::default(),
            permitted_email_addresses: core::default::Default::default(),
            permitted_ip_ranges: core::default::Default::default(),
            permitted_uris: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsElRef {
        PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `critical` after provisioning.\nIndicates whether or not the name constraints are marked critical."]
    pub fn critical(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.critical", self.base))
    }
    #[doc = "Get a reference to the value of field `excluded_dns_names` after provisioning.\nContains excluded DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn excluded_dns_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_dns_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_email_addresses` after provisioning.\nContains the excluded email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn excluded_email_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_email_addresses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_ip_ranges` after provisioning.\nContains the excluded IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn excluded_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_uris` after provisioning.\nContains the excluded URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn excluded_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_uris", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `permitted_dns_names` after provisioning.\nContains permitted DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn permitted_dns_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permitted_dns_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `permitted_email_addresses` after provisioning.\nContains the permitted email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn permitted_email_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permitted_email_addresses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `permitted_ip_ranges` after provisioning.\nContains the permitted IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn permitted_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permitted_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `permitted_uris` after provisioning.\nContains the permitted URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn permitted_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permitted_uris", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl {
    object_id_path: ListField<PrimField<f64>>,
}
impl PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl {}
impl ToListMappable for PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl {
    type O = BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl {
    #[doc = "Required. The parts of an OID path. The most significant parts of the path come first."]
    pub object_id_path: ListField<PrimField<f64>>,
}
impl BuildPrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl {
        PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl {
            object_id_path: self.object_id_path,
        }
    }
}
pub struct PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsElRef {
        PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\nRequired. The parts of an OID path. The most significant parts of the path come first."]
    pub fn object_id_path(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_id_path", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrivatecaCertificateTemplatePredefinedValuesElDynamic {
    additional_extensions:
        Option<DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl>>,
    ca_options: Option<DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl>>,
    key_usage: Option<DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl>>,
    name_constraints:
        Option<DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsEl>>,
    policy_ids: Option<DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl>>,
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplatePredefinedValuesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aia_ocsp_servers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_extensions:
        Option<Vec<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_options: Option<Vec<PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_usage: Option<Vec<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_constraints: Option<Vec<PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_ids: Option<Vec<PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl>>,
    dynamic: PrivatecaCertificateTemplatePredefinedValuesElDynamic,
}
impl PrivatecaCertificateTemplatePredefinedValuesEl {
    #[doc = "Set the field `aia_ocsp_servers`.\nOptional. Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the \"Authority Information Access\" extension in the certificate."]
    pub fn set_aia_ocsp_servers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.aia_ocsp_servers = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_extensions`.\n"]
    pub fn set_additional_extensions(
        mut self,
        v: impl Into<
            BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.additional_extensions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_extensions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ca_options`.\n"]
    pub fn set_ca_options(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElCaOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ca_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ca_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `key_usage`.\n"]
    pub fn set_key_usage(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.key_usage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.key_usage = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `name_constraints`.\n"]
    pub fn set_name_constraints(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.name_constraints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.name_constraints = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `policy_ids`.\n"]
    pub fn set_policy_ids(
        mut self,
        v: impl Into<BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.policy_ids = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_ids = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrivatecaCertificateTemplatePredefinedValuesEl {
    type O = BlockAssignable<PrivatecaCertificateTemplatePredefinedValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplatePredefinedValuesEl {}
impl BuildPrivatecaCertificateTemplatePredefinedValuesEl {
    pub fn build(self) -> PrivatecaCertificateTemplatePredefinedValuesEl {
        PrivatecaCertificateTemplatePredefinedValuesEl {
            aia_ocsp_servers: core::default::Default::default(),
            additional_extensions: core::default::Default::default(),
            ca_options: core::default::Default::default(),
            key_usage: core::default::Default::default(),
            name_constraints: core::default::Default::default(),
            policy_ids: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrivatecaCertificateTemplatePredefinedValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplatePredefinedValuesElRef {
    fn new(shared: StackShared, base: String) -> PrivatecaCertificateTemplatePredefinedValuesElRef {
        PrivatecaCertificateTemplatePredefinedValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplatePredefinedValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aia_ocsp_servers` after provisioning.\nOptional. Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the \"Authority Information Access\" extension in the certificate."]
    pub fn aia_ocsp_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aia_ocsp_servers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_extensions` after provisioning.\n"]
    pub fn additional_extensions(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplatePredefinedValuesElAdditionalExtensionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_extensions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ca_options` after provisioning.\n"]
    pub fn ca_options(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplatePredefinedValuesElCaOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_options", self.base))
    }
    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplatePredefinedValuesElKeyUsageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.key_usage", self.base))
    }
    #[doc = "Get a reference to the value of field `name_constraints` after provisioning.\n"]
    pub fn name_constraints(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplatePredefinedValuesElNameConstraintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.name_constraints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_ids` after provisioning.\n"]
    pub fn policy_ids(
        &self,
    ) -> ListRef<PrivatecaCertificateTemplatePredefinedValuesElPolicyIdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.base))
    }
}
#[derive(Serialize)]
pub struct PrivatecaCertificateTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl PrivatecaCertificateTemplateTimeoutsEl {
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
impl ToListMappable for PrivatecaCertificateTemplateTimeoutsEl {
    type O = BlockAssignable<PrivatecaCertificateTemplateTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrivatecaCertificateTemplateTimeoutsEl {}
impl BuildPrivatecaCertificateTemplateTimeoutsEl {
    pub fn build(self) -> PrivatecaCertificateTemplateTimeoutsEl {
        PrivatecaCertificateTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct PrivatecaCertificateTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrivatecaCertificateTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PrivatecaCertificateTemplateTimeoutsElRef {
        PrivatecaCertificateTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrivatecaCertificateTemplateTimeoutsElRef {
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
struct PrivatecaCertificateTemplateDynamic {
    identity_constraints: Option<DynamicBlock<PrivatecaCertificateTemplateIdentityConstraintsEl>>,
    passthrough_extensions:
        Option<DynamicBlock<PrivatecaCertificateTemplatePassthroughExtensionsEl>>,
    predefined_values: Option<DynamicBlock<PrivatecaCertificateTemplatePredefinedValuesEl>>,
}
