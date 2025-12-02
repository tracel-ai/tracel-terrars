use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct WorkspacesDirectoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_identity_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_directory_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_directory_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_directory_config: Option<Vec<WorkspacesDirectoryActiveDirectoryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_based_auth_properties:
        Option<Vec<WorkspacesDirectoryCertificateBasedAuthPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml_properties: Option<Vec<WorkspacesDirectorySamlPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_service_permissions: Option<Vec<WorkspacesDirectorySelfServicePermissionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_access_properties: Option<Vec<WorkspacesDirectoryWorkspaceAccessPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_creation_properties: Option<Vec<WorkspacesDirectoryWorkspaceCreationPropertiesEl>>,
    dynamic: WorkspacesDirectoryDynamic,
}
struct WorkspacesDirectory_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WorkspacesDirectoryData>,
}
#[derive(Clone)]
pub struct WorkspacesDirectory(Rc<WorkspacesDirectory_>);
impl WorkspacesDirectory {
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
    #[doc = "Set the field `directory_id`.\n"]
    pub fn set_directory_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().directory_id = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_group_ids`.\n"]
    pub fn set_ip_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ip_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().subnet_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `user_identity_type`.\n"]
    pub fn set_user_identity_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_identity_type = Some(v.into());
        self
    }
    #[doc = "Set the field `workspace_directory_description`.\n"]
    pub fn set_workspace_directory_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workspace_directory_description = Some(v.into());
        self
    }
    #[doc = "Set the field `workspace_directory_name`.\n"]
    pub fn set_workspace_directory_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workspace_directory_name = Some(v.into());
        self
    }
    #[doc = "Set the field `workspace_type`.\n"]
    pub fn set_workspace_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workspace_type = Some(v.into());
        self
    }
    #[doc = "Set the field `active_directory_config`.\n"]
    pub fn set_active_directory_config(
        self,
        v: impl Into<BlockAssignable<WorkspacesDirectoryActiveDirectoryConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().active_directory_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.active_directory_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `certificate_based_auth_properties`.\n"]
    pub fn set_certificate_based_auth_properties(
        self,
        v: impl Into<BlockAssignable<WorkspacesDirectoryCertificateBasedAuthPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certificate_based_auth_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .certificate_based_auth_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `saml_properties`.\n"]
    pub fn set_saml_properties(
        self,
        v: impl Into<BlockAssignable<WorkspacesDirectorySamlPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().saml_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.saml_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `self_service_permissions`.\n"]
    pub fn set_self_service_permissions(
        self,
        v: impl Into<BlockAssignable<WorkspacesDirectorySelfServicePermissionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().self_service_permissions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.self_service_permissions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `workspace_access_properties`.\n"]
    pub fn set_workspace_access_properties(
        self,
        v: impl Into<BlockAssignable<WorkspacesDirectoryWorkspaceAccessPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workspace_access_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workspace_access_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `workspace_creation_properties`.\n"]
    pub fn set_workspace_creation_properties(
        self,
        v: impl Into<BlockAssignable<WorkspacesDirectoryWorkspaceCreationPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workspace_creation_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .workspace_creation_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_user_name` after provisioning.\n"]
    pub fn customer_user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_user_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `directory_type` after provisioning.\n"]
    pub fn directory_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.dns_ip_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_role_id` after provisioning.\n"]
    pub fn iam_role_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_role_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_group_ids` after provisioning.\n"]
    pub fn ip_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ip_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registration_code` after provisioning.\n"]
    pub fn registration_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registration_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_identity_type` after provisioning.\n"]
    pub fn user_identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_identity_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_directory_description` after provisioning.\n"]
    pub fn workspace_directory_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_directory_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_directory_name` after provisioning.\n"]
    pub fn workspace_directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_directory_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_security_group_id` after provisioning.\n"]
    pub fn workspace_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_security_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_type` after provisioning.\n"]
    pub fn workspace_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `active_directory_config` after provisioning.\n"]
    pub fn active_directory_config(
        &self,
    ) -> ListRef<WorkspacesDirectoryActiveDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.active_directory_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_based_auth_properties` after provisioning.\n"]
    pub fn certificate_based_auth_properties(
        &self,
    ) -> ListRef<WorkspacesDirectoryCertificateBasedAuthPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_based_auth_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `saml_properties` after provisioning.\n"]
    pub fn saml_properties(&self) -> ListRef<WorkspacesDirectorySamlPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saml_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_service_permissions` after provisioning.\n"]
    pub fn self_service_permissions(
        &self,
    ) -> ListRef<WorkspacesDirectorySelfServicePermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.self_service_permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_access_properties` after provisioning.\n"]
    pub fn workspace_access_properties(
        &self,
    ) -> ListRef<WorkspacesDirectoryWorkspaceAccessPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workspace_access_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_creation_properties` after provisioning.\n"]
    pub fn workspace_creation_properties(
        &self,
    ) -> ListRef<WorkspacesDirectoryWorkspaceCreationPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workspace_creation_properties", self.extract_ref()),
        )
    }
}
impl Referable for WorkspacesDirectory {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for WorkspacesDirectory {}
impl ToListMappable for WorkspacesDirectory {
    type O = ListRef<WorkspacesDirectoryRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for WorkspacesDirectory_ {
    fn extract_resource_type(&self) -> String {
        "aws_workspaces_directory".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildWorkspacesDirectory {
    pub tf_id: String,
}
impl BuildWorkspacesDirectory {
    pub fn build(self, stack: &mut Stack) -> WorkspacesDirectory {
        let out = WorkspacesDirectory(Rc::new(WorkspacesDirectory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WorkspacesDirectoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                directory_id: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_group_ids: core::default::Default::default(),
                region: core::default::Default::default(),
                subnet_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_identity_type: core::default::Default::default(),
                workspace_directory_description: core::default::Default::default(),
                workspace_directory_name: core::default::Default::default(),
                workspace_type: core::default::Default::default(),
                active_directory_config: core::default::Default::default(),
                certificate_based_auth_properties: core::default::Default::default(),
                saml_properties: core::default::Default::default(),
                self_service_permissions: core::default::Default::default(),
                workspace_access_properties: core::default::Default::default(),
                workspace_creation_properties: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct WorkspacesDirectoryRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspacesDirectoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl WorkspacesDirectoryRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_user_name` after provisioning.\n"]
    pub fn customer_user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_user_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `directory_type` after provisioning.\n"]
    pub fn directory_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.dns_ip_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_role_id` after provisioning.\n"]
    pub fn iam_role_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_role_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_group_ids` after provisioning.\n"]
    pub fn ip_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.ip_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registration_code` after provisioning.\n"]
    pub fn registration_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registration_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_identity_type` after provisioning.\n"]
    pub fn user_identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_identity_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_directory_description` after provisioning.\n"]
    pub fn workspace_directory_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_directory_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_directory_name` after provisioning.\n"]
    pub fn workspace_directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_directory_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_security_group_id` after provisioning.\n"]
    pub fn workspace_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_security_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_type` after provisioning.\n"]
    pub fn workspace_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `active_directory_config` after provisioning.\n"]
    pub fn active_directory_config(
        &self,
    ) -> ListRef<WorkspacesDirectoryActiveDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.active_directory_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `certificate_based_auth_properties` after provisioning.\n"]
    pub fn certificate_based_auth_properties(
        &self,
    ) -> ListRef<WorkspacesDirectoryCertificateBasedAuthPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_based_auth_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `saml_properties` after provisioning.\n"]
    pub fn saml_properties(&self) -> ListRef<WorkspacesDirectorySamlPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saml_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_service_permissions` after provisioning.\n"]
    pub fn self_service_permissions(
        &self,
    ) -> ListRef<WorkspacesDirectorySelfServicePermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.self_service_permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_access_properties` after provisioning.\n"]
    pub fn workspace_access_properties(
        &self,
    ) -> ListRef<WorkspacesDirectoryWorkspaceAccessPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workspace_access_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workspace_creation_properties` after provisioning.\n"]
    pub fn workspace_creation_properties(
        &self,
    ) -> ListRef<WorkspacesDirectoryWorkspaceCreationPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workspace_creation_properties", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct WorkspacesDirectoryActiveDirectoryConfigEl {
    domain_name: PrimField<String>,
    service_account_secret_arn: PrimField<String>,
}
impl WorkspacesDirectoryActiveDirectoryConfigEl {}
impl ToListMappable for WorkspacesDirectoryActiveDirectoryConfigEl {
    type O = BlockAssignable<WorkspacesDirectoryActiveDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkspacesDirectoryActiveDirectoryConfigEl {
    #[doc = ""]
    pub domain_name: PrimField<String>,
    #[doc = ""]
    pub service_account_secret_arn: PrimField<String>,
}
impl BuildWorkspacesDirectoryActiveDirectoryConfigEl {
    pub fn build(self) -> WorkspacesDirectoryActiveDirectoryConfigEl {
        WorkspacesDirectoryActiveDirectoryConfigEl {
            domain_name: self.domain_name,
            service_account_secret_arn: self.service_account_secret_arn,
        }
    }
}
pub struct WorkspacesDirectoryActiveDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspacesDirectoryActiveDirectoryConfigElRef {
    fn new(shared: StackShared, base: String) -> WorkspacesDirectoryActiveDirectoryConfigElRef {
        WorkspacesDirectoryActiveDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkspacesDirectoryActiveDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_secret_arn` after provisioning.\n"]
    pub fn service_account_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_secret_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct WorkspacesDirectoryCertificateBasedAuthPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl WorkspacesDirectoryCertificateBasedAuthPropertiesEl {
    #[doc = "Set the field `certificate_authority_arn`.\n"]
    pub fn set_certificate_authority_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_authority_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for WorkspacesDirectoryCertificateBasedAuthPropertiesEl {
    type O = BlockAssignable<WorkspacesDirectoryCertificateBasedAuthPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkspacesDirectoryCertificateBasedAuthPropertiesEl {}
impl BuildWorkspacesDirectoryCertificateBasedAuthPropertiesEl {
    pub fn build(self) -> WorkspacesDirectoryCertificateBasedAuthPropertiesEl {
        WorkspacesDirectoryCertificateBasedAuthPropertiesEl {
            certificate_authority_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct WorkspacesDirectoryCertificateBasedAuthPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspacesDirectoryCertificateBasedAuthPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkspacesDirectoryCertificateBasedAuthPropertiesElRef {
        WorkspacesDirectoryCertificateBasedAuthPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkspacesDirectoryCertificateBasedAuthPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_authority_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkspacesDirectorySamlPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    relay_state_parameter_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_access_url: Option<PrimField<String>>,
}
impl WorkspacesDirectorySamlPropertiesEl {
    #[doc = "Set the field `relay_state_parameter_name`.\n"]
    pub fn set_relay_state_parameter_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.relay_state_parameter_name = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `user_access_url`.\n"]
    pub fn set_user_access_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_access_url = Some(v.into());
        self
    }
}
impl ToListMappable for WorkspacesDirectorySamlPropertiesEl {
    type O = BlockAssignable<WorkspacesDirectorySamlPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkspacesDirectorySamlPropertiesEl {}
impl BuildWorkspacesDirectorySamlPropertiesEl {
    pub fn build(self) -> WorkspacesDirectorySamlPropertiesEl {
        WorkspacesDirectorySamlPropertiesEl {
            relay_state_parameter_name: core::default::Default::default(),
            status: core::default::Default::default(),
            user_access_url: core::default::Default::default(),
        }
    }
}
pub struct WorkspacesDirectorySamlPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspacesDirectorySamlPropertiesElRef {
    fn new(shared: StackShared, base: String) -> WorkspacesDirectorySamlPropertiesElRef {
        WorkspacesDirectorySamlPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkspacesDirectorySamlPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `relay_state_parameter_name` after provisioning.\n"]
    pub fn relay_state_parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.relay_state_parameter_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `user_access_url` after provisioning.\n"]
    pub fn user_access_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_access_url", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct WorkspacesDirectorySelfServicePermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    change_compute_type: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    increase_volume_size: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rebuild_workspace: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restart_workspace: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_running_mode: Option<PrimField<bool>>,
}
impl WorkspacesDirectorySelfServicePermissionsEl {
    #[doc = "Set the field `change_compute_type`.\n"]
    pub fn set_change_compute_type(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.change_compute_type = Some(v.into());
        self
    }
    #[doc = "Set the field `increase_volume_size`.\n"]
    pub fn set_increase_volume_size(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.increase_volume_size = Some(v.into());
        self
    }
    #[doc = "Set the field `rebuild_workspace`.\n"]
    pub fn set_rebuild_workspace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.rebuild_workspace = Some(v.into());
        self
    }
    #[doc = "Set the field `restart_workspace`.\n"]
    pub fn set_restart_workspace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.restart_workspace = Some(v.into());
        self
    }
    #[doc = "Set the field `switch_running_mode`.\n"]
    pub fn set_switch_running_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.switch_running_mode = Some(v.into());
        self
    }
}
impl ToListMappable for WorkspacesDirectorySelfServicePermissionsEl {
    type O = BlockAssignable<WorkspacesDirectorySelfServicePermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkspacesDirectorySelfServicePermissionsEl {}
impl BuildWorkspacesDirectorySelfServicePermissionsEl {
    pub fn build(self) -> WorkspacesDirectorySelfServicePermissionsEl {
        WorkspacesDirectorySelfServicePermissionsEl {
            change_compute_type: core::default::Default::default(),
            increase_volume_size: core::default::Default::default(),
            rebuild_workspace: core::default::Default::default(),
            restart_workspace: core::default::Default::default(),
            switch_running_mode: core::default::Default::default(),
        }
    }
}
pub struct WorkspacesDirectorySelfServicePermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspacesDirectorySelfServicePermissionsElRef {
    fn new(shared: StackShared, base: String) -> WorkspacesDirectorySelfServicePermissionsElRef {
        WorkspacesDirectorySelfServicePermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkspacesDirectorySelfServicePermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `change_compute_type` after provisioning.\n"]
    pub fn change_compute_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.change_compute_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `increase_volume_size` after provisioning.\n"]
    pub fn increase_volume_size(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.increase_volume_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rebuild_workspace` after provisioning.\n"]
    pub fn rebuild_workspace(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rebuild_workspace", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `restart_workspace` after provisioning.\n"]
    pub fn restart_workspace(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restart_workspace", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `switch_running_mode` after provisioning.\n"]
    pub fn switch_running_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.switch_running_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct WorkspacesDirectoryWorkspaceAccessPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type_android: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type_chromeos: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type_ios: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type_linux: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type_osx: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type_web: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type_windows: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type_zeroclient: Option<PrimField<String>>,
}
impl WorkspacesDirectoryWorkspaceAccessPropertiesEl {
    #[doc = "Set the field `device_type_android`.\n"]
    pub fn set_device_type_android(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_type_android = Some(v.into());
        self
    }
    #[doc = "Set the field `device_type_chromeos`.\n"]
    pub fn set_device_type_chromeos(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_type_chromeos = Some(v.into());
        self
    }
    #[doc = "Set the field `device_type_ios`.\n"]
    pub fn set_device_type_ios(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_type_ios = Some(v.into());
        self
    }
    #[doc = "Set the field `device_type_linux`.\n"]
    pub fn set_device_type_linux(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_type_linux = Some(v.into());
        self
    }
    #[doc = "Set the field `device_type_osx`.\n"]
    pub fn set_device_type_osx(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_type_osx = Some(v.into());
        self
    }
    #[doc = "Set the field `device_type_web`.\n"]
    pub fn set_device_type_web(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_type_web = Some(v.into());
        self
    }
    #[doc = "Set the field `device_type_windows`.\n"]
    pub fn set_device_type_windows(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_type_windows = Some(v.into());
        self
    }
    #[doc = "Set the field `device_type_zeroclient`.\n"]
    pub fn set_device_type_zeroclient(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_type_zeroclient = Some(v.into());
        self
    }
}
impl ToListMappable for WorkspacesDirectoryWorkspaceAccessPropertiesEl {
    type O = BlockAssignable<WorkspacesDirectoryWorkspaceAccessPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkspacesDirectoryWorkspaceAccessPropertiesEl {}
impl BuildWorkspacesDirectoryWorkspaceAccessPropertiesEl {
    pub fn build(self) -> WorkspacesDirectoryWorkspaceAccessPropertiesEl {
        WorkspacesDirectoryWorkspaceAccessPropertiesEl {
            device_type_android: core::default::Default::default(),
            device_type_chromeos: core::default::Default::default(),
            device_type_ios: core::default::Default::default(),
            device_type_linux: core::default::Default::default(),
            device_type_osx: core::default::Default::default(),
            device_type_web: core::default::Default::default(),
            device_type_windows: core::default::Default::default(),
            device_type_zeroclient: core::default::Default::default(),
        }
    }
}
pub struct WorkspacesDirectoryWorkspaceAccessPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspacesDirectoryWorkspaceAccessPropertiesElRef {
    fn new(shared: StackShared, base: String) -> WorkspacesDirectoryWorkspaceAccessPropertiesElRef {
        WorkspacesDirectoryWorkspaceAccessPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkspacesDirectoryWorkspaceAccessPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `device_type_android` after provisioning.\n"]
    pub fn device_type_android(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.device_type_android", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `device_type_chromeos` after provisioning.\n"]
    pub fn device_type_chromeos(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.device_type_chromeos", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `device_type_ios` after provisioning.\n"]
    pub fn device_type_ios(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.device_type_ios", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `device_type_linux` after provisioning.\n"]
    pub fn device_type_linux(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.device_type_linux", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `device_type_osx` after provisioning.\n"]
    pub fn device_type_osx(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.device_type_osx", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `device_type_web` after provisioning.\n"]
    pub fn device_type_web(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.device_type_web", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `device_type_windows` after provisioning.\n"]
    pub fn device_type_windows(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.device_type_windows", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `device_type_zeroclient` after provisioning.\n"]
    pub fn device_type_zeroclient(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.device_type_zeroclient", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct WorkspacesDirectoryWorkspaceCreationPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_security_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_ou: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_internet_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_maintenance_mode: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_enabled_as_local_administrator: Option<PrimField<bool>>,
}
impl WorkspacesDirectoryWorkspaceCreationPropertiesEl {
    #[doc = "Set the field `custom_security_group_id`.\n"]
    pub fn set_custom_security_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_security_group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `default_ou`.\n"]
    pub fn set_default_ou(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_ou = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_internet_access`.\n"]
    pub fn set_enable_internet_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_internet_access = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_maintenance_mode`.\n"]
    pub fn set_enable_maintenance_mode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_maintenance_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `user_enabled_as_local_administrator`.\n"]
    pub fn set_user_enabled_as_local_administrator(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.user_enabled_as_local_administrator = Some(v.into());
        self
    }
}
impl ToListMappable for WorkspacesDirectoryWorkspaceCreationPropertiesEl {
    type O = BlockAssignable<WorkspacesDirectoryWorkspaceCreationPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkspacesDirectoryWorkspaceCreationPropertiesEl {}
impl BuildWorkspacesDirectoryWorkspaceCreationPropertiesEl {
    pub fn build(self) -> WorkspacesDirectoryWorkspaceCreationPropertiesEl {
        WorkspacesDirectoryWorkspaceCreationPropertiesEl {
            custom_security_group_id: core::default::Default::default(),
            default_ou: core::default::Default::default(),
            enable_internet_access: core::default::Default::default(),
            enable_maintenance_mode: core::default::Default::default(),
            user_enabled_as_local_administrator: core::default::Default::default(),
        }
    }
}
pub struct WorkspacesDirectoryWorkspaceCreationPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspacesDirectoryWorkspaceCreationPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkspacesDirectoryWorkspaceCreationPropertiesElRef {
        WorkspacesDirectoryWorkspaceCreationPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkspacesDirectoryWorkspaceCreationPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_security_group_id` after provisioning.\n"]
    pub fn custom_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_security_group_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_ou` after provisioning.\n"]
    pub fn default_ou(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ou", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_internet_access` after provisioning.\n"]
    pub fn enable_internet_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_internet_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_maintenance_mode` after provisioning.\n"]
    pub fn enable_maintenance_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_maintenance_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_enabled_as_local_administrator` after provisioning.\n"]
    pub fn user_enabled_as_local_administrator(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_enabled_as_local_administrator", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct WorkspacesDirectoryDynamic {
    active_directory_config: Option<DynamicBlock<WorkspacesDirectoryActiveDirectoryConfigEl>>,
    certificate_based_auth_properties:
        Option<DynamicBlock<WorkspacesDirectoryCertificateBasedAuthPropertiesEl>>,
    saml_properties: Option<DynamicBlock<WorkspacesDirectorySamlPropertiesEl>>,
    self_service_permissions: Option<DynamicBlock<WorkspacesDirectorySelfServicePermissionsEl>>,
    workspace_access_properties:
        Option<DynamicBlock<WorkspacesDirectoryWorkspaceAccessPropertiesEl>>,
    workspace_creation_properties:
        Option<DynamicBlock<WorkspacesDirectoryWorkspaceCreationPropertiesEl>>,
}
