use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoIdentityPoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_classic_flow: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_unauthenticated_identities: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_provider_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    identity_pool_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    openid_connect_provider_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml_provider_arns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_login_providers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cognito_identity_providers: Option<Vec<CognitoIdentityPoolCognitoIdentityProvidersEl>>,
    dynamic: CognitoIdentityPoolDynamic,
}

struct CognitoIdentityPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CognitoIdentityPoolData>,
}

#[derive(Clone)]
pub struct CognitoIdentityPool(Rc<CognitoIdentityPool_>);

impl CognitoIdentityPool {
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

    #[doc = "Set the field `allow_classic_flow`.\n"]
    pub fn set_allow_classic_flow(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_classic_flow = Some(v.into());
        self
    }

    #[doc = "Set the field `allow_unauthenticated_identities`.\n"]
    pub fn set_allow_unauthenticated_identities(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_unauthenticated_identities = Some(v.into());
        self
    }

    #[doc = "Set the field `developer_provider_name`.\n"]
    pub fn set_developer_provider_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().developer_provider_name = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `openid_connect_provider_arns`.\n"]
    pub fn set_openid_connect_provider_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().openid_connect_provider_arns = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `saml_provider_arns`.\n"]
    pub fn set_saml_provider_arns(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().saml_provider_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `supported_login_providers`.\n"]
    pub fn set_supported_login_providers(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().supported_login_providers = Some(v.into());
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

    #[doc = "Set the field `cognito_identity_providers`.\n"]
    pub fn set_cognito_identity_providers(
        self,
        v: impl Into<BlockAssignable<CognitoIdentityPoolCognitoIdentityProvidersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cognito_identity_providers = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cognito_identity_providers = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `allow_classic_flow` after provisioning.\n"]
    pub fn allow_classic_flow(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_classic_flow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow_unauthenticated_identities` after provisioning.\n"]
    pub fn allow_unauthenticated_identities(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_unauthenticated_identities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `developer_provider_name` after provisioning.\n"]
    pub fn developer_provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.developer_provider_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_pool_name` after provisioning.\n"]
    pub fn identity_pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `openid_connect_provider_arns` after provisioning.\n"]
    pub fn openid_connect_provider_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.openid_connect_provider_arns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_provider_arns` after provisioning.\n"]
    pub fn saml_provider_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.saml_provider_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_login_providers` after provisioning.\n"]
    pub fn supported_login_providers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.supported_login_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl Referable for CognitoIdentityPool {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CognitoIdentityPool { }

impl ToListMappable for CognitoIdentityPool {
    type O = ListRef<CognitoIdentityPoolRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CognitoIdentityPool_ {
    fn extract_resource_type(&self) -> String {
        "aws_cognito_identity_pool".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoIdentityPool {
    pub tf_id: String,
    #[doc = ""]
    pub identity_pool_name: PrimField<String>,
}

impl BuildCognitoIdentityPool {
    pub fn build(self, stack: &mut Stack) -> CognitoIdentityPool {
        let out = CognitoIdentityPool(Rc::new(CognitoIdentityPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CognitoIdentityPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_classic_flow: core::default::Default::default(),
                allow_unauthenticated_identities: core::default::Default::default(),
                developer_provider_name: core::default::Default::default(),
                id: core::default::Default::default(),
                identity_pool_name: self.identity_pool_name,
                openid_connect_provider_arns: core::default::Default::default(),
                region: core::default::Default::default(),
                saml_provider_arns: core::default::Default::default(),
                supported_login_providers: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                cognito_identity_providers: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoIdentityPoolRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoIdentityPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CognitoIdentityPoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_classic_flow` after provisioning.\n"]
    pub fn allow_classic_flow(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_classic_flow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow_unauthenticated_identities` after provisioning.\n"]
    pub fn allow_unauthenticated_identities(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_unauthenticated_identities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `developer_provider_name` after provisioning.\n"]
    pub fn developer_provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.developer_provider_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_pool_name` after provisioning.\n"]
    pub fn identity_pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `openid_connect_provider_arns` after provisioning.\n"]
    pub fn openid_connect_provider_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.openid_connect_provider_arns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_provider_arns` after provisioning.\n"]
    pub fn saml_provider_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.saml_provider_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_login_providers` after provisioning.\n"]
    pub fn supported_login_providers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.supported_login_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CognitoIdentityPoolCognitoIdentityProvidersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_token_check: Option<PrimField<bool>>,
}

impl CognitoIdentityPoolCognitoIdentityProvidersEl {
    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }

    #[doc = "Set the field `provider_name`.\n"]
    pub fn set_provider_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provider_name = Some(v.into());
        self
    }

    #[doc = "Set the field `server_side_token_check`.\n"]
    pub fn set_server_side_token_check(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.server_side_token_check = Some(v.into());
        self
    }
}

impl ToListMappable for CognitoIdentityPoolCognitoIdentityProvidersEl {
    type O = BlockAssignable<CognitoIdentityPoolCognitoIdentityProvidersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoIdentityPoolCognitoIdentityProvidersEl {}

impl BuildCognitoIdentityPoolCognitoIdentityProvidersEl {
    pub fn build(self) -> CognitoIdentityPoolCognitoIdentityProvidersEl {
        CognitoIdentityPoolCognitoIdentityProvidersEl {
            client_id: core::default::Default::default(),
            provider_name: core::default::Default::default(),
            server_side_token_check: core::default::Default::default(),
        }
    }
}

pub struct CognitoIdentityPoolCognitoIdentityProvidersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoIdentityPoolCognitoIdentityProvidersElRef {
    fn new(shared: StackShared, base: String) -> CognitoIdentityPoolCognitoIdentityProvidersElRef {
        CognitoIdentityPoolCognitoIdentityProvidersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoIdentityPoolCognitoIdentityProvidersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.base))
    }

    #[doc = "Get a reference to the value of field `server_side_token_check` after provisioning.\n"]
    pub fn server_side_token_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_side_token_check", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoIdentityPoolDynamic {
    cognito_identity_providers: Option<DynamicBlock<CognitoIdentityPoolCognitoIdentityProvidersEl>>,
}
