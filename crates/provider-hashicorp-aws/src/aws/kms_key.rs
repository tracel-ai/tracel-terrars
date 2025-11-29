use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KmsKeyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_policy_lockout_safety_check: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_key_store_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_master_key_spec: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_window_in_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_key_rotation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_usage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_region: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_period_in_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xks_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<KmsKeyTimeoutsEl>,
}

struct KmsKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KmsKeyData>,
}

#[derive(Clone)]
pub struct KmsKey(Rc<KmsKey_>);

impl KmsKey {
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

    #[doc = "Set the field `bypass_policy_lockout_safety_check`.\n"]
    pub fn set_bypass_policy_lockout_safety_check(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bypass_policy_lockout_safety_check = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_key_store_id`.\n"]
    pub fn set_custom_key_store_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_key_store_id = Some(v.into());
        self
    }

    #[doc = "Set the field `customer_master_key_spec`.\n"]
    pub fn set_customer_master_key_spec(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_master_key_spec = Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_window_in_days`.\n"]
    pub fn set_deletion_window_in_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().deletion_window_in_days = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_key_rotation`.\n"]
    pub fn set_enable_key_rotation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_key_rotation = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `is_enabled`.\n"]
    pub fn set_is_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `key_usage`.\n"]
    pub fn set_key_usage(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_usage = Some(v.into());
        self
    }

    #[doc = "Set the field `multi_region`.\n"]
    pub fn set_multi_region(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_region = Some(v.into());
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

    #[doc = "Set the field `rotation_period_in_days`.\n"]
    pub fn set_rotation_period_in_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().rotation_period_in_days = Some(v.into());
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

    #[doc = "Set the field `xks_key_id`.\n"]
    pub fn set_xks_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().xks_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<KmsKeyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bypass_policy_lockout_safety_check` after provisioning.\n"]
    pub fn bypass_policy_lockout_safety_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bypass_policy_lockout_safety_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_key_store_id` after provisioning.\n"]
    pub fn custom_key_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_key_store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_master_key_spec` after provisioning.\n"]
    pub fn customer_master_key_spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_master_key_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_window_in_days` after provisioning.\n"]
    pub fn deletion_window_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_window_in_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_key_rotation` after provisioning.\n"]
    pub fn enable_key_rotation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_key_rotation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_enabled` after provisioning.\n"]
    pub fn is_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_usage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region` after provisioning.\n"]
    pub fn multi_region(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rotation_period_in_days` after provisioning.\n"]
    pub fn rotation_period_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotation_period_in_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_key_id` after provisioning.\n"]
    pub fn xks_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.xks_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsKeyTimeoutsElRef {
        KmsKeyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for KmsKey {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for KmsKey { }

impl ToListMappable for KmsKey {
    type O = ListRef<KmsKeyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for KmsKey_ {
    fn extract_resource_type(&self) -> String {
        "aws_kms_key".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKmsKey {
    pub tf_id: String,
}

impl BuildKmsKey {
    pub fn build(self, stack: &mut Stack) -> KmsKey {
        let out = KmsKey(Rc::new(KmsKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KmsKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bypass_policy_lockout_safety_check: core::default::Default::default(),
                custom_key_store_id: core::default::Default::default(),
                customer_master_key_spec: core::default::Default::default(),
                deletion_window_in_days: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_key_rotation: core::default::Default::default(),
                id: core::default::Default::default(),
                is_enabled: core::default::Default::default(),
                key_usage: core::default::Default::default(),
                multi_region: core::default::Default::default(),
                policy: core::default::Default::default(),
                region: core::default::Default::default(),
                rotation_period_in_days: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                xks_key_id: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KmsKeyRef {
    shared: StackShared,
    base: String,
}

impl Ref for KmsKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl KmsKeyRef {
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

    #[doc = "Get a reference to the value of field `bypass_policy_lockout_safety_check` after provisioning.\n"]
    pub fn bypass_policy_lockout_safety_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bypass_policy_lockout_safety_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_key_store_id` after provisioning.\n"]
    pub fn custom_key_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_key_store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_master_key_spec` after provisioning.\n"]
    pub fn customer_master_key_spec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_master_key_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_window_in_days` after provisioning.\n"]
    pub fn deletion_window_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_window_in_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_key_rotation` after provisioning.\n"]
    pub fn enable_key_rotation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_key_rotation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_enabled` after provisioning.\n"]
    pub fn is_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_usage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region` after provisioning.\n"]
    pub fn multi_region(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rotation_period_in_days` after provisioning.\n"]
    pub fn rotation_period_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotation_period_in_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_key_id` after provisioning.\n"]
    pub fn xks_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.xks_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsKeyTimeoutsElRef {
        KmsKeyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KmsKeyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl KmsKeyTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for KmsKeyTimeoutsEl {
    type O = BlockAssignable<KmsKeyTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsKeyTimeoutsEl {}

impl BuildKmsKeyTimeoutsEl {
    pub fn build(self) -> KmsKeyTimeoutsEl {
        KmsKeyTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct KmsKeyTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KmsKeyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> KmsKeyTimeoutsElRef {
        KmsKeyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsKeyTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
