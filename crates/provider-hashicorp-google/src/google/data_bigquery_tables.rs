use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBigqueryTablesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    dataset_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataBigqueryTables_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBigqueryTablesData>,
}
#[derive(Clone)]
pub struct DataBigqueryTables(Rc<DataBigqueryTables_>);
impl DataBigqueryTables {
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
    #[doc = "Set the field `project`.\nThe ID of the project in which the dataset is located. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing the tables."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the dataset is located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(&self) -> ListRef<DataBigqueryTablesTablesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tables", self.extract_ref()),
        )
    }
}
impl Referable for DataBigqueryTables {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBigqueryTables {}
impl ToListMappable for DataBigqueryTables {
    type O = ListRef<DataBigqueryTablesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBigqueryTables_ {
    fn extract_datasource_type(&self) -> String {
        "google_bigquery_tables".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBigqueryTables {
    pub tf_id: String,
    #[doc = "The ID of the dataset containing the tables."]
    pub dataset_id: PrimField<String>,
}
impl BuildDataBigqueryTables {
    pub fn build(self, stack: &mut Stack) -> DataBigqueryTables {
        let out = DataBigqueryTables(Rc::new(DataBigqueryTables_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBigqueryTablesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                dataset_id: self.dataset_id,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBigqueryTablesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTablesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBigqueryTablesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing the tables."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the dataset is located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(&self) -> ListRef<DataBigqueryTablesTablesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tables", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTablesTablesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_id: Option<PrimField<String>>,
}
impl DataBigqueryTablesTablesEl {
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `table_id`.\n"]
    pub fn set_table_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTablesTablesEl {
    type O = BlockAssignable<DataBigqueryTablesTablesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTablesTablesEl {}
impl BuildDataBigqueryTablesTablesEl {
    pub fn build(self) -> DataBigqueryTablesTablesEl {
        DataBigqueryTablesTablesEl {
            labels: core::default::Default::default(),
            table_id: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTablesTablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTablesTablesElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTablesTablesElRef {
        DataBigqueryTablesTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTablesTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\n"]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
