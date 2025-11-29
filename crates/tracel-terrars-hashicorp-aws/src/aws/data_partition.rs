use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataPartitionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataPartition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataPartitionData>,
}

#[derive(Clone)]
pub struct DataPartition(Rc<DataPartition_>);

impl DataPartition {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `dns_suffix` after provisioning.\n"]
    pub fn dns_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_suffix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition` after provisioning.\n"]
    pub fn partition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.partition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reverse_dns_prefix` after provisioning.\n"]
    pub fn reverse_dns_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reverse_dns_prefix", self.extract_ref()))
    }
}

impl Referable for DataPartition {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataPartition { }

impl ToListMappable for DataPartition {
    type O = ListRef<DataPartitionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataPartition_ {
    fn extract_datasource_type(&self) -> String {
        "aws_partition".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPartition {
    pub tf_id: String,
}

impl BuildDataPartition {
    pub fn build(self, stack: &mut Stack) -> DataPartition {
        let out = DataPartition(Rc::new(DataPartition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataPartitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPartitionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataPartitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataPartitionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `dns_suffix` after provisioning.\n"]
    pub fn dns_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_suffix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition` after provisioning.\n"]
    pub fn partition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.partition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reverse_dns_prefix` after provisioning.\n"]
    pub fn reverse_dns_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reverse_dns_prefix", self.extract_ref()))
    }
}
