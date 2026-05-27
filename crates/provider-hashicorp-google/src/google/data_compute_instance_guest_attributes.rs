use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeInstanceGuestAttributesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
struct DataComputeInstanceGuestAttributes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeInstanceGuestAttributesData>,
}
#[derive(Clone)]
pub struct DataComputeInstanceGuestAttributes(Rc<DataComputeInstanceGuestAttributes_>);
impl DataComputeInstanceGuestAttributes {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `query_path`.\n"]
    pub fn set_query_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().query_path = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `variable_key`.\n"]
    pub fn set_variable_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().variable_key = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `query_path` after provisioning.\n"]
    pub fn query_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `query_value` after provisioning.\n"]
    pub fn query_value(&self) -> ListRef<DataComputeInstanceGuestAttributesQueryValueElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_value", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `variable_key` after provisioning.\n"]
    pub fn variable_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.variable_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `variable_value` after provisioning.\n"]
    pub fn variable_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.variable_value", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeInstanceGuestAttributes {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeInstanceGuestAttributes {}
impl ToListMappable for DataComputeInstanceGuestAttributes {
    type O = ListRef<DataComputeInstanceGuestAttributesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeInstanceGuestAttributes_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_instance_guest_attributes".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeInstanceGuestAttributes {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataComputeInstanceGuestAttributes {
    pub fn build(self, stack: &mut Stack) -> DataComputeInstanceGuestAttributes {
        let out =
            DataComputeInstanceGuestAttributes(Rc::new(DataComputeInstanceGuestAttributes_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataComputeInstanceGuestAttributesData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    name: self.name,
                    project: core::default::Default::default(),
                    query_path: core::default::Default::default(),
                    region: core::default::Default::default(),
                    variable_key: core::default::Default::default(),
                    zone: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeInstanceGuestAttributesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceGuestAttributesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeInstanceGuestAttributesRef {
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
    #[doc = "Get a reference to the value of field `query_path` after provisioning.\n"]
    pub fn query_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `query_value` after provisioning.\n"]
    pub fn query_value(&self) -> ListRef<DataComputeInstanceGuestAttributesQueryValueElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_value", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `variable_key` after provisioning.\n"]
    pub fn variable_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.variable_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `variable_value` after provisioning.\n"]
    pub fn variable_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.variable_value", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeInstanceGuestAttributesQueryValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComputeInstanceGuestAttributesQueryValueEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeInstanceGuestAttributesQueryValueEl {
    type O = BlockAssignable<DataComputeInstanceGuestAttributesQueryValueEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeInstanceGuestAttributesQueryValueEl {}
impl BuildDataComputeInstanceGuestAttributesQueryValueEl {
    pub fn build(self) -> DataComputeInstanceGuestAttributesQueryValueEl {
        DataComputeInstanceGuestAttributesQueryValueEl {
            key: core::default::Default::default(),
            namespace: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataComputeInstanceGuestAttributesQueryValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeInstanceGuestAttributesQueryValueElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceGuestAttributesQueryValueElRef {
        DataComputeInstanceGuestAttributesQueryValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeInstanceGuestAttributesQueryValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
