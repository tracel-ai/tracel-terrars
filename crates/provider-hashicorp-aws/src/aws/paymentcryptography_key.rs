use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct PaymentcryptographyKeyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_window_in_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    exportable: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_check_value_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_attributes: Option<Vec<PaymentcryptographyKeyKeyAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PaymentcryptographyKeyTimeoutsEl>,
    dynamic: PaymentcryptographyKeyDynamic,
}

struct PaymentcryptographyKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PaymentcryptographyKeyData>,
}

#[derive(Clone)]
pub struct PaymentcryptographyKey(Rc<PaymentcryptographyKey_>);

impl PaymentcryptographyKey {
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

    #[doc = "Set the field `deletion_window_in_days`.\n"]
    pub fn set_deletion_window_in_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().deletion_window_in_days = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `key_check_value_algorithm`.\n"]
    pub fn set_key_check_value_algorithm(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_check_value_algorithm = Some(v.into());
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

    #[doc = "Set the field `key_attributes`.\n"]
    pub fn set_key_attributes(
        self,
        v: impl Into<BlockAssignable<PaymentcryptographyKeyKeyAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().key_attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.key_attributes = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PaymentcryptographyKeyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_window_in_days` after provisioning.\n"]
    pub fn deletion_window_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_window_in_days", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `exportable` after provisioning.\n"]
    pub fn exportable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exportable", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_check_value` after provisioning.\n"]
    pub fn key_check_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_check_value", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `key_check_value_algorithm` after provisioning.\n"]
    pub fn key_check_value_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_check_value_algorithm", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `key_origin` after provisioning.\n"]
    pub fn key_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_origin", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `key_state` after provisioning.\n"]
    pub fn key_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_state", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `key_attributes` after provisioning.\n"]
    pub fn key_attributes(&self) -> ListRef<PaymentcryptographyKeyKeyAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PaymentcryptographyKeyTimeoutsElRef {
        PaymentcryptographyKeyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for PaymentcryptographyKey {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for PaymentcryptographyKey {}

impl ToListMappable for PaymentcryptographyKey {
    type O = ListRef<PaymentcryptographyKeyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for PaymentcryptographyKey_ {
    fn extract_resource_type(&self) -> String {
        "aws_paymentcryptography_key".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPaymentcryptographyKey {
    pub tf_id: String,
    #[doc = ""]
    pub exportable: PrimField<bool>,
}

impl BuildPaymentcryptographyKey {
    pub fn build(self, stack: &mut Stack) -> PaymentcryptographyKey {
        let out = PaymentcryptographyKey(Rc::new(PaymentcryptographyKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PaymentcryptographyKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_window_in_days: core::default::Default::default(),
                enabled: core::default::Default::default(),
                exportable: self.exportable,
                key_check_value_algorithm: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                key_attributes: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PaymentcryptographyKeyRef {
    shared: StackShared,
    base: String,
}

impl Ref for PaymentcryptographyKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl PaymentcryptographyKeyRef {
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

    #[doc = "Get a reference to the value of field `deletion_window_in_days` after provisioning.\n"]
    pub fn deletion_window_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_window_in_days", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `exportable` after provisioning.\n"]
    pub fn exportable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exportable", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_check_value` after provisioning.\n"]
    pub fn key_check_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_check_value", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `key_check_value_algorithm` after provisioning.\n"]
    pub fn key_check_value_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_check_value_algorithm", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `key_origin` after provisioning.\n"]
    pub fn key_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_origin", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `key_state` after provisioning.\n"]
    pub fn key_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_state", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `key_attributes` after provisioning.\n"]
    pub fn key_attributes(&self) -> ListRef<PaymentcryptographyKeyKeyAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PaymentcryptographyKeyTimeoutsElRef {
        PaymentcryptographyKeyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct PaymentcryptographyKeyKeyAttributesElKeyModesOfUseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    decrypt: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    derive_key: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypt: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generate: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_restrictions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sign: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unwrap: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wrap: Option<PrimField<bool>>,
}

impl PaymentcryptographyKeyKeyAttributesElKeyModesOfUseEl {
    #[doc = "Set the field `decrypt`.\n"]
    pub fn set_decrypt(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.decrypt = Some(v.into());
        self
    }

    #[doc = "Set the field `derive_key`.\n"]
    pub fn set_derive_key(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.derive_key = Some(v.into());
        self
    }

    #[doc = "Set the field `encrypt`.\n"]
    pub fn set_encrypt(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encrypt = Some(v.into());
        self
    }

    #[doc = "Set the field `generate`.\n"]
    pub fn set_generate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.generate = Some(v.into());
        self
    }

    #[doc = "Set the field `no_restrictions`.\n"]
    pub fn set_no_restrictions(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_restrictions = Some(v.into());
        self
    }

    #[doc = "Set the field `sign`.\n"]
    pub fn set_sign(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.sign = Some(v.into());
        self
    }

    #[doc = "Set the field `unwrap`.\n"]
    pub fn set_unwrap(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.unwrap = Some(v.into());
        self
    }

    #[doc = "Set the field `verify`.\n"]
    pub fn set_verify(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.verify = Some(v.into());
        self
    }

    #[doc = "Set the field `wrap`.\n"]
    pub fn set_wrap(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.wrap = Some(v.into());
        self
    }
}

impl ToListMappable for PaymentcryptographyKeyKeyAttributesElKeyModesOfUseEl {
    type O = BlockAssignable<PaymentcryptographyKeyKeyAttributesElKeyModesOfUseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPaymentcryptographyKeyKeyAttributesElKeyModesOfUseEl {}

impl BuildPaymentcryptographyKeyKeyAttributesElKeyModesOfUseEl {
    pub fn build(self) -> PaymentcryptographyKeyKeyAttributesElKeyModesOfUseEl {
        PaymentcryptographyKeyKeyAttributesElKeyModesOfUseEl {
            decrypt: core::default::Default::default(),
            derive_key: core::default::Default::default(),
            encrypt: core::default::Default::default(),
            generate: core::default::Default::default(),
            no_restrictions: core::default::Default::default(),
            sign: core::default::Default::default(),
            unwrap: core::default::Default::default(),
            verify: core::default::Default::default(),
            wrap: core::default::Default::default(),
        }
    }
}

pub struct PaymentcryptographyKeyKeyAttributesElKeyModesOfUseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PaymentcryptographyKeyKeyAttributesElKeyModesOfUseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PaymentcryptographyKeyKeyAttributesElKeyModesOfUseElRef {
        PaymentcryptographyKeyKeyAttributesElKeyModesOfUseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PaymentcryptographyKeyKeyAttributesElKeyModesOfUseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `decrypt` after provisioning.\n"]
    pub fn decrypt(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.decrypt", self.base))
    }

    #[doc = "Get a reference to the value of field `derive_key` after provisioning.\n"]
    pub fn derive_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.derive_key", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypt` after provisioning.\n"]
    pub fn encrypt(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypt", self.base))
    }

    #[doc = "Get a reference to the value of field `generate` after provisioning.\n"]
    pub fn generate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.generate", self.base))
    }

    #[doc = "Get a reference to the value of field `no_restrictions` after provisioning.\n"]
    pub fn no_restrictions(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_restrictions", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sign` after provisioning.\n"]
    pub fn sign(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sign", self.base))
    }

    #[doc = "Get a reference to the value of field `unwrap` after provisioning.\n"]
    pub fn unwrap(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unwrap", self.base))
    }

    #[doc = "Get a reference to the value of field `verify` after provisioning.\n"]
    pub fn verify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.verify", self.base))
    }

    #[doc = "Get a reference to the value of field `wrap` after provisioning.\n"]
    pub fn wrap(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wrap", self.base))
    }
}

#[derive(Serialize, Default)]
struct PaymentcryptographyKeyKeyAttributesElDynamic {
    key_modes_of_use: Option<DynamicBlock<PaymentcryptographyKeyKeyAttributesElKeyModesOfUseEl>>,
}

#[derive(Serialize)]
pub struct PaymentcryptographyKeyKeyAttributesEl {
    key_algorithm: PrimField<String>,
    key_class: PrimField<String>,
    key_usage: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_modes_of_use: Option<Vec<PaymentcryptographyKeyKeyAttributesElKeyModesOfUseEl>>,
    dynamic: PaymentcryptographyKeyKeyAttributesElDynamic,
}

impl PaymentcryptographyKeyKeyAttributesEl {
    #[doc = "Set the field `key_modes_of_use`.\n"]
    pub fn set_key_modes_of_use(
        mut self,
        v: impl Into<BlockAssignable<PaymentcryptographyKeyKeyAttributesElKeyModesOfUseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.key_modes_of_use = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.key_modes_of_use = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for PaymentcryptographyKeyKeyAttributesEl {
    type O = BlockAssignable<PaymentcryptographyKeyKeyAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPaymentcryptographyKeyKeyAttributesEl {
    #[doc = ""]
    pub key_algorithm: PrimField<String>,
    #[doc = ""]
    pub key_class: PrimField<String>,
    #[doc = ""]
    pub key_usage: PrimField<String>,
}

impl BuildPaymentcryptographyKeyKeyAttributesEl {
    pub fn build(self) -> PaymentcryptographyKeyKeyAttributesEl {
        PaymentcryptographyKeyKeyAttributesEl {
            key_algorithm: self.key_algorithm,
            key_class: self.key_class,
            key_usage: self.key_usage,
            key_modes_of_use: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct PaymentcryptographyKeyKeyAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PaymentcryptographyKeyKeyAttributesElRef {
    fn new(shared: StackShared, base: String) -> PaymentcryptographyKeyKeyAttributesElRef {
        PaymentcryptographyKeyKeyAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PaymentcryptographyKeyKeyAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\n"]
    pub fn key_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_algorithm", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `key_class` after provisioning.\n"]
    pub fn key_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_class", self.base))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `key_modes_of_use` after provisioning.\n"]
    pub fn key_modes_of_use(
        &self,
    ) -> ListRef<PaymentcryptographyKeyKeyAttributesElKeyModesOfUseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_modes_of_use", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct PaymentcryptographyKeyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl PaymentcryptographyKeyTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for PaymentcryptographyKeyTimeoutsEl {
    type O = BlockAssignable<PaymentcryptographyKeyTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPaymentcryptographyKeyTimeoutsEl {}

impl BuildPaymentcryptographyKeyTimeoutsEl {
    pub fn build(self) -> PaymentcryptographyKeyTimeoutsEl {
        PaymentcryptographyKeyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct PaymentcryptographyKeyTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PaymentcryptographyKeyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PaymentcryptographyKeyTimeoutsElRef {
        PaymentcryptographyKeyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PaymentcryptographyKeyTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct PaymentcryptographyKeyDynamic {
    key_attributes: Option<DynamicBlock<PaymentcryptographyKeyKeyAttributesEl>>,
}
