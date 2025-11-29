use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecretsmanagerSecretRotationData {
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
    rotate_immediately: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_lambda_arn: Option<PrimField<String>>,
    secret_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_rules: Option<Vec<SecretsmanagerSecretRotationRotationRulesEl>>,
    dynamic: SecretsmanagerSecretRotationDynamic,
}

struct SecretsmanagerSecretRotation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecretsmanagerSecretRotationData>,
}

#[derive(Clone)]
pub struct SecretsmanagerSecretRotation(Rc<SecretsmanagerSecretRotation_>);

impl SecretsmanagerSecretRotation {
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

    #[doc = "Set the field `rotate_immediately`.\n"]
    pub fn set_rotate_immediately(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().rotate_immediately = Some(v.into());
        self
    }

    #[doc = "Set the field `rotation_lambda_arn`.\n"]
    pub fn set_rotation_lambda_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().rotation_lambda_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `rotation_rules`.\n"]
    pub fn set_rotation_rules(self, v: impl Into<BlockAssignable<SecretsmanagerSecretRotationRotationRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rotation_rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rotation_rules = Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `rotate_immediately` after provisioning.\n"]
    pub fn rotate_immediately(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotate_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_enabled` after provisioning.\n"]
    pub fn rotation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotation_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_lambda_arn` after provisioning.\n"]
    pub fn rotation_lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotation_lambda_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_rules` after provisioning.\n"]
    pub fn rotation_rules(&self) -> ListRef<SecretsmanagerSecretRotationRotationRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rotation_rules", self.extract_ref()))
    }
}

impl Referable for SecretsmanagerSecretRotation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SecretsmanagerSecretRotation { }

impl ToListMappable for SecretsmanagerSecretRotation {
    type O = ListRef<SecretsmanagerSecretRotationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SecretsmanagerSecretRotation_ {
    fn extract_resource_type(&self) -> String {
        "aws_secretsmanager_secret_rotation".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecretsmanagerSecretRotation {
    pub tf_id: String,
    #[doc = ""]
    pub secret_id: PrimField<String>,
}

impl BuildSecretsmanagerSecretRotation {
    pub fn build(self, stack: &mut Stack) -> SecretsmanagerSecretRotation {
        let out = SecretsmanagerSecretRotation(Rc::new(SecretsmanagerSecretRotation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecretsmanagerSecretRotationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                rotate_immediately: core::default::Default::default(),
                rotation_lambda_arn: core::default::Default::default(),
                secret_id: self.secret_id,
                rotation_rules: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecretsmanagerSecretRotationRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecretsmanagerSecretRotationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SecretsmanagerSecretRotationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `rotate_immediately` after provisioning.\n"]
    pub fn rotate_immediately(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotate_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_enabled` after provisioning.\n"]
    pub fn rotation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotation_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_lambda_arn` after provisioning.\n"]
    pub fn rotation_lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotation_lambda_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation_rules` after provisioning.\n"]
    pub fn rotation_rules(&self) -> ListRef<SecretsmanagerSecretRotationRotationRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rotation_rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SecretsmanagerSecretRotationRotationRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatically_after_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_expression: Option<PrimField<String>>,
}

impl SecretsmanagerSecretRotationRotationRulesEl {
    #[doc = "Set the field `automatically_after_days`.\n"]
    pub fn set_automatically_after_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.automatically_after_days = Some(v.into());
        self
    }

    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.duration = Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_expression`.\n"]
    pub fn set_schedule_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule_expression = Some(v.into());
        self
    }
}

impl ToListMappable for SecretsmanagerSecretRotationRotationRulesEl {
    type O = BlockAssignable<SecretsmanagerSecretRotationRotationRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretsmanagerSecretRotationRotationRulesEl {}

impl BuildSecretsmanagerSecretRotationRotationRulesEl {
    pub fn build(self) -> SecretsmanagerSecretRotationRotationRulesEl {
        SecretsmanagerSecretRotationRotationRulesEl {
            automatically_after_days: core::default::Default::default(),
            duration: core::default::Default::default(),
            schedule_expression: core::default::Default::default(),
        }
    }
}

pub struct SecretsmanagerSecretRotationRotationRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecretsmanagerSecretRotationRotationRulesElRef {
    fn new(shared: StackShared, base: String) -> SecretsmanagerSecretRotationRotationRulesElRef {
        SecretsmanagerSecretRotationRotationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretsmanagerSecretRotationRotationRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automatically_after_days` after provisioning.\n"]
    pub fn automatically_after_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatically_after_days", self.base))
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecretsmanagerSecretRotationDynamic {
    rotation_rules: Option<DynamicBlock<SecretsmanagerSecretRotationRotationRulesEl>>,
}
