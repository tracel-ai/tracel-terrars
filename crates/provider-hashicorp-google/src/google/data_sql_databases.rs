use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSqlDatabasesData {
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
}
struct DataSqlDatabases_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSqlDatabasesData>,
}
#[derive(Clone)]
pub struct DataSqlDatabases(Rc<DataSqlDatabases_>);
impl DataSqlDatabases {
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
    #[doc = "Set the field `project`.\nProject ID of the project that contains the instance."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `databases` after provisioning.\n"]
    pub fn databases(&self) -> ListRef<DataSqlDatabasesDatabasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.databases", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the Cloud SQL database instance in which the database belongs."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project that contains the instance."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
impl Referable for DataSqlDatabases {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSqlDatabases {}
impl ToListMappable for DataSqlDatabases {
    type O = ListRef<DataSqlDatabasesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSqlDatabases_ {
    fn extract_datasource_type(&self) -> String {
        "google_sql_databases".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSqlDatabases {
    pub tf_id: String,
    #[doc = "The name of the Cloud SQL database instance in which the database belongs."]
    pub instance: PrimField<String>,
}
impl BuildDataSqlDatabases {
    pub fn build(self, stack: &mut Stack) -> DataSqlDatabases {
        let out = DataSqlDatabases(Rc::new(DataSqlDatabases_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSqlDatabasesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                instance: self.instance,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSqlDatabasesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSqlDatabasesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSqlDatabasesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `databases` after provisioning.\n"]
    pub fn databases(&self) -> ListRef<DataSqlDatabasesDatabasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.databases", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the Cloud SQL database instance in which the database belongs."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project that contains the instance."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataSqlDatabasesDatabasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    charset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
}
impl DataSqlDatabasesDatabasesEl {
    #[doc = "Set the field `charset`.\n"]
    pub fn set_charset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.charset = Some(v.into());
        self
    }
    #[doc = "Set the field `collation`.\n"]
    pub fn set_collation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.collation = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
}
impl ToListMappable for DataSqlDatabasesDatabasesEl {
    type O = BlockAssignable<DataSqlDatabasesDatabasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSqlDatabasesDatabasesEl {}
impl BuildDataSqlDatabasesDatabasesEl {
    pub fn build(self) -> DataSqlDatabasesDatabasesEl {
        DataSqlDatabasesDatabasesEl {
            charset: core::default::Default::default(),
            collation: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            instance: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
            self_link: core::default::Default::default(),
        }
    }
}
pub struct DataSqlDatabasesDatabasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSqlDatabasesDatabasesElRef {
    fn new(shared: StackShared, base: String) -> DataSqlDatabasesDatabasesElRef {
        DataSqlDatabasesDatabasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSqlDatabasesDatabasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `charset` after provisioning.\n"]
    pub fn charset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.charset", self.base))
    }
    #[doc = "Get a reference to the value of field `collation` after provisioning.\n"]
    pub fn collation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collation", self.base))
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
}
