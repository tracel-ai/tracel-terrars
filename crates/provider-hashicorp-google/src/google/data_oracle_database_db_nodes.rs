use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOracleDatabaseDbNodesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cloud_vm_cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataOracleDatabaseDbNodes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOracleDatabaseDbNodesData>,
}
#[derive(Clone)]
pub struct DataOracleDatabaseDbNodes(Rc<DataOracleDatabaseDbNodes_>);
impl DataOracleDatabaseDbNodes {
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
    #[doc = "Get a reference to the value of field `cloud_vm_cluster` after provisioning.\nvmcluster"]
    pub fn cloud_vm_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_vm_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_nodes` after provisioning.\n"]
    pub fn db_nodes(&self) -> ListRef<DataOracleDatabaseDbNodesDbNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.db_nodes", self.extract_ref()),
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
impl Referable for DataOracleDatabaseDbNodes {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOracleDatabaseDbNodes {}
impl ToListMappable for DataOracleDatabaseDbNodes {
    type O = ListRef<DataOracleDatabaseDbNodesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOracleDatabaseDbNodes_ {
    fn extract_datasource_type(&self) -> String {
        "google_oracle_database_db_nodes".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOracleDatabaseDbNodes {
    pub tf_id: String,
    #[doc = "vmcluster"]
    pub cloud_vm_cluster: PrimField<String>,
    #[doc = "location"]
    pub location: PrimField<String>,
}
impl BuildDataOracleDatabaseDbNodes {
    pub fn build(self, stack: &mut Stack) -> DataOracleDatabaseDbNodes {
        let out = DataOracleDatabaseDbNodes(Rc::new(DataOracleDatabaseDbNodes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOracleDatabaseDbNodesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cloud_vm_cluster: self.cloud_vm_cluster,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataOracleDatabaseDbNodesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseDbNodesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOracleDatabaseDbNodesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `cloud_vm_cluster` after provisioning.\nvmcluster"]
    pub fn cloud_vm_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_vm_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_nodes` after provisioning.\n"]
    pub fn db_nodes(&self) -> ListRef<DataOracleDatabaseDbNodesDbNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.db_nodes", self.extract_ref()),
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
pub struct DataOracleDatabaseDbNodesDbNodesElPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    db_node_storage_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_server_ocid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocpu_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_cpu_core_count: Option<PrimField<f64>>,
}
impl DataOracleDatabaseDbNodesDbNodesElPropertiesEl {
    #[doc = "Set the field `db_node_storage_size_gb`.\n"]
    pub fn set_db_node_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.db_node_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `db_server_ocid`.\n"]
    pub fn set_db_server_ocid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_server_ocid = Some(v.into());
        self
    }
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname = Some(v.into());
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
    #[doc = "Set the field `total_cpu_core_count`.\n"]
    pub fn set_total_cpu_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_cpu_core_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataOracleDatabaseDbNodesDbNodesElPropertiesEl {
    type O = BlockAssignable<DataOracleDatabaseDbNodesDbNodesElPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseDbNodesDbNodesElPropertiesEl {}
impl BuildDataOracleDatabaseDbNodesDbNodesElPropertiesEl {
    pub fn build(self) -> DataOracleDatabaseDbNodesDbNodesElPropertiesEl {
        DataOracleDatabaseDbNodesDbNodesElPropertiesEl {
            db_node_storage_size_gb: core::default::Default::default(),
            db_server_ocid: core::default::Default::default(),
            hostname: core::default::Default::default(),
            memory_size_gb: core::default::Default::default(),
            ocid: core::default::Default::default(),
            ocpu_count: core::default::Default::default(),
            state: core::default::Default::default(),
            total_cpu_core_count: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseDbNodesDbNodesElPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseDbNodesDbNodesElPropertiesElRef {
    fn new(shared: StackShared, base: String) -> DataOracleDatabaseDbNodesDbNodesElPropertiesElRef {
        DataOracleDatabaseDbNodesDbNodesElPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseDbNodesDbNodesElPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `db_node_storage_size_gb` after provisioning.\n"]
    pub fn db_node_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_node_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `db_server_ocid` after provisioning.\n"]
    pub fn db_server_ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_server_ocid", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
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
    #[doc = "Get a reference to the value of field `total_cpu_core_count` after provisioning.\n"]
    pub fn total_cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_cpu_core_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseDbNodesDbNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<ListField<DataOracleDatabaseDbNodesDbNodesElPropertiesEl>>,
}
impl DataOracleDatabaseDbNodesDbNodesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl Into<ListField<DataOracleDatabaseDbNodesDbNodesElPropertiesEl>>,
    ) -> Self {
        self.properties = Some(v.into());
        self
    }
}
impl ToListMappable for DataOracleDatabaseDbNodesDbNodesEl {
    type O = BlockAssignable<DataOracleDatabaseDbNodesDbNodesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseDbNodesDbNodesEl {}
impl BuildDataOracleDatabaseDbNodesDbNodesEl {
    pub fn build(self) -> DataOracleDatabaseDbNodesDbNodesEl {
        DataOracleDatabaseDbNodesDbNodesEl {
            name: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseDbNodesDbNodesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseDbNodesDbNodesElRef {
    fn new(shared: StackShared, base: String) -> DataOracleDatabaseDbNodesDbNodesElRef {
        DataOracleDatabaseDbNodesDbNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseDbNodesDbNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> ListRef<DataOracleDatabaseDbNodesDbNodesElPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}
