use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IdentityPlatformConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorized_domains: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autodelete_anonymous_users: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocking_functions: Option<Vec<IdentityPlatformConfigBlockingFunctionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client: Option<Vec<IdentityPlatformConfigClientEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mfa: Option<Vec<IdentityPlatformConfigMfaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring: Option<Vec<IdentityPlatformConfigMonitoringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_tenant: Option<Vec<IdentityPlatformConfigMultiTenantEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota: Option<Vec<IdentityPlatformConfigQuotaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sign_in: Option<Vec<IdentityPlatformConfigSignInEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sms_region_config: Option<Vec<IdentityPlatformConfigSmsRegionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IdentityPlatformConfigTimeoutsEl>,
    dynamic: IdentityPlatformConfigDynamic,
}
struct IdentityPlatformConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IdentityPlatformConfigData>,
}
#[derive(Clone)]
pub struct IdentityPlatformConfig(Rc<IdentityPlatformConfig_>);
impl IdentityPlatformConfig {
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
    #[doc = "Set the field `authorized_domains`.\nList of domains authorized for OAuth redirects."]
    pub fn set_authorized_domains(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().authorized_domains = Some(v.into());
        self
    }
    #[doc = "Set the field `autodelete_anonymous_users`.\nWhether anonymous users will be auto-deleted after a period of 30 days"]
    pub fn set_autodelete_anonymous_users(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().autodelete_anonymous_users = Some(v.into());
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
    #[doc = "Set the field `blocking_functions`.\n"]
    pub fn set_blocking_functions(
        self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigBlockingFunctionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().blocking_functions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.blocking_functions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `client`.\n"]
    pub fn set_client(self, v: impl Into<BlockAssignable<IdentityPlatformConfigClientEl>>) -> Self {
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
    #[doc = "Set the field `mfa`.\n"]
    pub fn set_mfa(self, v: impl Into<BlockAssignable<IdentityPlatformConfigMfaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mfa = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mfa = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(
        self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigMonitoringEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `multi_tenant`.\n"]
    pub fn set_multi_tenant(
        self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigMultiTenantEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().multi_tenant = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.multi_tenant = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `quota`.\n"]
    pub fn set_quota(self, v: impl Into<BlockAssignable<IdentityPlatformConfigQuotaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().quota = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.quota = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sign_in`.\n"]
    pub fn set_sign_in(
        self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigSignInEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sign_in = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sign_in = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sms_region_config`.\n"]
    pub fn set_sms_region_config(
        self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigSmsRegionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sms_region_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sms_region_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IdentityPlatformConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `authorized_domains` after provisioning.\nList of domains authorized for OAuth redirects."]
    pub fn authorized_domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorized_domains", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autodelete_anonymous_users` after provisioning.\nWhether anonymous users will be auto-deleted after a period of 30 days"]
    pub fn autodelete_anonymous_users(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.autodelete_anonymous_users", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Config resource"]
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
    #[doc = "Get a reference to the value of field `blocking_functions` after provisioning.\n"]
    pub fn blocking_functions(&self) -> ListRef<IdentityPlatformConfigBlockingFunctionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.blocking_functions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client` after provisioning.\n"]
    pub fn client(&self) -> ListRef<IdentityPlatformConfigClientElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mfa` after provisioning.\n"]
    pub fn mfa(&self) -> ListRef<IdentityPlatformConfigMfaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mfa", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> ListRef<IdentityPlatformConfigMonitoringElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_tenant` after provisioning.\n"]
    pub fn multi_tenant(&self) -> ListRef<IdentityPlatformConfigMultiTenantElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_tenant", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota` after provisioning.\n"]
    pub fn quota(&self) -> ListRef<IdentityPlatformConfigQuotaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quota", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sign_in` after provisioning.\n"]
    pub fn sign_in(&self) -> ListRef<IdentityPlatformConfigSignInElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sign_in", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sms_region_config` after provisioning.\n"]
    pub fn sms_region_config(&self) -> ListRef<IdentityPlatformConfigSmsRegionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sms_region_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformConfigTimeoutsElRef {
        IdentityPlatformConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IdentityPlatformConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IdentityPlatformConfig {}
impl ToListMappable for IdentityPlatformConfig {
    type O = ListRef<IdentityPlatformConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IdentityPlatformConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_identity_platform_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIdentityPlatformConfig {
    pub tf_id: String,
}
impl BuildIdentityPlatformConfig {
    pub fn build(self, stack: &mut Stack) -> IdentityPlatformConfig {
        let out = IdentityPlatformConfig(Rc::new(IdentityPlatformConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IdentityPlatformConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                authorized_domains: core::default::Default::default(),
                autodelete_anonymous_users: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                blocking_functions: core::default::Default::default(),
                client: core::default::Default::default(),
                mfa: core::default::Default::default(),
                monitoring: core::default::Default::default(),
                multi_tenant: core::default::Default::default(),
                quota: core::default::Default::default(),
                sign_in: core::default::Default::default(),
                sms_region_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IdentityPlatformConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IdentityPlatformConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorized_domains` after provisioning.\nList of domains authorized for OAuth redirects."]
    pub fn authorized_domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorized_domains", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autodelete_anonymous_users` after provisioning.\nWhether anonymous users will be auto-deleted after a period of 30 days"]
    pub fn autodelete_anonymous_users(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.autodelete_anonymous_users", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Config resource"]
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
    #[doc = "Get a reference to the value of field `blocking_functions` after provisioning.\n"]
    pub fn blocking_functions(&self) -> ListRef<IdentityPlatformConfigBlockingFunctionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.blocking_functions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client` after provisioning.\n"]
    pub fn client(&self) -> ListRef<IdentityPlatformConfigClientElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mfa` after provisioning.\n"]
    pub fn mfa(&self) -> ListRef<IdentityPlatformConfigMfaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mfa", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> ListRef<IdentityPlatformConfigMonitoringElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_tenant` after provisioning.\n"]
    pub fn multi_tenant(&self) -> ListRef<IdentityPlatformConfigMultiTenantElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_tenant", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota` after provisioning.\n"]
    pub fn quota(&self) -> ListRef<IdentityPlatformConfigQuotaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quota", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sign_in` after provisioning.\n"]
    pub fn sign_in(&self) -> ListRef<IdentityPlatformConfigSignInElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sign_in", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sms_region_config` after provisioning.\n"]
    pub fn sms_region_config(&self) -> ListRef<IdentityPlatformConfigSmsRegionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sms_region_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformConfigTimeoutsElRef {
        IdentityPlatformConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_token: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<PrimField<bool>>,
}
impl IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {
    #[doc = "Set the field `access_token`.\nWhether to pass the user's OAuth identity provider's access token."]
    pub fn set_access_token(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `id_token`.\nWhether to pass the user's OIDC identity provider's ID token."]
    pub fn set_id_token(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.id_token = Some(v.into());
        self
    }
    #[doc = "Set the field `refresh_token`.\nWhether to pass the user's OAuth identity provider's refresh token."]
    pub fn set_refresh_token(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.refresh_token = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {
    type O = BlockAssignable<IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {}
impl BuildIdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {
    pub fn build(self) -> IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {
        IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {
            access_token: core::default::Default::default(),
            id_token: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsElRef {
        IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_token` after provisioning.\nWhether to pass the user's OAuth identity provider's access token."]
    pub fn access_token(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }
    #[doc = "Get a reference to the value of field `id_token` after provisioning.\nWhether to pass the user's OIDC identity provider's ID token."]
    pub fn id_token(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.id_token", self.base))
    }
    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\nWhether to pass the user's OAuth identity provider's refresh token."]
    pub fn refresh_token(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_token", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigBlockingFunctionsElTriggersEl {
    event_type: PrimField<String>,
    function_uri: PrimField<String>,
}
impl IdentityPlatformConfigBlockingFunctionsElTriggersEl {}
impl ToListMappable for IdentityPlatformConfigBlockingFunctionsElTriggersEl {
    type O = BlockAssignable<IdentityPlatformConfigBlockingFunctionsElTriggersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigBlockingFunctionsElTriggersEl {
    #[doc = ""]
    pub event_type: PrimField<String>,
    #[doc = "HTTP URI trigger for the Cloud Function."]
    pub function_uri: PrimField<String>,
}
impl BuildIdentityPlatformConfigBlockingFunctionsElTriggersEl {
    pub fn build(self) -> IdentityPlatformConfigBlockingFunctionsElTriggersEl {
        IdentityPlatformConfigBlockingFunctionsElTriggersEl {
            event_type: self.event_type,
            function_uri: self.function_uri,
        }
    }
}
pub struct IdentityPlatformConfigBlockingFunctionsElTriggersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigBlockingFunctionsElTriggersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IdentityPlatformConfigBlockingFunctionsElTriggersElRef {
        IdentityPlatformConfigBlockingFunctionsElTriggersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigBlockingFunctionsElTriggersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
    #[doc = "Get a reference to the value of field `function_uri` after provisioning.\nHTTP URI trigger for the Cloud Function."]
    pub fn function_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nWhen the trigger was changed."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct IdentityPlatformConfigBlockingFunctionsElDynamic {
    forward_inbound_credentials:
        Option<DynamicBlock<IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl>>,
    triggers: Option<DynamicBlock<IdentityPlatformConfigBlockingFunctionsElTriggersEl>>,
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigBlockingFunctionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_inbound_credentials:
        Option<Vec<IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    triggers: Option<Vec<IdentityPlatformConfigBlockingFunctionsElTriggersEl>>,
    dynamic: IdentityPlatformConfigBlockingFunctionsElDynamic,
}
impl IdentityPlatformConfigBlockingFunctionsEl {
    #[doc = "Set the field `forward_inbound_credentials`.\n"]
    pub fn set_forward_inbound_credentials(
        mut self,
        v: impl Into<
            BlockAssignable<IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forward_inbound_credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forward_inbound_credentials = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `triggers`.\n"]
    pub fn set_triggers(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigBlockingFunctionsElTriggersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.triggers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.triggers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IdentityPlatformConfigBlockingFunctionsEl {
    type O = BlockAssignable<IdentityPlatformConfigBlockingFunctionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigBlockingFunctionsEl {}
impl BuildIdentityPlatformConfigBlockingFunctionsEl {
    pub fn build(self) -> IdentityPlatformConfigBlockingFunctionsEl {
        IdentityPlatformConfigBlockingFunctionsEl {
            forward_inbound_credentials: core::default::Default::default(),
            triggers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigBlockingFunctionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigBlockingFunctionsElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigBlockingFunctionsElRef {
        IdentityPlatformConfigBlockingFunctionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigBlockingFunctionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `forward_inbound_credentials` after provisioning.\n"]
    pub fn forward_inbound_credentials(
        &self,
    ) -> ListRef<IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forward_inbound_credentials", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigClientElPermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_user_deletion: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_user_signup: Option<PrimField<bool>>,
}
impl IdentityPlatformConfigClientElPermissionsEl {
    #[doc = "Set the field `disabled_user_deletion`.\nWhen true, end users cannot delete their account on the associated project through any of our API methods"]
    pub fn set_disabled_user_deletion(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled_user_deletion = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled_user_signup`.\nWhen true, end users cannot sign up for a new account on the associated project through any of our API methods"]
    pub fn set_disabled_user_signup(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled_user_signup = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformConfigClientElPermissionsEl {
    type O = BlockAssignable<IdentityPlatformConfigClientElPermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigClientElPermissionsEl {}
impl BuildIdentityPlatformConfigClientElPermissionsEl {
    pub fn build(self) -> IdentityPlatformConfigClientElPermissionsEl {
        IdentityPlatformConfigClientElPermissionsEl {
            disabled_user_deletion: core::default::Default::default(),
            disabled_user_signup: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigClientElPermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigClientElPermissionsElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigClientElPermissionsElRef {
        IdentityPlatformConfigClientElPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigClientElPermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled_user_deletion` after provisioning.\nWhen true, end users cannot delete their account on the associated project through any of our API methods"]
    pub fn disabled_user_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled_user_deletion", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disabled_user_signup` after provisioning.\nWhen true, end users cannot sign up for a new account on the associated project through any of our API methods"]
    pub fn disabled_user_signup(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled_user_signup", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IdentityPlatformConfigClientElDynamic {
    permissions: Option<DynamicBlock<IdentityPlatformConfigClientElPermissionsEl>>,
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigClientEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Vec<IdentityPlatformConfigClientElPermissionsEl>>,
    dynamic: IdentityPlatformConfigClientElDynamic,
}
impl IdentityPlatformConfigClientEl {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigClientElPermissionsEl>>,
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
impl ToListMappable for IdentityPlatformConfigClientEl {
    type O = BlockAssignable<IdentityPlatformConfigClientEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigClientEl {}
impl BuildIdentityPlatformConfigClientEl {
    pub fn build(self) -> IdentityPlatformConfigClientEl {
        IdentityPlatformConfigClientEl {
            permissions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigClientElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigClientElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigClientElRef {
        IdentityPlatformConfigClientElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigClientElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nAPI key that can be used when making requests for this project."]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
    #[doc = "Get a reference to the value of field `firebase_subdomain` after provisioning.\nFirebase subdomain."]
    pub fn firebase_subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.firebase_subdomain", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<IdentityPlatformConfigClientElPermissionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    adjacent_intervals: Option<PrimField<f64>>,
}
impl IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigEl {
    #[doc = "Set the field `adjacent_intervals`.\nThe allowed number of adjacent intervals that will be used for verification to avoid clock skew."]
    pub fn set_adjacent_intervals(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.adjacent_intervals = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigEl {
    type O = BlockAssignable<IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigEl {}
impl BuildIdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigEl {
    pub fn build(self) -> IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigEl {
        IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigEl {
            adjacent_intervals: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigElRef {
        IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `adjacent_intervals` after provisioning.\nThe allowed number of adjacent intervals that will be used for verification to avoid clock skew."]
    pub fn adjacent_intervals(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.adjacent_intervals", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IdentityPlatformConfigMfaElProviderConfigsElDynamic {
    totp_provider_config:
        Option<DynamicBlock<IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigEl>>,
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigMfaElProviderConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    totp_provider_config:
        Option<Vec<IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigEl>>,
    dynamic: IdentityPlatformConfigMfaElProviderConfigsElDynamic,
}
impl IdentityPlatformConfigMfaElProviderConfigsEl {
    #[doc = "Set the field `state`.\nWhether MultiFactor Authentication has been enabled for this project. Possible values: [\"DISABLED\", \"ENABLED\", \"MANDATORY\"]"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `totp_provider_config`.\n"]
    pub fn set_totp_provider_config(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.totp_provider_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.totp_provider_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IdentityPlatformConfigMfaElProviderConfigsEl {
    type O = BlockAssignable<IdentityPlatformConfigMfaElProviderConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigMfaElProviderConfigsEl {}
impl BuildIdentityPlatformConfigMfaElProviderConfigsEl {
    pub fn build(self) -> IdentityPlatformConfigMfaElProviderConfigsEl {
        IdentityPlatformConfigMfaElProviderConfigsEl {
            state: core::default::Default::default(),
            totp_provider_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigMfaElProviderConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigMfaElProviderConfigsElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigMfaElProviderConfigsElRef {
        IdentityPlatformConfigMfaElProviderConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigMfaElProviderConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nWhether MultiFactor Authentication has been enabled for this project. Possible values: [\"DISABLED\", \"ENABLED\", \"MANDATORY\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `totp_provider_config` after provisioning.\n"]
    pub fn totp_provider_config(
        &self,
    ) -> ListRef<IdentityPlatformConfigMfaElProviderConfigsElTotpProviderConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.totp_provider_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IdentityPlatformConfigMfaElDynamic {
    provider_configs: Option<DynamicBlock<IdentityPlatformConfigMfaElProviderConfigsEl>>,
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigMfaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_providers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_configs: Option<Vec<IdentityPlatformConfigMfaElProviderConfigsEl>>,
    dynamic: IdentityPlatformConfigMfaElDynamic,
}
impl IdentityPlatformConfigMfaEl {
    #[doc = "Set the field `enabled_providers`.\nA list of usable second factors for this project. Possible values: [\"PHONE_SMS\"]"]
    pub fn set_enabled_providers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enabled_providers = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\nWhether MultiFactor Authentication has been enabled for this project. Possible values: [\"DISABLED\", \"ENABLED\", \"MANDATORY\"]"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `provider_configs`.\n"]
    pub fn set_provider_configs(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigMfaElProviderConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.provider_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.provider_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IdentityPlatformConfigMfaEl {
    type O = BlockAssignable<IdentityPlatformConfigMfaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigMfaEl {}
impl BuildIdentityPlatformConfigMfaEl {
    pub fn build(self) -> IdentityPlatformConfigMfaEl {
        IdentityPlatformConfigMfaEl {
            enabled_providers: core::default::Default::default(),
            state: core::default::Default::default(),
            provider_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigMfaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigMfaElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigMfaElRef {
        IdentityPlatformConfigMfaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigMfaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled_providers` after provisioning.\nA list of usable second factors for this project. Possible values: [\"PHONE_SMS\"]"]
    pub fn enabled_providers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enabled_providers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nWhether MultiFactor Authentication has been enabled for this project. Possible values: [\"DISABLED\", \"ENABLED\", \"MANDATORY\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `provider_configs` after provisioning.\n"]
    pub fn provider_configs(&self) -> ListRef<IdentityPlatformConfigMfaElProviderConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provider_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigMonitoringElRequestLoggingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl IdentityPlatformConfigMonitoringElRequestLoggingEl {
    #[doc = "Set the field `enabled`.\nWhether logging is enabled for this project or not."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformConfigMonitoringElRequestLoggingEl {
    type O = BlockAssignable<IdentityPlatformConfigMonitoringElRequestLoggingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigMonitoringElRequestLoggingEl {}
impl BuildIdentityPlatformConfigMonitoringElRequestLoggingEl {
    pub fn build(self) -> IdentityPlatformConfigMonitoringElRequestLoggingEl {
        IdentityPlatformConfigMonitoringElRequestLoggingEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigMonitoringElRequestLoggingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigMonitoringElRequestLoggingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IdentityPlatformConfigMonitoringElRequestLoggingElRef {
        IdentityPlatformConfigMonitoringElRequestLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigMonitoringElRequestLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether logging is enabled for this project or not."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct IdentityPlatformConfigMonitoringElDynamic {
    request_logging: Option<DynamicBlock<IdentityPlatformConfigMonitoringElRequestLoggingEl>>,
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigMonitoringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_logging: Option<Vec<IdentityPlatformConfigMonitoringElRequestLoggingEl>>,
    dynamic: IdentityPlatformConfigMonitoringElDynamic,
}
impl IdentityPlatformConfigMonitoringEl {
    #[doc = "Set the field `request_logging`.\n"]
    pub fn set_request_logging(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigMonitoringElRequestLoggingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_logging = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_logging = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IdentityPlatformConfigMonitoringEl {
    type O = BlockAssignable<IdentityPlatformConfigMonitoringEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigMonitoringEl {}
impl BuildIdentityPlatformConfigMonitoringEl {
    pub fn build(self) -> IdentityPlatformConfigMonitoringEl {
        IdentityPlatformConfigMonitoringEl {
            request_logging: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigMonitoringElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigMonitoringElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigMonitoringElRef {
        IdentityPlatformConfigMonitoringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigMonitoringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_logging` after provisioning.\n"]
    pub fn request_logging(
        &self,
    ) -> ListRef<IdentityPlatformConfigMonitoringElRequestLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_logging", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigMultiTenantEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_tenants: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_tenant_location: Option<PrimField<String>>,
}
impl IdentityPlatformConfigMultiTenantEl {
    #[doc = "Set the field `allow_tenants`.\nWhether this project can have tenants or not."]
    pub fn set_allow_tenants(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_tenants = Some(v.into());
        self
    }
    #[doc = "Set the field `default_tenant_location`.\nThe default cloud parent org or folder that the tenant project should be created under.\nThe parent resource name should be in the format of \"/\", such as \"folders/123\" or \"organizations/456\".\nIf the value is not set, the tenant will be created under the same organization or folder as the agent project."]
    pub fn set_default_tenant_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_tenant_location = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformConfigMultiTenantEl {
    type O = BlockAssignable<IdentityPlatformConfigMultiTenantEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigMultiTenantEl {}
impl BuildIdentityPlatformConfigMultiTenantEl {
    pub fn build(self) -> IdentityPlatformConfigMultiTenantEl {
        IdentityPlatformConfigMultiTenantEl {
            allow_tenants: core::default::Default::default(),
            default_tenant_location: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigMultiTenantElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigMultiTenantElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigMultiTenantElRef {
        IdentityPlatformConfigMultiTenantElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigMultiTenantElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_tenants` after provisioning.\nWhether this project can have tenants or not."]
    pub fn allow_tenants(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_tenants", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_tenant_location` after provisioning.\nThe default cloud parent org or folder that the tenant project should be created under.\nThe parent resource name should be in the format of \"/\", such as \"folders/123\" or \"organizations/456\".\nIf the value is not set, the tenant will be created under the same organization or folder as the agent project."]
    pub fn default_tenant_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_tenant_location", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigQuotaElSignUpQuotaConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    quota: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl IdentityPlatformConfigQuotaElSignUpQuotaConfigEl {
    #[doc = "Set the field `quota`.\nA sign up APIs quota that customers can override temporarily. Value can be in between 1 and 1000."]
    pub fn set_quota(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.quota = Some(v.into());
        self
    }
    #[doc = "Set the field `quota_duration`.\nHow long this quota will be active for. It is measurred in seconds, e.g., Example: \"9.615s\"."]
    pub fn set_quota_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.quota_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\nWhen this quota will take affect."]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformConfigQuotaElSignUpQuotaConfigEl {
    type O = BlockAssignable<IdentityPlatformConfigQuotaElSignUpQuotaConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigQuotaElSignUpQuotaConfigEl {}
impl BuildIdentityPlatformConfigQuotaElSignUpQuotaConfigEl {
    pub fn build(self) -> IdentityPlatformConfigQuotaElSignUpQuotaConfigEl {
        IdentityPlatformConfigQuotaElSignUpQuotaConfigEl {
            quota: core::default::Default::default(),
            quota_duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigQuotaElSignUpQuotaConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigQuotaElSignUpQuotaConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IdentityPlatformConfigQuotaElSignUpQuotaConfigElRef {
        IdentityPlatformConfigQuotaElSignUpQuotaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigQuotaElSignUpQuotaConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `quota` after provisioning.\nA sign up APIs quota that customers can override temporarily. Value can be in between 1 and 1000."]
    pub fn quota(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.quota", self.base))
    }
    #[doc = "Get a reference to the value of field `quota_duration` after provisioning.\nHow long this quota will be active for. It is measurred in seconds, e.g., Example: \"9.615s\"."]
    pub fn quota_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nWhen this quota will take affect."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct IdentityPlatformConfigQuotaElDynamic {
    sign_up_quota_config: Option<DynamicBlock<IdentityPlatformConfigQuotaElSignUpQuotaConfigEl>>,
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigQuotaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sign_up_quota_config: Option<Vec<IdentityPlatformConfigQuotaElSignUpQuotaConfigEl>>,
    dynamic: IdentityPlatformConfigQuotaElDynamic,
}
impl IdentityPlatformConfigQuotaEl {
    #[doc = "Set the field `sign_up_quota_config`.\n"]
    pub fn set_sign_up_quota_config(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigQuotaElSignUpQuotaConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sign_up_quota_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sign_up_quota_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IdentityPlatformConfigQuotaEl {
    type O = BlockAssignable<IdentityPlatformConfigQuotaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigQuotaEl {}
impl BuildIdentityPlatformConfigQuotaEl {
    pub fn build(self) -> IdentityPlatformConfigQuotaEl {
        IdentityPlatformConfigQuotaEl {
            sign_up_quota_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigQuotaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigQuotaElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigQuotaElRef {
        IdentityPlatformConfigQuotaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigQuotaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sign_up_quota_config` after provisioning.\n"]
    pub fn sign_up_quota_config(
        &self,
    ) -> ListRef<IdentityPlatformConfigQuotaElSignUpQuotaConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sign_up_quota_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigSignInElHashConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_cost: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rounds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    salt_separator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signer_key: Option<PrimField<String>>,
}
impl IdentityPlatformConfigSignInElHashConfigEl {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_cost`.\n"]
    pub fn set_memory_cost(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_cost = Some(v.into());
        self
    }
    #[doc = "Set the field `rounds`.\n"]
    pub fn set_rounds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rounds = Some(v.into());
        self
    }
    #[doc = "Set the field `salt_separator`.\n"]
    pub fn set_salt_separator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.salt_separator = Some(v.into());
        self
    }
    #[doc = "Set the field `signer_key`.\n"]
    pub fn set_signer_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.signer_key = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformConfigSignInElHashConfigEl {
    type O = BlockAssignable<IdentityPlatformConfigSignInElHashConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigSignInElHashConfigEl {}
impl BuildIdentityPlatformConfigSignInElHashConfigEl {
    pub fn build(self) -> IdentityPlatformConfigSignInElHashConfigEl {
        IdentityPlatformConfigSignInElHashConfigEl {
            algorithm: core::default::Default::default(),
            memory_cost: core::default::Default::default(),
            rounds: core::default::Default::default(),
            salt_separator: core::default::Default::default(),
            signer_key: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigSignInElHashConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigSignInElHashConfigElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigSignInElHashConfigElRef {
        IdentityPlatformConfigSignInElHashConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigSignInElHashConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_cost` after provisioning.\n"]
    pub fn memory_cost(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_cost", self.base))
    }
    #[doc = "Get a reference to the value of field `rounds` after provisioning.\n"]
    pub fn rounds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rounds", self.base))
    }
    #[doc = "Get a reference to the value of field `salt_separator` after provisioning.\n"]
    pub fn salt_separator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.salt_separator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `signer_key` after provisioning.\n"]
    pub fn signer_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signer_key", self.base))
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigSignInElAnonymousEl {
    enabled: PrimField<bool>,
}
impl IdentityPlatformConfigSignInElAnonymousEl {}
impl ToListMappable for IdentityPlatformConfigSignInElAnonymousEl {
    type O = BlockAssignable<IdentityPlatformConfigSignInElAnonymousEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigSignInElAnonymousEl {
    #[doc = "Whether anonymous user auth is enabled for the project or not."]
    pub enabled: PrimField<bool>,
}
impl BuildIdentityPlatformConfigSignInElAnonymousEl {
    pub fn build(self) -> IdentityPlatformConfigSignInElAnonymousEl {
        IdentityPlatformConfigSignInElAnonymousEl {
            enabled: self.enabled,
        }
    }
}
pub struct IdentityPlatformConfigSignInElAnonymousElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigSignInElAnonymousElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigSignInElAnonymousElRef {
        IdentityPlatformConfigSignInElAnonymousElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigSignInElAnonymousElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether anonymous user auth is enabled for the project or not."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigSignInElEmailEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_required: Option<PrimField<bool>>,
}
impl IdentityPlatformConfigSignInElEmailEl {
    #[doc = "Set the field `password_required`.\nWhether a password is required for email auth or not. If true, both an email and\npassword must be provided to sign in. If false, a user may sign in via either\nemail/password or email link."]
    pub fn set_password_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.password_required = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformConfigSignInElEmailEl {
    type O = BlockAssignable<IdentityPlatformConfigSignInElEmailEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigSignInElEmailEl {
    #[doc = "Whether email auth is enabled for the project or not."]
    pub enabled: PrimField<bool>,
}
impl BuildIdentityPlatformConfigSignInElEmailEl {
    pub fn build(self) -> IdentityPlatformConfigSignInElEmailEl {
        IdentityPlatformConfigSignInElEmailEl {
            enabled: self.enabled,
            password_required: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigSignInElEmailElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigSignInElEmailElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigSignInElEmailElRef {
        IdentityPlatformConfigSignInElEmailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigSignInElEmailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether email auth is enabled for the project or not."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `password_required` after provisioning.\nWhether a password is required for email auth or not. If true, both an email and\npassword must be provided to sign in. If false, a user may sign in via either\nemail/password or email link."]
    pub fn password_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_required", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigSignInElPhoneNumberEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_phone_numbers: Option<RecField<PrimField<String>>>,
}
impl IdentityPlatformConfigSignInElPhoneNumberEl {
    #[doc = "Set the field `test_phone_numbers`.\nA map of <test phone number, fake code> that can be used for phone auth testing."]
    pub fn set_test_phone_numbers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.test_phone_numbers = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformConfigSignInElPhoneNumberEl {
    type O = BlockAssignable<IdentityPlatformConfigSignInElPhoneNumberEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigSignInElPhoneNumberEl {
    #[doc = "Whether phone number auth is enabled for the project or not."]
    pub enabled: PrimField<bool>,
}
impl BuildIdentityPlatformConfigSignInElPhoneNumberEl {
    pub fn build(self) -> IdentityPlatformConfigSignInElPhoneNumberEl {
        IdentityPlatformConfigSignInElPhoneNumberEl {
            enabled: self.enabled,
            test_phone_numbers: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigSignInElPhoneNumberElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigSignInElPhoneNumberElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigSignInElPhoneNumberElRef {
        IdentityPlatformConfigSignInElPhoneNumberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigSignInElPhoneNumberElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether phone number auth is enabled for the project or not."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `test_phone_numbers` after provisioning.\nA map of <test phone number, fake code> that can be used for phone auth testing."]
    pub fn test_phone_numbers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.test_phone_numbers", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IdentityPlatformConfigSignInElDynamic {
    anonymous: Option<DynamicBlock<IdentityPlatformConfigSignInElAnonymousEl>>,
    email: Option<DynamicBlock<IdentityPlatformConfigSignInElEmailEl>>,
    phone_number: Option<DynamicBlock<IdentityPlatformConfigSignInElPhoneNumberEl>>,
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigSignInEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_duplicate_emails: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anonymous: Option<Vec<IdentityPlatformConfigSignInElAnonymousEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<Vec<IdentityPlatformConfigSignInElEmailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<Vec<IdentityPlatformConfigSignInElPhoneNumberEl>>,
    dynamic: IdentityPlatformConfigSignInElDynamic,
}
impl IdentityPlatformConfigSignInEl {
    #[doc = "Set the field `allow_duplicate_emails`.\nWhether to allow more than one account to have the same email."]
    pub fn set_allow_duplicate_emails(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_duplicate_emails = Some(v.into());
        self
    }
    #[doc = "Set the field `anonymous`.\n"]
    pub fn set_anonymous(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigSignInElAnonymousEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.anonymous = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.anonymous = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigSignInElEmailEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.email = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.email = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigSignInElPhoneNumberEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.phone_number = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.phone_number = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IdentityPlatformConfigSignInEl {
    type O = BlockAssignable<IdentityPlatformConfigSignInEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigSignInEl {}
impl BuildIdentityPlatformConfigSignInEl {
    pub fn build(self) -> IdentityPlatformConfigSignInEl {
        IdentityPlatformConfigSignInEl {
            allow_duplicate_emails: core::default::Default::default(),
            anonymous: core::default::Default::default(),
            email: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigSignInElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigSignInElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigSignInElRef {
        IdentityPlatformConfigSignInElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigSignInElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_duplicate_emails` after provisioning.\nWhether to allow more than one account to have the same email."]
    pub fn allow_duplicate_emails(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_duplicate_emails", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hash_config` after provisioning.\nOutput only. Hash config information."]
    pub fn hash_config(&self) -> ListRef<IdentityPlatformConfigSignInElHashConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hash_config", self.base))
    }
    #[doc = "Get a reference to the value of field `anonymous` after provisioning.\n"]
    pub fn anonymous(&self) -> ListRef<IdentityPlatformConfigSignInElAnonymousElRef> {
        ListRef::new(self.shared().clone(), format!("{}.anonymous", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<IdentityPlatformConfigSignInElEmailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> ListRef<IdentityPlatformConfigSignInElPhoneNumberElRef> {
        ListRef::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disallowed_regions: Option<ListField<PrimField<String>>>,
}
impl IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {
    #[doc = "Set the field `disallowed_regions`.\nTwo letter unicode region codes to disallow as defined by https://cldr.unicode.org/ The full list of these region codes is here: https://github.com/unicode-cldr/cldr-localenames-full/blob/master/main/en/territories.json"]
    pub fn set_disallowed_regions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.disallowed_regions = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {
    type O = BlockAssignable<IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {}
impl BuildIdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {
    pub fn build(self) -> IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {
        IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {
            disallowed_regions: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigSmsRegionConfigElAllowByDefaultElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigSmsRegionConfigElAllowByDefaultElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IdentityPlatformConfigSmsRegionConfigElAllowByDefaultElRef {
        IdentityPlatformConfigSmsRegionConfigElAllowByDefaultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigSmsRegionConfigElAllowByDefaultElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disallowed_regions` after provisioning.\nTwo letter unicode region codes to disallow as defined by https://cldr.unicode.org/ The full list of these region codes is here: https://github.com/unicode-cldr/cldr-localenames-full/blob/master/main/en/territories.json"]
    pub fn disallowed_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disallowed_regions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_regions: Option<ListField<PrimField<String>>>,
}
impl IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {
    #[doc = "Set the field `allowed_regions`.\nTwo letter unicode region codes to allow as defined by https://cldr.unicode.org/ The full list of these region codes is here: https://github.com/unicode-cldr/cldr-localenames-full/blob/master/main/en/territories.json"]
    pub fn set_allowed_regions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_regions = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {
    type O = BlockAssignable<IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {}
impl BuildIdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {
    pub fn build(self) -> IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {
        IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {
            allowed_regions: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyElRef {
        IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_regions` after provisioning.\nTwo letter unicode region codes to allow as defined by https://cldr.unicode.org/ The full list of these region codes is here: https://github.com/unicode-cldr/cldr-localenames-full/blob/master/main/en/territories.json"]
    pub fn allowed_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_regions", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IdentityPlatformConfigSmsRegionConfigElDynamic {
    allow_by_default: Option<DynamicBlock<IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl>>,
    allowlist_only: Option<DynamicBlock<IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl>>,
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigSmsRegionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_by_default: Option<Vec<IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowlist_only: Option<Vec<IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl>>,
    dynamic: IdentityPlatformConfigSmsRegionConfigElDynamic,
}
impl IdentityPlatformConfigSmsRegionConfigEl {
    #[doc = "Set the field `allow_by_default`.\n"]
    pub fn set_allow_by_default(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allow_by_default = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allow_by_default = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `allowlist_only`.\n"]
    pub fn set_allowlist_only(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allowlist_only = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allowlist_only = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IdentityPlatformConfigSmsRegionConfigEl {
    type O = BlockAssignable<IdentityPlatformConfigSmsRegionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigSmsRegionConfigEl {}
impl BuildIdentityPlatformConfigSmsRegionConfigEl {
    pub fn build(self) -> IdentityPlatformConfigSmsRegionConfigEl {
        IdentityPlatformConfigSmsRegionConfigEl {
            allow_by_default: core::default::Default::default(),
            allowlist_only: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigSmsRegionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigSmsRegionConfigElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigSmsRegionConfigElRef {
        IdentityPlatformConfigSmsRegionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigSmsRegionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_by_default` after provisioning.\n"]
    pub fn allow_by_default(
        &self,
    ) -> ListRef<IdentityPlatformConfigSmsRegionConfigElAllowByDefaultElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_by_default", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowlist_only` after provisioning.\n"]
    pub fn allowlist_only(
        &self,
    ) -> ListRef<IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowlist_only", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IdentityPlatformConfigTimeoutsEl {
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
impl ToListMappable for IdentityPlatformConfigTimeoutsEl {
    type O = BlockAssignable<IdentityPlatformConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformConfigTimeoutsEl {}
impl BuildIdentityPlatformConfigTimeoutsEl {
    pub fn build(self) -> IdentityPlatformConfigTimeoutsEl {
        IdentityPlatformConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformConfigTimeoutsElRef {
        IdentityPlatformConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformConfigTimeoutsElRef {
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
struct IdentityPlatformConfigDynamic {
    blocking_functions: Option<DynamicBlock<IdentityPlatformConfigBlockingFunctionsEl>>,
    client: Option<DynamicBlock<IdentityPlatformConfigClientEl>>,
    mfa: Option<DynamicBlock<IdentityPlatformConfigMfaEl>>,
    monitoring: Option<DynamicBlock<IdentityPlatformConfigMonitoringEl>>,
    multi_tenant: Option<DynamicBlock<IdentityPlatformConfigMultiTenantEl>>,
    quota: Option<DynamicBlock<IdentityPlatformConfigQuotaEl>>,
    sign_in: Option<DynamicBlock<IdentityPlatformConfigSignInEl>>,
    sms_region_config: Option<DynamicBlock<IdentityPlatformConfigSmsRegionConfigEl>>,
}
