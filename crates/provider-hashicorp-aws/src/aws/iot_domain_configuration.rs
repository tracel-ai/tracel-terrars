use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct IotDomainConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_certificate_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_config: Option<Vec<IotDomainConfigurationAuthorizerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<Vec<IotDomainConfigurationTlsConfigEl>>,
    dynamic: IotDomainConfigurationDynamic,
}

struct IotDomainConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IotDomainConfigurationData>,
}

#[derive(Clone)]
pub struct IotDomainConfiguration(Rc<IotDomainConfiguration_>);

impl IotDomainConfiguration {
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

    #[doc = "Set the field `application_protocol`.\n"]
    pub fn set_application_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().application_protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_type`.\n"]
    pub fn set_authentication_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authentication_type = Some(v.into());
        self
    }

    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_name = Some(v.into());
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

    #[doc = "Set the field `server_certificate_arns`.\n"]
    pub fn set_server_certificate_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().server_certificate_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `service_type`.\n"]
    pub fn set_service_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_type = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
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

    #[doc = "Set the field `validation_certificate_arn`.\n"]
    pub fn set_validation_certificate_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().validation_certificate_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_config`.\n"]
    pub fn set_authorizer_config(
        self,
        v: impl Into<BlockAssignable<IotDomainConfigurationAuthorizerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authorizer_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authorizer_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        self,
        v: impl Into<BlockAssignable<IotDomainConfigurationTlsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tls_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tls_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `application_protocol` after provisioning.\n"]
    pub fn application_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_protocol", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_type` after provisioning.\n"]
    pub fn domain_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `server_certificate_arns` after provisioning.\n"]
    pub fn server_certificate_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.server_certificate_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_type` after provisioning.\n"]
    pub fn service_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `validation_certificate_arn` after provisioning.\n"]
    pub fn validation_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.validation_certificate_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authorizer_config` after provisioning.\n"]
    pub fn authorizer_config(&self) -> ListRef<IotDomainConfigurationAuthorizerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<IotDomainConfigurationTlsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_config", self.extract_ref()),
        )
    }
}

impl Referable for IotDomainConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for IotDomainConfiguration {}

impl ToListMappable for IotDomainConfiguration {
    type O = ListRef<IotDomainConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IotDomainConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_iot_domain_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIotDomainConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildIotDomainConfiguration {
    pub fn build(self, stack: &mut Stack) -> IotDomainConfiguration {
        let out = IotDomainConfiguration(Rc::new(IotDomainConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IotDomainConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                application_protocol: core::default::Default::default(),
                authentication_type: core::default::Default::default(),
                domain_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                server_certificate_arns: core::default::Default::default(),
                service_type: core::default::Default::default(),
                status: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                validation_certificate_arn: core::default::Default::default(),
                authorizer_config: core::default::Default::default(),
                tls_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IotDomainConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotDomainConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl IotDomainConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_protocol` after provisioning.\n"]
    pub fn application_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_protocol", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_type` after provisioning.\n"]
    pub fn domain_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `server_certificate_arns` after provisioning.\n"]
    pub fn server_certificate_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.server_certificate_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_type` after provisioning.\n"]
    pub fn service_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `validation_certificate_arn` after provisioning.\n"]
    pub fn validation_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.validation_certificate_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authorizer_config` after provisioning.\n"]
    pub fn authorizer_config(&self) -> ListRef<IotDomainConfigurationAuthorizerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<IotDomainConfigurationTlsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tls_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct IotDomainConfigurationAuthorizerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_authorizer_override: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_authorizer_name: Option<PrimField<String>>,
}

impl IotDomainConfigurationAuthorizerConfigEl {
    #[doc = "Set the field `allow_authorizer_override`.\n"]
    pub fn set_allow_authorizer_override(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_authorizer_override = Some(v.into());
        self
    }

    #[doc = "Set the field `default_authorizer_name`.\n"]
    pub fn set_default_authorizer_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_authorizer_name = Some(v.into());
        self
    }
}

impl ToListMappable for IotDomainConfigurationAuthorizerConfigEl {
    type O = BlockAssignable<IotDomainConfigurationAuthorizerConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotDomainConfigurationAuthorizerConfigEl {}

impl BuildIotDomainConfigurationAuthorizerConfigEl {
    pub fn build(self) -> IotDomainConfigurationAuthorizerConfigEl {
        IotDomainConfigurationAuthorizerConfigEl {
            allow_authorizer_override: core::default::Default::default(),
            default_authorizer_name: core::default::Default::default(),
        }
    }
}

pub struct IotDomainConfigurationAuthorizerConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotDomainConfigurationAuthorizerConfigElRef {
    fn new(shared: StackShared, base: String) -> IotDomainConfigurationAuthorizerConfigElRef {
        IotDomainConfigurationAuthorizerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotDomainConfigurationAuthorizerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_authorizer_override` after provisioning.\n"]
    pub fn allow_authorizer_override(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_authorizer_override", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `default_authorizer_name` after provisioning.\n"]
    pub fn default_authorizer_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_authorizer_name", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct IotDomainConfigurationTlsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_policy: Option<PrimField<String>>,
}

impl IotDomainConfigurationTlsConfigEl {
    #[doc = "Set the field `security_policy`.\n"]
    pub fn set_security_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_policy = Some(v.into());
        self
    }
}

impl ToListMappable for IotDomainConfigurationTlsConfigEl {
    type O = BlockAssignable<IotDomainConfigurationTlsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotDomainConfigurationTlsConfigEl {}

impl BuildIotDomainConfigurationTlsConfigEl {
    pub fn build(self) -> IotDomainConfigurationTlsConfigEl {
        IotDomainConfigurationTlsConfigEl {
            security_policy: core::default::Default::default(),
        }
    }
}

pub struct IotDomainConfigurationTlsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotDomainConfigurationTlsConfigElRef {
    fn new(shared: StackShared, base: String) -> IotDomainConfigurationTlsConfigElRef {
        IotDomainConfigurationTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotDomainConfigurationTlsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\n"]
    pub fn security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct IotDomainConfigurationDynamic {
    authorizer_config: Option<DynamicBlock<IotDomainConfigurationAuthorizerConfigEl>>,
    tls_config: Option<DynamicBlock<IotDomainConfigurationTlsConfigEl>>,
}
