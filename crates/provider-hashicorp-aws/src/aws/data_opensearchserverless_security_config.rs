use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataOpensearchserverlessSecurityConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml_options: Option<Vec<DataOpensearchserverlessSecurityConfigSamlOptionsEl>>,
    dynamic: DataOpensearchserverlessSecurityConfigDynamic,
}

struct DataOpensearchserverlessSecurityConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOpensearchserverlessSecurityConfigData>,
}

#[derive(Clone)]
pub struct DataOpensearchserverlessSecurityConfig(Rc<DataOpensearchserverlessSecurityConfig_>);

impl DataOpensearchserverlessSecurityConfig {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `saml_options`.\n"]
    pub fn set_saml_options(
        self,
        v: impl Into<BlockAssignable<DataOpensearchserverlessSecurityConfigSamlOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().saml_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.saml_options = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `config_version` after provisioning.\nThe version of the security configuration."]
    pub fn config_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\nThe date the configuration was created."]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the security configuration."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the security configuration."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_date` after provisioning.\nThe date the configuration was last modified."]
    pub fn last_modified_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of security configuration."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `saml_options` after provisioning.\n"]
    pub fn saml_options(&self) -> ListRef<DataOpensearchserverlessSecurityConfigSamlOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saml_options", self.extract_ref()),
        )
    }
}

impl Referable for DataOpensearchserverlessSecurityConfig {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataOpensearchserverlessSecurityConfig {}

impl ToListMappable for DataOpensearchserverlessSecurityConfig {
    type O = ListRef<DataOpensearchserverlessSecurityConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOpensearchserverlessSecurityConfig_ {
    fn extract_datasource_type(&self) -> String {
        "aws_opensearchserverless_security_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOpensearchserverlessSecurityConfig {
    pub tf_id: String,
    #[doc = "The unique identifier of the security configuration."]
    pub id: PrimField<String>,
}

impl BuildDataOpensearchserverlessSecurityConfig {
    pub fn build(self, stack: &mut Stack) -> DataOpensearchserverlessSecurityConfig {
        let out = DataOpensearchserverlessSecurityConfig(Rc::new(
            DataOpensearchserverlessSecurityConfig_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataOpensearchserverlessSecurityConfigData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: self.id,
                    region: core::default::Default::default(),
                    saml_options: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOpensearchserverlessSecurityConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchserverlessSecurityConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataOpensearchserverlessSecurityConfigRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `config_version` after provisioning.\nThe version of the security configuration."]
    pub fn config_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\nThe date the configuration was created."]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the security configuration."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the security configuration."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_date` after provisioning.\nThe date the configuration was last modified."]
    pub fn last_modified_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of security configuration."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `saml_options` after provisioning.\n"]
    pub fn saml_options(&self) -> ListRef<DataOpensearchserverlessSecurityConfigSamlOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saml_options", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataOpensearchserverlessSecurityConfigSamlOptionsEl {}

impl DataOpensearchserverlessSecurityConfigSamlOptionsEl {}

impl ToListMappable for DataOpensearchserverlessSecurityConfigSamlOptionsEl {
    type O = BlockAssignable<DataOpensearchserverlessSecurityConfigSamlOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchserverlessSecurityConfigSamlOptionsEl {}

impl BuildDataOpensearchserverlessSecurityConfigSamlOptionsEl {
    pub fn build(self) -> DataOpensearchserverlessSecurityConfigSamlOptionsEl {
        DataOpensearchserverlessSecurityConfigSamlOptionsEl {}
    }
}

pub struct DataOpensearchserverlessSecurityConfigSamlOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchserverlessSecurityConfigSamlOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOpensearchserverlessSecurityConfigSamlOptionsElRef {
        DataOpensearchserverlessSecurityConfigSamlOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchserverlessSecurityConfigSamlOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_attribute` after provisioning.\nGroup attribute for this SAML integration."]
    pub fn group_attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_attribute", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nThe XML IdP metadata file generated from your identity provider."]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `session_timeout` after provisioning.\nSession timeout, in minutes. Minimum is 5 minutes and maximum is 720 minutes (12 hours). Default is 60 minutes."]
    pub fn session_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_timeout", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_attribute` after provisioning.\nUser attribute for this SAML integration."]
    pub fn user_attribute(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_attribute", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct DataOpensearchserverlessSecurityConfigDynamic {
    saml_options: Option<DynamicBlock<DataOpensearchserverlessSecurityConfigSamlOptionsEl>>,
}
