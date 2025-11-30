use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SsoadminTrustedTokenIssuerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_token: Option<PrimField<String>>,
    instance_arn: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    trusted_token_issuer_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trusted_token_issuer_configuration:
        Option<Vec<SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationEl>>,
    dynamic: SsoadminTrustedTokenIssuerDynamic,
}

struct SsoadminTrustedTokenIssuer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsoadminTrustedTokenIssuerData>,
}

#[derive(Clone)]
pub struct SsoadminTrustedTokenIssuer(Rc<SsoadminTrustedTokenIssuer_>);

impl SsoadminTrustedTokenIssuer {
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

    #[doc = "Set the field `client_token`.\n"]
    pub fn set_client_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_token = Some(v.into());
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

    #[doc = "Set the field `trusted_token_issuer_configuration`.\n"]
    pub fn set_trusted_token_issuer_configuration(
        self,
        v: impl Into<BlockAssignable<SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trusted_token_issuer_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .trusted_token_issuer_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_token", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_arn", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `trusted_token_issuer_type` after provisioning.\n"]
    pub fn trusted_token_issuer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trusted_token_issuer_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `trusted_token_issuer_configuration` after provisioning.\n"]
    pub fn trusted_token_issuer_configuration(
        &self,
    ) -> ListRef<SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_token_issuer_configuration", self.extract_ref()),
        )
    }
}

impl Referable for SsoadminTrustedTokenIssuer {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SsoadminTrustedTokenIssuer {}

impl ToListMappable for SsoadminTrustedTokenIssuer {
    type O = ListRef<SsoadminTrustedTokenIssuerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SsoadminTrustedTokenIssuer_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssoadmin_trusted_token_issuer".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsoadminTrustedTokenIssuer {
    pub tf_id: String,
    #[doc = ""]
    pub instance_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub trusted_token_issuer_type: PrimField<String>,
}

impl BuildSsoadminTrustedTokenIssuer {
    pub fn build(self, stack: &mut Stack) -> SsoadminTrustedTokenIssuer {
        let out = SsoadminTrustedTokenIssuer(Rc::new(SsoadminTrustedTokenIssuer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SsoadminTrustedTokenIssuerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                client_token: core::default::Default::default(),
                instance_arn: self.instance_arn,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                trusted_token_issuer_type: self.trusted_token_issuer_type,
                trusted_token_issuer_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsoadminTrustedTokenIssuerRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminTrustedTokenIssuerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SsoadminTrustedTokenIssuerRef {
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

    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_token", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_arn", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `trusted_token_issuer_type` after provisioning.\n"]
    pub fn trusted_token_issuer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trusted_token_issuer_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `trusted_token_issuer_configuration` after provisioning.\n"]
    pub fn trusted_token_issuer_configuration(
        &self,
    ) -> ListRef<SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_token_issuer_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationEl {
    claim_attribute_path: PrimField<String>,
    identity_store_attribute_path: PrimField<String>,
    issuer_url: PrimField<String>,
    jwks_retrieval_option: PrimField<String>,
}

impl SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationEl {}

impl ToListMappable
    for SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationEl
{
    type O = BlockAssignable<
        SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationEl {
    #[doc = ""]
    pub claim_attribute_path: PrimField<String>,
    #[doc = ""]
    pub identity_store_attribute_path: PrimField<String>,
    #[doc = ""]
    pub issuer_url: PrimField<String>,
    #[doc = ""]
    pub jwks_retrieval_option: PrimField<String>,
}

impl BuildSsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationEl {
    pub fn build(
        self,
    ) -> SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationEl {
        SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationEl {
            claim_attribute_path: self.claim_attribute_path,
            identity_store_attribute_path: self.identity_store_attribute_path,
            issuer_url: self.issuer_url,
            jwks_retrieval_option: self.jwks_retrieval_option,
        }
    }
}

pub struct SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationElRef {
        SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_attribute_path` after provisioning.\n"]
    pub fn claim_attribute_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.claim_attribute_path", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `identity_store_attribute_path` after provisioning.\n"]
    pub fn identity_store_attribute_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_store_attribute_path", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `issuer_url` after provisioning.\n"]
    pub fn issuer_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer_url", self.base))
    }

    #[doc = "Get a reference to the value of field `jwks_retrieval_option` after provisioning.\n"]
    pub fn jwks_retrieval_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.jwks_retrieval_option", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElDynamic {
    oidc_jwt_configuration: Option<
        DynamicBlock<
            SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc_jwt_configuration: Option<
        Vec<SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationEl>,
    >,
    dynamic: SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElDynamic,
}

impl SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationEl {
    #[doc = "Set the field `oidc_jwt_configuration`.\n"]
    pub fn set_oidc_jwt_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oidc_jwt_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oidc_jwt_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationEl {
    type O = BlockAssignable<SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationEl {}

impl BuildSsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationEl {
    pub fn build(self) -> SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationEl {
        SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationEl {
            oidc_jwt_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElRef {
        SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oidc_jwt_configuration` after provisioning.\n"]
    pub fn oidc_jwt_configuration(
        &self,
    ) -> ListRef<SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationElOidcJwtConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oidc_jwt_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SsoadminTrustedTokenIssuerDynamic {
    trusted_token_issuer_configuration:
        Option<DynamicBlock<SsoadminTrustedTokenIssuerTrustedTokenIssuerConfigurationEl>>,
}
