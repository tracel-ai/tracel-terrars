use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VerifiedaccessTrustProviderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_trust_provider_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    policy_reference_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    trust_provider_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_trust_provider_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_options: Option<Vec<VerifiedaccessTrustProviderDeviceOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    native_application_oidc_options: Option<Vec<VerifiedaccessTrustProviderNativeApplicationOidcOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc_options: Option<Vec<VerifiedaccessTrustProviderOidcOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sse_specification: Option<Vec<VerifiedaccessTrustProviderSseSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VerifiedaccessTrustProviderTimeoutsEl>,
    dynamic: VerifiedaccessTrustProviderDynamic,
}

struct VerifiedaccessTrustProvider_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VerifiedaccessTrustProviderData>,
}

#[derive(Clone)]
pub struct VerifiedaccessTrustProvider(Rc<VerifiedaccessTrustProvider_>);

impl VerifiedaccessTrustProvider {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `device_trust_provider_type`.\n"]
    pub fn set_device_trust_provider_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().device_trust_provider_type = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `user_trust_provider_type`.\n"]
    pub fn set_user_trust_provider_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_trust_provider_type = Some(v.into());
        self
    }

    #[doc = "Set the field `device_options`.\n"]
    pub fn set_device_options(self, v: impl Into<BlockAssignable<VerifiedaccessTrustProviderDeviceOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().device_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.device_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `native_application_oidc_options`.\n"]
    pub fn set_native_application_oidc_options(
        self,
        v: impl Into<BlockAssignable<VerifiedaccessTrustProviderNativeApplicationOidcOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().native_application_oidc_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.native_application_oidc_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oidc_options`.\n"]
    pub fn set_oidc_options(self, v: impl Into<BlockAssignable<VerifiedaccessTrustProviderOidcOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oidc_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oidc_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sse_specification`.\n"]
    pub fn set_sse_specification(
        self,
        v: impl Into<BlockAssignable<VerifiedaccessTrustProviderSseSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sse_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sse_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VerifiedaccessTrustProviderTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_trust_provider_type` after provisioning.\n"]
    pub fn device_trust_provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_trust_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_reference_name` after provisioning.\n"]
    pub fn policy_reference_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_reference_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `trust_provider_type` after provisioning.\n"]
    pub fn trust_provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_trust_provider_type` after provisioning.\n"]
    pub fn user_trust_provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_trust_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_options` after provisioning.\n"]
    pub fn device_options(&self) -> ListRef<VerifiedaccessTrustProviderDeviceOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.device_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `native_application_oidc_options` after provisioning.\n"]
    pub fn native_application_oidc_options(
        &self,
    ) -> ListRef<VerifiedaccessTrustProviderNativeApplicationOidcOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.native_application_oidc_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oidc_options` after provisioning.\n"]
    pub fn oidc_options(&self) -> ListRef<VerifiedaccessTrustProviderOidcOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oidc_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sse_specification` after provisioning.\n"]
    pub fn sse_specification(&self) -> ListRef<VerifiedaccessTrustProviderSseSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sse_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VerifiedaccessTrustProviderTimeoutsElRef {
        VerifiedaccessTrustProviderTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for VerifiedaccessTrustProvider {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VerifiedaccessTrustProvider { }

impl ToListMappable for VerifiedaccessTrustProvider {
    type O = ListRef<VerifiedaccessTrustProviderRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VerifiedaccessTrustProvider_ {
    fn extract_resource_type(&self) -> String {
        "aws_verifiedaccess_trust_provider".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVerifiedaccessTrustProvider {
    pub tf_id: String,
    #[doc = ""]
    pub policy_reference_name: PrimField<String>,
    #[doc = ""]
    pub trust_provider_type: PrimField<String>,
}

impl BuildVerifiedaccessTrustProvider {
    pub fn build(self, stack: &mut Stack) -> VerifiedaccessTrustProvider {
        let out = VerifiedaccessTrustProvider(Rc::new(VerifiedaccessTrustProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VerifiedaccessTrustProviderData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                device_trust_provider_type: core::default::Default::default(),
                id: core::default::Default::default(),
                policy_reference_name: self.policy_reference_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                trust_provider_type: self.trust_provider_type,
                user_trust_provider_type: core::default::Default::default(),
                device_options: core::default::Default::default(),
                native_application_oidc_options: core::default::Default::default(),
                oidc_options: core::default::Default::default(),
                sse_specification: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VerifiedaccessTrustProviderRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedaccessTrustProviderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl VerifiedaccessTrustProviderRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_trust_provider_type` after provisioning.\n"]
    pub fn device_trust_provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_trust_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_reference_name` after provisioning.\n"]
    pub fn policy_reference_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_reference_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `trust_provider_type` after provisioning.\n"]
    pub fn trust_provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_trust_provider_type` after provisioning.\n"]
    pub fn user_trust_provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_trust_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_options` after provisioning.\n"]
    pub fn device_options(&self) -> ListRef<VerifiedaccessTrustProviderDeviceOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.device_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `native_application_oidc_options` after provisioning.\n"]
    pub fn native_application_oidc_options(
        &self,
    ) -> ListRef<VerifiedaccessTrustProviderNativeApplicationOidcOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.native_application_oidc_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oidc_options` after provisioning.\n"]
    pub fn oidc_options(&self) -> ListRef<VerifiedaccessTrustProviderOidcOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oidc_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sse_specification` after provisioning.\n"]
    pub fn sse_specification(&self) -> ListRef<VerifiedaccessTrustProviderSseSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sse_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VerifiedaccessTrustProviderTimeoutsElRef {
        VerifiedaccessTrustProviderTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessTrustProviderDeviceOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tenant_id: Option<PrimField<String>>,
}

impl VerifiedaccessTrustProviderDeviceOptionsEl {
    #[doc = "Set the field `tenant_id`.\n"]
    pub fn set_tenant_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tenant_id = Some(v.into());
        self
    }
}

impl ToListMappable for VerifiedaccessTrustProviderDeviceOptionsEl {
    type O = BlockAssignable<VerifiedaccessTrustProviderDeviceOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessTrustProviderDeviceOptionsEl {}

impl BuildVerifiedaccessTrustProviderDeviceOptionsEl {
    pub fn build(self) -> VerifiedaccessTrustProviderDeviceOptionsEl {
        VerifiedaccessTrustProviderDeviceOptionsEl { tenant_id: core::default::Default::default() }
    }
}

pub struct VerifiedaccessTrustProviderDeviceOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedaccessTrustProviderDeviceOptionsElRef {
    fn new(shared: StackShared, base: String) -> VerifiedaccessTrustProviderDeviceOptionsElRef {
        VerifiedaccessTrustProviderDeviceOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessTrustProviderDeviceOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\n"]
    pub fn tenant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessTrustProviderNativeApplicationOidcOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    client_secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_signing_key_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_info_endpoint: Option<PrimField<String>>,
}

impl VerifiedaccessTrustProviderNativeApplicationOidcOptionsEl {
    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }

    #[doc = "Set the field `public_signing_key_endpoint`.\n"]
    pub fn set_public_signing_key_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_signing_key_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `user_info_endpoint`.\n"]
    pub fn set_user_info_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_info_endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for VerifiedaccessTrustProviderNativeApplicationOidcOptionsEl {
    type O = BlockAssignable<VerifiedaccessTrustProviderNativeApplicationOidcOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessTrustProviderNativeApplicationOidcOptionsEl {
    #[doc = ""]
    pub client_secret: PrimField<String>,
}

impl BuildVerifiedaccessTrustProviderNativeApplicationOidcOptionsEl {
    pub fn build(self) -> VerifiedaccessTrustProviderNativeApplicationOidcOptionsEl {
        VerifiedaccessTrustProviderNativeApplicationOidcOptionsEl {
            authorization_endpoint: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: self.client_secret,
            issuer: core::default::Default::default(),
            public_signing_key_endpoint: core::default::Default::default(),
            scope: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
            user_info_endpoint: core::default::Default::default(),
        }
    }
}

pub struct VerifiedaccessTrustProviderNativeApplicationOidcOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedaccessTrustProviderNativeApplicationOidcOptionsElRef {
    fn new(shared: StackShared, base: String) -> VerifiedaccessTrustProviderNativeApplicationOidcOptionsElRef {
        VerifiedaccessTrustProviderNativeApplicationOidcOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessTrustProviderNativeApplicationOidcOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `public_signing_key_endpoint` after provisioning.\n"]
    pub fn public_signing_key_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_signing_key_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `user_info_endpoint` after provisioning.\n"]
    pub fn user_info_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_info_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessTrustProviderOidcOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    client_secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_info_endpoint: Option<PrimField<String>>,
}

impl VerifiedaccessTrustProviderOidcOptionsEl {
    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `user_info_endpoint`.\n"]
    pub fn set_user_info_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_info_endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for VerifiedaccessTrustProviderOidcOptionsEl {
    type O = BlockAssignable<VerifiedaccessTrustProviderOidcOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessTrustProviderOidcOptionsEl {
    #[doc = ""]
    pub client_secret: PrimField<String>,
}

impl BuildVerifiedaccessTrustProviderOidcOptionsEl {
    pub fn build(self) -> VerifiedaccessTrustProviderOidcOptionsEl {
        VerifiedaccessTrustProviderOidcOptionsEl {
            authorization_endpoint: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: self.client_secret,
            issuer: core::default::Default::default(),
            scope: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
            user_info_endpoint: core::default::Default::default(),
        }
    }
}

pub struct VerifiedaccessTrustProviderOidcOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedaccessTrustProviderOidcOptionsElRef {
    fn new(shared: StackShared, base: String) -> VerifiedaccessTrustProviderOidcOptionsElRef {
        VerifiedaccessTrustProviderOidcOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessTrustProviderOidcOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `user_info_endpoint` after provisioning.\n"]
    pub fn user_info_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_info_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessTrustProviderSseSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_key_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
}

impl VerifiedaccessTrustProviderSseSpecificationEl {
    #[doc = "Set the field `customer_managed_key_enabled`.\n"]
    pub fn set_customer_managed_key_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.customer_managed_key_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
}

impl ToListMappable for VerifiedaccessTrustProviderSseSpecificationEl {
    type O = BlockAssignable<VerifiedaccessTrustProviderSseSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessTrustProviderSseSpecificationEl {}

impl BuildVerifiedaccessTrustProviderSseSpecificationEl {
    pub fn build(self) -> VerifiedaccessTrustProviderSseSpecificationEl {
        VerifiedaccessTrustProviderSseSpecificationEl {
            customer_managed_key_enabled: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}

pub struct VerifiedaccessTrustProviderSseSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedaccessTrustProviderSseSpecificationElRef {
    fn new(shared: StackShared, base: String) -> VerifiedaccessTrustProviderSseSpecificationElRef {
        VerifiedaccessTrustProviderSseSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessTrustProviderSseSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer_managed_key_enabled` after provisioning.\n"]
    pub fn customer_managed_key_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessTrustProviderTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl VerifiedaccessTrustProviderTimeoutsEl {
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

impl ToListMappable for VerifiedaccessTrustProviderTimeoutsEl {
    type O = BlockAssignable<VerifiedaccessTrustProviderTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessTrustProviderTimeoutsEl {}

impl BuildVerifiedaccessTrustProviderTimeoutsEl {
    pub fn build(self) -> VerifiedaccessTrustProviderTimeoutsEl {
        VerifiedaccessTrustProviderTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct VerifiedaccessTrustProviderTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedaccessTrustProviderTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VerifiedaccessTrustProviderTimeoutsElRef {
        VerifiedaccessTrustProviderTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessTrustProviderTimeoutsElRef {
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
struct VerifiedaccessTrustProviderDynamic {
    device_options: Option<DynamicBlock<VerifiedaccessTrustProviderDeviceOptionsEl>>,
    native_application_oidc_options: Option<DynamicBlock<VerifiedaccessTrustProviderNativeApplicationOidcOptionsEl>>,
    oidc_options: Option<DynamicBlock<VerifiedaccessTrustProviderOidcOptionsEl>>,
    sse_specification: Option<DynamicBlock<VerifiedaccessTrustProviderSseSpecificationEl>>,
}
