use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SccOrganizationCustomModuleData {
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
    display_name: PrimField<String>,
    enablement_state: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    organization: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_config: Option<Vec<SccOrganizationCustomModuleCustomConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SccOrganizationCustomModuleTimeoutsEl>,
    dynamic: SccOrganizationCustomModuleDynamic,
}
struct SccOrganizationCustomModule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SccOrganizationCustomModuleData>,
}
#[derive(Clone)]
pub struct SccOrganizationCustomModule(Rc<SccOrganizationCustomModule_>);
impl SccOrganizationCustomModule {
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
    #[doc = "Set the field `custom_config`.\n"]
    pub fn set_custom_config(
        self,
        v: impl Into<BlockAssignable<SccOrganizationCustomModuleCustomConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SccOrganizationCustomModuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `ancestor_module` after provisioning.\nIf empty, indicates that the custom module was created in the organization, folder,\nor project in which you are viewing the custom module. Otherwise, ancestor_module\nspecifies the organization or folder from which the custom module is inherited."]
    pub fn ancestor_module(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ancestor_module", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Security Health Analytics custom module. This\ndisplay name becomes the finding category for all findings that are\nreturned by this custom module. The display name must be between 1 and\n128 characters, start with a lowercase letter, and contain alphanumeric\ncharacters or underscores only."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enablement_state` after provisioning.\nThe enablement state of the custom module. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn enablement_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enablement_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_editor` after provisioning.\nThe editor that last updated the custom module."]
    pub fn last_editor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_editor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the custom module. Its format is \"organizations/{org_id}/securityHealthAnalyticsSettings/customModules/{customModule}\".\nThe id {customModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nNumerical ID of the parent organization."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the custom module was last updated.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_config` after provisioning.\n"]
    pub fn custom_config(&self) -> ListRef<SccOrganizationCustomModuleCustomConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SccOrganizationCustomModuleTimeoutsElRef {
        SccOrganizationCustomModuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SccOrganizationCustomModule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SccOrganizationCustomModule {}
impl ToListMappable for SccOrganizationCustomModule {
    type O = ListRef<SccOrganizationCustomModuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SccOrganizationCustomModule_ {
    fn extract_resource_type(&self) -> String {
        "google_scc_organization_custom_module".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSccOrganizationCustomModule {
    pub tf_id: String,
    #[doc = "The display name of the Security Health Analytics custom module. This\ndisplay name becomes the finding category for all findings that are\nreturned by this custom module. The display name must be between 1 and\n128 characters, start with a lowercase letter, and contain alphanumeric\ncharacters or underscores only."]
    pub display_name: PrimField<String>,
    #[doc = "The enablement state of the custom module. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub enablement_state: PrimField<String>,
    #[doc = "Numerical ID of the parent organization."]
    pub organization: PrimField<String>,
}
impl BuildSccOrganizationCustomModule {
    pub fn build(self, stack: &mut Stack) -> SccOrganizationCustomModule {
        let out = SccOrganizationCustomModule(Rc::new(SccOrganizationCustomModule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SccOrganizationCustomModuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                enablement_state: self.enablement_state,
                id: core::default::Default::default(),
                organization: self.organization,
                custom_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SccOrganizationCustomModuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccOrganizationCustomModuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SccOrganizationCustomModuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ancestor_module` after provisioning.\nIf empty, indicates that the custom module was created in the organization, folder,\nor project in which you are viewing the custom module. Otherwise, ancestor_module\nspecifies the organization or folder from which the custom module is inherited."]
    pub fn ancestor_module(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ancestor_module", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Security Health Analytics custom module. This\ndisplay name becomes the finding category for all findings that are\nreturned by this custom module. The display name must be between 1 and\n128 characters, start with a lowercase letter, and contain alphanumeric\ncharacters or underscores only."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enablement_state` after provisioning.\nThe enablement state of the custom module. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn enablement_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enablement_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_editor` after provisioning.\nThe editor that last updated the custom module."]
    pub fn last_editor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_editor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the custom module. Its format is \"organizations/{org_id}/securityHealthAnalyticsSettings/customModules/{customModule}\".\nThe id {customModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nNumerical ID of the parent organization."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the custom module was last updated.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_config` after provisioning.\n"]
    pub fn custom_config(&self) -> ListRef<SccOrganizationCustomModuleCustomConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SccOrganizationCustomModuleTimeoutsElRef {
        SccOrganizationCustomModuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl {
    #[doc = "Set the field `description`.\nDescription of the expression. This is a longer text which describes the\nexpression, e.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a\nfile name and a position in the file."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose. This can\nbe used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable
    for SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl
{
    type O = BlockAssignable<
        SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl
{
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildSccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl {
    pub fn build(
        self,
    ) -> SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl {
        SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef
    {
        SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the\nexpression, e.g. when hovered over it in a UI."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a\nfile name and a position in the file."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose. This can\nbe used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize, Default)]
struct SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElDynamic {
    value_expression: Option<
        DynamicBlock<
            SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_expression: Option<
        Vec<SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl>,
    >,
    dynamic: SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElDynamic,
}
impl SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesEl {
    #[doc = "Set the field `name`.\nName of the property for the custom output."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value_expression`.\n"]
    pub fn set_value_expression(
        mut self,
        v : impl Into < BlockAssignable < SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value_expression = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value_expression = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesEl {
    type O = BlockAssignable<SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesEl {}
impl BuildSccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesEl {
    pub fn build(self) -> SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesEl {
        SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesEl {
            name: core::default::Default::default(),
            value_expression: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElRef {
        SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the property for the custom output."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value_expression` after provisioning.\n"]
    pub fn value_expression(
        &self,
    ) -> ListRef<
        SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.value_expression", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SccOrganizationCustomModuleCustomConfigElCustomOutputElDynamic {
    properties:
        Option<DynamicBlock<SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesEl>>,
}
#[derive(Serialize)]
pub struct SccOrganizationCustomModuleCustomConfigElCustomOutputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesEl>>,
    dynamic: SccOrganizationCustomModuleCustomConfigElCustomOutputElDynamic,
}
impl SccOrganizationCustomModuleCustomConfigElCustomOutputEl {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl Into<
            BlockAssignable<SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.properties = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SccOrganizationCustomModuleCustomConfigElCustomOutputEl {
    type O = BlockAssignable<SccOrganizationCustomModuleCustomConfigElCustomOutputEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccOrganizationCustomModuleCustomConfigElCustomOutputEl {}
impl BuildSccOrganizationCustomModuleCustomConfigElCustomOutputEl {
    pub fn build(self) -> SccOrganizationCustomModuleCustomConfigElCustomOutputEl {
        SccOrganizationCustomModuleCustomConfigElCustomOutputEl {
            properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SccOrganizationCustomModuleCustomConfigElCustomOutputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccOrganizationCustomModuleCustomConfigElCustomOutputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SccOrganizationCustomModuleCustomConfigElCustomOutputElRef {
        SccOrganizationCustomModuleCustomConfigElCustomOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccOrganizationCustomModuleCustomConfigElCustomOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(
        &self,
    ) -> ListRef<SccOrganizationCustomModuleCustomConfigElCustomOutputElPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}
#[derive(Serialize)]
pub struct SccOrganizationCustomModuleCustomConfigElPredicateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl SccOrganizationCustomModuleCustomConfigElPredicateEl {
    #[doc = "Set the field `description`.\nDescription of the expression. This is a longer text which describes the\nexpression, e.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a\nfile name and a position in the file."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose. This can\nbe used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for SccOrganizationCustomModuleCustomConfigElPredicateEl {
    type O = BlockAssignable<SccOrganizationCustomModuleCustomConfigElPredicateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccOrganizationCustomModuleCustomConfigElPredicateEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildSccOrganizationCustomModuleCustomConfigElPredicateEl {
    pub fn build(self) -> SccOrganizationCustomModuleCustomConfigElPredicateEl {
        SccOrganizationCustomModuleCustomConfigElPredicateEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct SccOrganizationCustomModuleCustomConfigElPredicateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccOrganizationCustomModuleCustomConfigElPredicateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SccOrganizationCustomModuleCustomConfigElPredicateElRef {
        SccOrganizationCustomModuleCustomConfigElPredicateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccOrganizationCustomModuleCustomConfigElPredicateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the\nexpression, e.g. when hovered over it in a UI."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a\nfile name and a position in the file."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose. This can\nbe used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize)]
pub struct SccOrganizationCustomModuleCustomConfigElResourceSelectorEl {
    resource_types: ListField<PrimField<String>>,
}
impl SccOrganizationCustomModuleCustomConfigElResourceSelectorEl {}
impl ToListMappable for SccOrganizationCustomModuleCustomConfigElResourceSelectorEl {
    type O = BlockAssignable<SccOrganizationCustomModuleCustomConfigElResourceSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccOrganizationCustomModuleCustomConfigElResourceSelectorEl {
    #[doc = "The resource types to run the detector on."]
    pub resource_types: ListField<PrimField<String>>,
}
impl BuildSccOrganizationCustomModuleCustomConfigElResourceSelectorEl {
    pub fn build(self) -> SccOrganizationCustomModuleCustomConfigElResourceSelectorEl {
        SccOrganizationCustomModuleCustomConfigElResourceSelectorEl {
            resource_types: self.resource_types,
        }
    }
}
pub struct SccOrganizationCustomModuleCustomConfigElResourceSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccOrganizationCustomModuleCustomConfigElResourceSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SccOrganizationCustomModuleCustomConfigElResourceSelectorElRef {
        SccOrganizationCustomModuleCustomConfigElResourceSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccOrganizationCustomModuleCustomConfigElResourceSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\nThe resource types to run the detector on."]
    pub fn resource_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_types", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SccOrganizationCustomModuleCustomConfigElDynamic {
    custom_output: Option<DynamicBlock<SccOrganizationCustomModuleCustomConfigElCustomOutputEl>>,
    predicate: Option<DynamicBlock<SccOrganizationCustomModuleCustomConfigElPredicateEl>>,
    resource_selector:
        Option<DynamicBlock<SccOrganizationCustomModuleCustomConfigElResourceSelectorEl>>,
}
#[derive(Serialize)]
pub struct SccOrganizationCustomModuleCustomConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    recommendation: PrimField<String>,
    severity: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_output: Option<Vec<SccOrganizationCustomModuleCustomConfigElCustomOutputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    predicate: Option<Vec<SccOrganizationCustomModuleCustomConfigElPredicateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_selector: Option<Vec<SccOrganizationCustomModuleCustomConfigElResourceSelectorEl>>,
    dynamic: SccOrganizationCustomModuleCustomConfigElDynamic,
}
impl SccOrganizationCustomModuleCustomConfigEl {
    #[doc = "Set the field `description`.\nText that describes the vulnerability or misconfiguration that the custom\nmodule detects. This explanation is returned with each finding instance to\nhelp investigators understand the detected issue. The text must be enclosed in quotation marks."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_output`.\n"]
    pub fn set_custom_output(
        mut self,
        v: impl Into<BlockAssignable<SccOrganizationCustomModuleCustomConfigElCustomOutputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_output = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_output = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `predicate`.\n"]
    pub fn set_predicate(
        mut self,
        v: impl Into<BlockAssignable<SccOrganizationCustomModuleCustomConfigElPredicateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.predicate = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.predicate = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_selector`.\n"]
    pub fn set_resource_selector(
        mut self,
        v: impl Into<BlockAssignable<SccOrganizationCustomModuleCustomConfigElResourceSelectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_selector = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SccOrganizationCustomModuleCustomConfigEl {
    type O = BlockAssignable<SccOrganizationCustomModuleCustomConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccOrganizationCustomModuleCustomConfigEl {
    #[doc = "An explanation of the recommended steps that security teams can take to resolve\nthe detected issue. This explanation is returned with each finding generated by\nthis module in the nextSteps property of the finding JSON."]
    pub recommendation: PrimField<String>,
    #[doc = "The severity to assign to findings generated by the module. Possible values: [\"CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub severity: PrimField<String>,
}
impl BuildSccOrganizationCustomModuleCustomConfigEl {
    pub fn build(self) -> SccOrganizationCustomModuleCustomConfigEl {
        SccOrganizationCustomModuleCustomConfigEl {
            description: core::default::Default::default(),
            recommendation: self.recommendation,
            severity: self.severity,
            custom_output: core::default::Default::default(),
            predicate: core::default::Default::default(),
            resource_selector: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SccOrganizationCustomModuleCustomConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccOrganizationCustomModuleCustomConfigElRef {
    fn new(shared: StackShared, base: String) -> SccOrganizationCustomModuleCustomConfigElRef {
        SccOrganizationCustomModuleCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccOrganizationCustomModuleCustomConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nText that describes the vulnerability or misconfiguration that the custom\nmodule detects. This explanation is returned with each finding instance to\nhelp investigators understand the detected issue. The text must be enclosed in quotation marks."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `recommendation` after provisioning.\nAn explanation of the recommended steps that security teams can take to resolve\nthe detected issue. This explanation is returned with each finding generated by\nthis module in the nextSteps property of the finding JSON."]
    pub fn recommendation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.recommendation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `severity` after provisioning.\nThe severity to assign to findings generated by the module. Possible values: [\"CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub fn severity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
    #[doc = "Get a reference to the value of field `custom_output` after provisioning.\n"]
    pub fn custom_output(
        &self,
    ) -> ListRef<SccOrganizationCustomModuleCustomConfigElCustomOutputElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_output", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `predicate` after provisioning.\n"]
    pub fn predicate(&self) -> ListRef<SccOrganizationCustomModuleCustomConfigElPredicateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.predicate", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_selector` after provisioning.\n"]
    pub fn resource_selector(
        &self,
    ) -> ListRef<SccOrganizationCustomModuleCustomConfigElResourceSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_selector", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SccOrganizationCustomModuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SccOrganizationCustomModuleTimeoutsEl {
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
impl ToListMappable for SccOrganizationCustomModuleTimeoutsEl {
    type O = BlockAssignable<SccOrganizationCustomModuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccOrganizationCustomModuleTimeoutsEl {}
impl BuildSccOrganizationCustomModuleTimeoutsEl {
    pub fn build(self) -> SccOrganizationCustomModuleTimeoutsEl {
        SccOrganizationCustomModuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SccOrganizationCustomModuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccOrganizationCustomModuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SccOrganizationCustomModuleTimeoutsElRef {
        SccOrganizationCustomModuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccOrganizationCustomModuleTimeoutsElRef {
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
struct SccOrganizationCustomModuleDynamic {
    custom_config: Option<DynamicBlock<SccOrganizationCustomModuleCustomConfigEl>>,
}
