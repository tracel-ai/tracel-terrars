use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct BedrockagentcoreOauth2CredentialProviderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    credential_provider_vendor: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_provider_config:
        Option<Vec<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigEl>>,
    dynamic: BedrockagentcoreOauth2CredentialProviderDynamic,
}

struct BedrockagentcoreOauth2CredentialProvider_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentcoreOauth2CredentialProviderData>,
}

#[derive(Clone)]
pub struct BedrockagentcoreOauth2CredentialProvider(Rc<BedrockagentcoreOauth2CredentialProvider_>);

impl BedrockagentcoreOauth2CredentialProvider {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_provider_config`.\n"]
    pub fn set_oauth2_provider_config(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oauth2_provider_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oauth2_provider_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `client_secret_arn` after provisioning.\n"]
    pub fn client_secret_arn(
        &self,
    ) -> ListRef<BedrockagentcoreOauth2CredentialProviderClientSecretArnElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_secret_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `credential_provider_arn` after provisioning.\n"]
    pub fn credential_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_provider_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `credential_provider_vendor` after provisioning.\n"]
    pub fn credential_provider_vendor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_provider_vendor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oauth2_provider_config` after provisioning.\n"]
    pub fn oauth2_provider_config(
        &self,
    ) -> ListRef<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth2_provider_config", self.extract_ref()),
        )
    }
}

impl Referable for BedrockagentcoreOauth2CredentialProvider {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for BedrockagentcoreOauth2CredentialProvider {}

impl ToListMappable for BedrockagentcoreOauth2CredentialProvider {
    type O = ListRef<BedrockagentcoreOauth2CredentialProviderRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockagentcoreOauth2CredentialProvider_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagentcore_oauth2_credential_provider".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProvider {
    pub tf_id: String,
    #[doc = ""]
    pub credential_provider_vendor: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentcoreOauth2CredentialProvider {
    pub fn build(self, stack: &mut Stack) -> BedrockagentcoreOauth2CredentialProvider {
        let out = BedrockagentcoreOauth2CredentialProvider(Rc::new(
            BedrockagentcoreOauth2CredentialProvider_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(BedrockagentcoreOauth2CredentialProviderData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    credential_provider_vendor: self.credential_provider_vendor,
                    name: self.name,
                    region: core::default::Default::default(),
                    oauth2_provider_config: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl BedrockagentcoreOauth2CredentialProviderRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_secret_arn` after provisioning.\n"]
    pub fn client_secret_arn(
        &self,
    ) -> ListRef<BedrockagentcoreOauth2CredentialProviderClientSecretArnElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_secret_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `credential_provider_arn` after provisioning.\n"]
    pub fn credential_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_provider_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `credential_provider_vendor` after provisioning.\n"]
    pub fn credential_provider_vendor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_provider_vendor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oauth2_provider_config` after provisioning.\n"]
    pub fn oauth2_provider_config(
        &self,
    ) -> ListRef<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth2_provider_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderClientSecretArnEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_arn: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderClientSecretArnEl {
    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_arn = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderClientSecretArnEl {
    type O = BlockAssignable<BedrockagentcoreOauth2CredentialProviderClientSecretArnEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderClientSecretArnEl {}

impl BuildBedrockagentcoreOauth2CredentialProviderClientSecretArnEl {
    pub fn build(self) -> BedrockagentcoreOauth2CredentialProviderClientSecretArnEl {
        BedrockagentcoreOauth2CredentialProviderClientSecretArnEl {
            secret_arn: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderClientSecretArnElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderClientSecretArnElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderClientSecretArnElRef {
        BedrockagentcoreOauth2CredentialProviderClientSecretArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderClientSecretArnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl
{
    authorization_endpoint: PrimField<String>,
    issuer: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_types: Option<SetField<PrimField<String>>>,
    token_endpoint: PrimField<String>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    #[doc = "Set the field `response_types`.\n"]
    pub fn set_response_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.response_types = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl
{
    #[doc = ""]
    pub authorization_endpoint: PrimField<String>,
    #[doc = ""]
    pub issuer: PrimField<String>,
    #[doc = ""]
    pub token_endpoint: PrimField<String>,
}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
            authorization_endpoint: self.authorization_endpoint,
            issuer: self.issuer,
            response_types: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `response_types` after provisioning.\n"]
    pub fn response_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.response_types", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElDynamic {
    authorization_server_metadata: Option<
        DynamicBlock<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_server_metadata: Option<
        Vec<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElDynamic,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryEl {
    #[doc = "Set the field `discovery_url`.\n"]
    pub fn set_discovery_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.discovery_url = Some(v.into());
        self
    }

    #[doc = "Set the field `authorization_server_metadata`.\n"]
    pub fn set_authorization_server_metadata(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authorization_server_metadata = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authorization_server_metadata = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryEl {
            discovery_url: core::default::Default::default(),
            authorization_server_metadata: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_server_metadata` after provisioning.\n"]
    pub fn authorization_server_metadata(
        &self,
    ) -> ListRef<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.authorization_server_metadata", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElDynamic {
    oauth_discovery: Option<
        DynamicBlock<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_credentials_wo_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_discovery: Option<
        Vec<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryEl,
        >,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElDynamic,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigEl {
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id_wo = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret_wo = Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_discovery`.\n"]
    pub fn set_oauth_discovery(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_discovery = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_discovery = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigEl
{
    type O = BlockAssignable<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigEl
{}

impl
    BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigEl
{
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigEl
    {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigEl {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
            oauth_discovery: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_credentials_wo_version", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret_wo", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> ListRef<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElOauthDiscoveryElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_discovery", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }

    #[doc = "Set the field `response_types`.\n"]
    pub fn set_response_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.response_types = Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
            authorization_endpoint: core::default::Default::default(),
            issuer: core::default::Default::default(),
            response_types: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `response_types` after provisioning.\n"]
    pub fn response_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.response_types", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_server_metadata: Option<
        ListField<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_url: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryEl {
    #[doc = "Set the field `authorization_server_metadata`.\n"]
    pub fn set_authorization_server_metadata(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
                        >,
                    >,
    ) -> Self {
        self.authorization_server_metadata = Some(v.into());
        self
    }

    #[doc = "Set the field `discovery_url`.\n"]
    pub fn set_discovery_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.discovery_url = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryEl {
            authorization_server_metadata: core::default::Default::default(),
            discovery_url: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_server_metadata` after provisioning.\n"]
    pub fn authorization_server_metadata(
        &self,
    ) -> ListRef<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.authorization_server_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    client_credentials_wo_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret_wo: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigEl {
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id_wo = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret_wo = Some(v.into());
        self
    }
}

impl ToListMappable
    for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigEl
{
    type O = BlockAssignable<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigEl
{}

impl
    BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigEl
{
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigEl
    {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigEl {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_credentials_wo_version", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret_wo", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> ListRef<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElOauthDiscoveryElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_discovery", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }

    #[doc = "Set the field `response_types`.\n"]
    pub fn set_response_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.response_types = Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
            authorization_endpoint: core::default::Default::default(),
            issuer: core::default::Default::default(),
            response_types: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `response_types` after provisioning.\n"]
    pub fn response_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.response_types", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_server_metadata: Option<
        ListField<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_url: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryEl {
    #[doc = "Set the field `authorization_server_metadata`.\n"]
    pub fn set_authorization_server_metadata(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
                        >,
                    >,
    ) -> Self {
        self.authorization_server_metadata = Some(v.into());
        self
    }

    #[doc = "Set the field `discovery_url`.\n"]
    pub fn set_discovery_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.discovery_url = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryEl {
            authorization_server_metadata: core::default::Default::default(),
            discovery_url: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_server_metadata` after provisioning.\n"]
    pub fn authorization_server_metadata(
        &self,
    ) -> ListRef<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.authorization_server_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    client_credentials_wo_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret_wo: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigEl {
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id_wo = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret_wo = Some(v.into());
        self
    }
}

impl ToListMappable
    for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigEl
{
    type O = BlockAssignable<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigEl
{}

impl
    BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigEl
{
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigEl
    {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigEl {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_credentials_wo_version", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret_wo", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> ListRef<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElOauthDiscoveryElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_discovery", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }

    #[doc = "Set the field `response_types`.\n"]
    pub fn set_response_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.response_types = Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
            authorization_endpoint: core::default::Default::default(),
            issuer: core::default::Default::default(),
            response_types: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `response_types` after provisioning.\n"]
    pub fn response_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.response_types", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_server_metadata: Option<
        ListField<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_url: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryEl {
    #[doc = "Set the field `authorization_server_metadata`.\n"]
    pub fn set_authorization_server_metadata(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
                        >,
                    >,
    ) -> Self {
        self.authorization_server_metadata = Some(v.into());
        self
    }

    #[doc = "Set the field `discovery_url`.\n"]
    pub fn set_discovery_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.discovery_url = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryEl {
            authorization_server_metadata: core::default::Default::default(),
            discovery_url: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_server_metadata` after provisioning.\n"]
    pub fn authorization_server_metadata(
        &self,
    ) -> ListRef<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.authorization_server_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    client_credentials_wo_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret_wo: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigEl {
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id_wo = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret_wo = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigEl {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_credentials_wo_version", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret_wo", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> ListRef<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElOauthDiscoveryElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_discovery", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }

    #[doc = "Set the field `response_types`.\n"]
    pub fn set_response_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.response_types = Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
            authorization_endpoint: core::default::Default::default(),
            issuer: core::default::Default::default(),
            response_types: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `response_types` after provisioning.\n"]
    pub fn response_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.response_types", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_server_metadata: Option<
        ListField<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_url: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryEl {
    #[doc = "Set the field `authorization_server_metadata`.\n"]
    pub fn set_authorization_server_metadata(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
                        >,
                    >,
    ) -> Self {
        self.authorization_server_metadata = Some(v.into());
        self
    }

    #[doc = "Set the field `discovery_url`.\n"]
    pub fn set_discovery_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.discovery_url = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryEl {
            authorization_server_metadata: core::default::Default::default(),
            discovery_url: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_server_metadata` after provisioning.\n"]
    pub fn authorization_server_metadata(
        &self,
    ) -> ListRef<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.authorization_server_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    client_credentials_wo_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret_wo: Option<PrimField<String>>,
}

impl
    BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigEl
{
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id_wo = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret_wo = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigEl {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_credentials_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> ListRef<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElOauthDiscoveryElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.oauth_discovery", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_endpoint = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }

    #[doc = "Set the field `response_types`.\n"]
    pub fn set_response_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.response_types = Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl {
            authorization_endpoint: core::default::Default::default(),
            issuer: core::default::Default::default(),
            response_types: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `response_types` after provisioning.\n"]
    pub fn response_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.response_types", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_server_metadata: Option<
        ListField<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_url: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryEl {
    #[doc = "Set the field `authorization_server_metadata`.\n"]
    pub fn set_authorization_server_metadata(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataEl,
                        >,
                    >,
    ) -> Self {
        self.authorization_server_metadata = Some(v.into());
        self
    }

    #[doc = "Set the field `discovery_url`.\n"]
    pub fn set_discovery_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.discovery_url = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryEl {
    type O =
        BlockAssignable<
            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryEl
{}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryEl {
            authorization_server_metadata: core::default::Default::default(),
            discovery_url: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_server_metadata` after provisioning.\n"]
    pub fn authorization_server_metadata(
        &self,
    ) -> ListRef<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElAuthorizationServerMetadataElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.authorization_server_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    client_credentials_wo_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret_wo: Option<PrimField<String>>,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigEl {
    #[doc = "Set the field `client_credentials_wo_version`.\n"]
    pub fn set_client_credentials_wo_version(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.client_credentials_wo_version = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `client_id_wo`.\n"]
    pub fn set_client_id_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id_wo = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\n"]
    pub fn set_client_secret_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret_wo = Some(v.into());
        self
    }
}

impl ToListMappable
    for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigEl
{
    type O = BlockAssignable<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigEl
{}

impl
    BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigEl
{
    pub fn build(
        self,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigEl
    {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigEl {
            client_credentials_wo_version: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_id_wo: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElRef
    {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_credentials_wo_version` after provisioning.\n"]
    pub fn client_credentials_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_credentials_wo_version", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id_wo` after provisioning.\n"]
    pub fn client_id_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `client_secret_wo` after provisioning.\n"]
    pub fn client_secret_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret_wo", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oauth_discovery` after provisioning.\n"]
    pub fn oauth_discovery(
        &self,
    ) -> ListRef<
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElOauthDiscoveryElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_discovery", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElDynamic {
    custom_oauth2_provider_config: Option<
        DynamicBlock<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigEl>,
    >,
    github_oauth2_provider_config: Option<
        DynamicBlock<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigEl>,
    >,
    google_oauth2_provider_config: Option<
        DynamicBlock<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigEl>,
    >,
    microsoft_oauth2_provider_config: Option<
        DynamicBlock<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigEl>,
    >,
    salesforce_oauth2_provider_config: Option<
        DynamicBlock<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigEl>,
    >,
    slack_oauth2_provider_config: Option<
        DynamicBlock<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_oauth2_provider_config: Option<
        Vec<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    github_oauth2_provider_config: Option<
        Vec<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_oauth2_provider_config: Option<
        Vec<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    microsoft_oauth2_provider_config: Option<
        Vec<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    salesforce_oauth2_provider_config: Option<
        Vec<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    slack_oauth2_provider_config: Option<
        Vec<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigEl>,
    >,
    dynamic: BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElDynamic,
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigEl {
    #[doc = "Set the field `custom_oauth2_provider_config`.\n"]
    pub fn set_custom_oauth2_provider_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_oauth2_provider_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_oauth2_provider_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `github_oauth2_provider_config`.\n"]
    pub fn set_github_oauth2_provider_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github_oauth2_provider_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github_oauth2_provider_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `google_oauth2_provider_config`.\n"]
    pub fn set_google_oauth2_provider_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_oauth2_provider_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_oauth2_provider_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `microsoft_oauth2_provider_config`.\n"]
    pub fn set_microsoft_oauth2_provider_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.microsoft_oauth2_provider_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.microsoft_oauth2_provider_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `salesforce_oauth2_provider_config`.\n"]
    pub fn set_salesforce_oauth2_provider_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.salesforce_oauth2_provider_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce_oauth2_provider_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `slack_oauth2_provider_config`.\n"]
    pub fn set_slack_oauth2_provider_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.slack_oauth2_provider_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.slack_oauth2_provider_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigEl {
    type O = BlockAssignable<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigEl {}

impl BuildBedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigEl {
    pub fn build(self) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigEl {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigEl {
            custom_oauth2_provider_config: core::default::Default::default(),
            github_oauth2_provider_config: core::default::Default::default(),
            google_oauth2_provider_config: core::default::Default::default(),
            microsoft_oauth2_provider_config: core::default::Default::default(),
            salesforce_oauth2_provider_config: core::default::Default::default(),
            slack_oauth2_provider_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElRef {
        BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_oauth2_provider_config` after provisioning.\n"]
    pub fn custom_oauth2_provider_config(
        &self,
    ) -> ListRef<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElCustomOauth2ProviderConfigElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_oauth2_provider_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `github_oauth2_provider_config` after provisioning.\n"]
    pub fn github_oauth2_provider_config(
        &self,
    ) -> ListRef<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGithubOauth2ProviderConfigElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.github_oauth2_provider_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `google_oauth2_provider_config` after provisioning.\n"]
    pub fn google_oauth2_provider_config(
        &self,
    ) -> ListRef<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElGoogleOauth2ProviderConfigElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_oauth2_provider_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `microsoft_oauth2_provider_config` after provisioning.\n"]
    pub fn microsoft_oauth2_provider_config(
        &self,
    ) -> ListRef<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElMicrosoftOauth2ProviderConfigElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.microsoft_oauth2_provider_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `salesforce_oauth2_provider_config` after provisioning.\n"]
    pub fn salesforce_oauth2_provider_config(
        &self,
    ) -> ListRef<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSalesforceOauth2ProviderConfigElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.salesforce_oauth2_provider_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `slack_oauth2_provider_config` after provisioning.\n"]
    pub fn slack_oauth2_provider_config(
        &self,
    ) -> ListRef<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigElSlackOauth2ProviderConfigElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.slack_oauth2_provider_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOauth2CredentialProviderDynamic {
    oauth2_provider_config:
        Option<DynamicBlock<BedrockagentcoreOauth2CredentialProviderOauth2ProviderConfigEl>>,
}
