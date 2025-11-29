use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontkeyvaluestoreKeyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    key: PrimField<String>,
    key_value_store_arn: PrimField<String>,
    value: PrimField<String>,
}

struct CloudfrontkeyvaluestoreKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfrontkeyvaluestoreKeyData>,
}

#[derive(Clone)]
pub struct CloudfrontkeyvaluestoreKey(Rc<CloudfrontkeyvaluestoreKey_>);

impl CloudfrontkeyvaluestoreKey {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe key to put."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_value_store_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the Key Value Store."]
    pub fn key_value_store_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_value_store_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_size_in_bytes` after provisioning.\nTotal size of the Key Value Store in bytes."]
    pub fn total_size_in_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_size_in_bytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value to put."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}

impl Referable for CloudfrontkeyvaluestoreKey {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CloudfrontkeyvaluestoreKey { }

impl ToListMappable for CloudfrontkeyvaluestoreKey {
    type O = ListRef<CloudfrontkeyvaluestoreKeyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudfrontkeyvaluestoreKey_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudfrontkeyvaluestore_key".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontkeyvaluestoreKey {
    pub tf_id: String,
    #[doc = "The key to put."]
    pub key: PrimField<String>,
    #[doc = "The Amazon Resource Name (ARN) of the Key Value Store."]
    pub key_value_store_arn: PrimField<String>,
    #[doc = "The value to put."]
    pub value: PrimField<String>,
}

impl BuildCloudfrontkeyvaluestoreKey {
    pub fn build(self, stack: &mut Stack) -> CloudfrontkeyvaluestoreKey {
        let out = CloudfrontkeyvaluestoreKey(Rc::new(CloudfrontkeyvaluestoreKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudfrontkeyvaluestoreKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                key: self.key,
                key_value_store_arn: self.key_value_store_arn,
                value: self.value,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontkeyvaluestoreKeyRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontkeyvaluestoreKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CloudfrontkeyvaluestoreKeyRef {
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

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe key to put."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_value_store_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the Key Value Store."]
    pub fn key_value_store_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_value_store_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_size_in_bytes` after provisioning.\nTotal size of the Key Value Store in bytes."]
    pub fn total_size_in_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_size_in_bytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value to put."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}
