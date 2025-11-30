use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataCognitoIdentityPoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    identity_pool_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataCognitoIdentityPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCognitoIdentityPoolData>,
}

#[derive(Clone)]
pub struct DataCognitoIdentityPool(Rc<DataCognitoIdentityPool_>);

impl DataCognitoIdentityPool {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allow_classic_flow` after provisioning.\n"]
    pub fn allow_classic_flow(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_classic_flow", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `allow_unauthenticated_identities` after provisioning.\n"]
    pub fn allow_unauthenticated_identities(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_unauthenticated_identities", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_identity_providers` after provisioning.\n"]
    pub fn cognito_identity_providers(
        &self,
    ) -> SetRef<DataCognitoIdentityPoolCognitoIdentityProvidersElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cognito_identity_providers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `developer_provider_name` after provisioning.\n"]
    pub fn developer_provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.developer_provider_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_pool_name` after provisioning.\n"]
    pub fn identity_pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_pool_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `openid_connect_provider_arns` after provisioning.\n"]
    pub fn openid_connect_provider_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.openid_connect_provider_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `saml_provider_arns` after provisioning.\n"]
    pub fn saml_provider_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saml_provider_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supported_login_providers` after provisioning.\n"]
    pub fn supported_login_providers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.supported_login_providers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

impl Referable for DataCognitoIdentityPool {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataCognitoIdentityPool {}

impl ToListMappable for DataCognitoIdentityPool {
    type O = ListRef<DataCognitoIdentityPoolRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCognitoIdentityPool_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cognito_identity_pool".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCognitoIdentityPool {
    pub tf_id: String,
    #[doc = ""]
    pub identity_pool_name: PrimField<String>,
}

impl BuildDataCognitoIdentityPool {
    pub fn build(self, stack: &mut Stack) -> DataCognitoIdentityPool {
        let out = DataCognitoIdentityPool(Rc::new(DataCognitoIdentityPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCognitoIdentityPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                identity_pool_name: self.identity_pool_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCognitoIdentityPoolRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoIdentityPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataCognitoIdentityPoolRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `allow_classic_flow` after provisioning.\n"]
    pub fn allow_classic_flow(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_classic_flow", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `allow_unauthenticated_identities` after provisioning.\n"]
    pub fn allow_unauthenticated_identities(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_unauthenticated_identities", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cognito_identity_providers` after provisioning.\n"]
    pub fn cognito_identity_providers(
        &self,
    ) -> SetRef<DataCognitoIdentityPoolCognitoIdentityProvidersElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.cognito_identity_providers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `developer_provider_name` after provisioning.\n"]
    pub fn developer_provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.developer_provider_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_pool_name` after provisioning.\n"]
    pub fn identity_pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_pool_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `openid_connect_provider_arns` after provisioning.\n"]
    pub fn openid_connect_provider_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.openid_connect_provider_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `saml_provider_arns` after provisioning.\n"]
    pub fn saml_provider_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saml_provider_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supported_login_providers` after provisioning.\n"]
    pub fn supported_login_providers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.supported_login_providers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataCognitoIdentityPoolCognitoIdentityProvidersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_token_check: Option<PrimField<bool>>,
}

impl DataCognitoIdentityPoolCognitoIdentityProvidersEl {
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

impl ToListMappable for DataCognitoIdentityPoolCognitoIdentityProvidersEl {
    type O = BlockAssignable<DataCognitoIdentityPoolCognitoIdentityProvidersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoIdentityPoolCognitoIdentityProvidersEl {}

impl BuildDataCognitoIdentityPoolCognitoIdentityProvidersEl {
    pub fn build(self) -> DataCognitoIdentityPoolCognitoIdentityProvidersEl {
        DataCognitoIdentityPoolCognitoIdentityProvidersEl {
            client_id: core::default::Default::default(),
            provider_name: core::default::Default::default(),
            server_side_token_check: core::default::Default::default(),
        }
    }
}

pub struct DataCognitoIdentityPoolCognitoIdentityProvidersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCognitoIdentityPoolCognitoIdentityProvidersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCognitoIdentityPoolCognitoIdentityProvidersElRef {
        DataCognitoIdentityPoolCognitoIdentityProvidersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoIdentityPoolCognitoIdentityProvidersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `server_side_token_check` after provisioning.\n"]
    pub fn server_side_token_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_side_token_check", self.base),
        )
    }
}
