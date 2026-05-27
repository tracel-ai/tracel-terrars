use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBigqueryDatasetsData {
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
struct DataBigqueryDatasets_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBigqueryDatasetsData>,
}
#[derive(Clone)]
pub struct DataBigqueryDatasets(Rc<DataBigqueryDatasets_>);
impl DataBigqueryDatasets {
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
    #[doc = "Set the field `project`.\nThe ID of the project in which the datasets are located. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `datasets` after provisioning.\n"]
    pub fn datasets(&self) -> ListRef<DataBigqueryDatasetsDatasetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.datasets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the datasets are located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
impl Referable for DataBigqueryDatasets {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBigqueryDatasets {}
impl ToListMappable for DataBigqueryDatasets {
    type O = ListRef<DataBigqueryDatasetsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBigqueryDatasets_ {
    fn extract_datasource_type(&self) -> String {
        "google_bigquery_datasets".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBigqueryDatasets {
    pub tf_id: String,
}
impl BuildDataBigqueryDatasets {
    pub fn build(self, stack: &mut Stack) -> DataBigqueryDatasets {
        let out = DataBigqueryDatasets(Rc::new(DataBigqueryDatasets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBigqueryDatasetsData {
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
pub struct DataBigqueryDatasetsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryDatasetsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBigqueryDatasetsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `datasets` after provisioning.\n"]
    pub fn datasets(&self) -> ListRef<DataBigqueryDatasetsDatasetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.datasets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the datasets are located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryDatasetsDatasetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    friendly_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
}
impl DataBigqueryDatasetsDatasetsEl {
    #[doc = "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }
    #[doc = "Set the field `friendly_name`.\n"]
    pub fn set_friendly_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.friendly_name = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryDatasetsDatasetsEl {
    type O = BlockAssignable<DataBigqueryDatasetsDatasetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryDatasetsDatasetsEl {}
impl BuildDataBigqueryDatasetsDatasetsEl {
    pub fn build(self) -> DataBigqueryDatasetsDatasetsEl {
        DataBigqueryDatasetsDatasetsEl {
            dataset_id: core::default::Default::default(),
            friendly_name: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryDatasetsDatasetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryDatasetsDatasetsElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryDatasetsDatasetsElRef {
        DataBigqueryDatasetsDatasetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryDatasetsDatasetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `friendly_name` after provisioning.\n"]
    pub fn friendly_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.friendly_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}
