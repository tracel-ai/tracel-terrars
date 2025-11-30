use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct Apigatewayv2DomainNameData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name_configuration: Option<Vec<Apigatewayv2DomainNameDomainNameConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutual_tls_authentication: Option<Vec<Apigatewayv2DomainNameMutualTlsAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Apigatewayv2DomainNameTimeoutsEl>,
    dynamic: Apigatewayv2DomainNameDynamic,
}

struct Apigatewayv2DomainName_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Apigatewayv2DomainNameData>,
}

#[derive(Clone)]
pub struct Apigatewayv2DomainName(Rc<Apigatewayv2DomainName_>);

impl Apigatewayv2DomainName {
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

    #[doc = "Set the field `domain_name_configuration`.\n"]
    pub fn set_domain_name_configuration(
        self,
        v: impl Into<BlockAssignable<Apigatewayv2DomainNameDomainNameConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().domain_name_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.domain_name_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `mutual_tls_authentication`.\n"]
    pub fn set_mutual_tls_authentication(
        self,
        v: impl Into<BlockAssignable<Apigatewayv2DomainNameMutualTlsAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mutual_tls_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mutual_tls_authentication = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Apigatewayv2DomainNameTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `api_mapping_selection_expression` after provisioning.\n"]
    pub fn api_mapping_selection_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_mapping_selection_expression", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `domain_name_configuration` after provisioning.\n"]
    pub fn domain_name_configuration(
        &self,
    ) -> ListRef<Apigatewayv2DomainNameDomainNameConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_name_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mutual_tls_authentication` after provisioning.\n"]
    pub fn mutual_tls_authentication(
        &self,
    ) -> ListRef<Apigatewayv2DomainNameMutualTlsAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mutual_tls_authentication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Apigatewayv2DomainNameTimeoutsElRef {
        Apigatewayv2DomainNameTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for Apigatewayv2DomainName {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Apigatewayv2DomainName {}

impl ToListMappable for Apigatewayv2DomainName {
    type O = ListRef<Apigatewayv2DomainNameRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Apigatewayv2DomainName_ {
    fn extract_resource_type(&self) -> String {
        "aws_apigatewayv2_domain_name".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigatewayv2DomainName {
    pub tf_id: String,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}

impl BuildApigatewayv2DomainName {
    pub fn build(self, stack: &mut Stack) -> Apigatewayv2DomainName {
        let out = Apigatewayv2DomainName(Rc::new(Apigatewayv2DomainName_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Apigatewayv2DomainNameData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                domain_name_configuration: core::default::Default::default(),
                mutual_tls_authentication: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Apigatewayv2DomainNameRef {
    shared: StackShared,
    base: String,
}

impl Ref for Apigatewayv2DomainNameRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl Apigatewayv2DomainNameRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_mapping_selection_expression` after provisioning.\n"]
    pub fn api_mapping_selection_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_mapping_selection_expression", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `domain_name_configuration` after provisioning.\n"]
    pub fn domain_name_configuration(
        &self,
    ) -> ListRef<Apigatewayv2DomainNameDomainNameConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_name_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `mutual_tls_authentication` after provisioning.\n"]
    pub fn mutual_tls_authentication(
        &self,
    ) -> ListRef<Apigatewayv2DomainNameMutualTlsAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mutual_tls_authentication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Apigatewayv2DomainNameTimeoutsElRef {
        Apigatewayv2DomainNameTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2DomainNameDomainNameConfigurationEl {
    certificate_arn: PrimField<String>,
    endpoint_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ownership_verification_certificate_arn: Option<PrimField<String>>,
    security_policy: PrimField<String>,
}

impl Apigatewayv2DomainNameDomainNameConfigurationEl {
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address_type = Some(v.into());
        self
    }

    #[doc = "Set the field `ownership_verification_certificate_arn`.\n"]
    pub fn set_ownership_verification_certificate_arn(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.ownership_verification_certificate_arn = Some(v.into());
        self
    }
}

impl ToListMappable for Apigatewayv2DomainNameDomainNameConfigurationEl {
    type O = BlockAssignable<Apigatewayv2DomainNameDomainNameConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2DomainNameDomainNameConfigurationEl {
    #[doc = ""]
    pub certificate_arn: PrimField<String>,
    #[doc = ""]
    pub endpoint_type: PrimField<String>,
    #[doc = ""]
    pub security_policy: PrimField<String>,
}

impl BuildApigatewayv2DomainNameDomainNameConfigurationEl {
    pub fn build(self) -> Apigatewayv2DomainNameDomainNameConfigurationEl {
        Apigatewayv2DomainNameDomainNameConfigurationEl {
            certificate_arn: self.certificate_arn,
            endpoint_type: self.endpoint_type,
            ip_address_type: core::default::Default::default(),
            ownership_verification_certificate_arn: core::default::Default::default(),
            security_policy: self.security_policy,
        }
    }
}

pub struct Apigatewayv2DomainNameDomainNameConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Apigatewayv2DomainNameDomainNameConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Apigatewayv2DomainNameDomainNameConfigurationElRef {
        Apigatewayv2DomainNameDomainNameConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2DomainNameDomainNameConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ownership_verification_certificate_arn` after provisioning.\n"]
    pub fn ownership_verification_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ownership_verification_certificate_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\n"]
    pub fn security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `target_domain_name` after provisioning.\n"]
    pub fn target_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_domain_name", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2DomainNameMutualTlsAuthenticationEl {
    truststore_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    truststore_version: Option<PrimField<String>>,
}

impl Apigatewayv2DomainNameMutualTlsAuthenticationEl {
    #[doc = "Set the field `truststore_version`.\n"]
    pub fn set_truststore_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.truststore_version = Some(v.into());
        self
    }
}

impl ToListMappable for Apigatewayv2DomainNameMutualTlsAuthenticationEl {
    type O = BlockAssignable<Apigatewayv2DomainNameMutualTlsAuthenticationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2DomainNameMutualTlsAuthenticationEl {
    #[doc = ""]
    pub truststore_uri: PrimField<String>,
}

impl BuildApigatewayv2DomainNameMutualTlsAuthenticationEl {
    pub fn build(self) -> Apigatewayv2DomainNameMutualTlsAuthenticationEl {
        Apigatewayv2DomainNameMutualTlsAuthenticationEl {
            truststore_uri: self.truststore_uri,
            truststore_version: core::default::Default::default(),
        }
    }
}

pub struct Apigatewayv2DomainNameMutualTlsAuthenticationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Apigatewayv2DomainNameMutualTlsAuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Apigatewayv2DomainNameMutualTlsAuthenticationElRef {
        Apigatewayv2DomainNameMutualTlsAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2DomainNameMutualTlsAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `truststore_uri` after provisioning.\n"]
    pub fn truststore_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.truststore_uri", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `truststore_version` after provisioning.\n"]
    pub fn truststore_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.truststore_version", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2DomainNameTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl Apigatewayv2DomainNameTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for Apigatewayv2DomainNameTimeoutsEl {
    type O = BlockAssignable<Apigatewayv2DomainNameTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2DomainNameTimeoutsEl {}

impl BuildApigatewayv2DomainNameTimeoutsEl {
    pub fn build(self) -> Apigatewayv2DomainNameTimeoutsEl {
        Apigatewayv2DomainNameTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct Apigatewayv2DomainNameTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Apigatewayv2DomainNameTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Apigatewayv2DomainNameTimeoutsElRef {
        Apigatewayv2DomainNameTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2DomainNameTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct Apigatewayv2DomainNameDynamic {
    domain_name_configuration:
        Option<DynamicBlock<Apigatewayv2DomainNameDomainNameConfigurationEl>>,
    mutual_tls_authentication:
        Option<DynamicBlock<Apigatewayv2DomainNameMutualTlsAuthenticationEl>>,
}
