use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IapSettingsData {
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
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_settings: Option<Vec<IapSettingsAccessSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_settings: Option<Vec<IapSettingsApplicationSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IapSettingsTimeoutsEl>,
    dynamic: IapSettingsDynamic,
}
struct IapSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IapSettingsData>,
}
#[derive(Clone)]
pub struct IapSettings(Rc<IapSettings_>);
impl IapSettings {
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
    #[doc = "Set the field `access_settings`.\n"]
    pub fn set_access_settings(
        self,
        v: impl Into<BlockAssignable<IapSettingsAccessSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `application_settings`.\n"]
    pub fn set_application_settings(
        self,
        v: impl Into<BlockAssignable<IapSettingsApplicationSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().application_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.application_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IapSettingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the IAP protected resource. Name can have below resources:\n* organizations/{organization_id}\n* folders/{folder_id}\n* projects/{project_id}\n* projects/{project_id}/iap_web\n* projects/{project_id}/iap_web/compute\n* projects/{project_id}/iap_web/compute-{region}\n* projects/{project_id}/iap_web/compute/services/{service_id}\n* projects/{project_id}/iap_web/compute-{region}/services/{service_id}\n* projects/{project_id}/iap_web/appengine-{app_id}\n* projects/{project_id}/iap_web/appengine-{app_id}/services/{service_id}\n* projects/{project_id}/iap_web/appengine-{app_id}/services/{service_id}/version/{version_id}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_settings` after provisioning.\n"]
    pub fn access_settings(&self) -> ListRef<IapSettingsAccessSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `application_settings` after provisioning.\n"]
    pub fn application_settings(&self) -> ListRef<IapSettingsApplicationSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IapSettingsTimeoutsElRef {
        IapSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IapSettings {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IapSettings {}
impl ToListMappable for IapSettings {
    type O = ListRef<IapSettingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IapSettings_ {
    fn extract_resource_type(&self) -> String {
        "google_iap_settings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIapSettings {
    pub tf_id: String,
    #[doc = "The resource name of the IAP protected resource. Name can have below resources:\n* organizations/{organization_id}\n* folders/{folder_id}\n* projects/{project_id}\n* projects/{project_id}/iap_web\n* projects/{project_id}/iap_web/compute\n* projects/{project_id}/iap_web/compute-{region}\n* projects/{project_id}/iap_web/compute/services/{service_id}\n* projects/{project_id}/iap_web/compute-{region}/services/{service_id}\n* projects/{project_id}/iap_web/appengine-{app_id}\n* projects/{project_id}/iap_web/appengine-{app_id}/services/{service_id}\n* projects/{project_id}/iap_web/appengine-{app_id}/services/{service_id}/version/{version_id}"]
    pub name: PrimField<String>,
}
impl BuildIapSettings {
    pub fn build(self, stack: &mut Stack) -> IapSettings {
        let out = IapSettings(Rc::new(IapSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IapSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                access_settings: core::default::Default::default(),
                application_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IapSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IapSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the IAP protected resource. Name can have below resources:\n* organizations/{organization_id}\n* folders/{folder_id}\n* projects/{project_id}\n* projects/{project_id}/iap_web\n* projects/{project_id}/iap_web/compute\n* projects/{project_id}/iap_web/compute-{region}\n* projects/{project_id}/iap_web/compute/services/{service_id}\n* projects/{project_id}/iap_web/compute-{region}/services/{service_id}\n* projects/{project_id}/iap_web/appengine-{app_id}\n* projects/{project_id}/iap_web/appengine-{app_id}/services/{service_id}\n* projects/{project_id}/iap_web/appengine-{app_id}/services/{service_id}/version/{version_id}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_settings` after provisioning.\n"]
    pub fn access_settings(&self) -> ListRef<IapSettingsAccessSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `application_settings` after provisioning.\n"]
    pub fn application_settings(&self) -> ListRef<IapSettingsApplicationSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IapSettingsTimeoutsElRef {
        IapSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IapSettingsAccessSettingsElAllowedDomainsSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domains: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
}
impl IapSettingsAccessSettingsElAllowedDomainsSettingsEl {
    #[doc = "Set the field `domains`.\nList of trusted domains."]
    pub fn set_domains(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.domains = Some(v.into());
        self
    }
    #[doc = "Set the field `enable`.\nConfiguration for customers to opt in for the feature."]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }
}
impl ToListMappable for IapSettingsAccessSettingsElAllowedDomainsSettingsEl {
    type O = BlockAssignable<IapSettingsAccessSettingsElAllowedDomainsSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsAccessSettingsElAllowedDomainsSettingsEl {}
impl BuildIapSettingsAccessSettingsElAllowedDomainsSettingsEl {
    pub fn build(self) -> IapSettingsAccessSettingsElAllowedDomainsSettingsEl {
        IapSettingsAccessSettingsElAllowedDomainsSettingsEl {
            domains: core::default::Default::default(),
            enable: core::default::Default::default(),
        }
    }
}
pub struct IapSettingsAccessSettingsElAllowedDomainsSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsAccessSettingsElAllowedDomainsSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IapSettingsAccessSettingsElAllowedDomainsSettingsElRef {
        IapSettingsAccessSettingsElAllowedDomainsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsAccessSettingsElAllowedDomainsSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domains` after provisioning.\nList of trusted domains."]
    pub fn domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\nConfiguration for customers to opt in for the feature."]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
}
#[derive(Serialize)]
pub struct IapSettingsAccessSettingsElCorsSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_http_options: Option<PrimField<bool>>,
}
impl IapSettingsAccessSettingsElCorsSettingsEl {
    #[doc = "Set the field `allow_http_options`.\nConfiguration to allow HTTP OPTIONS calls to skip authorization.\nIf undefined, IAP will not apply any special logic to OPTIONS requests."]
    pub fn set_allow_http_options(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_http_options = Some(v.into());
        self
    }
}
impl ToListMappable for IapSettingsAccessSettingsElCorsSettingsEl {
    type O = BlockAssignable<IapSettingsAccessSettingsElCorsSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsAccessSettingsElCorsSettingsEl {}
impl BuildIapSettingsAccessSettingsElCorsSettingsEl {
    pub fn build(self) -> IapSettingsAccessSettingsElCorsSettingsEl {
        IapSettingsAccessSettingsElCorsSettingsEl {
            allow_http_options: core::default::Default::default(),
        }
    }
}
pub struct IapSettingsAccessSettingsElCorsSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsAccessSettingsElCorsSettingsElRef {
    fn new(shared: StackShared, base: String) -> IapSettingsAccessSettingsElCorsSettingsElRef {
        IapSettingsAccessSettingsElCorsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsAccessSettingsElCorsSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_http_options` after provisioning.\nConfiguration to allow HTTP OPTIONS calls to skip authorization.\nIf undefined, IAP will not apply any special logic to OPTIONS requests."]
    pub fn allow_http_options(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_http_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IapSettingsAccessSettingsElGcipSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    login_page_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tenant_ids: Option<ListField<PrimField<String>>>,
}
impl IapSettingsAccessSettingsElGcipSettingsEl {
    #[doc = "Set the field `login_page_uri`.\nLogin page URI associated with the GCIP tenants. Typically, all resources within\nthe same project share the same login page, though it could be overridden at the\nsub resource level."]
    pub fn set_login_page_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.login_page_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `tenant_ids`.\nGCIP tenant ids that are linked to the IAP resource. tenantIds could be a string\nbeginning with a number character to indicate authenticating with GCIP tenant flow,\nor in the format of _ to indicate authenticating with GCIP agent flow. If agent flow\nis used, tenantIds should only contain one single element, while for tenant flow,\ntenantIds can contain multiple elements."]
    pub fn set_tenant_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tenant_ids = Some(v.into());
        self
    }
}
impl ToListMappable for IapSettingsAccessSettingsElGcipSettingsEl {
    type O = BlockAssignable<IapSettingsAccessSettingsElGcipSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsAccessSettingsElGcipSettingsEl {}
impl BuildIapSettingsAccessSettingsElGcipSettingsEl {
    pub fn build(self) -> IapSettingsAccessSettingsElGcipSettingsEl {
        IapSettingsAccessSettingsElGcipSettingsEl {
            login_page_uri: core::default::Default::default(),
            tenant_ids: core::default::Default::default(),
        }
    }
}
pub struct IapSettingsAccessSettingsElGcipSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsAccessSettingsElGcipSettingsElRef {
    fn new(shared: StackShared, base: String) -> IapSettingsAccessSettingsElGcipSettingsElRef {
        IapSettingsAccessSettingsElGcipSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsAccessSettingsElGcipSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `login_page_uri` after provisioning.\nLogin page URI associated with the GCIP tenants. Typically, all resources within\nthe same project share the same login page, though it could be overridden at the\nsub resource level."]
    pub fn login_page_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.login_page_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tenant_ids` after provisioning.\nGCIP tenant ids that are linked to the IAP resource. tenantIds could be a string\nbeginning with a number character to indicate authenticating with GCIP tenant flow,\nor in the format of _ to indicate authenticating with GCIP agent flow. If agent flow\nis used, tenantIds should only contain one single element, while for tenant flow,\ntenantIds can contain multiple elements."]
    pub fn tenant_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tenant_ids", self.base))
    }
}
#[derive(Serialize)]
pub struct IapSettingsAccessSettingsElOauthSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_hint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    programmatic_clients: Option<ListField<PrimField<String>>>,
}
impl IapSettingsAccessSettingsElOauthSettingsEl {
    #[doc = "Set the field `client_id`.\nOAuth 2.0 client ID used in the OAuth flow to generate an access token. If this field is set, you can skip obtaining the OAuth credentials in this."]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `client_secret`.\nOAuth secret paired with client ID."]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `login_hint`.\nDomain hint to send as hd=? parameter in OAuth request flow.\nEnables redirect to primary IDP by skipping Google's login screen.\n(https://developers.google.com/identity/protocols/OpenIDConnect#hd-param)\nNote: IAP does not verify that the id token's hd claim matches this value\nsince access behavior is managed by IAM policies.\n* loginHint setting is not a replacement for access control. Always enforce an appropriate access policy if you want to restrict access to users outside your domain."]
    pub fn set_login_hint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.login_hint = Some(v.into());
        self
    }
    #[doc = "Set the field `programmatic_clients`.\nList of client ids allowed to use IAP programmatically."]
    pub fn set_programmatic_clients(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.programmatic_clients = Some(v.into());
        self
    }
}
impl ToListMappable for IapSettingsAccessSettingsElOauthSettingsEl {
    type O = BlockAssignable<IapSettingsAccessSettingsElOauthSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsAccessSettingsElOauthSettingsEl {}
impl BuildIapSettingsAccessSettingsElOauthSettingsEl {
    pub fn build(self) -> IapSettingsAccessSettingsElOauthSettingsEl {
        IapSettingsAccessSettingsElOauthSettingsEl {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            login_hint: core::default::Default::default(),
            programmatic_clients: core::default::Default::default(),
        }
    }
}
pub struct IapSettingsAccessSettingsElOauthSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsAccessSettingsElOauthSettingsElRef {
    fn new(shared: StackShared, base: String) -> IapSettingsAccessSettingsElOauthSettingsElRef {
        IapSettingsAccessSettingsElOauthSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsAccessSettingsElOauthSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nOAuth 2.0 client ID used in the OAuth flow to generate an access token. If this field is set, you can skip obtaining the OAuth credentials in this."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nOAuth secret paired with client ID."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_secret_sha256` after provisioning.\nOAuth secret sha256 paired with client ID."]
    pub fn client_secret_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret_sha256", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `login_hint` after provisioning.\nDomain hint to send as hd=? parameter in OAuth request flow.\nEnables redirect to primary IDP by skipping Google's login screen.\n(https://developers.google.com/identity/protocols/OpenIDConnect#hd-param)\nNote: IAP does not verify that the id token's hd claim matches this value\nsince access behavior is managed by IAM policies.\n* loginHint setting is not a replacement for access control. Always enforce an appropriate access policy if you want to restrict access to users outside your domain."]
    pub fn login_hint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login_hint", self.base))
    }
    #[doc = "Get a reference to the value of field `programmatic_clients` after provisioning.\nList of client ids allowed to use IAP programmatically."]
    pub fn programmatic_clients(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.programmatic_clients", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IapSettingsAccessSettingsElReauthSettingsEl {
    max_age: PrimField<String>,
    method: PrimField<String>,
    policy_type: PrimField<String>,
}
impl IapSettingsAccessSettingsElReauthSettingsEl {}
impl ToListMappable for IapSettingsAccessSettingsElReauthSettingsEl {
    type O = BlockAssignable<IapSettingsAccessSettingsElReauthSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsAccessSettingsElReauthSettingsEl {
    #[doc = "Reauth session lifetime, how long before a user has to reauthenticate again.\nA duration in seconds with up to nine fractional digits, ending with 's'.\nExample: \"3.5s\"."]
    pub max_age: PrimField<String>,
    #[doc = "Reauth method requested. The possible values are:\n\n* 'LOGIN': Prompts the user to log in again.\n* 'SECURE_KEY': User must use their secure key 2nd factor device.\n* 'ENROLLED_SECOND_FACTORS': User can use any enabled 2nd factor. Possible values: [\"LOGIN\", \"SECURE_KEY\", \"ENROLLED_SECOND_FACTORS\"]"]
    pub method: PrimField<String>,
    #[doc = "How IAP determines the effective policy in cases of hierarchical policies.\nPolicies are merged from higher in the hierarchy to lower in the hierarchy.\nThe possible values are:\n\n* 'MINIMUM': This policy acts as a minimum to other policies, lower in the hierarchy.\n\t\t   Effective policy may only be the same or stricter.\n* 'DEFAULT': This policy acts as a default if no other reauth policy is set. Possible values: [\"MINIMUM\", \"DEFAULT\"]"]
    pub policy_type: PrimField<String>,
}
impl BuildIapSettingsAccessSettingsElReauthSettingsEl {
    pub fn build(self) -> IapSettingsAccessSettingsElReauthSettingsEl {
        IapSettingsAccessSettingsElReauthSettingsEl {
            max_age: self.max_age,
            method: self.method,
            policy_type: self.policy_type,
        }
    }
}
pub struct IapSettingsAccessSettingsElReauthSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsAccessSettingsElReauthSettingsElRef {
    fn new(shared: StackShared, base: String) -> IapSettingsAccessSettingsElReauthSettingsElRef {
        IapSettingsAccessSettingsElReauthSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsAccessSettingsElReauthSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_age` after provisioning.\nReauth session lifetime, how long before a user has to reauthenticate again.\nA duration in seconds with up to nine fractional digits, ending with 's'.\nExample: \"3.5s\"."]
    pub fn max_age(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
    #[doc = "Get a reference to the value of field `method` after provisioning.\nReauth method requested. The possible values are:\n\n* 'LOGIN': Prompts the user to log in again.\n* 'SECURE_KEY': User must use their secure key 2nd factor device.\n* 'ENROLLED_SECOND_FACTORS': User can use any enabled 2nd factor. Possible values: [\"LOGIN\", \"SECURE_KEY\", \"ENROLLED_SECOND_FACTORS\"]"]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }
    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\nHow IAP determines the effective policy in cases of hierarchical policies.\nPolicies are merged from higher in the hierarchy to lower in the hierarchy.\nThe possible values are:\n\n* 'MINIMUM': This policy acts as a minimum to other policies, lower in the hierarchy.\n\t\t   Effective policy may only be the same or stricter.\n* 'DEFAULT': This policy acts as a default if no other reauth policy is set. Possible values: [\"MINIMUM\", \"DEFAULT\"]"]
    pub fn policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.base))
    }
}
#[derive(Serialize)]
pub struct IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2El {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
}
impl IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2El {
    #[doc = "Set the field `client_id`.\nThe OAuth 2.0 client ID registered in the workforce identity\nfederation OAuth 2.0 Server."]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `client_secret`.\nInput only. The OAuth 2.0 client secret created while registering\nthe client ID."]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }
}
impl ToListMappable for IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2El {
    type O = BlockAssignable<IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2El {}
impl BuildIapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2El {
    pub fn build(self) -> IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2El {
        IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2El {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
        }
    }
}
pub struct IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2ElRef {
        IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe OAuth 2.0 client ID registered in the workforce identity\nfederation OAuth 2.0 Server."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nInput only. The OAuth 2.0 client secret created while registering\nthe client ID."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_secret_sha256` after provisioning.\nOutput only. SHA256 hash value for the client secret. This field\nis returned by IAP when the settings are retrieved."]
    pub fn client_secret_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret_sha256", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IapSettingsAccessSettingsElWorkforceIdentitySettingsElDynamic {
    oauth2: Option<DynamicBlock<IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2El>>,
}
#[derive(Serialize)]
pub struct IapSettingsAccessSettingsElWorkforceIdentitySettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    workforce_pools: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2: Option<Vec<IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2El>>,
    dynamic: IapSettingsAccessSettingsElWorkforceIdentitySettingsElDynamic,
}
impl IapSettingsAccessSettingsElWorkforceIdentitySettingsEl {
    #[doc = "Set the field `workforce_pools`.\nThe workforce pool resources. Only one workforce pool is accepted."]
    pub fn set_workforce_pools(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.workforce_pools = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth2`.\n"]
    pub fn set_oauth2(
        mut self,
        v: impl Into<BlockAssignable<IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth2 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2 = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IapSettingsAccessSettingsElWorkforceIdentitySettingsEl {
    type O = BlockAssignable<IapSettingsAccessSettingsElWorkforceIdentitySettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsAccessSettingsElWorkforceIdentitySettingsEl {}
impl BuildIapSettingsAccessSettingsElWorkforceIdentitySettingsEl {
    pub fn build(self) -> IapSettingsAccessSettingsElWorkforceIdentitySettingsEl {
        IapSettingsAccessSettingsElWorkforceIdentitySettingsEl {
            workforce_pools: core::default::Default::default(),
            oauth2: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IapSettingsAccessSettingsElWorkforceIdentitySettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsAccessSettingsElWorkforceIdentitySettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IapSettingsAccessSettingsElWorkforceIdentitySettingsElRef {
        IapSettingsAccessSettingsElWorkforceIdentitySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsAccessSettingsElWorkforceIdentitySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `workforce_pools` after provisioning.\nThe workforce pool resources. Only one workforce pool is accepted."]
    pub fn workforce_pools(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workforce_pools", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth2` after provisioning.\n"]
    pub fn oauth2(
        &self,
    ) -> ListRef<IapSettingsAccessSettingsElWorkforceIdentitySettingsElOauth2ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth2", self.base))
    }
}
#[derive(Serialize, Default)]
struct IapSettingsAccessSettingsElDynamic {
    allowed_domains_settings:
        Option<DynamicBlock<IapSettingsAccessSettingsElAllowedDomainsSettingsEl>>,
    cors_settings: Option<DynamicBlock<IapSettingsAccessSettingsElCorsSettingsEl>>,
    gcip_settings: Option<DynamicBlock<IapSettingsAccessSettingsElGcipSettingsEl>>,
    oauth_settings: Option<DynamicBlock<IapSettingsAccessSettingsElOauthSettingsEl>>,
    reauth_settings: Option<DynamicBlock<IapSettingsAccessSettingsElReauthSettingsEl>>,
    workforce_identity_settings:
        Option<DynamicBlock<IapSettingsAccessSettingsElWorkforceIdentitySettingsEl>>,
}
#[derive(Serialize)]
pub struct IapSettingsAccessSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_sources: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_domains_settings: Option<Vec<IapSettingsAccessSettingsElAllowedDomainsSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_settings: Option<Vec<IapSettingsAccessSettingsElCorsSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcip_settings: Option<Vec<IapSettingsAccessSettingsElGcipSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_settings: Option<Vec<IapSettingsAccessSettingsElOauthSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reauth_settings: Option<Vec<IapSettingsAccessSettingsElReauthSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workforce_identity_settings:
        Option<Vec<IapSettingsAccessSettingsElWorkforceIdentitySettingsEl>>,
    dynamic: IapSettingsAccessSettingsElDynamic,
}
impl IapSettingsAccessSettingsEl {
    #[doc = "Set the field `identity_sources`.\nIdentity sources that IAP can use to authenticate the end user. Only one identity source\ncan be configured. The possible values are:\n\n* 'WORKFORCE_IDENTITY_FEDERATION': Use external identities set up on Google Cloud Workforce\n  \t\t\t\t     Identity Federation. Possible values: [\"WORKFORCE_IDENTITY_FEDERATION\"]"]
    pub fn set_identity_sources(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.identity_sources = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_domains_settings`.\n"]
    pub fn set_allowed_domains_settings(
        mut self,
        v: impl Into<BlockAssignable<IapSettingsAccessSettingsElAllowedDomainsSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allowed_domains_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_domains_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cors_settings`.\n"]
    pub fn set_cors_settings(
        mut self,
        v: impl Into<BlockAssignable<IapSettingsAccessSettingsElCorsSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cors_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcip_settings`.\n"]
    pub fn set_gcip_settings(
        mut self,
        v: impl Into<BlockAssignable<IapSettingsAccessSettingsElGcipSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcip_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcip_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oauth_settings`.\n"]
    pub fn set_oauth_settings(
        mut self,
        v: impl Into<BlockAssignable<IapSettingsAccessSettingsElOauthSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `reauth_settings`.\n"]
    pub fn set_reauth_settings(
        mut self,
        v: impl Into<BlockAssignable<IapSettingsAccessSettingsElReauthSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.reauth_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.reauth_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `workforce_identity_settings`.\n"]
    pub fn set_workforce_identity_settings(
        mut self,
        v: impl Into<BlockAssignable<IapSettingsAccessSettingsElWorkforceIdentitySettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.workforce_identity_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.workforce_identity_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IapSettingsAccessSettingsEl {
    type O = BlockAssignable<IapSettingsAccessSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsAccessSettingsEl {}
impl BuildIapSettingsAccessSettingsEl {
    pub fn build(self) -> IapSettingsAccessSettingsEl {
        IapSettingsAccessSettingsEl {
            identity_sources: core::default::Default::default(),
            allowed_domains_settings: core::default::Default::default(),
            cors_settings: core::default::Default::default(),
            gcip_settings: core::default::Default::default(),
            oauth_settings: core::default::Default::default(),
            reauth_settings: core::default::Default::default(),
            workforce_identity_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IapSettingsAccessSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsAccessSettingsElRef {
    fn new(shared: StackShared, base: String) -> IapSettingsAccessSettingsElRef {
        IapSettingsAccessSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsAccessSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identity_sources` after provisioning.\nIdentity sources that IAP can use to authenticate the end user. Only one identity source\ncan be configured. The possible values are:\n\n* 'WORKFORCE_IDENTITY_FEDERATION': Use external identities set up on Google Cloud Workforce\n  \t\t\t\t     Identity Federation. Possible values: [\"WORKFORCE_IDENTITY_FEDERATION\"]"]
    pub fn identity_sources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_sources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_domains_settings` after provisioning.\n"]
    pub fn allowed_domains_settings(
        &self,
    ) -> ListRef<IapSettingsAccessSettingsElAllowedDomainsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_domains_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cors_settings` after provisioning.\n"]
    pub fn cors_settings(&self) -> ListRef<IapSettingsAccessSettingsElCorsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cors_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcip_settings` after provisioning.\n"]
    pub fn gcip_settings(&self) -> ListRef<IapSettingsAccessSettingsElGcipSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcip_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_settings` after provisioning.\n"]
    pub fn oauth_settings(&self) -> ListRef<IapSettingsAccessSettingsElOauthSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reauth_settings` after provisioning.\n"]
    pub fn reauth_settings(&self) -> ListRef<IapSettingsAccessSettingsElReauthSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reauth_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workforce_identity_settings` after provisioning.\n"]
    pub fn workforce_identity_settings(
        &self,
    ) -> ListRef<IapSettingsAccessSettingsElWorkforceIdentitySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workforce_identity_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IapSettingsApplicationSettingsElAccessDeniedPageSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_denied_page_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generate_troubleshooting_uri: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remediation_token_generation_enabled: Option<PrimField<bool>>,
}
impl IapSettingsApplicationSettingsElAccessDeniedPageSettingsEl {
    #[doc = "Set the field `access_denied_page_uri`.\nThe URI to be redirected to when access is denied."]
    pub fn set_access_denied_page_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_denied_page_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `generate_troubleshooting_uri`.\nWhether to generate a troubleshooting URL on access denied events to this application."]
    pub fn set_generate_troubleshooting_uri(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.generate_troubleshooting_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `remediation_token_generation_enabled`.\nWhether to generate remediation token on access denied events to this application."]
    pub fn set_remediation_token_generation_enabled(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.remediation_token_generation_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for IapSettingsApplicationSettingsElAccessDeniedPageSettingsEl {
    type O = BlockAssignable<IapSettingsApplicationSettingsElAccessDeniedPageSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsApplicationSettingsElAccessDeniedPageSettingsEl {}
impl BuildIapSettingsApplicationSettingsElAccessDeniedPageSettingsEl {
    pub fn build(self) -> IapSettingsApplicationSettingsElAccessDeniedPageSettingsEl {
        IapSettingsApplicationSettingsElAccessDeniedPageSettingsEl {
            access_denied_page_uri: core::default::Default::default(),
            generate_troubleshooting_uri: core::default::Default::default(),
            remediation_token_generation_enabled: core::default::Default::default(),
        }
    }
}
pub struct IapSettingsApplicationSettingsElAccessDeniedPageSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsApplicationSettingsElAccessDeniedPageSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IapSettingsApplicationSettingsElAccessDeniedPageSettingsElRef {
        IapSettingsApplicationSettingsElAccessDeniedPageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsApplicationSettingsElAccessDeniedPageSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_denied_page_uri` after provisioning.\nThe URI to be redirected to when access is denied."]
    pub fn access_denied_page_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_denied_page_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `generate_troubleshooting_uri` after provisioning.\nWhether to generate a troubleshooting URL on access denied events to this application."]
    pub fn generate_troubleshooting_uri(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generate_troubleshooting_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `remediation_token_generation_enabled` after provisioning.\nWhether to generate remediation token on access denied events to this application."]
    pub fn remediation_token_generation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.remediation_token_generation_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IapSettingsApplicationSettingsElAttributePropagationSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_credentials: Option<ListField<PrimField<String>>>,
}
impl IapSettingsApplicationSettingsElAttributePropagationSettingsEl {
    #[doc = "Set the field `enable`.\nWhether the provided attribute propagation settings should be evaluated on user requests.\nIf set to true, attributes returned from the expression will be propagated in the set output credentials."]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }
    #[doc = "Set the field `expression`.\nRaw string CEL expression. Must return a list of attributes. A maximum of 45 attributes can\nbe selected. Expressions can select different attribute types from attributes:\nattributes.saml_attributes, attributes.iap_attributes."]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
    #[doc = "Set the field `output_credentials`.\nWhich output credentials attributes selected by the CEL expression should be propagated in.\nAll attributes will be fully duplicated in each selected output credential.\nPossible values are:\n\n* 'HEADER': Propagate attributes in the headers with \"x-goog-iap-attr-\" prefix.\n* 'JWT': Propagate attributes in the JWT of the form:\n         \"additional_claims\": { \"my_attribute\": [\"value1\", \"value2\"] }\n* 'RCTOKEN': Propagate attributes in the RCToken of the form: \"\n             additional_claims\": { \"my_attribute\": [\"value1\", \"value2\"] } Possible values: [\"HEADER\", \"JWT\", \"RCTOKEN\"]"]
    pub fn set_output_credentials(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.output_credentials = Some(v.into());
        self
    }
}
impl ToListMappable for IapSettingsApplicationSettingsElAttributePropagationSettingsEl {
    type O = BlockAssignable<IapSettingsApplicationSettingsElAttributePropagationSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsApplicationSettingsElAttributePropagationSettingsEl {}
impl BuildIapSettingsApplicationSettingsElAttributePropagationSettingsEl {
    pub fn build(self) -> IapSettingsApplicationSettingsElAttributePropagationSettingsEl {
        IapSettingsApplicationSettingsElAttributePropagationSettingsEl {
            enable: core::default::Default::default(),
            expression: core::default::Default::default(),
            output_credentials: core::default::Default::default(),
        }
    }
}
pub struct IapSettingsApplicationSettingsElAttributePropagationSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsApplicationSettingsElAttributePropagationSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IapSettingsApplicationSettingsElAttributePropagationSettingsElRef {
        IapSettingsApplicationSettingsElAttributePropagationSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsApplicationSettingsElAttributePropagationSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\nWhether the provided attribute propagation settings should be evaluated on user requests.\nIf set to true, attributes returned from the expression will be propagated in the set output credentials."]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nRaw string CEL expression. Must return a list of attributes. A maximum of 45 attributes can\nbe selected. Expressions can select different attribute types from attributes:\nattributes.saml_attributes, attributes.iap_attributes."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `output_credentials` after provisioning.\nWhich output credentials attributes selected by the CEL expression should be propagated in.\nAll attributes will be fully duplicated in each selected output credential.\nPossible values are:\n\n* 'HEADER': Propagate attributes in the headers with \"x-goog-iap-attr-\" prefix.\n* 'JWT': Propagate attributes in the JWT of the form:\n         \"additional_claims\": { \"my_attribute\": [\"value1\", \"value2\"] }\n* 'RCTOKEN': Propagate attributes in the RCToken of the form: \"\n             additional_claims\": { \"my_attribute\": [\"value1\", \"value2\"] } Possible values: [\"HEADER\", \"JWT\", \"RCTOKEN\"]"]
    pub fn output_credentials(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_credentials", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IapSettingsApplicationSettingsElCsmSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rctoken_aud: Option<PrimField<String>>,
}
impl IapSettingsApplicationSettingsElCsmSettingsEl {
    #[doc = "Set the field `rctoken_aud`.\nAudience claim set in the generated RCToken. This value is not validated by IAP."]
    pub fn set_rctoken_aud(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rctoken_aud = Some(v.into());
        self
    }
}
impl ToListMappable for IapSettingsApplicationSettingsElCsmSettingsEl {
    type O = BlockAssignable<IapSettingsApplicationSettingsElCsmSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsApplicationSettingsElCsmSettingsEl {}
impl BuildIapSettingsApplicationSettingsElCsmSettingsEl {
    pub fn build(self) -> IapSettingsApplicationSettingsElCsmSettingsEl {
        IapSettingsApplicationSettingsElCsmSettingsEl {
            rctoken_aud: core::default::Default::default(),
        }
    }
}
pub struct IapSettingsApplicationSettingsElCsmSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsApplicationSettingsElCsmSettingsElRef {
    fn new(shared: StackShared, base: String) -> IapSettingsApplicationSettingsElCsmSettingsElRef {
        IapSettingsApplicationSettingsElCsmSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsApplicationSettingsElCsmSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rctoken_aud` after provisioning.\nAudience claim set in the generated RCToken. This value is not validated by IAP."]
    pub fn rctoken_aud(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rctoken_aud", self.base))
    }
}
#[derive(Serialize, Default)]
struct IapSettingsApplicationSettingsElDynamic {
    access_denied_page_settings:
        Option<DynamicBlock<IapSettingsApplicationSettingsElAccessDeniedPageSettingsEl>>,
    attribute_propagation_settings:
        Option<DynamicBlock<IapSettingsApplicationSettingsElAttributePropagationSettingsEl>>,
    csm_settings: Option<DynamicBlock<IapSettingsApplicationSettingsElCsmSettingsEl>>,
}
#[derive(Serialize)]
pub struct IapSettingsApplicationSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_denied_page_settings:
        Option<Vec<IapSettingsApplicationSettingsElAccessDeniedPageSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_propagation_settings:
        Option<Vec<IapSettingsApplicationSettingsElAttributePropagationSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csm_settings: Option<Vec<IapSettingsApplicationSettingsElCsmSettingsEl>>,
    dynamic: IapSettingsApplicationSettingsElDynamic,
}
impl IapSettingsApplicationSettingsEl {
    #[doc = "Set the field `cookie_domain`.\nThe Domain value to set for cookies generated by IAP. This value is not validated by the API,\nbut will be ignored at runtime if invalid."]
    pub fn set_cookie_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cookie_domain = Some(v.into());
        self
    }
    #[doc = "Set the field `access_denied_page_settings`.\n"]
    pub fn set_access_denied_page_settings(
        mut self,
        v: impl Into<BlockAssignable<IapSettingsApplicationSettingsElAccessDeniedPageSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_denied_page_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_denied_page_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `attribute_propagation_settings`.\n"]
    pub fn set_attribute_propagation_settings(
        mut self,
        v: impl Into<BlockAssignable<IapSettingsApplicationSettingsElAttributePropagationSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.attribute_propagation_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.attribute_propagation_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `csm_settings`.\n"]
    pub fn set_csm_settings(
        mut self,
        v: impl Into<BlockAssignable<IapSettingsApplicationSettingsElCsmSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.csm_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.csm_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IapSettingsApplicationSettingsEl {
    type O = BlockAssignable<IapSettingsApplicationSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsApplicationSettingsEl {}
impl BuildIapSettingsApplicationSettingsEl {
    pub fn build(self) -> IapSettingsApplicationSettingsEl {
        IapSettingsApplicationSettingsEl {
            cookie_domain: core::default::Default::default(),
            access_denied_page_settings: core::default::Default::default(),
            attribute_propagation_settings: core::default::Default::default(),
            csm_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IapSettingsApplicationSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsApplicationSettingsElRef {
    fn new(shared: StackShared, base: String) -> IapSettingsApplicationSettingsElRef {
        IapSettingsApplicationSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsApplicationSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cookie_domain` after provisioning.\nThe Domain value to set for cookies generated by IAP. This value is not validated by the API,\nbut will be ignored at runtime if invalid."]
    pub fn cookie_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cookie_domain", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `access_denied_page_settings` after provisioning.\n"]
    pub fn access_denied_page_settings(
        &self,
    ) -> ListRef<IapSettingsApplicationSettingsElAccessDeniedPageSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_denied_page_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `attribute_propagation_settings` after provisioning.\n"]
    pub fn attribute_propagation_settings(
        &self,
    ) -> ListRef<IapSettingsApplicationSettingsElAttributePropagationSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attribute_propagation_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `csm_settings` after provisioning.\n"]
    pub fn csm_settings(&self) -> ListRef<IapSettingsApplicationSettingsElCsmSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.csm_settings", self.base))
    }
}
#[derive(Serialize)]
pub struct IapSettingsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IapSettingsTimeoutsEl {
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
impl ToListMappable for IapSettingsTimeoutsEl {
    type O = BlockAssignable<IapSettingsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIapSettingsTimeoutsEl {}
impl BuildIapSettingsTimeoutsEl {
    pub fn build(self) -> IapSettingsTimeoutsEl {
        IapSettingsTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IapSettingsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IapSettingsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IapSettingsTimeoutsElRef {
        IapSettingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IapSettingsTimeoutsElRef {
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
struct IapSettingsDynamic {
    access_settings: Option<DynamicBlock<IapSettingsAccessSettingsEl>>,
    application_settings: Option<DynamicBlock<IapSettingsApplicationSettingsEl>>,
}
