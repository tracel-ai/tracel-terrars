use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataNetblockIpRangesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_type: Option<PrimField<String>>,
}
struct DataNetblockIpRanges_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataNetblockIpRangesData>,
}
#[derive(Clone)]
pub struct DataNetblockIpRanges(Rc<DataNetblockIpRanges_>);
impl DataNetblockIpRanges {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `range_type`.\n"]
    pub fn set_range_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().range_type = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cidr_blocks` after provisioning.\n"]
    pub fn cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cidr_blocks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cidr_blocks_ipv4` after provisioning.\n"]
    pub fn cidr_blocks_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cidr_blocks_ipv4", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cidr_blocks_ipv6` after provisioning.\n"]
    pub fn cidr_blocks_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cidr_blocks_ipv6", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `range_type` after provisioning.\n"]
    pub fn range_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.range_type", self.extract_ref()),
        )
    }
}
impl Referable for DataNetblockIpRanges {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataNetblockIpRanges {}
impl ToListMappable for DataNetblockIpRanges {
    type O = ListRef<DataNetblockIpRangesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataNetblockIpRanges_ {
    fn extract_datasource_type(&self) -> String {
        "google_netblock_ip_ranges".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataNetblockIpRanges {
    pub tf_id: String,
}
impl BuildDataNetblockIpRanges {
    pub fn build(self, stack: &mut Stack) -> DataNetblockIpRanges {
        let out = DataNetblockIpRanges(Rc::new(DataNetblockIpRanges_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataNetblockIpRangesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                range_type: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataNetblockIpRangesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetblockIpRangesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataNetblockIpRangesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `cidr_blocks` after provisioning.\n"]
    pub fn cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cidr_blocks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cidr_blocks_ipv4` after provisioning.\n"]
    pub fn cidr_blocks_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cidr_blocks_ipv4", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cidr_blocks_ipv6` after provisioning.\n"]
    pub fn cidr_blocks_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cidr_blocks_ipv6", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `range_type` after provisioning.\n"]
    pub fn range_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.range_type", self.extract_ref()),
        )
    }
}
