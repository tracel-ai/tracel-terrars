use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecretsmanagerSecretVersionData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_binary: Option<PrimField<String>>,
    secret_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_string: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_string_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_string_wo_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_stages: Option<SetField<PrimField<String>>>,
}

struct SecretsmanagerSecretVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecretsmanagerSecretVersionData>,
}

#[derive(Clone)]
pub struct SecretsmanagerSecretVersion(Rc<SecretsmanagerSecretVersion_>);

impl SecretsmanagerSecretVersion {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `secret_binary`.\n"]
    pub fn set_secret_binary(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secret_binary = Some(v.into());
        self
    }

    #[doc = "Set the field `secret_string`.\n"]
    pub fn set_secret_string(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secret_string = Some(v.into());
        self
    }

    #[doc = "Set the field `secret_string_wo`.\n"]
    pub fn set_secret_string_wo(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secret_string_wo = Some(v.into());
        self
    }

    #[doc = "Set the field `secret_string_wo_version`.\n"]
    pub fn set_secret_string_wo_version(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().secret_string_wo_version = Some(v.into());
        self
    }

    #[doc = "Set the field `version_stages`.\n"]
    pub fn set_version_stages(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().version_stages = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `has_secret_string_wo` after provisioning.\n"]
    pub fn has_secret_string_wo(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_secret_string_wo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_binary` after provisioning.\n"]
    pub fn secret_binary(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_binary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_string` after provisioning.\n"]
    pub fn secret_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_string_wo` after provisioning.\n"]
    pub fn secret_string_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_string_wo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_string_wo_version` after provisioning.\n"]
    pub fn secret_string_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_string_wo_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_stages` after provisioning.\n"]
    pub fn version_stages(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.version_stages", self.extract_ref()))
    }
}

impl Referable for SecretsmanagerSecretVersion {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SecretsmanagerSecretVersion { }

impl ToListMappable for SecretsmanagerSecretVersion {
    type O = ListRef<SecretsmanagerSecretVersionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SecretsmanagerSecretVersion_ {
    fn extract_resource_type(&self) -> String {
        "aws_secretsmanager_secret_version".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecretsmanagerSecretVersion {
    pub tf_id: String,
    #[doc = ""]
    pub secret_id: PrimField<String>,
}

impl BuildSecretsmanagerSecretVersion {
    pub fn build(self, stack: &mut Stack) -> SecretsmanagerSecretVersion {
        let out = SecretsmanagerSecretVersion(Rc::new(SecretsmanagerSecretVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecretsmanagerSecretVersionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                secret_binary: core::default::Default::default(),
                secret_id: self.secret_id,
                secret_string: core::default::Default::default(),
                secret_string_wo: core::default::Default::default(),
                secret_string_wo_version: core::default::Default::default(),
                version_stages: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecretsmanagerSecretVersionRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecretsmanagerSecretVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SecretsmanagerSecretVersionRef {
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

    #[doc = "Get a reference to the value of field `has_secret_string_wo` after provisioning.\n"]
    pub fn has_secret_string_wo(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_secret_string_wo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_binary` after provisioning.\n"]
    pub fn secret_binary(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_binary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_string` after provisioning.\n"]
    pub fn secret_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_string_wo` after provisioning.\n"]
    pub fn secret_string_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_string_wo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_string_wo_version` after provisioning.\n"]
    pub fn secret_string_wo_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_string_wo_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_stages` after provisioning.\n"]
    pub fn version_stages(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.version_stages", self.extract_ref()))
    }
}
