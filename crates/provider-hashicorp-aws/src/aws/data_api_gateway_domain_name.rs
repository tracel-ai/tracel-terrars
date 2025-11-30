use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataApiGatewayDomainNameData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataApiGatewayDomainName_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataApiGatewayDomainNameData>,
}

#[derive(Clone)]
pub struct DataApiGatewayDomainName(Rc<DataApiGatewayDomainName_>);

impl DataApiGatewayDomainName {
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

    #[doc = "Set the field `domain_name_id`.\n"]
    pub fn set_domain_name_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_name_id = Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `certificate_name` after provisioning.\n"]
    pub fn certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `certificate_upload_date` after provisioning.\n"]
    pub fn certificate_upload_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_upload_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cloudfront_domain_name` after provisioning.\n"]
    pub fn cloudfront_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudfront_domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cloudfront_zone_id` after provisioning.\n"]
    pub fn cloudfront_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudfront_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name_id` after provisioning.\n"]
    pub fn domain_name_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_configuration` after provisioning.\n"]
    pub fn endpoint_configuration(
        &self,
    ) -> ListRef<DataApiGatewayDomainNameEndpointConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `regional_certificate_arn` after provisioning.\n"]
    pub fn regional_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.regional_certificate_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `regional_certificate_name` after provisioning.\n"]
    pub fn regional_certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.regional_certificate_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `regional_domain_name` after provisioning.\n"]
    pub fn regional_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.regional_domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `regional_zone_id` after provisioning.\n"]
    pub fn regional_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.regional_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\n"]
    pub fn security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy", self.extract_ref()),
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

impl Referable for DataApiGatewayDomainName {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataApiGatewayDomainName {}

impl ToListMappable for DataApiGatewayDomainName {
    type O = ListRef<DataApiGatewayDomainNameRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataApiGatewayDomainName_ {
    fn extract_datasource_type(&self) -> String {
        "aws_api_gateway_domain_name".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApiGatewayDomainName {
    pub tf_id: String,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}

impl BuildDataApiGatewayDomainName {
    pub fn build(self, stack: &mut Stack) -> DataApiGatewayDomainName {
        let out = DataApiGatewayDomainName(Rc::new(DataApiGatewayDomainName_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataApiGatewayDomainNameData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                domain_name: self.domain_name,
                domain_name_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataApiGatewayDomainNameRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataApiGatewayDomainNameRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataApiGatewayDomainNameRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `certificate_name` after provisioning.\n"]
    pub fn certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `certificate_upload_date` after provisioning.\n"]
    pub fn certificate_upload_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_upload_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cloudfront_domain_name` after provisioning.\n"]
    pub fn cloudfront_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudfront_domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cloudfront_zone_id` after provisioning.\n"]
    pub fn cloudfront_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudfront_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name_id` after provisioning.\n"]
    pub fn domain_name_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_configuration` after provisioning.\n"]
    pub fn endpoint_configuration(
        &self,
    ) -> ListRef<DataApiGatewayDomainNameEndpointConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `regional_certificate_arn` after provisioning.\n"]
    pub fn regional_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.regional_certificate_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `regional_certificate_name` after provisioning.\n"]
    pub fn regional_certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.regional_certificate_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `regional_domain_name` after provisioning.\n"]
    pub fn regional_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.regional_domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `regional_zone_id` after provisioning.\n"]
    pub fn regional_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.regional_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\n"]
    pub fn security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy", self.extract_ref()),
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
pub struct DataApiGatewayDomainNameEndpointConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<ListField<PrimField<String>>>,
}

impl DataApiGatewayDomainNameEndpointConfigurationEl {
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address_type = Some(v.into());
        self
    }

    #[doc = "Set the field `types`.\n"]
    pub fn set_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.types = Some(v.into());
        self
    }
}

impl ToListMappable for DataApiGatewayDomainNameEndpointConfigurationEl {
    type O = BlockAssignable<DataApiGatewayDomainNameEndpointConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiGatewayDomainNameEndpointConfigurationEl {}

impl BuildDataApiGatewayDomainNameEndpointConfigurationEl {
    pub fn build(self) -> DataApiGatewayDomainNameEndpointConfigurationEl {
        DataApiGatewayDomainNameEndpointConfigurationEl {
            ip_address_type: core::default::Default::default(),
            types: core::default::Default::default(),
        }
    }
}

pub struct DataApiGatewayDomainNameEndpointConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataApiGatewayDomainNameEndpointConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataApiGatewayDomainNameEndpointConfigurationElRef {
        DataApiGatewayDomainNameEndpointConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiGatewayDomainNameEndpointConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `types` after provisioning.\n"]
    pub fn types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}
