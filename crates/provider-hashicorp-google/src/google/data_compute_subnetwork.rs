use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeSubnetworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
}
struct DataComputeSubnetwork_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeSubnetworkData>,
}
#[derive(Clone)]
pub struct DataComputeSubnetwork(Rc<DataComputeSubnetwork_>);
impl DataComputeSubnetwork {
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
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().self_link = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_ipv6_prefix` after provisioning.\n"]
    pub fn external_ipv6_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_ipv6_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_address` after provisioning.\n"]
    pub fn gateway_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `internal_ipv6_prefix` after provisioning.\n"]
    pub fn internal_ipv6_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.internal_ipv6_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\n"]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_access_type` after provisioning.\n"]
    pub fn ipv6_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_access_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_ip_google_access` after provisioning.\n"]
    pub fn private_ip_google_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip_google_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_ip_range` after provisioning.\n"]
    pub fn secondary_ip_range(&self) -> ListRef<DataComputeSubnetworkSecondaryIpRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stack_type` after provisioning.\n"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stack_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork_id` after provisioning.\n"]
    pub fn subnetwork_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork_id", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeSubnetwork {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeSubnetwork {}
impl ToListMappable for DataComputeSubnetwork {
    type O = ListRef<DataComputeSubnetworkRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeSubnetwork_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_subnetwork".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeSubnetwork {
    pub tf_id: String,
}
impl BuildDataComputeSubnetwork {
    pub fn build(self, stack: &mut Stack) -> DataComputeSubnetwork {
        let out = DataComputeSubnetwork(Rc::new(DataComputeSubnetwork_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeSubnetworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                self_link: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeSubnetworkRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSubnetworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeSubnetworkRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_ipv6_prefix` after provisioning.\n"]
    pub fn external_ipv6_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_ipv6_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_address` after provisioning.\n"]
    pub fn gateway_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `internal_ipv6_prefix` after provisioning.\n"]
    pub fn internal_ipv6_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.internal_ipv6_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\n"]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_access_type` after provisioning.\n"]
    pub fn ipv6_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_access_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_ip_google_access` after provisioning.\n"]
    pub fn private_ip_google_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip_google_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_ip_range` after provisioning.\n"]
    pub fn secondary_ip_range(&self) -> ListRef<DataComputeSubnetworkSecondaryIpRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stack_type` after provisioning.\n"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stack_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork_id` after provisioning.\n"]
    pub fn subnetwork_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork_id", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSubnetworkSecondaryIpRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_cidr_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_name: Option<PrimField<String>>,
}
impl DataComputeSubnetworkSecondaryIpRangeEl {
    #[doc = "Set the field `ip_cidr_range`.\n"]
    pub fn set_ip_cidr_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_cidr_range = Some(v.into());
        self
    }
    #[doc = "Set the field `range_name`.\n"]
    pub fn set_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeSubnetworkSecondaryIpRangeEl {
    type O = BlockAssignable<DataComputeSubnetworkSecondaryIpRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSubnetworkSecondaryIpRangeEl {}
impl BuildDataComputeSubnetworkSecondaryIpRangeEl {
    pub fn build(self) -> DataComputeSubnetworkSecondaryIpRangeEl {
        DataComputeSubnetworkSecondaryIpRangeEl {
            ip_cidr_range: core::default::Default::default(),
            range_name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSubnetworkSecondaryIpRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSubnetworkSecondaryIpRangeElRef {
    fn new(shared: StackShared, base: String) -> DataComputeSubnetworkSecondaryIpRangeElRef {
        DataComputeSubnetworkSecondaryIpRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSubnetworkSecondaryIpRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\n"]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `range_name` after provisioning.\n"]
    pub fn range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range_name", self.base))
    }
}
