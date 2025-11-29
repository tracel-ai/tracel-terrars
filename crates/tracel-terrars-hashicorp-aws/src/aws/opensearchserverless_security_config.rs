use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpensearchserverlessSecurityConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml_options: Option<Vec<OpensearchserverlessSecurityConfigSamlOptionsEl>>,
    dynamic: OpensearchserverlessSecurityConfigDynamic,
}

struct OpensearchserverlessSecurityConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OpensearchserverlessSecurityConfigData>,
}

#[derive(Clone)]
pub struct OpensearchserverlessSecurityConfig(Rc<OpensearchserverlessSecurityConfig_>);

impl OpensearchserverlessSecurityConfig {
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

    #[doc = "Set the field `description`.\nDescription of the security configuration."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `saml_options`.\n"]
    pub fn set_saml_options(
        self,
        v: impl Into<BlockAssignable<OpensearchserverlessSecurityConfigSamlOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().saml_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.saml_options = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `config_version` after provisioning.\nVersion of the configuration."]
    pub fn config_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the security configuration."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of configuration. Must be `saml`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_options` after provisioning.\n"]
    pub fn saml_options(&self) -> ListRef<OpensearchserverlessSecurityConfigSamlOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saml_options", self.extract_ref()))
    }
}

impl Referable for OpensearchserverlessSecurityConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for OpensearchserverlessSecurityConfig { }

impl ToListMappable for OpensearchserverlessSecurityConfig {
    type O = ListRef<OpensearchserverlessSecurityConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OpensearchserverlessSecurityConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_opensearchserverless_security_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpensearchserverlessSecurityConfig {
    pub tf_id: String,
    #[doc = "Name of the policy."]
    pub name: PrimField<String>,
    #[doc = "Type of configuration. Must be `saml`."]
    pub type_: PrimField<String>,
}

impl BuildOpensearchserverlessSecurityConfig {
    pub fn build(self, stack: &mut Stack) -> OpensearchserverlessSecurityConfig {
        let out = OpensearchserverlessSecurityConfig(Rc::new(OpensearchserverlessSecurityConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OpensearchserverlessSecurityConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                type_: self.type_,
                saml_options: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpensearchserverlessSecurityConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpensearchserverlessSecurityConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl OpensearchserverlessSecurityConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config_version` after provisioning.\nVersion of the configuration."]
    pub fn config_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the security configuration."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of configuration. Must be `saml`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_options` after provisioning.\n"]
    pub fn saml_options(&self) -> ListRef<OpensearchserverlessSecurityConfigSamlOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saml_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OpensearchserverlessSecurityConfigSamlOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_attribute: Option<PrimField<String>>,
    metadata: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_attribute: Option<PrimField<String>>,
}

impl OpensearchserverlessSecurityConfigSamlOptionsEl {
    #[doc = "Set the field `group_attribute`.\nGroup attribute for this SAML integration."]
    pub fn set_group_attribute(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_attribute = Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_timeout`.\nSession timeout, in minutes. Minimum is 5 minutes and maximum is 720 minutes (12 hours). Default is 60 minutes."]
    pub fn set_session_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.session_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `user_attribute`.\nUser attribute for this SAML integration."]
    pub fn set_user_attribute(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_attribute = Some(v.into());
        self
    }
}

impl ToListMappable for OpensearchserverlessSecurityConfigSamlOptionsEl {
    type O = BlockAssignable<OpensearchserverlessSecurityConfigSamlOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchserverlessSecurityConfigSamlOptionsEl {
    #[doc = "The XML IdP metadata file generated from your identity provider."]
    pub metadata: PrimField<String>,
}

impl BuildOpensearchserverlessSecurityConfigSamlOptionsEl {
    pub fn build(self) -> OpensearchserverlessSecurityConfigSamlOptionsEl {
        OpensearchserverlessSecurityConfigSamlOptionsEl {
            group_attribute: core::default::Default::default(),
            metadata: self.metadata,
            session_timeout: core::default::Default::default(),
            user_attribute: core::default::Default::default(),
        }
    }
}

pub struct OpensearchserverlessSecurityConfigSamlOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpensearchserverlessSecurityConfigSamlOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchserverlessSecurityConfigSamlOptionsElRef {
        OpensearchserverlessSecurityConfigSamlOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchserverlessSecurityConfigSamlOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `group_attribute` after provisioning.\nGroup attribute for this SAML integration."]
    pub fn group_attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_attribute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nThe XML IdP metadata file generated from your identity provider."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_timeout` after provisioning.\nSession timeout, in minutes. Minimum is 5 minutes and maximum is 720 minutes (12 hours). Default is 60 minutes."]
    pub fn session_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_attribute` after provisioning.\nUser attribute for this SAML integration."]
    pub fn user_attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_attribute", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpensearchserverlessSecurityConfigDynamic {
    saml_options: Option<DynamicBlock<OpensearchserverlessSecurityConfigSamlOptionsEl>>,
}
