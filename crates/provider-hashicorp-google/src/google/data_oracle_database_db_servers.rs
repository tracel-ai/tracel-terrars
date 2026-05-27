use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOracleDatabaseDbServersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cloud_exadata_infrastructure: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataOracleDatabaseDbServers_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOracleDatabaseDbServersData>,
}
#[derive(Clone)]
pub struct DataOracleDatabaseDbServers(Rc<DataOracleDatabaseDbServers_>);
impl DataOracleDatabaseDbServers {
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
    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructure` after provisioning.\nexadata"]
    pub fn cloud_exadata_infrastructure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_servers` after provisioning.\n"]
    pub fn db_servers(&self) -> ListRef<DataOracleDatabaseDbServersDbServersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.db_servers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nlocation"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the dataset is located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
impl Referable for DataOracleDatabaseDbServers {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOracleDatabaseDbServers {}
impl ToListMappable for DataOracleDatabaseDbServers {
    type O = ListRef<DataOracleDatabaseDbServersRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOracleDatabaseDbServers_ {
    fn extract_datasource_type(&self) -> String {
        "google_oracle_database_db_servers".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOracleDatabaseDbServers {
    pub tf_id: String,
    #[doc = "exadata"]
    pub cloud_exadata_infrastructure: PrimField<String>,
    #[doc = "location"]
    pub location: PrimField<String>,
}
impl BuildDataOracleDatabaseDbServers {
    pub fn build(self, stack: &mut Stack) -> DataOracleDatabaseDbServers {
        let out = DataOracleDatabaseDbServers(Rc::new(DataOracleDatabaseDbServers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOracleDatabaseDbServersData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cloud_exadata_infrastructure: self.cloud_exadata_infrastructure,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataOracleDatabaseDbServersRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseDbServersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOracleDatabaseDbServersRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructure` after provisioning.\nexadata"]
    pub fn cloud_exadata_infrastructure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_servers` after provisioning.\n"]
    pub fn db_servers(&self) -> ListRef<DataOracleDatabaseDbServersDbServersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.db_servers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nlocation"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the dataset is located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseDbServersDbServersElPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    db_node_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_node_storage_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_db_node_storage_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_memory_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ocpu_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocpu_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_count: Option<PrimField<f64>>,
}
impl DataOracleDatabaseDbServersDbServersElPropertiesEl {
    #[doc = "Set the field `db_node_ids`.\n"]
    pub fn set_db_node_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.db_node_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `db_node_storage_size_gb`.\n"]
    pub fn set_db_node_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.db_node_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `max_db_node_storage_size_gb`.\n"]
    pub fn set_max_db_node_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_db_node_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `max_memory_size_gb`.\n"]
    pub fn set_max_memory_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_memory_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `max_ocpu_count`.\n"]
    pub fn set_max_ocpu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_ocpu_count = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_size_gb`.\n"]
    pub fn set_memory_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `ocid`.\n"]
    pub fn set_ocid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ocid = Some(v.into());
        self
    }
    #[doc = "Set the field `ocpu_count`.\n"]
    pub fn set_ocpu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ocpu_count = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_count`.\n"]
    pub fn set_vm_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vm_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataOracleDatabaseDbServersDbServersElPropertiesEl {
    type O = BlockAssignable<DataOracleDatabaseDbServersDbServersElPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseDbServersDbServersElPropertiesEl {}
impl BuildDataOracleDatabaseDbServersDbServersElPropertiesEl {
    pub fn build(self) -> DataOracleDatabaseDbServersDbServersElPropertiesEl {
        DataOracleDatabaseDbServersDbServersElPropertiesEl {
            db_node_ids: core::default::Default::default(),
            db_node_storage_size_gb: core::default::Default::default(),
            max_db_node_storage_size_gb: core::default::Default::default(),
            max_memory_size_gb: core::default::Default::default(),
            max_ocpu_count: core::default::Default::default(),
            memory_size_gb: core::default::Default::default(),
            ocid: core::default::Default::default(),
            ocpu_count: core::default::Default::default(),
            state: core::default::Default::default(),
            vm_count: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseDbServersDbServersElPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseDbServersDbServersElPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseDbServersDbServersElPropertiesElRef {
        DataOracleDatabaseDbServersDbServersElPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseDbServersDbServersElPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `db_node_ids` after provisioning.\n"]
    pub fn db_node_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.db_node_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `db_node_storage_size_gb` after provisioning.\n"]
    pub fn db_node_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_node_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_db_node_storage_size_gb` after provisioning.\n"]
    pub fn max_db_node_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_db_node_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_memory_size_gb` after provisioning.\n"]
    pub fn max_memory_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_memory_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_ocpu_count` after provisioning.\n"]
    pub fn max_ocpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_ocpu_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\n"]
    pub fn memory_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }
    #[doc = "Get a reference to the value of field `ocpu_count` after provisioning.\n"]
    pub fn ocpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocpu_count", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_count` after provisioning.\n"]
    pub fn vm_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vm_count", self.base))
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseDbServersDbServersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<ListField<DataOracleDatabaseDbServersDbServersElPropertiesEl>>,
}
impl DataOracleDatabaseDbServersDbServersEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl Into<ListField<DataOracleDatabaseDbServersDbServersElPropertiesEl>>,
    ) -> Self {
        self.properties = Some(v.into());
        self
    }
}
impl ToListMappable for DataOracleDatabaseDbServersDbServersEl {
    type O = BlockAssignable<DataOracleDatabaseDbServersDbServersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseDbServersDbServersEl {}
impl BuildDataOracleDatabaseDbServersDbServersEl {
    pub fn build(self) -> DataOracleDatabaseDbServersDbServersEl {
        DataOracleDatabaseDbServersDbServersEl {
            display_name: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseDbServersDbServersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseDbServersDbServersElRef {
    fn new(shared: StackShared, base: String) -> DataOracleDatabaseDbServersDbServersElRef {
        DataOracleDatabaseDbServersDbServersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseDbServersDbServersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> ListRef<DataOracleDatabaseDbServersDbServersElPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}
