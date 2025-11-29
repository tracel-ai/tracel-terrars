use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataWorkspacesDirectoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    directory_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataWorkspacesDirectory_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataWorkspacesDirectoryData>,
}

#[derive(Clone)]
pub struct DataWorkspacesDirectory(Rc<DataWorkspacesDirectory_>);

impl DataWorkspacesDirectory {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `active_directory_config` after provisioning.\n"]
    pub fn active_directory_config(&self) -> SetRef<DataWorkspacesDirectoryActiveDirectoryConfigElRef> {
        SetRef::new(self.shared().clone(), format!("{}.active_directory_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_based_auth_properties` after provisioning.\n"]
    pub fn certificate_based_auth_properties(
        &self,
    ) -> ListRef<DataWorkspacesDirectoryCertificateBasedAuthPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_based_auth_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_user_name` after provisioning.\n"]
    pub fn customer_user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_user_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_type` after provisioning.\n"]
    pub fn directory_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_id` after provisioning.\n"]
    pub fn iam_role_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_group_ids` after provisioning.\n"]
    pub fn ip_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ip_group_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registration_code` after provisioning.\n"]
    pub fn registration_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registration_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_properties` after provisioning.\n"]
    pub fn saml_properties(&self) -> ListRef<DataWorkspacesDirectorySamlPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saml_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_permissions` after provisioning.\n"]
    pub fn self_service_permissions(&self) -> ListRef<DataWorkspacesDirectorySelfServicePermissionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.self_service_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_identity_type` after provisioning.\n"]
    pub fn user_identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_identity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_access_properties` after provisioning.\n"]
    pub fn workspace_access_properties(&self) -> ListRef<DataWorkspacesDirectoryWorkspaceAccessPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workspace_access_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_creation_properties` after provisioning.\n"]
    pub fn workspace_creation_properties(&self) -> ListRef<DataWorkspacesDirectoryWorkspaceCreationPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workspace_creation_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_directory_description` after provisioning.\n"]
    pub fn workspace_directory_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace_directory_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_directory_name` after provisioning.\n"]
    pub fn workspace_directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace_directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_security_group_id` after provisioning.\n"]
    pub fn workspace_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace_security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_type` after provisioning.\n"]
    pub fn workspace_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace_type", self.extract_ref()))
    }
}

impl Referable for DataWorkspacesDirectory {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataWorkspacesDirectory { }

impl ToListMappable for DataWorkspacesDirectory {
    type O = ListRef<DataWorkspacesDirectoryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataWorkspacesDirectory_ {
    fn extract_datasource_type(&self) -> String {
        "aws_workspaces_directory".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWorkspacesDirectory {
    pub tf_id: String,
    #[doc = ""]
    pub directory_id: PrimField<String>,
}

impl BuildDataWorkspacesDirectory {
    pub fn build(self, stack: &mut Stack) -> DataWorkspacesDirectory {
        let out = DataWorkspacesDirectory(Rc::new(DataWorkspacesDirectory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataWorkspacesDirectoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                directory_id: self.directory_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWorkspacesDirectoryRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesDirectoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataWorkspacesDirectoryRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `active_directory_config` after provisioning.\n"]
    pub fn active_directory_config(&self) -> SetRef<DataWorkspacesDirectoryActiveDirectoryConfigElRef> {
        SetRef::new(self.shared().clone(), format!("{}.active_directory_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_based_auth_properties` after provisioning.\n"]
    pub fn certificate_based_auth_properties(
        &self,
    ) -> ListRef<DataWorkspacesDirectoryCertificateBasedAuthPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_based_auth_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_user_name` after provisioning.\n"]
    pub fn customer_user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_user_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_type` after provisioning.\n"]
    pub fn directory_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_id` after provisioning.\n"]
    pub fn iam_role_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_group_ids` after provisioning.\n"]
    pub fn ip_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ip_group_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registration_code` after provisioning.\n"]
    pub fn registration_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registration_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_properties` after provisioning.\n"]
    pub fn saml_properties(&self) -> ListRef<DataWorkspacesDirectorySamlPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saml_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_permissions` after provisioning.\n"]
    pub fn self_service_permissions(&self) -> ListRef<DataWorkspacesDirectorySelfServicePermissionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.self_service_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_identity_type` after provisioning.\n"]
    pub fn user_identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_identity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_access_properties` after provisioning.\n"]
    pub fn workspace_access_properties(&self) -> ListRef<DataWorkspacesDirectoryWorkspaceAccessPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workspace_access_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_creation_properties` after provisioning.\n"]
    pub fn workspace_creation_properties(&self) -> ListRef<DataWorkspacesDirectoryWorkspaceCreationPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workspace_creation_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_directory_description` after provisioning.\n"]
    pub fn workspace_directory_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace_directory_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_directory_name` after provisioning.\n"]
    pub fn workspace_directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace_directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_security_group_id` after provisioning.\n"]
    pub fn workspace_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace_security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_type` after provisioning.\n"]
    pub fn workspace_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace_type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectoryActiveDirectoryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_secret_arn: Option<PrimField<String>>,
}

impl DataWorkspacesDirectoryActiveDirectoryConfigEl {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain_name = Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_secret_arn`.\n"]
    pub fn set_service_account_secret_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_secret_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataWorkspacesDirectoryActiveDirectoryConfigEl {
    type O = BlockAssignable<DataWorkspacesDirectoryActiveDirectoryConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectoryActiveDirectoryConfigEl {}

impl BuildDataWorkspacesDirectoryActiveDirectoryConfigEl {
    pub fn build(self) -> DataWorkspacesDirectoryActiveDirectoryConfigEl {
        DataWorkspacesDirectoryActiveDirectoryConfigEl {
            domain_name: core::default::Default::default(),
            service_account_secret_arn: core::default::Default::default(),
        }
    }
}

pub struct DataWorkspacesDirectoryActiveDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesDirectoryActiveDirectoryConfigElRef {
    fn new(shared: StackShared, base: String) -> DataWorkspacesDirectoryActiveDirectoryConfigElRef {
        DataWorkspacesDirectoryActiveDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectoryActiveDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_secret_arn` after provisioning.\n"]
    pub fn service_account_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_secret_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectoryCertificateBasedAuthPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_authority_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl DataWorkspacesDirectoryCertificateBasedAuthPropertiesEl {
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

impl ToListMappable for DataWorkspacesDirectoryCertificateBasedAuthPropertiesEl {
    type O = BlockAssignable<DataWorkspacesDirectoryCertificateBasedAuthPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectoryCertificateBasedAuthPropertiesEl {}

impl BuildDataWorkspacesDirectoryCertificateBasedAuthPropertiesEl {
    pub fn build(self) -> DataWorkspacesDirectoryCertificateBasedAuthPropertiesEl {
        DataWorkspacesDirectoryCertificateBasedAuthPropertiesEl {
            certificate_authority_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataWorkspacesDirectoryCertificateBasedAuthPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesDirectoryCertificateBasedAuthPropertiesElRef {
    fn new(shared: StackShared, base: String) -> DataWorkspacesDirectoryCertificateBasedAuthPropertiesElRef {
        DataWorkspacesDirectoryCertificateBasedAuthPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectoryCertificateBasedAuthPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectorySamlPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    relay_state_parameter_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_access_url: Option<PrimField<String>>,
}

impl DataWorkspacesDirectorySamlPropertiesEl {
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

impl ToListMappable for DataWorkspacesDirectorySamlPropertiesEl {
    type O = BlockAssignable<DataWorkspacesDirectorySamlPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectorySamlPropertiesEl {}

impl BuildDataWorkspacesDirectorySamlPropertiesEl {
    pub fn build(self) -> DataWorkspacesDirectorySamlPropertiesEl {
        DataWorkspacesDirectorySamlPropertiesEl {
            relay_state_parameter_name: core::default::Default::default(),
            status: core::default::Default::default(),
            user_access_url: core::default::Default::default(),
        }
    }
}

pub struct DataWorkspacesDirectorySamlPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesDirectorySamlPropertiesElRef {
    fn new(shared: StackShared, base: String) -> DataWorkspacesDirectorySamlPropertiesElRef {
        DataWorkspacesDirectorySamlPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectorySamlPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `relay_state_parameter_name` after provisioning.\n"]
    pub fn relay_state_parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.relay_state_parameter_name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `user_access_url` after provisioning.\n"]
    pub fn user_access_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_access_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectorySelfServicePermissionsEl {
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

impl DataWorkspacesDirectorySelfServicePermissionsEl {
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

impl ToListMappable for DataWorkspacesDirectorySelfServicePermissionsEl {
    type O = BlockAssignable<DataWorkspacesDirectorySelfServicePermissionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectorySelfServicePermissionsEl {}

impl BuildDataWorkspacesDirectorySelfServicePermissionsEl {
    pub fn build(self) -> DataWorkspacesDirectorySelfServicePermissionsEl {
        DataWorkspacesDirectorySelfServicePermissionsEl {
            change_compute_type: core::default::Default::default(),
            increase_volume_size: core::default::Default::default(),
            rebuild_workspace: core::default::Default::default(),
            restart_workspace: core::default::Default::default(),
            switch_running_mode: core::default::Default::default(),
        }
    }
}

pub struct DataWorkspacesDirectorySelfServicePermissionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesDirectorySelfServicePermissionsElRef {
    fn new(shared: StackShared, base: String) -> DataWorkspacesDirectorySelfServicePermissionsElRef {
        DataWorkspacesDirectorySelfServicePermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectorySelfServicePermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `change_compute_type` after provisioning.\n"]
    pub fn change_compute_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.change_compute_type", self.base))
    }

    #[doc = "Get a reference to the value of field `increase_volume_size` after provisioning.\n"]
    pub fn increase_volume_size(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.increase_volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `rebuild_workspace` after provisioning.\n"]
    pub fn rebuild_workspace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rebuild_workspace", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_workspace` after provisioning.\n"]
    pub fn restart_workspace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.restart_workspace", self.base))
    }

    #[doc = "Get a reference to the value of field `switch_running_mode` after provisioning.\n"]
    pub fn switch_running_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.switch_running_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectoryWorkspaceAccessPropertiesEl {
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

impl DataWorkspacesDirectoryWorkspaceAccessPropertiesEl {
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

impl ToListMappable for DataWorkspacesDirectoryWorkspaceAccessPropertiesEl {
    type O = BlockAssignable<DataWorkspacesDirectoryWorkspaceAccessPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectoryWorkspaceAccessPropertiesEl {}

impl BuildDataWorkspacesDirectoryWorkspaceAccessPropertiesEl {
    pub fn build(self) -> DataWorkspacesDirectoryWorkspaceAccessPropertiesEl {
        DataWorkspacesDirectoryWorkspaceAccessPropertiesEl {
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

pub struct DataWorkspacesDirectoryWorkspaceAccessPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesDirectoryWorkspaceAccessPropertiesElRef {
    fn new(shared: StackShared, base: String) -> DataWorkspacesDirectoryWorkspaceAccessPropertiesElRef {
        DataWorkspacesDirectoryWorkspaceAccessPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectoryWorkspaceAccessPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_type_android` after provisioning.\n"]
    pub fn device_type_android(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type_android", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_chromeos` after provisioning.\n"]
    pub fn device_type_chromeos(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type_chromeos", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_ios` after provisioning.\n"]
    pub fn device_type_ios(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type_ios", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_linux` after provisioning.\n"]
    pub fn device_type_linux(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type_linux", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_osx` after provisioning.\n"]
    pub fn device_type_osx(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type_osx", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_web` after provisioning.\n"]
    pub fn device_type_web(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type_web", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_windows` after provisioning.\n"]
    pub fn device_type_windows(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type_windows", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type_zeroclient` after provisioning.\n"]
    pub fn device_type_zeroclient(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type_zeroclient", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesDirectoryWorkspaceCreationPropertiesEl {
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

impl DataWorkspacesDirectoryWorkspaceCreationPropertiesEl {
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
    pub fn set_user_enabled_as_local_administrator(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.user_enabled_as_local_administrator = Some(v.into());
        self
    }
}

impl ToListMappable for DataWorkspacesDirectoryWorkspaceCreationPropertiesEl {
    type O = BlockAssignable<DataWorkspacesDirectoryWorkspaceCreationPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesDirectoryWorkspaceCreationPropertiesEl {}

impl BuildDataWorkspacesDirectoryWorkspaceCreationPropertiesEl {
    pub fn build(self) -> DataWorkspacesDirectoryWorkspaceCreationPropertiesEl {
        DataWorkspacesDirectoryWorkspaceCreationPropertiesEl {
            custom_security_group_id: core::default::Default::default(),
            default_ou: core::default::Default::default(),
            enable_internet_access: core::default::Default::default(),
            enable_maintenance_mode: core::default::Default::default(),
            user_enabled_as_local_administrator: core::default::Default::default(),
        }
    }
}

pub struct DataWorkspacesDirectoryWorkspaceCreationPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataWorkspacesDirectoryWorkspaceCreationPropertiesElRef {
    fn new(shared: StackShared, base: String) -> DataWorkspacesDirectoryWorkspaceCreationPropertiesElRef {
        DataWorkspacesDirectoryWorkspaceCreationPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesDirectoryWorkspaceCreationPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_security_group_id` after provisioning.\n"]
    pub fn custom_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_security_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `default_ou` after provisioning.\n"]
    pub fn default_ou(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ou", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_internet_access` after provisioning.\n"]
    pub fn enable_internet_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_internet_access", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_maintenance_mode` after provisioning.\n"]
    pub fn enable_maintenance_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_maintenance_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `user_enabled_as_local_administrator` after provisioning.\n"]
    pub fn user_enabled_as_local_administrator(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_enabled_as_local_administrator", self.base))
    }
}
