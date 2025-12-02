use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppsyncApiData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_contact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_config: Option<Vec<AppsyncApiEventConfigEl>>,
    dynamic: AppsyncApiDynamic,
}
struct AppsyncApi_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppsyncApiData>,
}
#[derive(Clone)]
pub struct AppsyncApi(Rc<AppsyncApi_>);
impl AppsyncApi {
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
    #[doc = "Set the field `owner_contact`.\n"]
    pub fn set_owner_contact(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().owner_contact = Some(v.into());
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
    #[doc = "Set the field `event_config`.\n"]
    pub fn set_event_config(self, v: impl Into<BlockAssignable<AppsyncApiEventConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `api_arn` after provisioning.\n"]
    pub fn api_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_contact` after provisioning.\n"]
    pub fn owner_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `waf_web_acl_arn` after provisioning.\n"]
    pub fn waf_web_acl_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.waf_web_acl_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `xray_enabled` after provisioning.\n"]
    pub fn xray_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.xray_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_config` after provisioning.\n"]
    pub fn event_config(&self) -> ListRef<AppsyncApiEventConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_config", self.extract_ref()),
        )
    }
}
impl Referable for AppsyncApi {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppsyncApi {}
impl ToListMappable for AppsyncApi {
    type O = ListRef<AppsyncApiRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppsyncApi_ {
    fn extract_resource_type(&self) -> String {
        "aws_appsync_api".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppsyncApi {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildAppsyncApi {
    pub fn build(self, stack: &mut Stack) -> AppsyncApi {
        let out = AppsyncApi(Rc::new(AppsyncApi_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppsyncApiData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                name: self.name,
                owner_contact: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                event_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AppsyncApiRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncApiRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppsyncApiRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_arn` after provisioning.\n"]
    pub fn api_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner_contact` after provisioning.\n"]
    pub fn owner_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `waf_web_acl_arn` after provisioning.\n"]
    pub fn waf_web_acl_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.waf_web_acl_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `xray_enabled` after provisioning.\n"]
    pub fn xray_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.xray_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_config` after provisioning.\n"]
    pub fn event_config(&self) -> ListRef<AppsyncApiEventConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppsyncApiEventConfigElAuthProviderElCognitoConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_id_client_regex: Option<PrimField<String>>,
    aws_region: PrimField<String>,
    user_pool_id: PrimField<String>,
}
impl AppsyncApiEventConfigElAuthProviderElCognitoConfigEl {
    #[doc = "Set the field `app_id_client_regex`.\n"]
    pub fn set_app_id_client_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.app_id_client_regex = Some(v.into());
        self
    }
}
impl ToListMappable for AppsyncApiEventConfigElAuthProviderElCognitoConfigEl {
    type O = BlockAssignable<AppsyncApiEventConfigElAuthProviderElCognitoConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppsyncApiEventConfigElAuthProviderElCognitoConfigEl {
    #[doc = ""]
    pub aws_region: PrimField<String>,
    #[doc = ""]
    pub user_pool_id: PrimField<String>,
}
impl BuildAppsyncApiEventConfigElAuthProviderElCognitoConfigEl {
    pub fn build(self) -> AppsyncApiEventConfigElAuthProviderElCognitoConfigEl {
        AppsyncApiEventConfigElAuthProviderElCognitoConfigEl {
            app_id_client_regex: core::default::Default::default(),
            aws_region: self.aws_region,
            user_pool_id: self.user_pool_id,
        }
    }
}
pub struct AppsyncApiEventConfigElAuthProviderElCognitoConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncApiEventConfigElAuthProviderElCognitoConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncApiEventConfigElAuthProviderElCognitoConfigElRef {
        AppsyncApiEventConfigElAuthProviderElCognitoConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppsyncApiEventConfigElAuthProviderElCognitoConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_id_client_regex` after provisioning.\n"]
    pub fn app_id_client_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_id_client_regex", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `aws_region` after provisioning.\n"]
    pub fn aws_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.base))
    }
    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.base))
    }
}
#[derive(Serialize)]
pub struct AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_result_ttl_in_seconds: Option<PrimField<f64>>,
    authorizer_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_validation_expression: Option<PrimField<String>>,
}
impl AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigEl {
    #[doc = "Set the field `authorizer_result_ttl_in_seconds`.\n"]
    pub fn set_authorizer_result_ttl_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.authorizer_result_ttl_in_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_validation_expression`.\n"]
    pub fn set_identity_validation_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_validation_expression = Some(v.into());
        self
    }
}
impl ToListMappable for AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigEl {
    type O = BlockAssignable<AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigEl {
    #[doc = ""]
    pub authorizer_uri: PrimField<String>,
}
impl BuildAppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigEl {
    pub fn build(self) -> AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigEl {
        AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigEl {
            authorizer_result_ttl_in_seconds: core::default::Default::default(),
            authorizer_uri: self.authorizer_uri,
            identity_validation_expression: core::default::Default::default(),
        }
    }
}
pub struct AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigElRef {
        AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorizer_result_ttl_in_seconds` after provisioning.\n"]
    pub fn authorizer_result_ttl_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorizer_result_ttl_in_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorizer_uri` after provisioning.\n"]
    pub fn authorizer_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorizer_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `identity_validation_expression` after provisioning.\n"]
    pub fn identity_validation_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_validation_expression", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iat_ttl: Option<PrimField<f64>>,
    issuer: PrimField<String>,
}
impl AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigEl {
    #[doc = "Set the field `auth_ttl`.\n"]
    pub fn set_auth_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.auth_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `iat_ttl`.\n"]
    pub fn set_iat_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iat_ttl = Some(v.into());
        self
    }
}
impl ToListMappable for AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigEl {
    type O = BlockAssignable<AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppsyncApiEventConfigElAuthProviderElOpenidConnectConfigEl {
    #[doc = ""]
    pub issuer: PrimField<String>,
}
impl BuildAppsyncApiEventConfigElAuthProviderElOpenidConnectConfigEl {
    pub fn build(self) -> AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigEl {
        AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigEl {
            auth_ttl: core::default::Default::default(),
            client_id: core::default::Default::default(),
            iat_ttl: core::default::Default::default(),
            issuer: self.issuer,
        }
    }
}
pub struct AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigElRef {
        AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_ttl` after provisioning.\n"]
    pub fn auth_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `iat_ttl` after provisioning.\n"]
    pub fn iat_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iat_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppsyncApiEventConfigElAuthProviderElDynamic {
    cognito_config: Option<DynamicBlock<AppsyncApiEventConfigElAuthProviderElCognitoConfigEl>>,
    lambda_authorizer_config:
        Option<DynamicBlock<AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigEl>>,
    openid_connect_config:
        Option<DynamicBlock<AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigEl>>,
}
#[derive(Serialize)]
pub struct AppsyncApiEventConfigElAuthProviderEl {
    auth_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cognito_config: Option<Vec<AppsyncApiEventConfigElAuthProviderElCognitoConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_authorizer_config:
        Option<Vec<AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    openid_connect_config: Option<Vec<AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigEl>>,
    dynamic: AppsyncApiEventConfigElAuthProviderElDynamic,
}
impl AppsyncApiEventConfigElAuthProviderEl {
    #[doc = "Set the field `cognito_config`.\n"]
    pub fn set_cognito_config(
        mut self,
        v: impl Into<BlockAssignable<AppsyncApiEventConfigElAuthProviderElCognitoConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cognito_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cognito_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lambda_authorizer_config`.\n"]
    pub fn set_lambda_authorizer_config(
        mut self,
        v: impl Into<BlockAssignable<AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_authorizer_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_authorizer_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `openid_connect_config`.\n"]
    pub fn set_openid_connect_config(
        mut self,
        v: impl Into<BlockAssignable<AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.openid_connect_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.openid_connect_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppsyncApiEventConfigElAuthProviderEl {
    type O = BlockAssignable<AppsyncApiEventConfigElAuthProviderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppsyncApiEventConfigElAuthProviderEl {
    #[doc = ""]
    pub auth_type: PrimField<String>,
}
impl BuildAppsyncApiEventConfigElAuthProviderEl {
    pub fn build(self) -> AppsyncApiEventConfigElAuthProviderEl {
        AppsyncApiEventConfigElAuthProviderEl {
            auth_type: self.auth_type,
            cognito_config: core::default::Default::default(),
            lambda_authorizer_config: core::default::Default::default(),
            openid_connect_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppsyncApiEventConfigElAuthProviderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncApiEventConfigElAuthProviderElRef {
    fn new(shared: StackShared, base: String) -> AppsyncApiEventConfigElAuthProviderElRef {
        AppsyncApiEventConfigElAuthProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppsyncApiEventConfigElAuthProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
    #[doc = "Get a reference to the value of field `cognito_config` after provisioning.\n"]
    pub fn cognito_config(
        &self,
    ) -> ListRef<AppsyncApiEventConfigElAuthProviderElCognitoConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cognito_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lambda_authorizer_config` after provisioning.\n"]
    pub fn lambda_authorizer_config(
        &self,
    ) -> ListRef<AppsyncApiEventConfigElAuthProviderElLambdaAuthorizerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_authorizer_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `openid_connect_config` after provisioning.\n"]
    pub fn openid_connect_config(
        &self,
    ) -> ListRef<AppsyncApiEventConfigElAuthProviderElOpenidConnectConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.openid_connect_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppsyncApiEventConfigElConnectionAuthModeEl {
    auth_type: PrimField<String>,
}
impl AppsyncApiEventConfigElConnectionAuthModeEl {}
impl ToListMappable for AppsyncApiEventConfigElConnectionAuthModeEl {
    type O = BlockAssignable<AppsyncApiEventConfigElConnectionAuthModeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppsyncApiEventConfigElConnectionAuthModeEl {
    #[doc = ""]
    pub auth_type: PrimField<String>,
}
impl BuildAppsyncApiEventConfigElConnectionAuthModeEl {
    pub fn build(self) -> AppsyncApiEventConfigElConnectionAuthModeEl {
        AppsyncApiEventConfigElConnectionAuthModeEl {
            auth_type: self.auth_type,
        }
    }
}
pub struct AppsyncApiEventConfigElConnectionAuthModeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncApiEventConfigElConnectionAuthModeElRef {
    fn new(shared: StackShared, base: String) -> AppsyncApiEventConfigElConnectionAuthModeElRef {
        AppsyncApiEventConfigElConnectionAuthModeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppsyncApiEventConfigElConnectionAuthModeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
}
#[derive(Serialize)]
pub struct AppsyncApiEventConfigElDefaultPublishAuthModeEl {
    auth_type: PrimField<String>,
}
impl AppsyncApiEventConfigElDefaultPublishAuthModeEl {}
impl ToListMappable for AppsyncApiEventConfigElDefaultPublishAuthModeEl {
    type O = BlockAssignable<AppsyncApiEventConfigElDefaultPublishAuthModeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppsyncApiEventConfigElDefaultPublishAuthModeEl {
    #[doc = ""]
    pub auth_type: PrimField<String>,
}
impl BuildAppsyncApiEventConfigElDefaultPublishAuthModeEl {
    pub fn build(self) -> AppsyncApiEventConfigElDefaultPublishAuthModeEl {
        AppsyncApiEventConfigElDefaultPublishAuthModeEl {
            auth_type: self.auth_type,
        }
    }
}
pub struct AppsyncApiEventConfigElDefaultPublishAuthModeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncApiEventConfigElDefaultPublishAuthModeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncApiEventConfigElDefaultPublishAuthModeElRef {
        AppsyncApiEventConfigElDefaultPublishAuthModeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppsyncApiEventConfigElDefaultPublishAuthModeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
}
#[derive(Serialize)]
pub struct AppsyncApiEventConfigElDefaultSubscribeAuthModeEl {
    auth_type: PrimField<String>,
}
impl AppsyncApiEventConfigElDefaultSubscribeAuthModeEl {}
impl ToListMappable for AppsyncApiEventConfigElDefaultSubscribeAuthModeEl {
    type O = BlockAssignable<AppsyncApiEventConfigElDefaultSubscribeAuthModeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppsyncApiEventConfigElDefaultSubscribeAuthModeEl {
    #[doc = ""]
    pub auth_type: PrimField<String>,
}
impl BuildAppsyncApiEventConfigElDefaultSubscribeAuthModeEl {
    pub fn build(self) -> AppsyncApiEventConfigElDefaultSubscribeAuthModeEl {
        AppsyncApiEventConfigElDefaultSubscribeAuthModeEl {
            auth_type: self.auth_type,
        }
    }
}
pub struct AppsyncApiEventConfigElDefaultSubscribeAuthModeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncApiEventConfigElDefaultSubscribeAuthModeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncApiEventConfigElDefaultSubscribeAuthModeElRef {
        AppsyncApiEventConfigElDefaultSubscribeAuthModeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppsyncApiEventConfigElDefaultSubscribeAuthModeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
}
#[derive(Serialize)]
pub struct AppsyncApiEventConfigElLogConfigEl {
    cloudwatch_logs_role_arn: PrimField<String>,
    log_level: PrimField<String>,
}
impl AppsyncApiEventConfigElLogConfigEl {}
impl ToListMappable for AppsyncApiEventConfigElLogConfigEl {
    type O = BlockAssignable<AppsyncApiEventConfigElLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppsyncApiEventConfigElLogConfigEl {
    #[doc = ""]
    pub cloudwatch_logs_role_arn: PrimField<String>,
    #[doc = ""]
    pub log_level: PrimField<String>,
}
impl BuildAppsyncApiEventConfigElLogConfigEl {
    pub fn build(self) -> AppsyncApiEventConfigElLogConfigEl {
        AppsyncApiEventConfigElLogConfigEl {
            cloudwatch_logs_role_arn: self.cloudwatch_logs_role_arn,
            log_level: self.log_level,
        }
    }
}
pub struct AppsyncApiEventConfigElLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncApiEventConfigElLogConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncApiEventConfigElLogConfigElRef {
        AppsyncApiEventConfigElLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppsyncApiEventConfigElLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloudwatch_logs_role_arn` after provisioning.\n"]
    pub fn cloudwatch_logs_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logs_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppsyncApiEventConfigElDynamic {
    auth_provider: Option<DynamicBlock<AppsyncApiEventConfigElAuthProviderEl>>,
    connection_auth_mode: Option<DynamicBlock<AppsyncApiEventConfigElConnectionAuthModeEl>>,
    default_publish_auth_mode:
        Option<DynamicBlock<AppsyncApiEventConfigElDefaultPublishAuthModeEl>>,
    default_subscribe_auth_mode:
        Option<DynamicBlock<AppsyncApiEventConfigElDefaultSubscribeAuthModeEl>>,
    log_config: Option<DynamicBlock<AppsyncApiEventConfigElLogConfigEl>>,
}
#[derive(Serialize)]
pub struct AppsyncApiEventConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_provider: Option<Vec<AppsyncApiEventConfigElAuthProviderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_auth_mode: Option<Vec<AppsyncApiEventConfigElConnectionAuthModeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_publish_auth_mode: Option<Vec<AppsyncApiEventConfigElDefaultPublishAuthModeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_subscribe_auth_mode: Option<Vec<AppsyncApiEventConfigElDefaultSubscribeAuthModeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_config: Option<Vec<AppsyncApiEventConfigElLogConfigEl>>,
    dynamic: AppsyncApiEventConfigElDynamic,
}
impl AppsyncApiEventConfigEl {
    #[doc = "Set the field `auth_provider`.\n"]
    pub fn set_auth_provider(
        mut self,
        v: impl Into<BlockAssignable<AppsyncApiEventConfigElAuthProviderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auth_provider = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_provider = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `connection_auth_mode`.\n"]
    pub fn set_connection_auth_mode(
        mut self,
        v: impl Into<BlockAssignable<AppsyncApiEventConfigElConnectionAuthModeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connection_auth_mode = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_auth_mode = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_publish_auth_mode`.\n"]
    pub fn set_default_publish_auth_mode(
        mut self,
        v: impl Into<BlockAssignable<AppsyncApiEventConfigElDefaultPublishAuthModeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_publish_auth_mode = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_publish_auth_mode = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_subscribe_auth_mode`.\n"]
    pub fn set_default_subscribe_auth_mode(
        mut self,
        v: impl Into<BlockAssignable<AppsyncApiEventConfigElDefaultSubscribeAuthModeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_subscribe_auth_mode = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_subscribe_auth_mode = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        mut self,
        v: impl Into<BlockAssignable<AppsyncApiEventConfigElLogConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.log_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.log_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppsyncApiEventConfigEl {
    type O = BlockAssignable<AppsyncApiEventConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppsyncApiEventConfigEl {}
impl BuildAppsyncApiEventConfigEl {
    pub fn build(self) -> AppsyncApiEventConfigEl {
        AppsyncApiEventConfigEl {
            auth_provider: core::default::Default::default(),
            connection_auth_mode: core::default::Default::default(),
            default_publish_auth_mode: core::default::Default::default(),
            default_subscribe_auth_mode: core::default::Default::default(),
            log_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppsyncApiEventConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncApiEventConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncApiEventConfigElRef {
        AppsyncApiEventConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppsyncApiEventConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_provider` after provisioning.\n"]
    pub fn auth_provider(&self) -> ListRef<AppsyncApiEventConfigElAuthProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auth_provider", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `connection_auth_mode` after provisioning.\n"]
    pub fn connection_auth_mode(&self) -> ListRef<AppsyncApiEventConfigElConnectionAuthModeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_auth_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_publish_auth_mode` after provisioning.\n"]
    pub fn default_publish_auth_mode(
        &self,
    ) -> ListRef<AppsyncApiEventConfigElDefaultPublishAuthModeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_publish_auth_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_subscribe_auth_mode` after provisioning.\n"]
    pub fn default_subscribe_auth_mode(
        &self,
    ) -> ListRef<AppsyncApiEventConfigElDefaultSubscribeAuthModeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_subscribe_auth_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<AppsyncApiEventConfigElLogConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppsyncApiDynamic {
    event_config: Option<DynamicBlock<AppsyncApiEventConfigEl>>,
}
