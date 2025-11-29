use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreTokenVaultCmkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_vault_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_configuration: Option<Vec<BedrockagentcoreTokenVaultCmkKmsConfigurationEl>>,
    dynamic: BedrockagentcoreTokenVaultCmkDynamic,
}

struct BedrockagentcoreTokenVaultCmk_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentcoreTokenVaultCmkData>,
}

#[derive(Clone)]
pub struct BedrockagentcoreTokenVaultCmk(Rc<BedrockagentcoreTokenVaultCmk_>);

impl BedrockagentcoreTokenVaultCmk {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `token_vault_id`.\n"]
    pub fn set_token_vault_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token_vault_id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_configuration`.\n"]
    pub fn set_kms_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreTokenVaultCmkKmsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kms_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kms_configuration = Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_vault_id` after provisioning.\n"]
    pub fn token_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_vault_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_configuration` after provisioning.\n"]
    pub fn kms_configuration(&self) -> ListRef<BedrockagentcoreTokenVaultCmkKmsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kms_configuration", self.extract_ref()))
    }
}

impl Referable for BedrockagentcoreTokenVaultCmk {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BedrockagentcoreTokenVaultCmk { }

impl ToListMappable for BedrockagentcoreTokenVaultCmk {
    type O = ListRef<BedrockagentcoreTokenVaultCmkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockagentcoreTokenVaultCmk_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagentcore_token_vault_cmk".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreTokenVaultCmk {
    pub tf_id: String,
}

impl BuildBedrockagentcoreTokenVaultCmk {
    pub fn build(self, stack: &mut Stack) -> BedrockagentcoreTokenVaultCmk {
        let out = BedrockagentcoreTokenVaultCmk(Rc::new(BedrockagentcoreTokenVaultCmk_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentcoreTokenVaultCmkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                token_vault_id: core::default::Default::default(),
                kms_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreTokenVaultCmkRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreTokenVaultCmkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl BedrockagentcoreTokenVaultCmkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_vault_id` after provisioning.\n"]
    pub fn token_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_vault_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_configuration` after provisioning.\n"]
    pub fn kms_configuration(&self) -> ListRef<BedrockagentcoreTokenVaultCmkKmsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kms_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreTokenVaultCmkKmsConfigurationEl {
    key_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
}

impl BedrockagentcoreTokenVaultCmkKmsConfigurationEl {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreTokenVaultCmkKmsConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreTokenVaultCmkKmsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreTokenVaultCmkKmsConfigurationEl {
    #[doc = ""]
    pub key_type: PrimField<String>,
}

impl BuildBedrockagentcoreTokenVaultCmkKmsConfigurationEl {
    pub fn build(self) -> BedrockagentcoreTokenVaultCmkKmsConfigurationEl {
        BedrockagentcoreTokenVaultCmkKmsConfigurationEl {
            key_type: self.key_type,
            kms_key_arn: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreTokenVaultCmkKmsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreTokenVaultCmkKmsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreTokenVaultCmkKmsConfigurationElRef {
        BedrockagentcoreTokenVaultCmkKmsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreTokenVaultCmkKmsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_type` after provisioning.\n"]
    pub fn key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_type", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreTokenVaultCmkDynamic {
    kms_configuration: Option<DynamicBlock<BedrockagentcoreTokenVaultCmkKmsConfigurationEl>>,
}
