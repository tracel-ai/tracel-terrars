use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApiGatewayDomainNameData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_chain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_private_key: Option<PrimField<String>>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ownership_verification_certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regional_certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regional_certificate_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_configuration: Option<Vec<ApiGatewayDomainNameEndpointConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutual_tls_authentication: Option<Vec<ApiGatewayDomainNameMutualTlsAuthenticationEl>>,
    dynamic: ApiGatewayDomainNameDynamic,
}

struct ApiGatewayDomainName_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApiGatewayDomainNameData>,
}

#[derive(Clone)]
pub struct ApiGatewayDomainName(Rc<ApiGatewayDomainName_>);

impl ApiGatewayDomainName {
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

    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_body`.\n"]
    pub fn set_certificate_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_body = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_chain = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_name`.\n"]
    pub fn set_certificate_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_name = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_private_key`.\n"]
    pub fn set_certificate_private_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_private_key = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `ownership_verification_certificate_arn`.\n"]
    pub fn set_ownership_verification_certificate_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ownership_verification_certificate_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `regional_certificate_arn`.\n"]
    pub fn set_regional_certificate_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().regional_certificate_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `regional_certificate_name`.\n"]
    pub fn set_regional_certificate_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().regional_certificate_name = Some(v.into());
        self
    }

    #[doc = "Set the field `security_policy`.\n"]
    pub fn set_security_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_policy = Some(v.into());
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

    #[doc = "Set the field `endpoint_configuration`.\n"]
    pub fn set_endpoint_configuration(
        self,
        v: impl Into<BlockAssignable<ApiGatewayDomainNameEndpointConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mutual_tls_authentication`.\n"]
    pub fn set_mutual_tls_authentication(
        self,
        v: impl Into<BlockAssignable<ApiGatewayDomainNameMutualTlsAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mutual_tls_authentication = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mutual_tls_authentication = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_body` after provisioning.\n"]
    pub fn certificate_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_name` after provisioning.\n"]
    pub fn certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_private_key` after provisioning.\n"]
    pub fn certificate_private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_private_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_upload_date` after provisioning.\n"]
    pub fn certificate_upload_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_upload_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudfront_domain_name` after provisioning.\n"]
    pub fn cloudfront_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudfront_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudfront_zone_id` after provisioning.\n"]
    pub fn cloudfront_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudfront_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name_id` after provisioning.\n"]
    pub fn domain_name_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ownership_verification_certificate_arn` after provisioning.\n"]
    pub fn ownership_verification_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ownership_verification_certificate_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_certificate_arn` after provisioning.\n"]
    pub fn regional_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regional_certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_certificate_name` after provisioning.\n"]
    pub fn regional_certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regional_certificate_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_domain_name` after provisioning.\n"]
    pub fn regional_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regional_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_zone_id` after provisioning.\n"]
    pub fn regional_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regional_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\n"]
    pub fn security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_configuration` after provisioning.\n"]
    pub fn endpoint_configuration(&self) -> ListRef<ApiGatewayDomainNameEndpointConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoint_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mutual_tls_authentication` after provisioning.\n"]
    pub fn mutual_tls_authentication(&self) -> ListRef<ApiGatewayDomainNameMutualTlsAuthenticationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mutual_tls_authentication", self.extract_ref()))
    }
}

impl Referable for ApiGatewayDomainName {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApiGatewayDomainName { }

impl ToListMappable for ApiGatewayDomainName {
    type O = ListRef<ApiGatewayDomainNameRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApiGatewayDomainName_ {
    fn extract_resource_type(&self) -> String {
        "aws_api_gateway_domain_name".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayDomainName {
    pub tf_id: String,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}

impl BuildApiGatewayDomainName {
    pub fn build(self, stack: &mut Stack) -> ApiGatewayDomainName {
        let out = ApiGatewayDomainName(Rc::new(ApiGatewayDomainName_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApiGatewayDomainNameData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                certificate_arn: core::default::Default::default(),
                certificate_body: core::default::Default::default(),
                certificate_chain: core::default::Default::default(),
                certificate_name: core::default::Default::default(),
                certificate_private_key: core::default::Default::default(),
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                ownership_verification_certificate_arn: core::default::Default::default(),
                policy: core::default::Default::default(),
                region: core::default::Default::default(),
                regional_certificate_arn: core::default::Default::default(),
                regional_certificate_name: core::default::Default::default(),
                security_policy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                endpoint_configuration: core::default::Default::default(),
                mutual_tls_authentication: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayDomainNameRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayDomainNameRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ApiGatewayDomainNameRef {
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

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_body` after provisioning.\n"]
    pub fn certificate_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_name` after provisioning.\n"]
    pub fn certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_private_key` after provisioning.\n"]
    pub fn certificate_private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_private_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_upload_date` after provisioning.\n"]
    pub fn certificate_upload_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_upload_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudfront_domain_name` after provisioning.\n"]
    pub fn cloudfront_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudfront_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudfront_zone_id` after provisioning.\n"]
    pub fn cloudfront_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudfront_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name_id` after provisioning.\n"]
    pub fn domain_name_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ownership_verification_certificate_arn` after provisioning.\n"]
    pub fn ownership_verification_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ownership_verification_certificate_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_certificate_arn` after provisioning.\n"]
    pub fn regional_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regional_certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_certificate_name` after provisioning.\n"]
    pub fn regional_certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regional_certificate_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_domain_name` after provisioning.\n"]
    pub fn regional_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regional_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_zone_id` after provisioning.\n"]
    pub fn regional_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regional_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\n"]
    pub fn security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_configuration` after provisioning.\n"]
    pub fn endpoint_configuration(&self) -> ListRef<ApiGatewayDomainNameEndpointConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoint_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mutual_tls_authentication` after provisioning.\n"]
    pub fn mutual_tls_authentication(&self) -> ListRef<ApiGatewayDomainNameMutualTlsAuthenticationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mutual_tls_authentication", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiGatewayDomainNameEndpointConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    types: ListField<PrimField<String>>,
}

impl ApiGatewayDomainNameEndpointConfigurationEl {
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address_type = Some(v.into());
        self
    }
}

impl ToListMappable for ApiGatewayDomainNameEndpointConfigurationEl {
    type O = BlockAssignable<ApiGatewayDomainNameEndpointConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayDomainNameEndpointConfigurationEl {
    #[doc = ""]
    pub types: ListField<PrimField<String>>,
}

impl BuildApiGatewayDomainNameEndpointConfigurationEl {
    pub fn build(self) -> ApiGatewayDomainNameEndpointConfigurationEl {
        ApiGatewayDomainNameEndpointConfigurationEl {
            ip_address_type: core::default::Default::default(),
            types: self.types,
        }
    }
}

pub struct ApiGatewayDomainNameEndpointConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayDomainNameEndpointConfigurationElRef {
    fn new(shared: StackShared, base: String) -> ApiGatewayDomainNameEndpointConfigurationElRef {
        ApiGatewayDomainNameEndpointConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayDomainNameEndpointConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `types` after provisioning.\n"]
    pub fn types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiGatewayDomainNameMutualTlsAuthenticationEl {
    truststore_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    truststore_version: Option<PrimField<String>>,
}

impl ApiGatewayDomainNameMutualTlsAuthenticationEl {
    #[doc = "Set the field `truststore_version`.\n"]
    pub fn set_truststore_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.truststore_version = Some(v.into());
        self
    }
}

impl ToListMappable for ApiGatewayDomainNameMutualTlsAuthenticationEl {
    type O = BlockAssignable<ApiGatewayDomainNameMutualTlsAuthenticationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayDomainNameMutualTlsAuthenticationEl {
    #[doc = ""]
    pub truststore_uri: PrimField<String>,
}

impl BuildApiGatewayDomainNameMutualTlsAuthenticationEl {
    pub fn build(self) -> ApiGatewayDomainNameMutualTlsAuthenticationEl {
        ApiGatewayDomainNameMutualTlsAuthenticationEl {
            truststore_uri: self.truststore_uri,
            truststore_version: core::default::Default::default(),
        }
    }
}

pub struct ApiGatewayDomainNameMutualTlsAuthenticationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayDomainNameMutualTlsAuthenticationElRef {
    fn new(shared: StackShared, base: String) -> ApiGatewayDomainNameMutualTlsAuthenticationElRef {
        ApiGatewayDomainNameMutualTlsAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayDomainNameMutualTlsAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `truststore_uri` after provisioning.\n"]
    pub fn truststore_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.truststore_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `truststore_version` after provisioning.\n"]
    pub fn truststore_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.truststore_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApiGatewayDomainNameDynamic {
    endpoint_configuration: Option<DynamicBlock<ApiGatewayDomainNameEndpointConfigurationEl>>,
    mutual_tls_authentication: Option<DynamicBlock<ApiGatewayDomainNameMutualTlsAuthenticationEl>>,
}
