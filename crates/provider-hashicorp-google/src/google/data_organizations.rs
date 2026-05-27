use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOrganizationsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}
struct DataOrganizations_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationsData>,
}
#[derive(Clone)]
pub struct DataOrganizations(Rc<DataOrganizations_>);
impl DataOrganizations {
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
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `organizations` after provisioning.\n"]
    pub fn organizations(&self) -> ListRef<DataOrganizationsOrganizationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.organizations", self.extract_ref()),
        )
    }
}
impl Referable for DataOrganizations {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOrganizations {}
impl ToListMappable for DataOrganizations {
    type O = ListRef<DataOrganizationsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOrganizations_ {
    fn extract_datasource_type(&self) -> String {
        "google_organizations".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOrganizations {
    pub tf_id: String,
}
impl BuildDataOrganizations {
    pub fn build(self, stack: &mut Stack) -> DataOrganizations {
        let out = DataOrganizations(Rc::new(DataOrganizations_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOrganizationsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataOrganizationsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOrganizationsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOrganizationsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `organizations` after provisioning.\n"]
    pub fn organizations(&self) -> ListRef<DataOrganizationsOrganizationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.organizations", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataOrganizationsOrganizationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_customer_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_id: Option<PrimField<String>>,
}
impl DataOrganizationsOrganizationsEl {
    #[doc = "Set the field `directory_customer_id`.\n"]
    pub fn set_directory_customer_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.directory_customer_id = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `lifecycle_state`.\n"]
    pub fn set_lifecycle_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_state = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `org_id`.\n"]
    pub fn set_org_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.org_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataOrganizationsOrganizationsEl {
    type O = BlockAssignable<DataOrganizationsOrganizationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOrganizationsOrganizationsEl {}
impl BuildDataOrganizationsOrganizationsEl {
    pub fn build(self) -> DataOrganizationsOrganizationsEl {
        DataOrganizationsOrganizationsEl {
            directory_customer_id: core::default::Default::default(),
            display_name: core::default::Default::default(),
            lifecycle_state: core::default::Default::default(),
            name: core::default::Default::default(),
            org_id: core::default::Default::default(),
        }
    }
}
pub struct DataOrganizationsOrganizationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOrganizationsOrganizationsElRef {
    fn new(shared: StackShared, base: String) -> DataOrganizationsOrganizationsElRef {
        DataOrganizationsOrganizationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOrganizationsOrganizationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `directory_customer_id` after provisioning.\n"]
    pub fn directory_customer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_customer_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `lifecycle_state` after provisioning.\n"]
    pub fn lifecycle_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\n"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.base))
    }
}
