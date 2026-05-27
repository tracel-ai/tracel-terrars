use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataProjectAncestryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataProjectAncestry_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectAncestryData>,
}
#[derive(Clone)]
pub struct DataProjectAncestry(Rc<DataProjectAncestry_>);
impl DataProjectAncestry {
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
    #[doc = "Get a reference to the value of field `ancestors` after provisioning.\n"]
    pub fn ancestors(&self) -> ListRef<DataProjectAncestryAncestorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ancestors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\n"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_type` after provisioning.\n"]
    pub fn parent_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_type", self.extract_ref()),
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
impl Referable for DataProjectAncestry {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataProjectAncestry {}
impl ToListMappable for DataProjectAncestry {
    type O = ListRef<DataProjectAncestryRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataProjectAncestry_ {
    fn extract_datasource_type(&self) -> String {
        "google_project_ancestry".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataProjectAncestry {
    pub tf_id: String,
}
impl BuildDataProjectAncestry {
    pub fn build(self, stack: &mut Stack) -> DataProjectAncestry {
        let out = DataProjectAncestry(Rc::new(DataProjectAncestry_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectAncestryData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataProjectAncestryRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataProjectAncestryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataProjectAncestryRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `ancestors` after provisioning.\n"]
    pub fn ancestors(&self) -> ListRef<DataProjectAncestryAncestorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ancestors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\n"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_type` after provisioning.\n"]
    pub fn parent_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_type", self.extract_ref()),
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
pub struct DataProjectAncestryAncestorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataProjectAncestryAncestorsEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataProjectAncestryAncestorsEl {
    type O = BlockAssignable<DataProjectAncestryAncestorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataProjectAncestryAncestorsEl {}
impl BuildDataProjectAncestryAncestorsEl {
    pub fn build(self) -> DataProjectAncestryAncestorsEl {
        DataProjectAncestryAncestorsEl {
            id: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataProjectAncestryAncestorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataProjectAncestryAncestorsElRef {
    fn new(shared: StackShared, base: String) -> DataProjectAncestryAncestorsElRef {
        DataProjectAncestryAncestorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataProjectAncestryAncestorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
