use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IdentityPlatformOauthIdpConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    client_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    issuer: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_type: Option<Vec<IdentityPlatformOauthIdpConfigResponseTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IdentityPlatformOauthIdpConfigTimeoutsEl>,
    dynamic: IdentityPlatformOauthIdpConfigDynamic,
}
struct IdentityPlatformOauthIdpConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IdentityPlatformOauthIdpConfigData>,
}
#[derive(Clone)]
pub struct IdentityPlatformOauthIdpConfig(Rc<IdentityPlatformOauthIdpConfig_>);
impl IdentityPlatformOauthIdpConfig {
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
    #[doc = "Set the field `client_secret`.\nThe client secret of the OAuth client, to enable OIDC code flow."]
    pub fn set_client_secret(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nHuman friendly display name."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\nIf this config allows users to sign in with the provider."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
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
    #[doc = "Set the field `response_type`.\n"]
    pub fn set_response_type(
        self,
        v: impl Into<BlockAssignable<IdentityPlatformOauthIdpConfigResponseTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().response_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.response_type = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IdentityPlatformOauthIdpConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client id of an OAuth client."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nThe client secret of the OAuth client, to enable OIDC code flow."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman friendly display name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nIf this config allows users to sign in with the provider."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\nFor OIDC Idps, the issuer identifier."]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.issuer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the OauthIdpConfig. Must start with 'oidc.'."]
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
    #[doc = "Get a reference to the value of field `response_type` after provisioning.\n"]
    pub fn response_type(&self) -> ListRef<IdentityPlatformOauthIdpConfigResponseTypeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformOauthIdpConfigTimeoutsElRef {
        IdentityPlatformOauthIdpConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IdentityPlatformOauthIdpConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IdentityPlatformOauthIdpConfig {}
impl ToListMappable for IdentityPlatformOauthIdpConfig {
    type O = ListRef<IdentityPlatformOauthIdpConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IdentityPlatformOauthIdpConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_identity_platform_oauth_idp_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIdentityPlatformOauthIdpConfig {
    pub tf_id: String,
    #[doc = "The client id of an OAuth client."]
    pub client_id: PrimField<String>,
    #[doc = "For OIDC Idps, the issuer identifier."]
    pub issuer: PrimField<String>,
    #[doc = "The name of the OauthIdpConfig. Must start with 'oidc.'."]
    pub name: PrimField<String>,
}
impl BuildIdentityPlatformOauthIdpConfig {
    pub fn build(self, stack: &mut Stack) -> IdentityPlatformOauthIdpConfig {
        let out = IdentityPlatformOauthIdpConfig(Rc::new(IdentityPlatformOauthIdpConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IdentityPlatformOauthIdpConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                client_id: self.client_id,
                client_secret: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                issuer: self.issuer,
                name: self.name,
                project: core::default::Default::default(),
                response_type: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IdentityPlatformOauthIdpConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformOauthIdpConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IdentityPlatformOauthIdpConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client id of an OAuth client."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nThe client secret of the OAuth client, to enable OIDC code flow."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman friendly display name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nIf this config allows users to sign in with the provider."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\nFor OIDC Idps, the issuer identifier."]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.issuer", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the OauthIdpConfig. Must start with 'oidc.'."]
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
    #[doc = "Get a reference to the value of field `response_type` after provisioning.\n"]
    pub fn response_type(&self) -> ListRef<IdentityPlatformOauthIdpConfigResponseTypeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformOauthIdpConfigTimeoutsElRef {
        IdentityPlatformOauthIdpConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformOauthIdpConfigResponseTypeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_token: Option<PrimField<bool>>,
}
impl IdentityPlatformOauthIdpConfigResponseTypeEl {
    #[doc = "Set the field `code`.\nIf true, authorization code is returned from IdP's authorization endpoint."]
    pub fn set_code(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `id_token`.\nIf true, ID token is returned from IdP's authorization endpoint."]
    pub fn set_id_token(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.id_token = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformOauthIdpConfigResponseTypeEl {
    type O = BlockAssignable<IdentityPlatformOauthIdpConfigResponseTypeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformOauthIdpConfigResponseTypeEl {}
impl BuildIdentityPlatformOauthIdpConfigResponseTypeEl {
    pub fn build(self) -> IdentityPlatformOauthIdpConfigResponseTypeEl {
        IdentityPlatformOauthIdpConfigResponseTypeEl {
            code: core::default::Default::default(),
            id_token: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformOauthIdpConfigResponseTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformOauthIdpConfigResponseTypeElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformOauthIdpConfigResponseTypeElRef {
        IdentityPlatformOauthIdpConfigResponseTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformOauthIdpConfigResponseTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\nIf true, authorization code is returned from IdP's authorization endpoint."]
    pub fn code(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `id_token` after provisioning.\nIf true, ID token is returned from IdP's authorization endpoint."]
    pub fn id_token(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.id_token", self.base))
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformOauthIdpConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IdentityPlatformOauthIdpConfigTimeoutsEl {
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
impl ToListMappable for IdentityPlatformOauthIdpConfigTimeoutsEl {
    type O = BlockAssignable<IdentityPlatformOauthIdpConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformOauthIdpConfigTimeoutsEl {}
impl BuildIdentityPlatformOauthIdpConfigTimeoutsEl {
    pub fn build(self) -> IdentityPlatformOauthIdpConfigTimeoutsEl {
        IdentityPlatformOauthIdpConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformOauthIdpConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformOauthIdpConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformOauthIdpConfigTimeoutsElRef {
        IdentityPlatformOauthIdpConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformOauthIdpConfigTimeoutsElRef {
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
struct IdentityPlatformOauthIdpConfigDynamic {
    response_type: Option<DynamicBlock<IdentityPlatformOauthIdpConfigResponseTypeEl>>,
}
