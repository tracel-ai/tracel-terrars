use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSqlDatabaseInstanceLatestRecoveryTimeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_instance_deletion_time: Option<PrimField<String>>,
}
struct DataSqlDatabaseInstanceLatestRecoveryTime_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSqlDatabaseInstanceLatestRecoveryTimeData>,
}
#[derive(Clone)]
pub struct DataSqlDatabaseInstanceLatestRecoveryTime(
    Rc<DataSqlDatabaseInstanceLatestRecoveryTime_>,
);
impl DataSqlDatabaseInstanceLatestRecoveryTime {
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
    #[doc = "Set the field `source_instance_deletion_time`.\nTimestamp, identifies when the source instance was deleted. If this instance is deleted, then you must set the timestamp."]
    pub fn set_source_instance_deletion_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_instance_deletion_time = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_recovery_time` after provisioning.\nTimestamp, identifies the latest recovery time of the source instance."]
    pub fn latest_recovery_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_recovery_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_instance_deletion_time` after provisioning.\nTimestamp, identifies when the source instance was deleted. If this instance is deleted, then you must set the timestamp."]
    pub fn source_instance_deletion_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instance_deletion_time", self.extract_ref()),
        )
    }
}
impl Referable for DataSqlDatabaseInstanceLatestRecoveryTime {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSqlDatabaseInstanceLatestRecoveryTime {}
impl ToListMappable for DataSqlDatabaseInstanceLatestRecoveryTime {
    type O = ListRef<DataSqlDatabaseInstanceLatestRecoveryTimeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSqlDatabaseInstanceLatestRecoveryTime_ {
    fn extract_datasource_type(&self) -> String {
        "google_sql_database_instance_latest_recovery_time".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSqlDatabaseInstanceLatestRecoveryTime {
    pub tf_id: String,
    #[doc = ""]
    pub instance: PrimField<String>,
}
impl BuildDataSqlDatabaseInstanceLatestRecoveryTime {
    pub fn build(self, stack: &mut Stack) -> DataSqlDatabaseInstanceLatestRecoveryTime {
        let out = DataSqlDatabaseInstanceLatestRecoveryTime(Rc::new(
            DataSqlDatabaseInstanceLatestRecoveryTime_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataSqlDatabaseInstanceLatestRecoveryTimeData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    instance: self.instance,
                    project: core::default::Default::default(),
                    source_instance_deletion_time: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSqlDatabaseInstanceLatestRecoveryTimeRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSqlDatabaseInstanceLatestRecoveryTimeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSqlDatabaseInstanceLatestRecoveryTimeRef {
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_recovery_time` after provisioning.\nTimestamp, identifies the latest recovery time of the source instance."]
    pub fn latest_recovery_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_recovery_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_instance_deletion_time` after provisioning.\nTimestamp, identifies when the source instance was deleted. If this instance is deleted, then you must set the timestamp."]
    pub fn source_instance_deletion_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instance_deletion_time", self.extract_ref()),
        )
    }
}
