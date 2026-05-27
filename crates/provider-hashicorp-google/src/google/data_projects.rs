use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataProjectsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    filter: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}
struct DataProjects_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectsData>,
}
#[derive(Clone)]
pub struct DataProjects(Rc<DataProjects_>);
impl DataProjects {
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
    #[doc = "Get a reference to the value of field `projects` after provisioning.\n"]
    pub fn projects(&self) -> ListRef<DataProjectsProjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.projects", self.extract_ref()),
        )
    }
}
impl Referable for DataProjects {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataProjects {}
impl ToListMappable for DataProjects {
    type O = ListRef<DataProjectsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataProjects_ {
    fn extract_datasource_type(&self) -> String {
        "google_projects".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataProjects {
    pub tf_id: String,
    #[doc = ""]
    pub filter: PrimField<String>,
}
impl BuildDataProjects {
    pub fn build(self, stack: &mut Stack) -> DataProjects {
        let out = DataProjects(Rc::new(DataProjects_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: self.filter,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataProjectsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataProjectsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataProjectsRef {
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
    #[doc = "Get a reference to the value of field `projects` after provisioning.\n"]
    pub fn projects(&self) -> ListRef<DataProjectsProjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.projects", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataProjectsProjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}
impl DataProjectsProjectsEl {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
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
    #[doc = "Set the field `number`.\n"]
    pub fn set_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.number = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\n"]
    pub fn set_parent(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parent = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataProjectsProjectsEl {
    type O = BlockAssignable<DataProjectsProjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataProjectsProjectsEl {}
impl BuildDataProjectsProjectsEl {
    pub fn build(self) -> DataProjectsProjectsEl {
        DataProjectsProjectsEl {
            create_time: core::default::Default::default(),
            labels: core::default::Default::default(),
            lifecycle_state: core::default::Default::default(),
            name: core::default::Default::default(),
            number: core::default::Default::default(),
            parent: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}
pub struct DataProjectsProjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataProjectsProjectsElRef {
    fn new(shared: StackShared, base: String) -> DataProjectsProjectsElRef {
        DataProjectsProjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataProjectsProjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
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
    #[doc = "Get a reference to the value of field `number` after provisioning.\n"]
    pub fn number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.number", self.base))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parent", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
