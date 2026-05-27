use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IdentityPlatformTenantInboundSamlConfigData {
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
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    tenant: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idp_config: Option<Vec<IdentityPlatformTenantInboundSamlConfigIdpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sp_config: Option<Vec<IdentityPlatformTenantInboundSamlConfigSpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IdentityPlatformTenantInboundSamlConfigTimeoutsEl>,
    dynamic: IdentityPlatformTenantInboundSamlConfigDynamic,
}
struct IdentityPlatformTenantInboundSamlConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IdentityPlatformTenantInboundSamlConfigData>,
}
#[derive(Clone)]
pub struct IdentityPlatformTenantInboundSamlConfig(Rc<IdentityPlatformTenantInboundSamlConfig_>);
impl IdentityPlatformTenantInboundSamlConfig {
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
    #[doc = "Set the field `idp_config`.\n"]
    pub fn set_idp_config(
        self,
        v: impl Into<BlockAssignable<IdentityPlatformTenantInboundSamlConfigIdpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().idp_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.idp_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sp_config`.\n"]
    pub fn set_sp_config(
        self,
        v: impl Into<BlockAssignable<IdentityPlatformTenantInboundSamlConfigSpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sp_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sp_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<IdentityPlatformTenantInboundSamlConfigTimeoutsEl>,
    ) -> Self {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the InboundSamlConfig resource. Must start with 'saml.' and can only have alphanumeric characters,\nhyphens, underscores or periods. The part after 'saml.' must also start with a lowercase letter, end with an\nalphanumeric character, and have at least 2 characters."]
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
    #[doc = "Get a reference to the value of field `tenant` after provisioning.\nThe name of the tenant where this inbound SAML config resource exists"]
    pub fn tenant(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tenant", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `idp_config` after provisioning.\n"]
    pub fn idp_config(&self) -> ListRef<IdentityPlatformTenantInboundSamlConfigIdpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.idp_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sp_config` after provisioning.\n"]
    pub fn sp_config(&self) -> ListRef<IdentityPlatformTenantInboundSamlConfigSpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sp_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
        IdentityPlatformTenantInboundSamlConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for IdentityPlatformTenantInboundSamlConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IdentityPlatformTenantInboundSamlConfig {}
impl ToListMappable for IdentityPlatformTenantInboundSamlConfig {
    type O = ListRef<IdentityPlatformTenantInboundSamlConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IdentityPlatformTenantInboundSamlConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_identity_platform_tenant_inbound_saml_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIdentityPlatformTenantInboundSamlConfig {
    pub tf_id: String,
    #[doc = "Human friendly display name."]
    pub display_name: PrimField<String>,
    #[doc = "The name of the InboundSamlConfig resource. Must start with 'saml.' and can only have alphanumeric characters,\nhyphens, underscores or periods. The part after 'saml.' must also start with a lowercase letter, end with an\nalphanumeric character, and have at least 2 characters."]
    pub name: PrimField<String>,
    #[doc = "The name of the tenant where this inbound SAML config resource exists"]
    pub tenant: PrimField<String>,
}
impl BuildIdentityPlatformTenantInboundSamlConfig {
    pub fn build(self, stack: &mut Stack) -> IdentityPlatformTenantInboundSamlConfig {
        let out = IdentityPlatformTenantInboundSamlConfig(Rc::new(
            IdentityPlatformTenantInboundSamlConfig_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(IdentityPlatformTenantInboundSamlConfigData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    display_name: self.display_name,
                    enabled: core::default::Default::default(),
                    id: core::default::Default::default(),
                    name: self.name,
                    project: core::default::Default::default(),
                    tenant: self.tenant,
                    idp_config: core::default::Default::default(),
                    sp_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IdentityPlatformTenantInboundSamlConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformTenantInboundSamlConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IdentityPlatformTenantInboundSamlConfigRef {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the InboundSamlConfig resource. Must start with 'saml.' and can only have alphanumeric characters,\nhyphens, underscores or periods. The part after 'saml.' must also start with a lowercase letter, end with an\nalphanumeric character, and have at least 2 characters."]
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
    #[doc = "Get a reference to the value of field `tenant` after provisioning.\nThe name of the tenant where this inbound SAML config resource exists"]
    pub fn tenant(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tenant", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `idp_config` after provisioning.\n"]
    pub fn idp_config(&self) -> ListRef<IdentityPlatformTenantInboundSamlConfigIdpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.idp_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sp_config` after provisioning.\n"]
    pub fn sp_config(&self) -> ListRef<IdentityPlatformTenantInboundSamlConfigSpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sp_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
        IdentityPlatformTenantInboundSamlConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    x509_certificate: Option<PrimField<String>>,
}
impl IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {
    #[doc = "Set the field `x509_certificate`.\nThe x509 certificate"]
    pub fn set_x509_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.x509_certificate = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {
    type O = BlockAssignable<IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {}
impl BuildIdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {
    pub fn build(self) -> IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {
        IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {
            x509_certificate: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesElRef {
        IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `x509_certificate` after provisioning.\nThe x509 certificate"]
    pub fn x509_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.x509_certificate", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IdentityPlatformTenantInboundSamlConfigIdpConfigElDynamic {
    idp_certificates:
        Option<DynamicBlock<IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl>>,
}
#[derive(Serialize)]
pub struct IdentityPlatformTenantInboundSamlConfigIdpConfigEl {
    idp_entity_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sign_request: Option<PrimField<bool>>,
    sso_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idp_certificates:
        Option<Vec<IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl>>,
    dynamic: IdentityPlatformTenantInboundSamlConfigIdpConfigElDynamic,
}
impl IdentityPlatformTenantInboundSamlConfigIdpConfigEl {
    #[doc = "Set the field `sign_request`.\nIndicates if outbounding SAMLRequest should be signed."]
    pub fn set_sign_request(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.sign_request = Some(v.into());
        self
    }
    #[doc = "Set the field `idp_certificates`.\n"]
    pub fn set_idp_certificates(
        mut self,
        v: impl Into<
            BlockAssignable<IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idp_certificates = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idp_certificates = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IdentityPlatformTenantInboundSamlConfigIdpConfigEl {
    type O = BlockAssignable<IdentityPlatformTenantInboundSamlConfigIdpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformTenantInboundSamlConfigIdpConfigEl {
    #[doc = "Unique identifier for all SAML entities"]
    pub idp_entity_id: PrimField<String>,
    #[doc = "URL to send Authentication request to."]
    pub sso_url: PrimField<String>,
}
impl BuildIdentityPlatformTenantInboundSamlConfigIdpConfigEl {
    pub fn build(self) -> IdentityPlatformTenantInboundSamlConfigIdpConfigEl {
        IdentityPlatformTenantInboundSamlConfigIdpConfigEl {
            idp_entity_id: self.idp_entity_id,
            sign_request: core::default::Default::default(),
            sso_url: self.sso_url,
            idp_certificates: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IdentityPlatformTenantInboundSamlConfigIdpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformTenantInboundSamlConfigIdpConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IdentityPlatformTenantInboundSamlConfigIdpConfigElRef {
        IdentityPlatformTenantInboundSamlConfigIdpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformTenantInboundSamlConfigIdpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `idp_entity_id` after provisioning.\nUnique identifier for all SAML entities"]
    pub fn idp_entity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idp_entity_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sign_request` after provisioning.\nIndicates if outbounding SAMLRequest should be signed."]
    pub fn sign_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sign_request", self.base))
    }
    #[doc = "Get a reference to the value of field `sso_url` after provisioning.\nURL to send Authentication request to."]
    pub fn sso_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sso_url", self.base))
    }
    #[doc = "Get a reference to the value of field `idp_certificates` after provisioning.\n"]
    pub fn idp_certificates(
        &self,
    ) -> ListRef<IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.idp_certificates", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    x509_certificate: Option<PrimField<String>>,
}
impl IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {
    #[doc = "Set the field `x509_certificate`.\n"]
    pub fn set_x509_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.x509_certificate = Some(v.into());
        self
    }
}
impl ToListMappable for IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {
    type O = BlockAssignable<IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {}
impl BuildIdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {
    pub fn build(self) -> IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {
        IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {
            x509_certificate: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesElRef {
        IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `x509_certificate` after provisioning.\n"]
    pub fn x509_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.x509_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformTenantInboundSamlConfigSpConfigEl {
    callback_uri: PrimField<String>,
    sp_entity_id: PrimField<String>,
}
impl IdentityPlatformTenantInboundSamlConfigSpConfigEl {}
impl ToListMappable for IdentityPlatformTenantInboundSamlConfigSpConfigEl {
    type O = BlockAssignable<IdentityPlatformTenantInboundSamlConfigSpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformTenantInboundSamlConfigSpConfigEl {
    #[doc = "Callback URI where responses from IDP are handled. Must start with 'https://'."]
    pub callback_uri: PrimField<String>,
    #[doc = "Unique identifier for all SAML entities."]
    pub sp_entity_id: PrimField<String>,
}
impl BuildIdentityPlatformTenantInboundSamlConfigSpConfigEl {
    pub fn build(self) -> IdentityPlatformTenantInboundSamlConfigSpConfigEl {
        IdentityPlatformTenantInboundSamlConfigSpConfigEl {
            callback_uri: self.callback_uri,
            sp_entity_id: self.sp_entity_id,
        }
    }
}
pub struct IdentityPlatformTenantInboundSamlConfigSpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformTenantInboundSamlConfigSpConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IdentityPlatformTenantInboundSamlConfigSpConfigElRef {
        IdentityPlatformTenantInboundSamlConfigSpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformTenantInboundSamlConfigSpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `callback_uri` after provisioning.\nCallback URI where responses from IDP are handled. Must start with 'https://'."]
    pub fn callback_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.callback_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `sp_certificates` after provisioning.\nThe IDP's certificate data to verify the signature in the SAMLResponse issued by the IDP."]
    pub fn sp_certificates(
        &self,
    ) -> ListRef<IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sp_certificates", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sp_entity_id` after provisioning.\nUnique identifier for all SAML entities."]
    pub fn sp_entity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sp_entity_id", self.base))
    }
}
#[derive(Serialize)]
pub struct IdentityPlatformTenantInboundSamlConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IdentityPlatformTenantInboundSamlConfigTimeoutsEl {
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
impl ToListMappable for IdentityPlatformTenantInboundSamlConfigTimeoutsEl {
    type O = BlockAssignable<IdentityPlatformTenantInboundSamlConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIdentityPlatformTenantInboundSamlConfigTimeoutsEl {}
impl BuildIdentityPlatformTenantInboundSamlConfigTimeoutsEl {
    pub fn build(self) -> IdentityPlatformTenantInboundSamlConfigTimeoutsEl {
        IdentityPlatformTenantInboundSamlConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
        IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
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
struct IdentityPlatformTenantInboundSamlConfigDynamic {
    idp_config: Option<DynamicBlock<IdentityPlatformTenantInboundSamlConfigIdpConfigEl>>,
    sp_config: Option<DynamicBlock<IdentityPlatformTenantInboundSamlConfigSpConfigEl>>,
}
