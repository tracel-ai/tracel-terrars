use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GrafanaWorkspaceSamlConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_role_values: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_organizations: Option<ListField<PrimField<String>>>,
    editor_role_values: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_assertion: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups_assertion: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idp_metadata_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idp_metadata_xml: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_assertion: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_validity_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_assertion: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_assertion: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_assertion: Option<PrimField<String>>,
    workspace_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GrafanaWorkspaceSamlConfigurationTimeoutsEl>,
}
struct GrafanaWorkspaceSamlConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GrafanaWorkspaceSamlConfigurationData>,
}
#[derive(Clone)]
pub struct GrafanaWorkspaceSamlConfiguration(Rc<GrafanaWorkspaceSamlConfiguration_>);
impl GrafanaWorkspaceSamlConfiguration {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `admin_role_values`.\n"]
    pub fn set_admin_role_values(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().admin_role_values = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_organizations`.\n"]
    pub fn set_allowed_organizations(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().allowed_organizations = Some(v.into());
        self
    }
    #[doc = "Set the field `email_assertion`.\n"]
    pub fn set_email_assertion(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().email_assertion = Some(v.into());
        self
    }
    #[doc = "Set the field `groups_assertion`.\n"]
    pub fn set_groups_assertion(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().groups_assertion = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `idp_metadata_url`.\n"]
    pub fn set_idp_metadata_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().idp_metadata_url = Some(v.into());
        self
    }
    #[doc = "Set the field `idp_metadata_xml`.\n"]
    pub fn set_idp_metadata_xml(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().idp_metadata_xml = Some(v.into());
        self
    }
    #[doc = "Set the field `login_assertion`.\n"]
    pub fn set_login_assertion(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().login_assertion = Some(v.into());
        self
    }
    #[doc = "Set the field `login_validity_duration`.\n"]
    pub fn set_login_validity_duration(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().login_validity_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `name_assertion`.\n"]
    pub fn set_name_assertion(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_assertion = Some(v.into());
        self
    }
    #[doc = "Set the field `org_assertion`.\n"]
    pub fn set_org_assertion(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().org_assertion = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `role_assertion`.\n"]
    pub fn set_role_assertion(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role_assertion = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GrafanaWorkspaceSamlConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `admin_role_values` after provisioning.\n"]
    pub fn admin_role_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_role_values", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_organizations` after provisioning.\n"]
    pub fn allowed_organizations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_organizations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `editor_role_values` after provisioning.\n"]
    pub fn editor_role_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.editor_role_values", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `email_assertion` after provisioning.\n"]
    pub fn email_assertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_assertion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `groups_assertion` after provisioning.\n"]
    pub fn groups_assertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.groups_assertion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `idp_metadata_url` after provisioning.\n"]
    pub fn idp_metadata_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idp_metadata_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `idp_metadata_xml` after provisioning.\n"]
    pub fn idp_metadata_xml(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idp_metadata_xml", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `login_assertion` after provisioning.\n"]
    pub fn login_assertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.login_assertion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `login_validity_duration` after provisioning.\n"]
    pub fn login_validity_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.login_validity_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_assertion` after provisioning.\n"]
    pub fn name_assertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_assertion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_assertion` after provisioning.\n"]
    pub fn org_assertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_assertion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_assertion` after provisioning.\n"]
    pub fn role_assertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_assertion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
        GrafanaWorkspaceSamlConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for GrafanaWorkspaceSamlConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GrafanaWorkspaceSamlConfiguration {}
impl ToListMappable for GrafanaWorkspaceSamlConfiguration {
    type O = ListRef<GrafanaWorkspaceSamlConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GrafanaWorkspaceSamlConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_grafana_workspace_saml_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGrafanaWorkspaceSamlConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub editor_role_values: ListField<PrimField<String>>,
    #[doc = ""]
    pub workspace_id: PrimField<String>,
}
impl BuildGrafanaWorkspaceSamlConfiguration {
    pub fn build(self, stack: &mut Stack) -> GrafanaWorkspaceSamlConfiguration {
        let out = GrafanaWorkspaceSamlConfiguration(Rc::new(GrafanaWorkspaceSamlConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GrafanaWorkspaceSamlConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admin_role_values: core::default::Default::default(),
                allowed_organizations: core::default::Default::default(),
                editor_role_values: self.editor_role_values,
                email_assertion: core::default::Default::default(),
                groups_assertion: core::default::Default::default(),
                id: core::default::Default::default(),
                idp_metadata_url: core::default::Default::default(),
                idp_metadata_xml: core::default::Default::default(),
                login_assertion: core::default::Default::default(),
                login_validity_duration: core::default::Default::default(),
                name_assertion: core::default::Default::default(),
                org_assertion: core::default::Default::default(),
                region: core::default::Default::default(),
                role_assertion: core::default::Default::default(),
                workspace_id: self.workspace_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GrafanaWorkspaceSamlConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for GrafanaWorkspaceSamlConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GrafanaWorkspaceSamlConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_role_values` after provisioning.\n"]
    pub fn admin_role_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_role_values", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_organizations` after provisioning.\n"]
    pub fn allowed_organizations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_organizations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `editor_role_values` after provisioning.\n"]
    pub fn editor_role_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.editor_role_values", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `email_assertion` after provisioning.\n"]
    pub fn email_assertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_assertion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `groups_assertion` after provisioning.\n"]
    pub fn groups_assertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.groups_assertion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `idp_metadata_url` after provisioning.\n"]
    pub fn idp_metadata_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idp_metadata_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `idp_metadata_xml` after provisioning.\n"]
    pub fn idp_metadata_xml(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idp_metadata_xml", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `login_assertion` after provisioning.\n"]
    pub fn login_assertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.login_assertion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `login_validity_duration` after provisioning.\n"]
    pub fn login_validity_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.login_validity_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_assertion` after provisioning.\n"]
    pub fn name_assertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_assertion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_assertion` after provisioning.\n"]
    pub fn org_assertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_assertion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_assertion` after provisioning.\n"]
    pub fn role_assertion(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_assertion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
        GrafanaWorkspaceSamlConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GrafanaWorkspaceSamlConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl GrafanaWorkspaceSamlConfigurationTimeoutsEl {
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
}
impl ToListMappable for GrafanaWorkspaceSamlConfigurationTimeoutsEl {
    type O = BlockAssignable<GrafanaWorkspaceSamlConfigurationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGrafanaWorkspaceSamlConfigurationTimeoutsEl {}
impl BuildGrafanaWorkspaceSamlConfigurationTimeoutsEl {
    pub fn build(self) -> GrafanaWorkspaceSamlConfigurationTimeoutsEl {
        GrafanaWorkspaceSamlConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
        GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GrafanaWorkspaceSamlConfigurationTimeoutsElRef {
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
}
