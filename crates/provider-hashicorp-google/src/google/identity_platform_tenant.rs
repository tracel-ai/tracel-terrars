use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IdentityPlatformTenantData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_password_signup: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_auth: Option<PrimField<bool>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_email_link_signin: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client: Option<Vec<IdentityPlatformTenantClientEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IdentityPlatformTenantTimeoutsEl>,
    dynamic: IdentityPlatformTenantDynamic,
}
struct IdentityPlatformTenant_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IdentityPlatformTenantData>,
}
#[derive(Clone)]
pub struct IdentityPlatformTenant(Rc<IdentityPlatformTenant_>);
impl IdentityPlatformTenant {
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
    #[doc = "Set the field `allow_password_signup`.\nWhether to allow email/password user authentication."]
    pub fn set_allow_password_signup(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_password_signup = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_auth`.\nWhether authentication is disabled for the tenant. If true, the users under\nthe disabled tenant are not allowed to sign-in. Admins of the disabled tenant\nare not able to manage its users."]
    pub fn set_disable_auth(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_auth = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_email_link_signin`.\nWhether to enable email link user authentication."]
    pub fn set_enable_email_link_signin(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_email_link_signin = Some(v.into());
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
    #[doc = "Set the field `client`.\n"]
    pub fn set_client(self, v: impl Into<BlockAssignable<IdentityPlatformTenantClientEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IdentityPlatformTenantTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `allow_password_signup` after provisioning.\nWhether to allow email/password user authentication."]
    pub fn allow_password_signup(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_password_signup", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_auth` after provisioning.\nWhether authentication is disabled for the tenant. If true, the users under\nthe disabled tenant are not allowed to sign-in. Admins of the disabled tenant\nare not able to manage its users."]
    pub fn disable_auth(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_auth", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman friendly display name of the tenant."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_email_link_signin` after provisioning.\nWhether to enable email link user authentication."]
    pub fn enable_email_link_signin(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_email_link_signin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the tenant that is generated by the server"]
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
    #[doc = "Get a reference to the value of field `client` after provisioning.\n"]
    pub fn client(&self) -> ListRef<IdentityPlatformTenantClientElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformTenantTimeoutsElRef {
        IdentityPlatformTenantTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IdentityPlatformTenant {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IdentityPlatformTenant {}
impl ToListMappable for IdentityPlatformTenant {
    type O = ListRef<IdentityPlatformTenantRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IdentityPlatformTenant_ {
    fn extract_resource_type(&self) -> String {
        "google_identity_platform_tenant".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIdentityPlatformTenant {
    pub tf_id: String,
    #[doc = "Human friendly display name of the tenant."]
    pub display_name: PrimField<String>,
}
impl BuildIdentityPlatformTenant {
    pub fn build(self, stack: &mut Stack) -> IdentityPlatformTenant {
        let out = IdentityPlatformTenant(Rc::new(IdentityPlatformTenant_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IdentityPlatformTenantData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_password_signup: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                disable_auth: core::default::Default::default(),
                display_name: self.display_name,
                enable_email_link_signin: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                client: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IdentityPlatformTenantRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformTenantRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IdentityPlatformTenantRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_password_signup` after provisioning.\nWhether to allow email/password user authentication."]
    pub fn allow_password_signup(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_password_signup", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_auth` after provisioning.\nWhether authentication is disabled for the tenant. If true, the users under\nthe disabled tenant are not allowed to sign-in. Admins of the disabled tenant\nare not able to manage its users."]
    pub fn disable_auth(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_auth", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman friendly display name of the tenant."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_email_link_signin` after provisioning.\nWhether to enable email link user authentication."]
    pub fn enable_email_link_signin(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_email_link_signin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the tenant that is generated by the server"]
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
    #[doc = "Get a reference to the value of field `client` after provisioning.\n"]
    pub fn client(&self) -> ListRef<IdentityPlatformTenantClientElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformTenantTimeoutsElRef {
        IdentityPlatformTenantTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformTenantClientElPermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_user_deletion: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_user_signup: Option<PrimField<bool>>,
}
impl IdentityPlatformTenantClientElPermissionsEl {
    #[doc = "Set the field `disabled_user_deletion`.\nWhen true, end users cannot delete their account on the associated project through any of our API methods."]
    pub fn set_disabled_user_deletion(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled_user_deletion = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled_user_signup`.\nWhen true, end users cannot sign up for a new account on the associated project through any of our API methods."]
    pub fn set_disabled_user_signup(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled_user_signup = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformTenantClientElPermissionsEl {
    type O = BlockAssignable<IdentityPlatformTenantClientElPermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformTenantClientElPermissionsEl {}
impl BuildIdentityPlatformTenantClientElPermissionsEl {
    pub fn build(self) -> IdentityPlatformTenantClientElPermissionsEl {
        IdentityPlatformTenantClientElPermissionsEl {
            disabled_user_deletion: core::default::Default::default(),
            disabled_user_signup: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformTenantClientElPermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformTenantClientElPermissionsElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformTenantClientElPermissionsElRef {
        IdentityPlatformTenantClientElPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformTenantClientElPermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled_user_deletion` after provisioning.\nWhen true, end users cannot delete their account on the associated project through any of our API methods."]
    pub fn disabled_user_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled_user_deletion", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disabled_user_signup` after provisioning.\nWhen true, end users cannot sign up for a new account on the associated project through any of our API methods."]
    pub fn disabled_user_signup(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled_user_signup", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IdentityPlatformTenantClientElDynamic {
    permissions: Option<DynamicBlock<IdentityPlatformTenantClientElPermissionsEl>>,
}
#[derive(Serialize)]
pub struct IdentityPlatformTenantClientEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Vec<IdentityPlatformTenantClientElPermissionsEl>>,
    dynamic: IdentityPlatformTenantClientElDynamic,
}
impl IdentityPlatformTenantClientEl {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformTenantClientElPermissionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.permissions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.permissions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IdentityPlatformTenantClientEl {
    type O = BlockAssignable<IdentityPlatformTenantClientEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformTenantClientEl {}
impl BuildIdentityPlatformTenantClientEl {
    pub fn build(self) -> IdentityPlatformTenantClientEl {
        IdentityPlatformTenantClientEl {
            permissions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IdentityPlatformTenantClientElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformTenantClientElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformTenantClientElRef {
        IdentityPlatformTenantClientElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformTenantClientElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<IdentityPlatformTenantClientElPermissionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformTenantTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IdentityPlatformTenantTimeoutsEl {
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
impl ToListMappable for IdentityPlatformTenantTimeoutsEl {
    type O = BlockAssignable<IdentityPlatformTenantTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformTenantTimeoutsEl {}
impl BuildIdentityPlatformTenantTimeoutsEl {
    pub fn build(self) -> IdentityPlatformTenantTimeoutsEl {
        IdentityPlatformTenantTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformTenantTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformTenantTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformTenantTimeoutsElRef {
        IdentityPlatformTenantTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformTenantTimeoutsElRef {
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
struct IdentityPlatformTenantDynamic {
    client: Option<DynamicBlock<IdentityPlatformTenantClientEl>>,
}
