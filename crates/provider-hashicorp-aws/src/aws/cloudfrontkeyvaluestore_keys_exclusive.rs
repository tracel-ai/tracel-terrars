use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudfrontkeyvaluestoreKeysExclusiveData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    key_value_store_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_batch_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_key_value_pair:
        Option<Vec<CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairEl>>,
    dynamic: CloudfrontkeyvaluestoreKeysExclusiveDynamic,
}
struct CloudfrontkeyvaluestoreKeysExclusive_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfrontkeyvaluestoreKeysExclusiveData>,
}
#[derive(Clone)]
pub struct CloudfrontkeyvaluestoreKeysExclusive(Rc<CloudfrontkeyvaluestoreKeysExclusive_>);
impl CloudfrontkeyvaluestoreKeysExclusive {
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
    #[doc = "Set the field `max_batch_size`.\nMaximum resource key values pairs that you wills update in a single API request. AWS has a default quota of 50 keys or a 3 MB payload, whichever is reached first"]
    pub fn set_max_batch_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_batch_size = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_key_value_pair`.\n"]
    pub fn set_resource_key_value_pair(
        self,
        v: impl Into<BlockAssignable<CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_key_value_pair = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_key_value_pair = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `key_value_store_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the Key Value Store."]
    pub fn key_value_store_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_value_store_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_batch_size` after provisioning.\nMaximum resource key values pairs that you wills update in a single API request. AWS has a default quota of 50 keys or a 3 MB payload, whichever is reached first"]
    pub fn max_batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_batch_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_size_in_bytes` after provisioning.\nTotal size of the Key Value Store in bytes."]
    pub fn total_size_in_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_size_in_bytes", self.extract_ref()),
        )
    }
}
impl Referable for CloudfrontkeyvaluestoreKeysExclusive {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudfrontkeyvaluestoreKeysExclusive {}
impl ToListMappable for CloudfrontkeyvaluestoreKeysExclusive {
    type O = ListRef<CloudfrontkeyvaluestoreKeysExclusiveRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudfrontkeyvaluestoreKeysExclusive_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudfrontkeyvaluestore_keys_exclusive".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudfrontkeyvaluestoreKeysExclusive {
    pub tf_id: String,
    #[doc = "The Amazon Resource Name (ARN) of the Key Value Store."]
    pub key_value_store_arn: PrimField<String>,
}
impl BuildCloudfrontkeyvaluestoreKeysExclusive {
    pub fn build(self, stack: &mut Stack) -> CloudfrontkeyvaluestoreKeysExclusive {
        let out =
            CloudfrontkeyvaluestoreKeysExclusive(Rc::new(CloudfrontkeyvaluestoreKeysExclusive_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(CloudfrontkeyvaluestoreKeysExclusiveData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    key_value_store_arn: self.key_value_store_arn,
                    max_batch_size: core::default::Default::default(),
                    resource_key_value_pair: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudfrontkeyvaluestoreKeysExclusiveRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontkeyvaluestoreKeysExclusiveRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudfrontkeyvaluestoreKeysExclusiveRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key_value_store_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the Key Value Store."]
    pub fn key_value_store_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_value_store_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_batch_size` after provisioning.\nMaximum resource key values pairs that you wills update in a single API request. AWS has a default quota of 50 keys or a 3 MB payload, whichever is reached first"]
    pub fn max_batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_batch_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_size_in_bytes` after provisioning.\nTotal size of the Key Value Store in bytes."]
    pub fn total_size_in_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_size_in_bytes", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairEl {}
impl ToListMappable for CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairEl {
    type O = BlockAssignable<CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairEl {
    #[doc = "The key to put."]
    pub key: PrimField<String>,
    #[doc = "The value to put."]
    pub value: PrimField<String>,
}
impl BuildCloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairEl {
    pub fn build(self) -> CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairEl {
        CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairElRef {
        CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe key to put."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value to put."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudfrontkeyvaluestoreKeysExclusiveDynamic {
    resource_key_value_pair:
        Option<DynamicBlock<CloudfrontkeyvaluestoreKeysExclusiveResourceKeyValuePairEl>>,
}
