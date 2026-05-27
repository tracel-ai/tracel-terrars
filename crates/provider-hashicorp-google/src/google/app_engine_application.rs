use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppEngineApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serving_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_settings: Option<Vec<AppEngineApplicationFeatureSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iap: Option<Vec<AppEngineApplicationIapEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AppEngineApplicationTimeoutsEl>,
    dynamic: AppEngineApplicationDynamic,
}
struct AppEngineApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppEngineApplicationData>,
}
#[derive(Clone)]
pub struct AppEngineApplication(Rc<AppEngineApplication_>);
impl AppEngineApplication {
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
    #[doc = "Set the field `auth_domain`.\nThe domain to authenticate users with when using App Engine's User API."]
    pub fn set_auth_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auth_domain = Some(v.into());
        self
    }
    #[doc = "Set the field `database_type`.\n"]
    pub fn set_database_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database_type = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project ID to create the application under."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `serving_status`.\nThe serving status of the app."]
    pub fn set_serving_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().serving_status = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_policy`.\nThe SSL policy that will be applied to the application. If set to Modern it will restrict traffic with TLS \\u003c 1.2 and allow only Modern Ciphers suite"]
    pub fn set_ssl_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ssl_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `feature_settings`.\n"]
    pub fn set_feature_settings(
        self,
        v: impl Into<BlockAssignable<AppEngineApplicationFeatureSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().feature_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.feature_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `iap`.\n"]
    pub fn set_iap(self, v: impl Into<BlockAssignable<AppEngineApplicationIapEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iap = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iap = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AppEngineApplicationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nIdentifier of the app."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_domain` after provisioning.\nThe domain to authenticate users with when using App Engine's User API."]
    pub fn auth_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `code_bucket` after provisioning.\nThe GCS bucket code is being stored in for this app."]
    pub fn code_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.code_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_type` after provisioning.\n"]
    pub fn database_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_bucket` after provisioning.\nThe GCS bucket content is being stored in for this app."]
    pub fn default_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_hostname` after provisioning.\nThe default hostname for this app."]
    pub fn default_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_hostname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcr_domain` after provisioning.\nThe GCR domain used for storing managed Docker images for this app."]
    pub fn gcr_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcr_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location_id` after provisioning.\nThe location to serve the app from."]
    pub fn location_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name of the app."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project ID to create the application under."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serving_status` after provisioning.\nThe serving status of the app."]
    pub fn serving_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serving_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\nThe SSL policy that will be applied to the application. If set to Modern it will restrict traffic with TLS \\u003c 1.2 and allow only Modern Ciphers suite"]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url_dispatch_rule` after provisioning.\nA list of dispatch rule blocks. Each block has a domain, path, and service field."]
    pub fn url_dispatch_rule(&self) -> ListRef<AppEngineApplicationUrlDispatchRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.url_dispatch_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feature_settings` after provisioning.\n"]
    pub fn feature_settings(&self) -> ListRef<AppEngineApplicationFeatureSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.feature_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iap` after provisioning.\n"]
    pub fn iap(&self) -> ListRef<AppEngineApplicationIapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineApplicationTimeoutsElRef {
        AppEngineApplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AppEngineApplication {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppEngineApplication {}
impl ToListMappable for AppEngineApplication {
    type O = ListRef<AppEngineApplicationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppEngineApplication_ {
    fn extract_resource_type(&self) -> String {
        "google_app_engine_application".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppEngineApplication {
    pub tf_id: String,
    #[doc = "The location to serve the app from."]
    pub location_id: PrimField<String>,
}
impl BuildAppEngineApplication {
    pub fn build(self, stack: &mut Stack) -> AppEngineApplication {
        let out = AppEngineApplication(Rc::new(AppEngineApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppEngineApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auth_domain: core::default::Default::default(),
                database_type: core::default::Default::default(),
                id: core::default::Default::default(),
                location_id: self.location_id,
                project: core::default::Default::default(),
                serving_status: core::default::Default::default(),
                ssl_policy: core::default::Default::default(),
                feature_settings: core::default::Default::default(),
                iap: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AppEngineApplicationRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppEngineApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nIdentifier of the app."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_domain` after provisioning.\nThe domain to authenticate users with when using App Engine's User API."]
    pub fn auth_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `code_bucket` after provisioning.\nThe GCS bucket code is being stored in for this app."]
    pub fn code_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.code_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_type` after provisioning.\n"]
    pub fn database_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_bucket` after provisioning.\nThe GCS bucket content is being stored in for this app."]
    pub fn default_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_hostname` after provisioning.\nThe default hostname for this app."]
    pub fn default_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_hostname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcr_domain` after provisioning.\nThe GCR domain used for storing managed Docker images for this app."]
    pub fn gcr_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcr_domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location_id` after provisioning.\nThe location to serve the app from."]
    pub fn location_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name of the app."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project ID to create the application under."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serving_status` after provisioning.\nThe serving status of the app."]
    pub fn serving_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serving_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\nThe SSL policy that will be applied to the application. If set to Modern it will restrict traffic with TLS \\u003c 1.2 and allow only Modern Ciphers suite"]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url_dispatch_rule` after provisioning.\nA list of dispatch rule blocks. Each block has a domain, path, and service field."]
    pub fn url_dispatch_rule(&self) -> ListRef<AppEngineApplicationUrlDispatchRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.url_dispatch_rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feature_settings` after provisioning.\n"]
    pub fn feature_settings(&self) -> ListRef<AppEngineApplicationFeatureSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.feature_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iap` after provisioning.\n"]
    pub fn iap(&self) -> ListRef<AppEngineApplicationIapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineApplicationTimeoutsElRef {
        AppEngineApplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppEngineApplicationUrlDispatchRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl AppEngineApplicationUrlDispatchRuleEl {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineApplicationUrlDispatchRuleEl {
    type O = BlockAssignable<AppEngineApplicationUrlDispatchRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineApplicationUrlDispatchRuleEl {}
impl BuildAppEngineApplicationUrlDispatchRuleEl {
    pub fn build(self) -> AppEngineApplicationUrlDispatchRuleEl {
        AppEngineApplicationUrlDispatchRuleEl {
            domain: core::default::Default::default(),
            path: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct AppEngineApplicationUrlDispatchRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineApplicationUrlDispatchRuleElRef {
    fn new(shared: StackShared, base: String) -> AppEngineApplicationUrlDispatchRuleElRef {
        AppEngineApplicationUrlDispatchRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineApplicationUrlDispatchRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct AppEngineApplicationFeatureSettingsEl {
    split_health_checks: PrimField<bool>,
}
impl AppEngineApplicationFeatureSettingsEl {}
impl ToListMappable for AppEngineApplicationFeatureSettingsEl {
    type O = BlockAssignable<AppEngineApplicationFeatureSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineApplicationFeatureSettingsEl {
    #[doc = ""]
    pub split_health_checks: PrimField<bool>,
}
impl BuildAppEngineApplicationFeatureSettingsEl {
    pub fn build(self) -> AppEngineApplicationFeatureSettingsEl {
        AppEngineApplicationFeatureSettingsEl {
            split_health_checks: self.split_health_checks,
        }
    }
}
pub struct AppEngineApplicationFeatureSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineApplicationFeatureSettingsElRef {
    fn new(shared: StackShared, base: String) -> AppEngineApplicationFeatureSettingsElRef {
        AppEngineApplicationFeatureSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineApplicationFeatureSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `split_health_checks` after provisioning.\n"]
    pub fn split_health_checks(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.split_health_checks", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppEngineApplicationIapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    oauth2_client_id: PrimField<String>,
    oauth2_client_secret: PrimField<String>,
}
impl AppEngineApplicationIapEl {
    #[doc = "Set the field `enabled`.\nAdapted for use with the app"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineApplicationIapEl {
    type O = BlockAssignable<AppEngineApplicationIapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineApplicationIapEl {
    #[doc = "OAuth2 client ID to use for the authentication flow."]
    pub oauth2_client_id: PrimField<String>,
    #[doc = "OAuth2 client secret to use for the authentication flow. The SHA-256 hash of the value is returned in the oauth2ClientSecretSha256 field."]
    pub oauth2_client_secret: PrimField<String>,
}
impl BuildAppEngineApplicationIapEl {
    pub fn build(self) -> AppEngineApplicationIapEl {
        AppEngineApplicationIapEl {
            enabled: core::default::Default::default(),
            oauth2_client_id: self.oauth2_client_id,
            oauth2_client_secret: self.oauth2_client_secret,
        }
    }
}
pub struct AppEngineApplicationIapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineApplicationIapElRef {
    fn new(shared: StackShared, base: String) -> AppEngineApplicationIapElRef {
        AppEngineApplicationIapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineApplicationIapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nAdapted for use with the app"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth2_client_id` after provisioning.\nOAuth2 client ID to use for the authentication flow."]
    pub fn oauth2_client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth2_client_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth2_client_secret` after provisioning.\nOAuth2 client secret to use for the authentication flow. The SHA-256 hash of the value is returned in the oauth2ClientSecretSha256 field."]
    pub fn oauth2_client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth2_client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth2_client_secret_sha256` after provisioning.\nHex-encoded SHA-256 hash of the client secret."]
    pub fn oauth2_client_secret_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth2_client_secret_sha256", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppEngineApplicationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AppEngineApplicationTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineApplicationTimeoutsEl {
    type O = BlockAssignable<AppEngineApplicationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineApplicationTimeoutsEl {}
impl BuildAppEngineApplicationTimeoutsEl {
    pub fn build(self) -> AppEngineApplicationTimeoutsEl {
        AppEngineApplicationTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AppEngineApplicationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineApplicationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AppEngineApplicationTimeoutsElRef {
        AppEngineApplicationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineApplicationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppEngineApplicationDynamic {
    feature_settings: Option<DynamicBlock<AppEngineApplicationFeatureSettingsEl>>,
    iap: Option<DynamicBlock<AppEngineApplicationIapEl>>,
}
