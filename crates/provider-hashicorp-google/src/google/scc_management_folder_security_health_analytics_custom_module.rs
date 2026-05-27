use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SccManagementFolderSecurityHealthAnalyticsCustomModuleData {
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
    enablement_state: Option<PrimField<String>>,
    folder: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_config:
        Option<Vec<SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsEl>,
    dynamic: SccManagementFolderSecurityHealthAnalyticsCustomModuleDynamic,
}
struct SccManagementFolderSecurityHealthAnalyticsCustomModule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SccManagementFolderSecurityHealthAnalyticsCustomModuleData>,
}
#[derive(Clone)]
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModule(
    Rc<SccManagementFolderSecurityHealthAnalyticsCustomModule_>,
);
impl SccManagementFolderSecurityHealthAnalyticsCustomModule {
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
    #[doc = "Set the field `display_name`.\nThe display name of the Security Health Analytics custom module. This\ndisplay name becomes the finding category for all findings that are\nreturned by this custom module. The display name must be between 1 and\n128 characters, start with a lowercase letter, and contain alphanumeric\ncharacters or underscores only."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `enablement_state`.\nThe enablement state of the custom module. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn set_enablement_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().enablement_state = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nLocation ID of the parent organization. If not provided, 'global' will be used as the default location."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_config`.\n"]
    pub fn set_custom_config(
        self,
        v: impl Into<
            BlockAssignable<SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigEl>,
        >,
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
    pub fn set_timeouts(
        self,
        v: impl Into<SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsEl>,
    ) -> Self {
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
    #[doc = "Get a reference to the value of field `folder` after provisioning.\nNumerical ID of the parent folder."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation ID of the parent organization. If not provided, 'global' will be used as the default location."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the custom module. Its format is \"folders/{folder}/locations/{location}/securityHealthAnalyticsCustomModules/{securityHealthAnalyticsCustomModule}\".\nThe id {securityHealthAnalyticsCustomModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
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
    pub fn custom_config(
        &self,
    ) -> ListRef<SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsElRef {
        SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SccManagementFolderSecurityHealthAnalyticsCustomModule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SccManagementFolderSecurityHealthAnalyticsCustomModule {}
impl ToListMappable for SccManagementFolderSecurityHealthAnalyticsCustomModule {
    type O = ListRef<SccManagementFolderSecurityHealthAnalyticsCustomModuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SccManagementFolderSecurityHealthAnalyticsCustomModule_ {
    fn extract_resource_type(&self) -> String {
        "google_scc_management_folder_security_health_analytics_custom_module".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSccManagementFolderSecurityHealthAnalyticsCustomModule {
    pub tf_id: String,
    #[doc = "Numerical ID of the parent folder."]
    pub folder: PrimField<String>,
}
impl BuildSccManagementFolderSecurityHealthAnalyticsCustomModule {
    pub fn build(
        self,
        stack: &mut Stack,
    ) -> SccManagementFolderSecurityHealthAnalyticsCustomModule {
        let out = SccManagementFolderSecurityHealthAnalyticsCustomModule(Rc::new(
            SccManagementFolderSecurityHealthAnalyticsCustomModule_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(SccManagementFolderSecurityHealthAnalyticsCustomModuleData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    display_name: core::default::Default::default(),
                    enablement_state: core::default::Default::default(),
                    folder: self.folder,
                    id: core::default::Default::default(),
                    location: core::default::Default::default(),
                    custom_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccManagementFolderSecurityHealthAnalyticsCustomModuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleRef {
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
    #[doc = "Get a reference to the value of field `folder` after provisioning.\nNumerical ID of the parent folder."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation ID of the parent organization. If not provided, 'global' will be used as the default location."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the custom module. Its format is \"folders/{folder}/locations/{location}/securityHealthAnalyticsCustomModules/{securityHealthAnalyticsCustomModule}\".\nThe id {securityHealthAnalyticsCustomModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
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
    pub fn custom_config(
        &self,
    ) -> ListRef<SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsElRef {
        SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl { # [doc = "Set the field `description`.\nDescription of the expression. This is a longer text which describes the\nexpression, e.g. when hovered over it in a UI."] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a\nfile name and a position in the file."] pub fn set_location (mut self , v : impl Into < PrimField < String > >) -> Self { self . location = Some (v . into ()) ; self } # [doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose. This can\nbe used e.g. in UIs which allow to enter the expression."] pub fn set_title (mut self , v : impl Into < PrimField < String > >) -> Self { self . title = Some (v . into ()) ; self } }
impl ToListMappable for SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl { type O = BlockAssignable < SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl
{
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl { pub fn build (self) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl { SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl { description : core :: default :: Default :: default () , expression : self . expression , location : core :: default :: Default :: default () , title : core :: default :: Default :: default () , } } }
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef { fn new (shared : StackShared , base : String) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef { SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef { shared : shared , base : base . to_string () , } } }
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the\nexpression, e.g. when hovered over it in a UI."] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."] pub fn expression (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.expression" , self . base)) } # [doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a\nfile name and a position in the file."] pub fn location (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.location" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose. This can\nbe used e.g. in UIs which allow to enter the expression."] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } }
#[derive(Serialize, Default)]
struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElDynamic { value_expression : Option < DynamicBlock < SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl >> , }
#[derive(Serialize)]
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesEl { # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] value_expression : Option < Vec < SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl > > , dynamic : SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElDynamic , }
impl
    SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesEl
{
    #[doc = "Set the field `name`.\nName of the property for the custom output."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value_expression`.\n"]
    pub fn set_value_expression(
        mut self,
        v : impl Into < BlockAssignable < SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionEl >>,
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
impl ToListMappable for SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesEl { type O = BlockAssignable < SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesEl
{}
impl BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesEl { pub fn build (self) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesEl { SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesEl { name : core :: default :: Default :: default () , value_expression : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElRef { fn new (shared : StackShared , base : String) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElRef { SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElRef { shared : shared , base : base . to_string () , } } }
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nName of the property for the custom output."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value_expression` after provisioning.\n"] pub fn value_expression (& self) -> ListRef < SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElValueExpressionElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.value_expression" , self . base)) } }
#[derive(Serialize, Default)]
struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElDynamic { properties : Option < DynamicBlock < SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesEl >> , }
#[derive(Serialize)]
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputEl { # [serde (skip_serializing_if = "Option::is_none")] properties : Option < Vec < SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesEl > > , dynamic : SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElDynamic , }
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputEl {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v : impl Into < BlockAssignable < SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesEl >>,
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
impl ToListMappable
    for SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputEl
{
    type O = BlockAssignable<
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputEl
{}
impl BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputEl {
    pub fn build(
        self,
    ) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputEl {
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputEl {
            properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElRef {
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]    pub fn properties (& self) -> ListRef < SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElPropertiesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}
#[derive(Serialize)]
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateEl {
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
    for SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateEl
{
    type O = BlockAssignable<
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateEl {
    pub fn build(
        self,
    ) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateEl {
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateElRef {
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateElRef {
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
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorEl {
    resource_types: ListField<PrimField<String>>,
}
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorEl {}
impl ToListMappable
    for SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorEl
{
    type O = BlockAssignable<
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorEl
{
    #[doc = "The resource types to run the detector on."]
    pub resource_types: ListField<PrimField<String>>,
}
impl BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorEl {
    pub fn build(
        self,
    ) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorEl
    {
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorEl {
            resource_types: self.resource_types,
        }
    }
}
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorElRef
    {
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorElRef {
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
struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElDynamic {
    custom_output: Option<
        DynamicBlock<
            SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputEl,
        >,
    >,
    predicate: Option<
        DynamicBlock<
            SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateEl,
        >,
    >,
    resource_selector: Option<
        DynamicBlock<
            SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recommendation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_output: Option<
        Vec<SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    predicate: Option<
        Vec<SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_selector: Option<
        Vec<SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorEl>,
    >,
    dynamic: SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElDynamic,
}
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigEl {
    #[doc = "Set the field `description`.\nText that describes the vulnerability or misconfiguration that the custom\nmodule detects. This explanation is returned with each finding instance to\nhelp investigators understand the detected issue. The text must be enclosed in quotation marks."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `recommendation`.\nAn explanation of the recommended steps that security teams can take to resolve\nthe detected issue. This explanation is returned with each finding generated by\nthis module in the nextSteps property of the finding JSON."]
    pub fn set_recommendation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.recommendation = Some(v.into());
        self
    }
    #[doc = "Set the field `severity`.\nThe severity to assign to findings generated by the module. Possible values: [\"CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub fn set_severity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.severity = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_output`.\n"]
    pub fn set_custom_output(
        mut self,
        v: impl Into<
            BlockAssignable<
                SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputEl,
            >,
        >,
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
        v: impl Into<
            BlockAssignable<
                SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateEl,
            >,
        >,
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
        v : impl Into < BlockAssignable < SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorEl >>,
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
impl ToListMappable for SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigEl {
    type O = BlockAssignable<SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigEl {}
impl BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigEl {
    pub fn build(self) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigEl {
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigEl {
            description: core::default::Default::default(),
            recommendation: core::default::Default::default(),
            severity: core::default::Default::default(),
            custom_output: core::default::Default::default(),
            predicate: core::default::Default::default(),
            resource_selector: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElRef {
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElRef {
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
    ) -> ListRef<
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElCustomOutputElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_output", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `predicate` after provisioning.\n"]
    pub fn predicate(
        &self,
    ) -> ListRef<SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElPredicateElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.predicate", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_selector` after provisioning.\n"]
    pub fn resource_selector(
        &self,
    ) -> ListRef<
        SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigElResourceSelectorElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_selector", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsEl {
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
impl ToListMappable for SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsEl {
    type O = BlockAssignable<SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsEl {}
impl BuildSccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsEl {
    pub fn build(self) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsEl {
        SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsElRef {
        SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccManagementFolderSecurityHealthAnalyticsCustomModuleTimeoutsElRef {
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
struct SccManagementFolderSecurityHealthAnalyticsCustomModuleDynamic {
    custom_config:
        Option<DynamicBlock<SccManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigEl>>,
}
