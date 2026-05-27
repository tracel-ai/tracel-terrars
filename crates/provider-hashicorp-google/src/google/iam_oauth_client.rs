use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamOauthClientData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    allowed_grant_types: ListField<PrimField<String>>,
    allowed_redirect_uris: ListField<PrimField<String>>,
    allowed_scopes: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    oauth_client_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamOauthClientTimeoutsEl>,
}
struct IamOauthClient_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamOauthClientData>,
}
#[derive(Clone)]
pub struct IamOauthClient(Rc<IamOauthClient_>);
impl IamOauthClient {
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
    #[doc = "Set the field `client_type`.\nImmutable. The type of OauthClient. Either public or private.\nFor private clients, the client secret can be managed using the dedicated\nOauthClientCredential resource.\nPossible values:\nCLIENT_TYPE_UNSPECIFIED\nPUBLIC_CLIENT\nCONFIDENTIAL_CLIENT"]
    pub fn set_client_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA user-specified description of the OauthClient.\n\nCannot exceed 256 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the OauthClient is disabled. You cannot use a disabled OAuth\nclient."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nA user-specified display name of the OauthClient.\n\nCannot exceed 32 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IamOauthClientTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `allowed_grant_types` after provisioning.\nRequired. The list of OAuth grant types is allowed for the OauthClient."]
    pub fn allowed_grant_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_grant_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_redirect_uris` after provisioning.\nRequired. The list of redirect uris that is allowed to redirect back\nwhen authorization process is completed."]
    pub fn allowed_redirect_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_redirect_uris", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_scopes` after provisioning.\nRequired. The list of scopes that the OauthClient is allowed to request during\nOAuth flows.\n\nThe following scopes are supported:\n\n* 'https://www.googleapis.com/auth/cloud-platform': See, edit, configure,\nand delete your Google Cloud data and see the email address for your Google\nAccount.\n* 'openid': The OAuth client can associate you with your personal\ninformation on Google Cloud.\n* 'email': The OAuth client can read a federated identity's email address.\n* 'groups': The OAuth client can read a federated identity's groups."]
    pub fn allowed_scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_scopes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nOutput only. The system-generated OauthClient id."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_type` after provisioning.\nImmutable. The type of OauthClient. Either public or private.\nFor private clients, the client secret can be managed using the dedicated\nOauthClientCredential resource.\nPossible values:\nCLIENT_TYPE_UNSPECIFIED\nPUBLIC_CLIENT\nCONFIDENTIAL_CLIENT"]
    pub fn client_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-specified description of the OauthClient.\n\nCannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the OauthClient is disabled. You cannot use a disabled OAuth\nclient."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-specified display name of the OauthClient.\n\nCannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nTime after which the OauthClient will be permanently purged and cannot\nbe recovered."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. Identifier. The resource name of the OauthClient.\n\nFormat:'projects/{project}/locations/{location}/oauthClients/{oauth_client}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_client_id` after provisioning.\nRequired. The ID to use for the OauthClient, which becomes the final component of\nthe resource name. This value should be a string of 6 to 63 lowercase\nletters, digits, or hyphens. It must start with a letter, and cannot have a\ntrailing hyphen. The prefix 'gcp-' is reserved for use by Google, and may\nnot be specified."]
    pub fn oauth_client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth_client_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the OauthClient.\nPossible values:\nSTATE_UNSPECIFIED\nACTIVE\nDELETED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamOauthClientTimeoutsElRef {
        IamOauthClientTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IamOauthClient {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamOauthClient {}
impl ToListMappable for IamOauthClient {
    type O = ListRef<IamOauthClientRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamOauthClient_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_oauth_client".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamOauthClient {
    pub tf_id: String,
    #[doc = "Required. The list of OAuth grant types is allowed for the OauthClient."]
    pub allowed_grant_types: ListField<PrimField<String>>,
    #[doc = "Required. The list of redirect uris that is allowed to redirect back\nwhen authorization process is completed."]
    pub allowed_redirect_uris: ListField<PrimField<String>>,
    #[doc = "Required. The list of scopes that the OauthClient is allowed to request during\nOAuth flows.\n\nThe following scopes are supported:\n\n* 'https://www.googleapis.com/auth/cloud-platform': See, edit, configure,\nand delete your Google Cloud data and see the email address for your Google\nAccount.\n* 'openid': The OAuth client can associate you with your personal\ninformation on Google Cloud.\n* 'email': The OAuth client can read a federated identity's email address.\n* 'groups': The OAuth client can read a federated identity's groups."]
    pub allowed_scopes: ListField<PrimField<String>>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "Required. The ID to use for the OauthClient, which becomes the final component of\nthe resource name. This value should be a string of 6 to 63 lowercase\nletters, digits, or hyphens. It must start with a letter, and cannot have a\ntrailing hyphen. The prefix 'gcp-' is reserved for use by Google, and may\nnot be specified."]
    pub oauth_client_id: PrimField<String>,
}
impl BuildIamOauthClient {
    pub fn build(self, stack: &mut Stack) -> IamOauthClient {
        let out = IamOauthClient(Rc::new(IamOauthClient_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamOauthClientData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allowed_grant_types: self.allowed_grant_types,
                allowed_redirect_uris: self.allowed_redirect_uris,
                allowed_scopes: self.allowed_scopes,
                client_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                oauth_client_id: self.oauth_client_id,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IamOauthClientRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamOauthClientRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamOauthClientRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_grant_types` after provisioning.\nRequired. The list of OAuth grant types is allowed for the OauthClient."]
    pub fn allowed_grant_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_grant_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_redirect_uris` after provisioning.\nRequired. The list of redirect uris that is allowed to redirect back\nwhen authorization process is completed."]
    pub fn allowed_redirect_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_redirect_uris", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_scopes` after provisioning.\nRequired. The list of scopes that the OauthClient is allowed to request during\nOAuth flows.\n\nThe following scopes are supported:\n\n* 'https://www.googleapis.com/auth/cloud-platform': See, edit, configure,\nand delete your Google Cloud data and see the email address for your Google\nAccount.\n* 'openid': The OAuth client can associate you with your personal\ninformation on Google Cloud.\n* 'email': The OAuth client can read a federated identity's email address.\n* 'groups': The OAuth client can read a federated identity's groups."]
    pub fn allowed_scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_scopes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nOutput only. The system-generated OauthClient id."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_type` after provisioning.\nImmutable. The type of OauthClient. Either public or private.\nFor private clients, the client secret can be managed using the dedicated\nOauthClientCredential resource.\nPossible values:\nCLIENT_TYPE_UNSPECIFIED\nPUBLIC_CLIENT\nCONFIDENTIAL_CLIENT"]
    pub fn client_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-specified description of the OauthClient.\n\nCannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the OauthClient is disabled. You cannot use a disabled OAuth\nclient."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-specified display name of the OauthClient.\n\nCannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nTime after which the OauthClient will be permanently purged and cannot\nbe recovered."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. Identifier. The resource name of the OauthClient.\n\nFormat:'projects/{project}/locations/{location}/oauthClients/{oauth_client}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_client_id` after provisioning.\nRequired. The ID to use for the OauthClient, which becomes the final component of\nthe resource name. This value should be a string of 6 to 63 lowercase\nletters, digits, or hyphens. It must start with a letter, and cannot have a\ntrailing hyphen. The prefix 'gcp-' is reserved for use by Google, and may\nnot be specified."]
    pub fn oauth_client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth_client_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the OauthClient.\nPossible values:\nSTATE_UNSPECIFIED\nACTIVE\nDELETED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamOauthClientTimeoutsElRef {
        IamOauthClientTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamOauthClientTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IamOauthClientTimeoutsEl {
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
impl ToListMappable for IamOauthClientTimeoutsEl {
    type O = BlockAssignable<IamOauthClientTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamOauthClientTimeoutsEl {}
impl BuildIamOauthClientTimeoutsEl {
    pub fn build(self) -> IamOauthClientTimeoutsEl {
        IamOauthClientTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IamOauthClientTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamOauthClientTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamOauthClientTimeoutsElRef {
        IamOauthClientTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamOauthClientTimeoutsElRef {
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
