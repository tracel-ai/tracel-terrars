use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataDiscoveryEngineDataStoresData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataDiscoveryEngineDataStores_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDiscoveryEngineDataStoresData>,
}
#[derive(Clone)]
pub struct DataDiscoveryEngineDataStores(Rc<DataDiscoveryEngineDataStores_>);
impl DataDiscoveryEngineDataStores {
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
    #[doc = "Set the field `location`.\nThe geographic location where the data stores reside. The value can only be one of \"global\", \"us\" and \"eu\"."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `data_stores` after provisioning.\n"]
    pub fn data_stores(&self) -> ListRef<DataDiscoveryEngineDataStoresDataStoresElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_stores", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data stores reside. The value can only be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
impl Referable for DataDiscoveryEngineDataStores {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataDiscoveryEngineDataStores {}
impl ToListMappable for DataDiscoveryEngineDataStores {
    type O = ListRef<DataDiscoveryEngineDataStoresRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataDiscoveryEngineDataStores_ {
    fn extract_datasource_type(&self) -> String {
        "google_discovery_engine_data_stores".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataDiscoveryEngineDataStores {
    pub tf_id: String,
}
impl BuildDataDiscoveryEngineDataStores {
    pub fn build(self, stack: &mut Stack) -> DataDiscoveryEngineDataStores {
        let out = DataDiscoveryEngineDataStores(Rc::new(DataDiscoveryEngineDataStores_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDiscoveryEngineDataStoresData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataDiscoveryEngineDataStoresRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoresRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataDiscoveryEngineDataStoresRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `data_stores` after provisioning.\n"]
    pub fn data_stores(&self) -> ListRef<DataDiscoveryEngineDataStoresDataStoresElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_stores", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data stores reside. The value can only be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoresDataStoresEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_schema_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    industry_vertical: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    solution_types: Option<ListField<PrimField<String>>>,
}
impl DataDiscoveryEngineDataStoresDataStoresEl {
    #[doc = "Set the field `content_config`.\n"]
    pub fn set_content_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_config = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `data_store_id`.\n"]
    pub fn set_data_store_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_store_id = Some(v.into());
        self
    }
    #[doc = "Set the field `default_schema_id`.\n"]
    pub fn set_default_schema_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_schema_id = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `industry_vertical`.\n"]
    pub fn set_industry_vertical(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.industry_vertical = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `solution_types`.\n"]
    pub fn set_solution_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.solution_types = Some(v.into());
        self
    }
}
impl ToListMappable for DataDiscoveryEngineDataStoresDataStoresEl {
    type O = BlockAssignable<DataDiscoveryEngineDataStoresDataStoresEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDiscoveryEngineDataStoresDataStoresEl {}
impl BuildDataDiscoveryEngineDataStoresDataStoresEl {
    pub fn build(self) -> DataDiscoveryEngineDataStoresDataStoresEl {
        DataDiscoveryEngineDataStoresDataStoresEl {
            content_config: core::default::Default::default(),
            create_time: core::default::Default::default(),
            data_store_id: core::default::Default::default(),
            default_schema_id: core::default::Default::default(),
            display_name: core::default::Default::default(),
            industry_vertical: core::default::Default::default(),
            name: core::default::Default::default(),
            solution_types: core::default::Default::default(),
        }
    }
}
pub struct DataDiscoveryEngineDataStoresDataStoresElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoresDataStoresElRef {
    fn new(shared: StackShared, base: String) -> DataDiscoveryEngineDataStoresDataStoresElRef {
        DataDiscoveryEngineDataStoresDataStoresElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDiscoveryEngineDataStoresDataStoresElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content_config` after provisioning.\n"]
    pub fn content_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `data_store_id` after provisioning.\n"]
    pub fn data_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_store_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_schema_id` after provisioning.\n"]
    pub fn default_schema_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_schema_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `industry_vertical` after provisioning.\n"]
    pub fn industry_vertical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.industry_vertical", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `solution_types` after provisioning.\n"]
    pub fn solution_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.solution_types", self.base),
        )
    }
}
