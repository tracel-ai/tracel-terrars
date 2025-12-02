use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SagemakerWorkforceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    workforce_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cognito_config: Option<Vec<SagemakerWorkforceCognitoConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc_config: Option<Vec<SagemakerWorkforceOidcConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ip_config: Option<Vec<SagemakerWorkforceSourceIpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workforce_vpc_config: Option<Vec<SagemakerWorkforceWorkforceVpcConfigEl>>,
    dynamic: SagemakerWorkforceDynamic,
}
struct SagemakerWorkforce_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerWorkforceData>,
}
#[derive(Clone)]
pub struct SagemakerWorkforce(Rc<SagemakerWorkforce_>);
impl SagemakerWorkforce {
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
    #[doc = "Set the field `cognito_config`.\n"]
    pub fn set_cognito_config(
        self,
        v: impl Into<BlockAssignable<SagemakerWorkforceCognitoConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cognito_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cognito_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oidc_config`.\n"]
    pub fn set_oidc_config(
        self,
        v: impl Into<BlockAssignable<SagemakerWorkforceOidcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oidc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oidc_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_ip_config`.\n"]
    pub fn set_source_ip_config(
        self,
        v: impl Into<BlockAssignable<SagemakerWorkforceSourceIpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_ip_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_ip_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `workforce_vpc_config`.\n"]
    pub fn set_workforce_vpc_config(
        self,
        v: impl Into<BlockAssignable<SagemakerWorkforceWorkforceVpcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workforce_vpc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workforce_vpc_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subdomain` after provisioning.\n"]
    pub fn subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subdomain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workforce_name` after provisioning.\n"]
    pub fn workforce_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workforce_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cognito_config` after provisioning.\n"]
    pub fn cognito_config(&self) -> ListRef<SagemakerWorkforceCognitoConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cognito_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oidc_config` after provisioning.\n"]
    pub fn oidc_config(&self) -> ListRef<SagemakerWorkforceOidcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oidc_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ip_config` after provisioning.\n"]
    pub fn source_ip_config(&self) -> ListRef<SagemakerWorkforceSourceIpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ip_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workforce_vpc_config` after provisioning.\n"]
    pub fn workforce_vpc_config(&self) -> ListRef<SagemakerWorkforceWorkforceVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workforce_vpc_config", self.extract_ref()),
        )
    }
}
impl Referable for SagemakerWorkforce {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SagemakerWorkforce {}
impl ToListMappable for SagemakerWorkforce {
    type O = ListRef<SagemakerWorkforceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SagemakerWorkforce_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_workforce".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSagemakerWorkforce {
    pub tf_id: String,
    #[doc = ""]
    pub workforce_name: PrimField<String>,
}
impl BuildSagemakerWorkforce {
    pub fn build(self, stack: &mut Stack) -> SagemakerWorkforce {
        let out = SagemakerWorkforce(Rc::new(SagemakerWorkforce_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerWorkforceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                workforce_name: self.workforce_name,
                cognito_config: core::default::Default::default(),
                oidc_config: core::default::Default::default(),
                source_ip_config: core::default::Default::default(),
                workforce_vpc_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SagemakerWorkforceRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerWorkforceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SagemakerWorkforceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subdomain` after provisioning.\n"]
    pub fn subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subdomain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workforce_name` after provisioning.\n"]
    pub fn workforce_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workforce_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cognito_config` after provisioning.\n"]
    pub fn cognito_config(&self) -> ListRef<SagemakerWorkforceCognitoConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cognito_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oidc_config` after provisioning.\n"]
    pub fn oidc_config(&self) -> ListRef<SagemakerWorkforceOidcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oidc_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ip_config` after provisioning.\n"]
    pub fn source_ip_config(&self) -> ListRef<SagemakerWorkforceSourceIpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ip_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workforce_vpc_config` after provisioning.\n"]
    pub fn workforce_vpc_config(&self) -> ListRef<SagemakerWorkforceWorkforceVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workforce_vpc_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerWorkforceCognitoConfigEl {
    client_id: PrimField<String>,
    user_pool: PrimField<String>,
}
impl SagemakerWorkforceCognitoConfigEl {}
impl ToListMappable for SagemakerWorkforceCognitoConfigEl {
    type O = BlockAssignable<SagemakerWorkforceCognitoConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerWorkforceCognitoConfigEl {
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub user_pool: PrimField<String>,
}
impl BuildSagemakerWorkforceCognitoConfigEl {
    pub fn build(self) -> SagemakerWorkforceCognitoConfigEl {
        SagemakerWorkforceCognitoConfigEl {
            client_id: self.client_id,
            user_pool: self.user_pool,
        }
    }
}
pub struct SagemakerWorkforceCognitoConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerWorkforceCognitoConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerWorkforceCognitoConfigElRef {
        SagemakerWorkforceCognitoConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerWorkforceCognitoConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `user_pool` after provisioning.\n"]
    pub fn user_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool", self.base))
    }
}
#[derive(Serialize)]
pub struct SagemakerWorkforceOidcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_request_extra_params: Option<RecField<PrimField<String>>>,
    authorization_endpoint: PrimField<String>,
    client_id: PrimField<String>,
    client_secret: PrimField<String>,
    issuer: PrimField<String>,
    jwks_uri: PrimField<String>,
    logout_endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    token_endpoint: PrimField<String>,
    user_info_endpoint: PrimField<String>,
}
impl SagemakerWorkforceOidcConfigEl {
    #[doc = "Set the field `authentication_request_extra_params`.\n"]
    pub fn set_authentication_request_extra_params(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.authentication_request_extra_params = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerWorkforceOidcConfigEl {
    type O = BlockAssignable<SagemakerWorkforceOidcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerWorkforceOidcConfigEl {
    #[doc = ""]
    pub authorization_endpoint: PrimField<String>,
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub client_secret: PrimField<String>,
    #[doc = ""]
    pub issuer: PrimField<String>,
    #[doc = ""]
    pub jwks_uri: PrimField<String>,
    #[doc = ""]
    pub logout_endpoint: PrimField<String>,
    #[doc = ""]
    pub token_endpoint: PrimField<String>,
    #[doc = ""]
    pub user_info_endpoint: PrimField<String>,
}
impl BuildSagemakerWorkforceOidcConfigEl {
    pub fn build(self) -> SagemakerWorkforceOidcConfigEl {
        SagemakerWorkforceOidcConfigEl {
            authentication_request_extra_params: core::default::Default::default(),
            authorization_endpoint: self.authorization_endpoint,
            client_id: self.client_id,
            client_secret: self.client_secret,
            issuer: self.issuer,
            jwks_uri: self.jwks_uri,
            logout_endpoint: self.logout_endpoint,
            scope: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
            user_info_endpoint: self.user_info_endpoint,
        }
    }
}
pub struct SagemakerWorkforceOidcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerWorkforceOidcConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerWorkforceOidcConfigElRef {
        SagemakerWorkforceOidcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerWorkforceOidcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authentication_request_extra_params` after provisioning.\n"]
    pub fn authentication_request_extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.authentication_request_extra_params", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
    #[doc = "Get a reference to the value of field `jwks_uri` after provisioning.\n"]
    pub fn jwks_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jwks_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `logout_endpoint` after provisioning.\n"]
    pub fn logout_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logout_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_info_endpoint` after provisioning.\n"]
    pub fn user_info_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_info_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerWorkforceSourceIpConfigEl {
    cidrs: SetField<PrimField<String>>,
}
impl SagemakerWorkforceSourceIpConfigEl {}
impl ToListMappable for SagemakerWorkforceSourceIpConfigEl {
    type O = BlockAssignable<SagemakerWorkforceSourceIpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerWorkforceSourceIpConfigEl {
    #[doc = ""]
    pub cidrs: SetField<PrimField<String>>,
}
impl BuildSagemakerWorkforceSourceIpConfigEl {
    pub fn build(self) -> SagemakerWorkforceSourceIpConfigEl {
        SagemakerWorkforceSourceIpConfigEl { cidrs: self.cidrs }
    }
}
pub struct SagemakerWorkforceSourceIpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerWorkforceSourceIpConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerWorkforceSourceIpConfigElRef {
        SagemakerWorkforceSourceIpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerWorkforceSourceIpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }
}
#[derive(Serialize)]
pub struct SagemakerWorkforceWorkforceVpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnets: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}
impl SagemakerWorkforceWorkforceVpcConfigEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnets = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerWorkforceWorkforceVpcConfigEl {
    type O = BlockAssignable<SagemakerWorkforceWorkforceVpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerWorkforceWorkforceVpcConfigEl {}
impl BuildSagemakerWorkforceWorkforceVpcConfigEl {
    pub fn build(self) -> SagemakerWorkforceWorkforceVpcConfigEl {
        SagemakerWorkforceWorkforceVpcConfigEl {
            security_group_ids: core::default::Default::default(),
            subnets: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}
pub struct SagemakerWorkforceWorkforceVpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerWorkforceWorkforceVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerWorkforceWorkforceVpcConfigElRef {
        SagemakerWorkforceWorkforceVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerWorkforceWorkforceVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct SagemakerWorkforceDynamic {
    cognito_config: Option<DynamicBlock<SagemakerWorkforceCognitoConfigEl>>,
    oidc_config: Option<DynamicBlock<SagemakerWorkforceOidcConfigEl>>,
    source_ip_config: Option<DynamicBlock<SagemakerWorkforceSourceIpConfigEl>>,
    workforce_vpc_config: Option<DynamicBlock<SagemakerWorkforceWorkforceVpcConfigEl>>,
}
