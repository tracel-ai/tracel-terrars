use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataNetworkSecurityAddressGroupsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataNetworkSecurityAddressGroups_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataNetworkSecurityAddressGroupsData>,
}
#[derive(Clone)]
pub struct DataNetworkSecurityAddressGroups(Rc<DataNetworkSecurityAddressGroups_>);
impl DataNetworkSecurityAddressGroups {
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
    #[doc = "Set the field `parent`.\nThe parent of the Address Group. Use \"organizations/{organization_id}\" for organization-level address groups or \"projects/{project_id}\" for project-level address groups."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `address_groups` after provisioning.\n"]
    pub fn address_groups(&self) -> ListRef<DataNetworkSecurityAddressGroupsAddressGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.address_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the Address Group. Use \"organizations/{organization_id}\" for organization-level address groups or \"projects/{project_id}\" for project-level address groups."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
impl Referable for DataNetworkSecurityAddressGroups {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataNetworkSecurityAddressGroups {}
impl ToListMappable for DataNetworkSecurityAddressGroups {
    type O = ListRef<DataNetworkSecurityAddressGroupsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataNetworkSecurityAddressGroups_ {
    fn extract_datasource_type(&self) -> String {
        "google_network_security_address_groups".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataNetworkSecurityAddressGroups {
    pub tf_id: String,
    #[doc = ""]
    pub location: PrimField<String>,
}
impl BuildDataNetworkSecurityAddressGroups {
    pub fn build(self, stack: &mut Stack) -> DataNetworkSecurityAddressGroups {
        let out = DataNetworkSecurityAddressGroups(Rc::new(DataNetworkSecurityAddressGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataNetworkSecurityAddressGroupsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                parent: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataNetworkSecurityAddressGroupsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkSecurityAddressGroupsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataNetworkSecurityAddressGroupsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `address_groups` after provisioning.\n"]
    pub fn address_groups(&self) -> ListRef<DataNetworkSecurityAddressGroupsAddressGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.address_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the Address Group. Use \"organizations/{organization_id}\" for organization-level address groups or \"projects/{project_id}\" for project-level address groups."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataNetworkSecurityAddressGroupsAddressGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataNetworkSecurityAddressGroupsAddressGroupsEl {
    #[doc = "Set the field `capacity`.\n"]
    pub fn set_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.capacity = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataNetworkSecurityAddressGroupsAddressGroupsEl {
    type O = BlockAssignable<DataNetworkSecurityAddressGroupsAddressGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkSecurityAddressGroupsAddressGroupsEl {}
impl BuildDataNetworkSecurityAddressGroupsAddressGroupsEl {
    pub fn build(self) -> DataNetworkSecurityAddressGroupsAddressGroupsEl {
        DataNetworkSecurityAddressGroupsAddressGroupsEl {
            capacity: core::default::Default::default(),
            items: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkSecurityAddressGroupsAddressGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkSecurityAddressGroupsAddressGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkSecurityAddressGroupsAddressGroupsElRef {
        DataNetworkSecurityAddressGroupsAddressGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkSecurityAddressGroupsAddressGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
