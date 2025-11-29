use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppfabricAppAuthorizationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app: PrimField<String>,
    app_bundle_arn: PrimField<String>,
    auth_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credential: Option<Vec<AppfabricAppAuthorizationCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tenant: Option<Vec<AppfabricAppAuthorizationTenantEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AppfabricAppAuthorizationTimeoutsEl>,
    dynamic: AppfabricAppAuthorizationDynamic,
}

struct AppfabricAppAuthorization_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppfabricAppAuthorizationData>,
}

#[derive(Clone)]
pub struct AppfabricAppAuthorization(Rc<AppfabricAppAuthorization_>);

impl AppfabricAppAuthorization {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
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

    #[doc = "Set the field `credential`.\n"]
    pub fn set_credential(self, v: impl Into<BlockAssignable<AppfabricAppAuthorizationCredentialEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().credential = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.credential = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tenant`.\n"]
    pub fn set_tenant(self, v: impl Into<BlockAssignable<AppfabricAppAuthorizationTenantEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tenant = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tenant = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AppfabricAppAuthorizationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `app` after provisioning.\n"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_bundle_arn` after provisioning.\n"]
    pub fn app_bundle_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_bundle_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_url` after provisioning.\n"]
    pub fn auth_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persona` after provisioning.\n"]
    pub fn persona(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.persona", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credential` after provisioning.\n"]
    pub fn credential(&self) -> ListRef<AppfabricAppAuthorizationCredentialElRef> {
        ListRef::new(self.shared().clone(), format!("{}.credential", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenant` after provisioning.\n"]
    pub fn tenant(&self) -> ListRef<AppfabricAppAuthorizationTenantElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tenant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppfabricAppAuthorizationTimeoutsElRef {
        AppfabricAppAuthorizationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for AppfabricAppAuthorization {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AppfabricAppAuthorization { }

impl ToListMappable for AppfabricAppAuthorization {
    type O = ListRef<AppfabricAppAuthorizationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppfabricAppAuthorization_ {
    fn extract_resource_type(&self) -> String {
        "aws_appfabric_app_authorization".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppfabricAppAuthorization {
    pub tf_id: String,
    #[doc = ""]
    pub app: PrimField<String>,
    #[doc = ""]
    pub app_bundle_arn: PrimField<String>,
    #[doc = ""]
    pub auth_type: PrimField<String>,
}

impl BuildAppfabricAppAuthorization {
    pub fn build(self, stack: &mut Stack) -> AppfabricAppAuthorization {
        let out = AppfabricAppAuthorization(Rc::new(AppfabricAppAuthorization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppfabricAppAuthorizationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app: self.app,
                app_bundle_arn: self.app_bundle_arn,
                auth_type: self.auth_type,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                credential: core::default::Default::default(),
                tenant: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppfabricAppAuthorizationRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppfabricAppAuthorizationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AppfabricAppAuthorizationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app` after provisioning.\n"]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_bundle_arn` after provisioning.\n"]
    pub fn app_bundle_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_bundle_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_url` after provisioning.\n"]
    pub fn auth_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persona` after provisioning.\n"]
    pub fn persona(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.persona", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credential` after provisioning.\n"]
    pub fn credential(&self) -> ListRef<AppfabricAppAuthorizationCredentialElRef> {
        ListRef::new(self.shared().clone(), format!("{}.credential", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenant` after provisioning.\n"]
    pub fn tenant(&self) -> ListRef<AppfabricAppAuthorizationTenantElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tenant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppfabricAppAuthorizationTimeoutsElRef {
        AppfabricAppAuthorizationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AppfabricAppAuthorizationCredentialElApiKeyCredentialEl {
    api_key: PrimField<String>,
}

impl AppfabricAppAuthorizationCredentialElApiKeyCredentialEl { }

impl ToListMappable for AppfabricAppAuthorizationCredentialElApiKeyCredentialEl {
    type O = BlockAssignable<AppfabricAppAuthorizationCredentialElApiKeyCredentialEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorizationCredentialElApiKeyCredentialEl {
    #[doc = ""]
    pub api_key: PrimField<String>,
}

impl BuildAppfabricAppAuthorizationCredentialElApiKeyCredentialEl {
    pub fn build(self) -> AppfabricAppAuthorizationCredentialElApiKeyCredentialEl {
        AppfabricAppAuthorizationCredentialElApiKeyCredentialEl { api_key: self.api_key }
    }
}

pub struct AppfabricAppAuthorizationCredentialElApiKeyCredentialElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppfabricAppAuthorizationCredentialElApiKeyCredentialElRef {
    fn new(shared: StackShared, base: String) -> AppfabricAppAuthorizationCredentialElApiKeyCredentialElRef {
        AppfabricAppAuthorizationCredentialElApiKeyCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorizationCredentialElApiKeyCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AppfabricAppAuthorizationCredentialElOauth2CredentialEl {
    client_id: PrimField<String>,
    client_secret: PrimField<String>,
}

impl AppfabricAppAuthorizationCredentialElOauth2CredentialEl { }

impl ToListMappable for AppfabricAppAuthorizationCredentialElOauth2CredentialEl {
    type O = BlockAssignable<AppfabricAppAuthorizationCredentialElOauth2CredentialEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorizationCredentialElOauth2CredentialEl {
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub client_secret: PrimField<String>,
}

impl BuildAppfabricAppAuthorizationCredentialElOauth2CredentialEl {
    pub fn build(self) -> AppfabricAppAuthorizationCredentialElOauth2CredentialEl {
        AppfabricAppAuthorizationCredentialElOauth2CredentialEl {
            client_id: self.client_id,
            client_secret: self.client_secret,
        }
    }
}

pub struct AppfabricAppAuthorizationCredentialElOauth2CredentialElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppfabricAppAuthorizationCredentialElOauth2CredentialElRef {
    fn new(shared: StackShared, base: String) -> AppfabricAppAuthorizationCredentialElOauth2CredentialElRef {
        AppfabricAppAuthorizationCredentialElOauth2CredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorizationCredentialElOauth2CredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppfabricAppAuthorizationCredentialElDynamic {
    api_key_credential: Option<DynamicBlock<AppfabricAppAuthorizationCredentialElApiKeyCredentialEl>>,
    oauth2_credential: Option<DynamicBlock<AppfabricAppAuthorizationCredentialElOauth2CredentialEl>>,
}

#[derive(Serialize)]
pub struct AppfabricAppAuthorizationCredentialEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_credential: Option<Vec<AppfabricAppAuthorizationCredentialElApiKeyCredentialEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_credential: Option<Vec<AppfabricAppAuthorizationCredentialElOauth2CredentialEl>>,
    dynamic: AppfabricAppAuthorizationCredentialElDynamic,
}

impl AppfabricAppAuthorizationCredentialEl {
    #[doc = "Set the field `api_key_credential`.\n"]
    pub fn set_api_key_credential(
        mut self,
        v: impl Into<BlockAssignable<AppfabricAppAuthorizationCredentialElApiKeyCredentialEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_key_credential = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key_credential = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2_credential`.\n"]
    pub fn set_oauth2_credential(
        mut self,
        v: impl Into<BlockAssignable<AppfabricAppAuthorizationCredentialElOauth2CredentialEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth2_credential = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_credential = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppfabricAppAuthorizationCredentialEl {
    type O = BlockAssignable<AppfabricAppAuthorizationCredentialEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorizationCredentialEl {}

impl BuildAppfabricAppAuthorizationCredentialEl {
    pub fn build(self) -> AppfabricAppAuthorizationCredentialEl {
        AppfabricAppAuthorizationCredentialEl {
            api_key_credential: core::default::Default::default(),
            oauth2_credential: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppfabricAppAuthorizationCredentialElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppfabricAppAuthorizationCredentialElRef {
    fn new(shared: StackShared, base: String) -> AppfabricAppAuthorizationCredentialElRef {
        AppfabricAppAuthorizationCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorizationCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_credential` after provisioning.\n"]
    pub fn api_key_credential(&self) -> ListRef<AppfabricAppAuthorizationCredentialElApiKeyCredentialElRef> {
        ListRef::new(self.shared().clone(), format!("{}.api_key_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_credential` after provisioning.\n"]
    pub fn oauth2_credential(&self) -> ListRef<AppfabricAppAuthorizationCredentialElOauth2CredentialElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth2_credential", self.base))
    }
}

#[derive(Serialize)]
pub struct AppfabricAppAuthorizationTenantEl {
    tenant_display_name: PrimField<String>,
    tenant_identifier: PrimField<String>,
}

impl AppfabricAppAuthorizationTenantEl { }

impl ToListMappable for AppfabricAppAuthorizationTenantEl {
    type O = BlockAssignable<AppfabricAppAuthorizationTenantEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorizationTenantEl {
    #[doc = ""]
    pub tenant_display_name: PrimField<String>,
    #[doc = ""]
    pub tenant_identifier: PrimField<String>,
}

impl BuildAppfabricAppAuthorizationTenantEl {
    pub fn build(self) -> AppfabricAppAuthorizationTenantEl {
        AppfabricAppAuthorizationTenantEl {
            tenant_display_name: self.tenant_display_name,
            tenant_identifier: self.tenant_identifier,
        }
    }
}

pub struct AppfabricAppAuthorizationTenantElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppfabricAppAuthorizationTenantElRef {
    fn new(shared: StackShared, base: String) -> AppfabricAppAuthorizationTenantElRef {
        AppfabricAppAuthorizationTenantElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorizationTenantElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tenant_display_name` after provisioning.\n"]
    pub fn tenant_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenant_display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_identifier` after provisioning.\n"]
    pub fn tenant_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenant_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct AppfabricAppAuthorizationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl AppfabricAppAuthorizationTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for AppfabricAppAuthorizationTimeoutsEl {
    type O = BlockAssignable<AppfabricAppAuthorizationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorizationTimeoutsEl {}

impl BuildAppfabricAppAuthorizationTimeoutsEl {
    pub fn build(self) -> AppfabricAppAuthorizationTimeoutsEl {
        AppfabricAppAuthorizationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AppfabricAppAuthorizationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppfabricAppAuthorizationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AppfabricAppAuthorizationTimeoutsElRef {
        AppfabricAppAuthorizationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorizationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppfabricAppAuthorizationDynamic {
    credential: Option<DynamicBlock<AppfabricAppAuthorizationCredentialEl>>,
    tenant: Option<DynamicBlock<AppfabricAppAuthorizationTenantEl>>,
}
