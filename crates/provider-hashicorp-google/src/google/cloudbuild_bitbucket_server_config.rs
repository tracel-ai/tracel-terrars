use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudbuildBitbucketServerConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    api_key: PrimField<String>,
    config_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    host_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peered_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_ca: Option<PrimField<String>>,
    username: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connected_repositories: Option<Vec<CloudbuildBitbucketServerConfigConnectedRepositoriesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Vec<CloudbuildBitbucketServerConfigSecretsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudbuildBitbucketServerConfigTimeoutsEl>,
    dynamic: CloudbuildBitbucketServerConfigDynamic,
}
struct CloudbuildBitbucketServerConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudbuildBitbucketServerConfigData>,
}
#[derive(Clone)]
pub struct CloudbuildBitbucketServerConfig(Rc<CloudbuildBitbucketServerConfig_>);
impl CloudbuildBitbucketServerConfig {
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
    #[doc = "Set the field `peered_network`.\nThe network to be used when reaching out to the Bitbucket Server instance. The VPC network must be enabled for private service connection.\nThis should be set if the Bitbucket Server instance is hosted on-premises and not reachable by public internet. If this field is left empty,\nno network peering will occur and calls to the Bitbucket Server instance will be made over the public internet. Must be in the format\nprojects/{project}/global/networks/{network}, where {project} is a project number or id and {network} is the name of a VPC network in the project."]
    pub fn set_peered_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().peered_network = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_ca`.\nSSL certificate to use for requests to Bitbucket Server. The format should be PEM format but the extension can be one of .pem, .cer, or .crt."]
    pub fn set_ssl_ca(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ssl_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `connected_repositories`.\n"]
    pub fn set_connected_repositories(
        self,
        v: impl Into<BlockAssignable<CloudbuildBitbucketServerConfigConnectedRepositoriesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connected_repositories = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connected_repositories = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secrets`.\n"]
    pub fn set_secrets(
        self,
        v: impl Into<BlockAssignable<CloudbuildBitbucketServerConfigSecretsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secrets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secrets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudbuildBitbucketServerConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nImmutable. API Key that will be attached to webhook. Once this field has been set, it cannot be changed.\nChanging this field will result in deleting/ recreating the resource."]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config_id` after provisioning.\nThe ID to use for the BitbucketServerConfig, which will become the final component of the BitbucketServerConfig's resource name."]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host_uri` after provisioning.\nImmutable. The URI of the Bitbucket Server host. Once this field has been set, it cannot be changed.\nIf you need to change it, please create another BitbucketServerConfig."]
    pub fn host_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of this bitbucket server config."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the config."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peered_network` after provisioning.\nThe network to be used when reaching out to the Bitbucket Server instance. The VPC network must be enabled for private service connection.\nThis should be set if the Bitbucket Server instance is hosted on-premises and not reachable by public internet. If this field is left empty,\nno network peering will occur and calls to the Bitbucket Server instance will be made over the public internet. Must be in the format\nprojects/{project}/global/networks/{network}, where {project} is a project number or id and {network} is the name of a VPC network in the project."]
    pub fn peered_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peered_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_ca` after provisioning.\nSSL certificate to use for requests to Bitbucket Server. The format should be PEM format but the extension can be one of .pem, .cer, or .crt."]
    pub fn ssl_ca(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_ca", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername of the account Cloud Build will use on Bitbucket Server."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `webhook_key` after provisioning.\nOutput only. UUID included in webhook requests. The UUID is used to look up the corresponding config."]
    pub fn webhook_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> ListRef<CloudbuildBitbucketServerConfigSecretsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secrets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudbuildBitbucketServerConfigTimeoutsElRef {
        CloudbuildBitbucketServerConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudbuildBitbucketServerConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudbuildBitbucketServerConfig {}
impl ToListMappable for CloudbuildBitbucketServerConfig {
    type O = ListRef<CloudbuildBitbucketServerConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudbuildBitbucketServerConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_cloudbuild_bitbucket_server_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudbuildBitbucketServerConfig {
    pub tf_id: String,
    #[doc = "Immutable. API Key that will be attached to webhook. Once this field has been set, it cannot be changed.\nChanging this field will result in deleting/ recreating the resource."]
    pub api_key: PrimField<String>,
    #[doc = "The ID to use for the BitbucketServerConfig, which will become the final component of the BitbucketServerConfig's resource name."]
    pub config_id: PrimField<String>,
    #[doc = "Immutable. The URI of the Bitbucket Server host. Once this field has been set, it cannot be changed.\nIf you need to change it, please create another BitbucketServerConfig."]
    pub host_uri: PrimField<String>,
    #[doc = "The location of this bitbucket server config."]
    pub location: PrimField<String>,
    #[doc = "Username of the account Cloud Build will use on Bitbucket Server."]
    pub username: PrimField<String>,
}
impl BuildCloudbuildBitbucketServerConfig {
    pub fn build(self, stack: &mut Stack) -> CloudbuildBitbucketServerConfig {
        let out = CloudbuildBitbucketServerConfig(Rc::new(CloudbuildBitbucketServerConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudbuildBitbucketServerConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_key: self.api_key,
                config_id: self.config_id,
                deletion_policy: core::default::Default::default(),
                host_uri: self.host_uri,
                id: core::default::Default::default(),
                location: self.location,
                peered_network: core::default::Default::default(),
                project: core::default::Default::default(),
                ssl_ca: core::default::Default::default(),
                username: self.username,
                connected_repositories: core::default::Default::default(),
                secrets: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudbuildBitbucketServerConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildBitbucketServerConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudbuildBitbucketServerConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nImmutable. API Key that will be attached to webhook. Once this field has been set, it cannot be changed.\nChanging this field will result in deleting/ recreating the resource."]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config_id` after provisioning.\nThe ID to use for the BitbucketServerConfig, which will become the final component of the BitbucketServerConfig's resource name."]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host_uri` after provisioning.\nImmutable. The URI of the Bitbucket Server host. Once this field has been set, it cannot be changed.\nIf you need to change it, please create another BitbucketServerConfig."]
    pub fn host_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of this bitbucket server config."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the config."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peered_network` after provisioning.\nThe network to be used when reaching out to the Bitbucket Server instance. The VPC network must be enabled for private service connection.\nThis should be set if the Bitbucket Server instance is hosted on-premises and not reachable by public internet. If this field is left empty,\nno network peering will occur and calls to the Bitbucket Server instance will be made over the public internet. Must be in the format\nprojects/{project}/global/networks/{network}, where {project} is a project number or id and {network} is the name of a VPC network in the project."]
    pub fn peered_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peered_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_ca` after provisioning.\nSSL certificate to use for requests to Bitbucket Server. The format should be PEM format but the extension can be one of .pem, .cer, or .crt."]
    pub fn ssl_ca(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_ca", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername of the account Cloud Build will use on Bitbucket Server."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `webhook_key` after provisioning.\nOutput only. UUID included in webhook requests. The UUID is used to look up the corresponding config."]
    pub fn webhook_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> ListRef<CloudbuildBitbucketServerConfigSecretsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secrets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudbuildBitbucketServerConfigTimeoutsElRef {
        CloudbuildBitbucketServerConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudbuildBitbucketServerConfigConnectedRepositoriesEl {
    project_key: PrimField<String>,
    repo_slug: PrimField<String>,
}
impl CloudbuildBitbucketServerConfigConnectedRepositoriesEl {}
impl ToListMappable for CloudbuildBitbucketServerConfigConnectedRepositoriesEl {
    type O = BlockAssignable<CloudbuildBitbucketServerConfigConnectedRepositoriesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildBitbucketServerConfigConnectedRepositoriesEl {
    #[doc = "Identifier for the project storing the repository."]
    pub project_key: PrimField<String>,
    #[doc = "Identifier for the repository."]
    pub repo_slug: PrimField<String>,
}
impl BuildCloudbuildBitbucketServerConfigConnectedRepositoriesEl {
    pub fn build(self) -> CloudbuildBitbucketServerConfigConnectedRepositoriesEl {
        CloudbuildBitbucketServerConfigConnectedRepositoriesEl {
            project_key: self.project_key,
            repo_slug: self.repo_slug,
        }
    }
}
pub struct CloudbuildBitbucketServerConfigConnectedRepositoriesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildBitbucketServerConfigConnectedRepositoriesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudbuildBitbucketServerConfigConnectedRepositoriesElRef {
        CloudbuildBitbucketServerConfigConnectedRepositoriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildBitbucketServerConfigConnectedRepositoriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project_key` after provisioning.\nIdentifier for the project storing the repository."]
    pub fn project_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_key", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_slug` after provisioning.\nIdentifier for the repository."]
    pub fn repo_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_slug", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudbuildBitbucketServerConfigSecretsEl {
    admin_access_token_version_name: PrimField<String>,
    read_access_token_version_name: PrimField<String>,
    webhook_secret_version_name: PrimField<String>,
}
impl CloudbuildBitbucketServerConfigSecretsEl {}
impl ToListMappable for CloudbuildBitbucketServerConfigSecretsEl {
    type O = BlockAssignable<CloudbuildBitbucketServerConfigSecretsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildBitbucketServerConfigSecretsEl {
    #[doc = "The resource name for the admin access token's secret version."]
    pub admin_access_token_version_name: PrimField<String>,
    #[doc = "The resource name for the read access token's secret version."]
    pub read_access_token_version_name: PrimField<String>,
    #[doc = "Immutable. The resource name for the webhook secret's secret version. Once this field has been set, it cannot be changed.\nChanging this field will result in deleting/ recreating the resource."]
    pub webhook_secret_version_name: PrimField<String>,
}
impl BuildCloudbuildBitbucketServerConfigSecretsEl {
    pub fn build(self) -> CloudbuildBitbucketServerConfigSecretsEl {
        CloudbuildBitbucketServerConfigSecretsEl {
            admin_access_token_version_name: self.admin_access_token_version_name,
            read_access_token_version_name: self.read_access_token_version_name,
            webhook_secret_version_name: self.webhook_secret_version_name,
        }
    }
}
pub struct CloudbuildBitbucketServerConfigSecretsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildBitbucketServerConfigSecretsElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildBitbucketServerConfigSecretsElRef {
        CloudbuildBitbucketServerConfigSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildBitbucketServerConfigSecretsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_access_token_version_name` after provisioning.\nThe resource name for the admin access token's secret version."]
    pub fn admin_access_token_version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_access_token_version_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_access_token_version_name` after provisioning.\nThe resource name for the read access token's secret version."]
    pub fn read_access_token_version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_access_token_version_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `webhook_secret_version_name` after provisioning.\nImmutable. The resource name for the webhook secret's secret version. Once this field has been set, it cannot be changed.\nChanging this field will result in deleting/ recreating the resource."]
    pub fn webhook_secret_version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.webhook_secret_version_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudbuildBitbucketServerConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudbuildBitbucketServerConfigTimeoutsEl {
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
impl ToListMappable for CloudbuildBitbucketServerConfigTimeoutsEl {
    type O = BlockAssignable<CloudbuildBitbucketServerConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildBitbucketServerConfigTimeoutsEl {}
impl BuildCloudbuildBitbucketServerConfigTimeoutsEl {
    pub fn build(self) -> CloudbuildBitbucketServerConfigTimeoutsEl {
        CloudbuildBitbucketServerConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildBitbucketServerConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildBitbucketServerConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildBitbucketServerConfigTimeoutsElRef {
        CloudbuildBitbucketServerConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildBitbucketServerConfigTimeoutsElRef {
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
struct CloudbuildBitbucketServerConfigDynamic {
    connected_repositories:
        Option<DynamicBlock<CloudbuildBitbucketServerConfigConnectedRepositoriesEl>>,
    secrets: Option<DynamicBlock<CloudbuildBitbucketServerConfigSecretsEl>>,
}
