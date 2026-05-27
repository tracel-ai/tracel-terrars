use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeRegionInstanceGroupData {
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
struct DataComputeRegionInstanceGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeRegionInstanceGroupData>,
}
#[derive(Clone)]
pub struct DataComputeRegionInstanceGroup(Rc<DataComputeRegionInstanceGroup_>);
impl DataComputeRegionInstanceGroup {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> ListRef<DataComputeRegionInstanceGroupInstancesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeRegionInstanceGroup {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeRegionInstanceGroup {}
impl ToListMappable for DataComputeRegionInstanceGroup {
    type O = ListRef<DataComputeRegionInstanceGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeRegionInstanceGroup_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_region_instance_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeRegionInstanceGroup {
    pub tf_id: String,
}
impl BuildDataComputeRegionInstanceGroup {
    pub fn build(self, stack: &mut Stack) -> DataComputeRegionInstanceGroup {
        let out = DataComputeRegionInstanceGroup(Rc::new(DataComputeRegionInstanceGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeRegionInstanceGroupData {
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
pub struct DataComputeRegionInstanceGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeRegionInstanceGroupRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> ListRef<DataComputeRegionInstanceGroupInstancesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupInstancesElNamedPortsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataComputeRegionInstanceGroupInstancesElNamedPortsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupInstancesElNamedPortsEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupInstancesElNamedPortsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupInstancesElNamedPortsEl {}
impl BuildDataComputeRegionInstanceGroupInstancesElNamedPortsEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupInstancesElNamedPortsEl {
        DataComputeRegionInstanceGroupInstancesElNamedPortsEl {
            name: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupInstancesElNamedPortsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupInstancesElNamedPortsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionInstanceGroupInstancesElNamedPortsElRef {
        DataComputeRegionInstanceGroupInstancesElNamedPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupInstancesElNamedPortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionInstanceGroupInstancesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    named_ports: Option<ListField<DataComputeRegionInstanceGroupInstancesElNamedPortsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl DataComputeRegionInstanceGroupInstancesEl {
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `named_ports`.\n"]
    pub fn set_named_ports(
        mut self,
        v: impl Into<ListField<DataComputeRegionInstanceGroupInstancesElNamedPortsEl>>,
    ) -> Self {
        self.named_ports = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionInstanceGroupInstancesEl {
    type O = BlockAssignable<DataComputeRegionInstanceGroupInstancesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionInstanceGroupInstancesEl {}
impl BuildDataComputeRegionInstanceGroupInstancesEl {
    pub fn build(self) -> DataComputeRegionInstanceGroupInstancesEl {
        DataComputeRegionInstanceGroupInstancesEl {
            instance: core::default::Default::default(),
            named_ports: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionInstanceGroupInstancesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionInstanceGroupInstancesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionInstanceGroupInstancesElRef {
        DataComputeRegionInstanceGroupInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionInstanceGroupInstancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `named_ports` after provisioning.\n"]
    pub fn named_ports(&self) -> ListRef<DataComputeRegionInstanceGroupInstancesElNamedPortsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.named_ports", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
