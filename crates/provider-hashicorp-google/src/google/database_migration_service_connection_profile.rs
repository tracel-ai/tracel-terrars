use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DatabaseMigrationServiceConnectionProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    connection_profile_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alloydb: Option<Vec<DatabaseMigrationServiceConnectionProfileAlloydbEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudsql: Option<Vec<DatabaseMigrationServiceConnectionProfileCloudsqlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mysql: Option<Vec<DatabaseMigrationServiceConnectionProfileMysqlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle: Option<Vec<DatabaseMigrationServiceConnectionProfileOracleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postgresql: Option<Vec<DatabaseMigrationServiceConnectionProfilePostgresqlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DatabaseMigrationServiceConnectionProfileTimeoutsEl>,
    dynamic: DatabaseMigrationServiceConnectionProfileDynamic,
}
struct DatabaseMigrationServiceConnectionProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatabaseMigrationServiceConnectionProfileData>,
}
#[derive(Clone)]
pub struct DatabaseMigrationServiceConnectionProfile(
    Rc<DatabaseMigrationServiceConnectionProfile_>,
);
impl DatabaseMigrationServiceConnectionProfile {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }
    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }
    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }
    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe connection profile display name."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location where the connection profile should reside."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `role`.\nThe connection profile role. Possible values: [\"SOURCE\", \"DESTINATION\"]"]
    pub fn set_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role = Some(v.into());
        self
    }
    #[doc = "Set the field `alloydb`.\n"]
    pub fn set_alloydb(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceConnectionProfileAlloydbEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alloydb = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alloydb = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloudsql`.\n"]
    pub fn set_cloudsql(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceConnectionProfileCloudsqlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudsql = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudsql = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mysql`.\n"]
    pub fn set_mysql(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceConnectionProfileMysqlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mysql = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mysql = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oracle`.\n"]
    pub fn set_oracle(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceConnectionProfileOracleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oracle = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oracle = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `postgresql`.\n"]
    pub fn set_postgresql(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceConnectionProfilePostgresqlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().postgresql = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.postgresql = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<DatabaseMigrationServiceConnectionProfileTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `connection_profile_id` after provisioning.\nThe ID of the connection profile."]
    pub fn connection_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_profile_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the resource was created. A timestamp in RFC3339 UTC 'Zulu' format, accurate to nanoseconds. Example: '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dbprovider` after provisioning.\nThe database provider."]
    pub fn dbprovider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dbprovider", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe connection profile display name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\nOutput only. The error details in case of state FAILED."]
    pub fn error(&self) -> ListRef<DatabaseMigrationServiceConnectionProfileErrorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the connection profile should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nThe connection profile role. Possible values: [\"SOURCE\", \"DESTINATION\"]"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current connection profile state."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alloydb` after provisioning.\n"]
    pub fn alloydb(&self) -> ListRef<DatabaseMigrationServiceConnectionProfileAlloydbElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alloydb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloudsql` after provisioning.\n"]
    pub fn cloudsql(&self) -> ListRef<DatabaseMigrationServiceConnectionProfileCloudsqlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudsql", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mysql` after provisioning.\n"]
    pub fn mysql(&self) -> ListRef<DatabaseMigrationServiceConnectionProfileMysqlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oracle` after provisioning.\n"]
    pub fn oracle(&self) -> ListRef<DatabaseMigrationServiceConnectionProfileOracleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `postgresql` after provisioning.\n"]
    pub fn postgresql(&self) -> ListRef<DatabaseMigrationServiceConnectionProfilePostgresqlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseMigrationServiceConnectionProfileTimeoutsElRef {
        DatabaseMigrationServiceConnectionProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DatabaseMigrationServiceConnectionProfile {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DatabaseMigrationServiceConnectionProfile {}
impl ToListMappable for DatabaseMigrationServiceConnectionProfile {
    type O = ListRef<DatabaseMigrationServiceConnectionProfileRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DatabaseMigrationServiceConnectionProfile_ {
    fn extract_resource_type(&self) -> String {
        "google_database_migration_service_connection_profile".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfile {
    pub tf_id: String,
    #[doc = "The ID of the connection profile."]
    pub connection_profile_id: PrimField<String>,
}
impl BuildDatabaseMigrationServiceConnectionProfile {
    pub fn build(self, stack: &mut Stack) -> DatabaseMigrationServiceConnectionProfile {
        let out = DatabaseMigrationServiceConnectionProfile(Rc::new(
            DatabaseMigrationServiceConnectionProfile_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DatabaseMigrationServiceConnectionProfileData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    connection_profile_id: self.connection_profile_id,
                    deletion_policy: core::default::Default::default(),
                    display_name: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: core::default::Default::default(),
                    project: core::default::Default::default(),
                    role: core::default::Default::default(),
                    alloydb: core::default::Default::default(),
                    cloudsql: core::default::Default::default(),
                    mysql: core::default::Default::default(),
                    oracle: core::default::Default::default(),
                    postgresql: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DatabaseMigrationServiceConnectionProfileRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DatabaseMigrationServiceConnectionProfileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_profile_id` after provisioning.\nThe ID of the connection profile."]
    pub fn connection_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_profile_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the resource was created. A timestamp in RFC3339 UTC 'Zulu' format, accurate to nanoseconds. Example: '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dbprovider` after provisioning.\nThe database provider."]
    pub fn dbprovider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dbprovider", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe connection profile display name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\nOutput only. The error details in case of state FAILED."]
    pub fn error(&self) -> ListRef<DatabaseMigrationServiceConnectionProfileErrorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the connection profile should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nThe connection profile role. Possible values: [\"SOURCE\", \"DESTINATION\"]"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current connection profile state."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alloydb` after provisioning.\n"]
    pub fn alloydb(&self) -> ListRef<DatabaseMigrationServiceConnectionProfileAlloydbElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alloydb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloudsql` after provisioning.\n"]
    pub fn cloudsql(&self) -> ListRef<DatabaseMigrationServiceConnectionProfileCloudsqlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudsql", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mysql` after provisioning.\n"]
    pub fn mysql(&self) -> ListRef<DatabaseMigrationServiceConnectionProfileMysqlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oracle` after provisioning.\n"]
    pub fn oracle(&self) -> ListRef<DatabaseMigrationServiceConnectionProfileOracleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `postgresql` after provisioning.\n"]
    pub fn postgresql(&self) -> ListRef<DatabaseMigrationServiceConnectionProfilePostgresqlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseMigrationServiceConnectionProfileTimeoutsElRef {
        DatabaseMigrationServiceConnectionProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<ListField<RecField<PrimField<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl DatabaseMigrationServiceConnectionProfileErrorEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<ListField<RecField<PrimField<String>>>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileErrorEl {
    type O = BlockAssignable<DatabaseMigrationServiceConnectionProfileErrorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileErrorEl {}
impl BuildDatabaseMigrationServiceConnectionProfileErrorEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfileErrorEl {
        DatabaseMigrationServiceConnectionProfileErrorEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileErrorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileErrorElRef {
        DatabaseMigrationServiceConnectionProfileErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<RecRef<PrimExpr<String>>> {
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserEl {
    password: PrimField<String>,
    user: PrimField<String>,
}
impl DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserEl {}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserEl {
    type O =
        BlockAssignable<DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserEl {
    #[doc = "The initial password for the user."]
    pub password: PrimField<String>,
    #[doc = "The database username."]
    pub user: PrimField<String>,
}
impl BuildDatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserEl {
    pub fn build(
        self,
    ) -> DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserEl {
        DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserEl {
            password: self.password,
            user: self.user,
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserElRef {
        DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe initial password for the user."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `password_set` after provisioning.\nOutput only. Indicates if the initialUser.password field has been set."]
    pub fn password_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_set", self.base))
    }
    #[doc = "Get a reference to the value of field `user` after provisioning.\nThe database username."]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigEl
{
    cpu_count: PrimField<f64>,
}
impl DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigEl { }
impl ToListMappable for DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigEl { type O = BlockAssignable < DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigEl
{
    #[doc = "The number of CPU's in the VM instance."]
    pub cpu_count: PrimField<f64>,
}
impl BuildDatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigEl { pub fn build (self) -> DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigEl { DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigEl { cpu_count : self . cpu_count , } } }
pub struct DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigElRef { fn new (shared : StackShared , base : String) -> DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigElRef { DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigElRef { shared : shared , base : base . to_string () , } } }
impl DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `cpu_count` after provisioning.\nThe number of CPU's in the VM instance."] pub fn cpu_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.cpu_count" , self . base)) } }
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElDynamic { machine_config : Option < DynamicBlock < DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigEl >> , }
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] database_flags : Option < RecField < PrimField < String > > > , id : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] labels : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] machine_config : Option < Vec < DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigEl > > , dynamic : DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElDynamic , }
impl DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsEl {
    #[doc = "Set the field `database_flags`.\nDatabase flags to pass to AlloyDB when DMS is creating the AlloyDB cluster and instances. See the AlloyDB documentation for how these can be used."]
    pub fn set_database_flags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.database_flags = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels for the AlloyDB primary instance created by DMS."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_config`.\n"]
    pub fn set_machine_config(
        mut self,
        v : impl Into < BlockAssignable < DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.machine_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.machine_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsEl
{
    type O = BlockAssignable<
        DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsEl
{
    #[doc = "The database username."]
    pub id: PrimField<String>,
}
impl BuildDatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsEl {
    pub fn build(
        self,
    ) -> DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsEl {
        DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsEl {
            database_flags: core::default::Default::default(),
            id: self.id,
            labels: core::default::Default::default(),
            machine_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElRef
    {
        DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database_flags` after provisioning.\nDatabase flags to pass to AlloyDB when DMS is creating the AlloyDB cluster and instances. See the AlloyDB documentation for how these can be used."]
    pub fn database_flags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.database_flags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe database username."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels for the AlloyDB primary instance created by DMS."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\nOutput only. The private IP address for the Instance. This is the connection endpoint for an end-user application."]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `machine_config` after provisioning.\n"]    pub fn machine_config (& self) -> ListRef < DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElMachineConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.machine_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElDynamic {
    initial_user: Option<
        DynamicBlock<DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserEl>,
    >,
    primary_instance_settings: Option<
        DynamicBlock<
            DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileAlloydbElSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    vpc_network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_user:
        Option<Vec<DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_instance_settings: Option<
        Vec<DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsEl>,
    >,
    dynamic: DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElDynamic,
}
impl DatabaseMigrationServiceConnectionProfileAlloydbElSettingsEl {
    #[doc = "Set the field `labels`.\nLabels for the AlloyDB cluster created by DMS."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_user`.\n"]
    pub fn set_initial_user(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.initial_user = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.initial_user = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `primary_instance_settings`.\n"]
    pub fn set_primary_instance_settings(
        mut self,
        v : impl Into < BlockAssignable < DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.primary_instance_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.primary_instance_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileAlloydbElSettingsEl {
    type O = BlockAssignable<DatabaseMigrationServiceConnectionProfileAlloydbElSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileAlloydbElSettingsEl {
    #[doc = "Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster.\nIt is specified in the form: 'projects/{project_number}/global/networks/{network_id}'. This is required to create a cluster."]
    pub vpc_network: PrimField<String>,
}
impl BuildDatabaseMigrationServiceConnectionProfileAlloydbElSettingsEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfileAlloydbElSettingsEl {
        DatabaseMigrationServiceConnectionProfileAlloydbElSettingsEl {
            labels: core::default::Default::default(),
            vpc_network: self.vpc_network,
            initial_user: core::default::Default::default(),
            primary_instance_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElRef {
        DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels for the AlloyDB cluster created by DMS."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_network` after provisioning.\nRequired. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster.\nIt is specified in the form: 'projects/{project_number}/global/networks/{network_id}'. This is required to create a cluster."]
    pub fn vpc_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_network", self.base))
    }
    #[doc = "Get a reference to the value of field `initial_user` after provisioning.\n"]
    pub fn initial_user(
        &self,
    ) -> ListRef<DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElInitialUserElRef> {
        ListRef::new(self.shared().clone(), format!("{}.initial_user", self.base))
    }
    #[doc = "Get a reference to the value of field `primary_instance_settings` after provisioning.\n"]
    pub fn primary_instance_settings(
        &self,
    ) -> ListRef<
        DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElPrimaryInstanceSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_instance_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfileAlloydbElDynamic {
    settings: Option<DynamicBlock<DatabaseMigrationServiceConnectionProfileAlloydbElSettingsEl>>,
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileAlloydbEl {
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<Vec<DatabaseMigrationServiceConnectionProfileAlloydbElSettingsEl>>,
    dynamic: DatabaseMigrationServiceConnectionProfileAlloydbElDynamic,
}
impl DatabaseMigrationServiceConnectionProfileAlloydbEl {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceConnectionProfileAlloydbElSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileAlloydbEl {
    type O = BlockAssignable<DatabaseMigrationServiceConnectionProfileAlloydbEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileAlloydbEl {
    #[doc = "Required. The AlloyDB cluster ID that this connection profile is associated with."]
    pub cluster_id: PrimField<String>,
}
impl BuildDatabaseMigrationServiceConnectionProfileAlloydbEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfileAlloydbEl {
        DatabaseMigrationServiceConnectionProfileAlloydbEl {
            cluster_id: self.cluster_id,
            settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileAlloydbElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileAlloydbElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileAlloydbElRef {
        DatabaseMigrationServiceConnectionProfileAlloydbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileAlloydbElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nRequired. The AlloyDB cluster ID that this connection profile is associated with."]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(
        &self,
    ) -> ListRef<DatabaseMigrationServiceConnectionProfileAlloydbElSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<String>>,
    value: PrimField<String>,
}
impl DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksEl {
    #[doc = "Set the field `expire_time`.\nThe time when this access control entry expires in RFC 3339 format."]
    pub fn set_expire_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expire_time = Some(v.into());
        self
    }
    #[doc = "Set the field `label`.\nA label to identify this entry."]
    pub fn set_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.label = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\nInput only. The time-to-leave of this access control entry."]
    pub fn set_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
}
impl ToListMappable
    for DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksEl
{
    type O = BlockAssignable<
        DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksEl
{
    #[doc = "The allowlisted value for the access control list."]
    pub value: PrimField<String>,
}
impl
    BuildDatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksEl
{
    pub fn build(
        self,
    ) -> DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksEl
    {
        DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksEl { expire_time : core :: default :: Default :: default () , label : core :: default :: Default :: default () , ttl : core :: default :: Default :: default () , value : self . value , }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksElRef { fn new (shared : StackShared , base : String) -> DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksElRef { DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksElRef { shared : shared , base : base . to_string () , } } }
impl
    DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nThe time when this access control entry expires in RFC 3339 format."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
    #[doc = "Get a reference to the value of field `label` after provisioning.\nA label to identify this entry."]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nInput only. The time-to-leave of this access control entry."]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe allowlisted value for the access control list."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElDynamic { authorized_networks : Option < DynamicBlock < DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksEl >> , }
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigEl { # [serde (skip_serializing_if = "Option::is_none")] enable_ipv4 : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] private_network : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] require_ssl : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] authorized_networks : Option < Vec < DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksEl > > , dynamic : DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElDynamic , }
impl DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigEl {
    #[doc = "Set the field `enable_ipv4`.\nWhether the instance should be assigned an IPv4 address or not."]
    pub fn set_enable_ipv4(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ipv4 = Some(v.into());
        self
    }
    #[doc = "Set the field `private_network`.\nThe resource link for the VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default.\nThis setting can be updated, but it cannot be removed after it is set."]
    pub fn set_private_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_network = Some(v.into());
        self
    }
    #[doc = "Set the field `require_ssl`.\nWhether SSL connections over IP should be enforced or not."]
    pub fn set_require_ssl(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_ssl = Some(v.into());
        self
    }
    #[doc = "Set the field `authorized_networks`.\n"]
    pub fn set_authorized_networks(
        mut self,
        v : impl Into < BlockAssignable < DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authorized_networks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authorized_networks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigEl {
    type O =
        BlockAssignable<DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigEl {}
impl BuildDatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigEl {
        DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigEl {
            enable_ipv4: core::default::Default::default(),
            private_network: core::default::Default::default(),
            require_ssl: core::default::Default::default(),
            authorized_networks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElRef {
        DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_ipv4` after provisioning.\nWhether the instance should be assigned an IPv4 address or not."]
    pub fn enable_ipv4(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ipv4", self.base))
    }
    #[doc = "Get a reference to the value of field `private_network` after provisioning.\nThe resource link for the VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default.\nThis setting can be updated, but it cannot be removed after it is set."]
    pub fn private_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `require_ssl` after provisioning.\nWhether SSL connections over IP should be enforced or not."]
    pub fn require_ssl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_ssl", self.base))
    }
    #[doc = "Get a reference to the value of field `authorized_networks` after provisioning.\n"]    pub fn authorized_networks (& self) -> ListRef < DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElAuthorizedNetworksElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorized_networks", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElDynamic {
    ip_config: Option<
        DynamicBlock<DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_storage_increase: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmek_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_disk_size_gb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_flags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_password: Option<PrimField<String>>,
    source_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_auto_resize_limit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_config: Option<Vec<DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigEl>>,
    dynamic: DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElDynamic,
}
impl DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsEl {
    #[doc = "Set the field `activation_policy`.\nThe activation policy specifies when the instance is activated; it is applicable only when the instance state is 'RUNNABLE'. Possible values: [\"ALWAYS\", \"NEVER\"]"]
    pub fn set_activation_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.activation_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_storage_increase`.\nIf you enable this setting, Cloud SQL checks your available storage every 30 seconds. If the available storage falls below a threshold size, Cloud SQL automatically adds additional storage capacity.\nIf the available storage repeatedly falls below the threshold size, Cloud SQL continues to add storage until it reaches the maximum of 30 TB."]
    pub fn set_auto_storage_increase(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_storage_increase = Some(v.into());
        self
    }
    #[doc = "Set the field `cmek_key_name`.\nThe KMS key name used for the csql instance."]
    pub fn set_cmek_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cmek_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `collation`.\nThe Cloud SQL default instance level collation."]
    pub fn set_collation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.collation = Some(v.into());
        self
    }
    #[doc = "Set the field `data_disk_size_gb`.\nThe storage capacity available to the database, in GB. The minimum (and default) size is 10GB."]
    pub fn set_data_disk_size_gb(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `data_disk_type`.\nThe type of storage. Possible values: [\"PD_SSD\", \"PD_HDD\"]"]
    pub fn set_data_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `database_flags`.\nThe database flags passed to the Cloud SQL instance at startup."]
    pub fn set_database_flags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.database_flags = Some(v.into());
        self
    }
    #[doc = "Set the field `database_version`.\nThe database engine type and version.\nCurrently supported values located at https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.connectionProfiles#sqldatabaseversion"]
    pub fn set_database_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database_version = Some(v.into());
        self
    }
    #[doc = "Set the field `edition`.\nThe edition of the given Cloud SQL instance. Possible values: [\"ENTERPRISE\", \"ENTERPRISE_PLUS\"]"]
    pub fn set_edition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.edition = Some(v.into());
        self
    }
    #[doc = "Set the field `root_password`.\nInput only. Initial root password."]
    pub fn set_root_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.root_password = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_auto_resize_limit`.\nThe maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit."]
    pub fn set_storage_auto_resize_limit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_auto_resize_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `tier`.\nThe tier (or machine type) for this instance, for example: db-n1-standard-1 (MySQL instances) or db-custom-1-3840 (PostgreSQL instances).\nFor more information, see https://cloud.google.com/sql/docs/mysql/instance-settings"]
    pub fn set_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tier = Some(v.into());
        self
    }
    #[doc = "Set the field `user_labels`.\nThe resource labels for a Cloud SQL instance to use to annotate any related underlying resources such as Compute Engine VMs."]
    pub fn set_user_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.user_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe Google Cloud Platform zone where your Cloud SQL datdabse instance is located."]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_config`.\n"]
    pub fn set_ip_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsEl {
    type O = BlockAssignable<DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileCloudsqlElSettingsEl {
    #[doc = "The Database Migration Service source connection profile ID, in the format: projects/my_project_name/locations/us-central1/connectionProfiles/connection_profile_ID"]
    pub source_id: PrimField<String>,
}
impl BuildDatabaseMigrationServiceConnectionProfileCloudsqlElSettingsEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsEl {
        DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsEl {
            activation_policy: core::default::Default::default(),
            auto_storage_increase: core::default::Default::default(),
            cmek_key_name: core::default::Default::default(),
            collation: core::default::Default::default(),
            data_disk_size_gb: core::default::Default::default(),
            data_disk_type: core::default::Default::default(),
            database_flags: core::default::Default::default(),
            database_version: core::default::Default::default(),
            edition: core::default::Default::default(),
            root_password: core::default::Default::default(),
            source_id: self.source_id,
            storage_auto_resize_limit: core::default::Default::default(),
            tier: core::default::Default::default(),
            user_labels: core::default::Default::default(),
            zone: core::default::Default::default(),
            ip_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElRef {
        DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `activation_policy` after provisioning.\nThe activation policy specifies when the instance is activated; it is applicable only when the instance state is 'RUNNABLE'. Possible values: [\"ALWAYS\", \"NEVER\"]"]
    pub fn activation_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activation_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auto_storage_increase` after provisioning.\nIf you enable this setting, Cloud SQL checks your available storage every 30 seconds. If the available storage falls below a threshold size, Cloud SQL automatically adds additional storage capacity.\nIf the available storage repeatedly falls below the threshold size, Cloud SQL continues to add storage until it reaches the maximum of 30 TB."]
    pub fn auto_storage_increase(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_storage_increase", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cmek_key_name` after provisioning.\nThe KMS key name used for the csql instance."]
    pub fn cmek_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cmek_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `collation` after provisioning.\nThe Cloud SQL default instance level collation."]
    pub fn collation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collation", self.base))
    }
    #[doc = "Get a reference to the value of field `data_disk_size_gb` after provisioning.\nThe storage capacity available to the database, in GB. The minimum (and default) size is 10GB."]
    pub fn data_disk_size_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_disk_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_disk_type` after provisioning.\nThe type of storage. Possible values: [\"PD_SSD\", \"PD_HDD\"]"]
    pub fn data_disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_disk_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database_flags` after provisioning.\nThe database flags passed to the Cloud SQL instance at startup."]
    pub fn database_flags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.database_flags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database_version` after provisioning.\nThe database engine type and version.\nCurrently supported values located at https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.connectionProfiles#sqldatabaseversion"]
    pub fn database_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `edition` after provisioning.\nThe edition of the given Cloud SQL instance. Possible values: [\"ENTERPRISE\", \"ENTERPRISE_PLUS\"]"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edition", self.base))
    }
    #[doc = "Get a reference to the value of field `root_password` after provisioning.\nInput only. Initial root password."]
    pub fn root_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `root_password_set` after provisioning.\nOutput only. Indicates If this connection profile root password is stored."]
    pub fn root_password_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_password_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_id` after provisioning.\nThe Database Migration Service source connection profile ID, in the format: projects/my_project_name/locations/us-central1/connectionProfiles/connection_profile_ID"]
    pub fn source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_auto_resize_limit` after provisioning.\nThe maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit."]
    pub fn storage_auto_resize_limit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_auto_resize_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nThe tier (or machine type) for this instance, for example: db-n1-standard-1 (MySQL instances) or db-custom-1-3840 (PostgreSQL instances).\nFor more information, see https://cloud.google.com/sql/docs/mysql/instance-settings"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }
    #[doc = "Get a reference to the value of field `user_labels` after provisioning.\nThe resource labels for a Cloud SQL instance to use to annotate any related underlying resources such as Compute Engine VMs."]
    pub fn user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.user_labels", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe Google Cloud Platform zone where your Cloud SQL datdabse instance is located."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_config` after provisioning.\n"]
    pub fn ip_config(
        &self,
    ) -> ListRef<DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElIpConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfileCloudsqlElDynamic {
    settings: Option<DynamicBlock<DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsEl>>,
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileCloudsqlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<Vec<DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsEl>>,
    dynamic: DatabaseMigrationServiceConnectionProfileCloudsqlElDynamic,
}
impl DatabaseMigrationServiceConnectionProfileCloudsqlEl {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileCloudsqlEl {
    type O = BlockAssignable<DatabaseMigrationServiceConnectionProfileCloudsqlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileCloudsqlEl {}
impl BuildDatabaseMigrationServiceConnectionProfileCloudsqlEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfileCloudsqlEl {
        DatabaseMigrationServiceConnectionProfileCloudsqlEl {
            settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileCloudsqlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileCloudsqlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileCloudsqlElRef {
        DatabaseMigrationServiceConnectionProfileCloudsqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileCloudsqlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_sql_id` after provisioning.\nOutput only. The Cloud SQL instance ID that this connection profile is associated with."]
    pub fn cloud_sql_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_id", self.base))
    }
    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\nOutput only. The Cloud SQL database instance's private IP."]
    pub fn private_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\nOutput only. The Cloud SQL database instance's public IP."]
    pub fn public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(
        &self,
    ) -> ListRef<DatabaseMigrationServiceConnectionProfileCloudsqlElSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileMysqlElSslEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DatabaseMigrationServiceConnectionProfileMysqlElSslEl {
    #[doc = "Set the field `ca_certificate`.\nInput only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate.\nThe replica will use this certificate to verify it's connecting to the right host."]
    pub fn set_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate`.\nInput only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.\nIf this field is used then the 'clientKey' field is mandatory"]
    pub fn set_client_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_key`.\nInput only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'clientCertificate' field is mandatory."]
    pub fn set_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_key = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe current connection profile state. Possible values: [\"SERVER_ONLY\", \"SERVER_CLIENT\", \"REQUIRED\", \"NONE\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileMysqlElSslEl {
    type O = BlockAssignable<DatabaseMigrationServiceConnectionProfileMysqlElSslEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileMysqlElSslEl {}
impl BuildDatabaseMigrationServiceConnectionProfileMysqlElSslEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfileMysqlElSslEl {
        DatabaseMigrationServiceConnectionProfileMysqlElSslEl {
            ca_certificate: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileMysqlElSslElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileMysqlElSslElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileMysqlElSslElRef {
        DatabaseMigrationServiceConnectionProfileMysqlElSslElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileMysqlElSslElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\nInput only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate.\nThe replica will use this certificate to verify it's connecting to the right host."]
    pub fn ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\nInput only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.\nIf this field is used then the 'clientKey' field is mandatory"]
    pub fn client_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_key` after provisioning.\nInput only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'clientCertificate' field is mandatory."]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe current connection profile state. Possible values: [\"SERVER_ONLY\", \"SERVER_CLIENT\", \"REQUIRED\", \"NONE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfileMysqlElDynamic {
    ssl: Option<DynamicBlock<DatabaseMigrationServiceConnectionProfileMysqlElSslEl>>,
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileMysqlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl: Option<Vec<DatabaseMigrationServiceConnectionProfileMysqlElSslEl>>,
    dynamic: DatabaseMigrationServiceConnectionProfileMysqlElDynamic,
}
impl DatabaseMigrationServiceConnectionProfileMysqlEl {
    #[doc = "Set the field `cloud_sql_id`.\nIf the connection profile is a Cloud SQL database, use this field to provide the Cloud SQL instance ID."]
    pub fn set_cloud_sql_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_sql_id = Some(v.into());
        self
    }
    #[doc = "Set the field `host`.\nThe IP or hostname of the source MySQL database."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `password`.\nInput only. The password for the user that Database Migration Service will be using to connect to the database.\nThis field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe network port of the source MySQL database."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\nThe username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl`.\n"]
    pub fn set_ssl(
        mut self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceConnectionProfileMysqlElSslEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssl = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileMysqlEl {
    type O = BlockAssignable<DatabaseMigrationServiceConnectionProfileMysqlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileMysqlEl {}
impl BuildDatabaseMigrationServiceConnectionProfileMysqlEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfileMysqlEl {
        DatabaseMigrationServiceConnectionProfileMysqlEl {
            cloud_sql_id: core::default::Default::default(),
            host: core::default::Default::default(),
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            username: core::default::Default::default(),
            ssl: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileMysqlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileMysqlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileMysqlElRef {
        DatabaseMigrationServiceConnectionProfileMysqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileMysqlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_sql_id` after provisioning.\nIf the connection profile is a Cloud SQL database, use this field to provide the Cloud SQL instance ID."]
    pub fn cloud_sql_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_id", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe IP or hostname of the source MySQL database."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nInput only. The password for the user that Database Migration Service will be using to connect to the database.\nThis field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `password_set` after provisioning.\nOutput only. Indicates If this connection profile password is stored."]
    pub fn password_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_set", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe network port of the source MySQL database."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> ListRef<DatabaseMigrationServiceConnectionProfileMysqlElSslElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssl", self.base))
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityEl {
    hostname: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<PrimField<String>>,
    username: PrimField<String>,
}
impl DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityEl {
    #[doc = "Set the field `password`.\nInput only. SSH password. Only one of 'password' and 'private_key' can be configured."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `private_key`.\nInput only. SSH private key. Only one of 'password' and 'private_key' can be configured."]
    pub fn set_private_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_key = Some(v.into());
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityEl {
    type O =
        BlockAssignable<DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityEl {
    #[doc = "Required. Hostname for the SSH tunnel."]
    pub hostname: PrimField<String>,
    #[doc = "Port for the SSH tunnel, default value is 22."]
    pub port: PrimField<f64>,
    #[doc = "Required. Username for the SSH tunnel."]
    pub username: PrimField<String>,
}
impl BuildDatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityEl {
    pub fn build(
        self,
    ) -> DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityEl {
        DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityEl {
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: self.port,
            private_key: core::default::Default::default(),
            username: self.username,
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityElRef {
        DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nRequired. Hostname for the SSH tunnel."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nInput only. SSH password. Only one of 'password' and 'private_key' can be configured."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the SSH tunnel, default value is 22."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `private_key` after provisioning.\nInput only. SSH private key. Only one of 'password' and 'private_key' can be configured."]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nRequired. Username for the SSH tunnel."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityEl {
    private_connection: PrimField<String>,
}
impl DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityEl {}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityEl {
    type O =
        BlockAssignable<DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityEl {
    #[doc = "Required. The resource name (URI) of the private connection."]
    pub private_connection: PrimField<String>,
}
impl BuildDatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityEl {
        DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityEl {
            private_connection: self.private_connection,
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityElRef {
        DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `private_connection` after provisioning.\nRequired. The resource name (URI) of the private connection."]
    pub fn private_connection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_connection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileOracleElSslEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
}
impl DatabaseMigrationServiceConnectionProfileOracleElSslEl {
    #[doc = "Set the field `ca_certificate`.\nInput only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate.\nThe replica will use this certificate to verify it's connecting to the right host."]
    pub fn set_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate`.\nInput only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.\nIf this field is used then the 'clientKey' field is mandatory"]
    pub fn set_client_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_key`.\nInput only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'clientCertificate' field is mandatory."]
    pub fn set_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_key = Some(v.into());
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileOracleElSslEl {
    type O = BlockAssignable<DatabaseMigrationServiceConnectionProfileOracleElSslEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileOracleElSslEl {}
impl BuildDatabaseMigrationServiceConnectionProfileOracleElSslEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfileOracleElSslEl {
        DatabaseMigrationServiceConnectionProfileOracleElSslEl {
            ca_certificate: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileOracleElSslElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileOracleElSslElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileOracleElSslElRef {
        DatabaseMigrationServiceConnectionProfileOracleElSslElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileOracleElSslElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\nInput only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate.\nThe replica will use this certificate to verify it's connecting to the right host."]
    pub fn ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\nInput only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.\nIf this field is used then the 'clientKey' field is mandatory"]
    pub fn client_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_key` after provisioning.\nInput only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'clientCertificate' field is mandatory."]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe current connection profile state."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityEl {}
impl DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityEl {}
impl ToListMappable
    for DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityEl
{
    type O = BlockAssignable<
        DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityEl {}
impl BuildDatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityEl {
    pub fn build(
        self,
    ) -> DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityEl {
        DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityEl {}
    }
}
pub struct DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityElRef {
        DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfileOracleElDynamic {
    forward_ssh_connectivity: Option<
        DynamicBlock<DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityEl>,
    >,
    private_connectivity: Option<
        DynamicBlock<DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityEl>,
    >,
    ssl: Option<DynamicBlock<DatabaseMigrationServiceConnectionProfileOracleElSslEl>>,
    static_service_ip_connectivity: Option<
        DynamicBlock<
            DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileOracleEl {
    database_service: PrimField<String>,
    host: PrimField<String>,
    password: PrimField<String>,
    port: PrimField<f64>,
    username: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_ssh_connectivity:
        Option<Vec<DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_connectivity:
        Option<Vec<DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl: Option<Vec<DatabaseMigrationServiceConnectionProfileOracleElSslEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_service_ip_connectivity:
        Option<Vec<DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityEl>>,
    dynamic: DatabaseMigrationServiceConnectionProfileOracleElDynamic,
}
impl DatabaseMigrationServiceConnectionProfileOracleEl {
    #[doc = "Set the field `forward_ssh_connectivity`.\n"]
    pub fn set_forward_ssh_connectivity(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forward_ssh_connectivity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forward_ssh_connectivity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `private_connectivity`.\n"]
    pub fn set_private_connectivity(
        mut self,
        v: impl Into<
            BlockAssignable<DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.private_connectivity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.private_connectivity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ssl`.\n"]
    pub fn set_ssl(
        mut self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceConnectionProfileOracleElSslEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssl = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `static_service_ip_connectivity`.\n"]
    pub fn set_static_service_ip_connectivity(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.static_service_ip_connectivity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.static_service_ip_connectivity = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileOracleEl {
    type O = BlockAssignable<DatabaseMigrationServiceConnectionProfileOracleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileOracleEl {
    #[doc = "Required. Database service for the Oracle connection."]
    pub database_service: PrimField<String>,
    #[doc = "Required. The IP or hostname of the source Oracle database."]
    pub host: PrimField<String>,
    #[doc = "Required. Input only. The password for the user that Database Migration Service will be using to connect to the database.\nThis field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub password: PrimField<String>,
    #[doc = "Required. The network port of the source Oracle database."]
    pub port: PrimField<f64>,
    #[doc = "Required. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub username: PrimField<String>,
}
impl BuildDatabaseMigrationServiceConnectionProfileOracleEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfileOracleEl {
        DatabaseMigrationServiceConnectionProfileOracleEl {
            database_service: self.database_service,
            host: self.host,
            password: self.password,
            port: self.port,
            username: self.username,
            forward_ssh_connectivity: core::default::Default::default(),
            private_connectivity: core::default::Default::default(),
            ssl: core::default::Default::default(),
            static_service_ip_connectivity: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileOracleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileOracleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileOracleElRef {
        DatabaseMigrationServiceConnectionProfileOracleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileOracleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database_service` after provisioning.\nRequired. Database service for the Oracle connection."]
    pub fn database_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nRequired. The IP or hostname of the source Oracle database."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nRequired. Input only. The password for the user that Database Migration Service will be using to connect to the database.\nThis field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `password_set` after provisioning.\nOutput only. Indicates If this connection profile password is stored."]
    pub fn password_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_set", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nRequired. The network port of the source Oracle database."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nRequired. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
    #[doc = "Get a reference to the value of field `forward_ssh_connectivity` after provisioning.\n"]
    pub fn forward_ssh_connectivity(
        &self,
    ) -> ListRef<DatabaseMigrationServiceConnectionProfileOracleElForwardSshConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forward_ssh_connectivity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_connectivity` after provisioning.\n"]
    pub fn private_connectivity(
        &self,
    ) -> ListRef<DatabaseMigrationServiceConnectionProfileOracleElPrivateConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_connectivity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> ListRef<DatabaseMigrationServiceConnectionProfileOracleElSslElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssl", self.base))
    }
    #[doc = "Get a reference to the value of field `static_service_ip_connectivity` after provisioning.\n"]
    pub fn static_service_ip_connectivity(
        &self,
    ) -> ListRef<DatabaseMigrationServiceConnectionProfileOracleElStaticServiceIpConnectivityElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.static_service_ip_connectivity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityEl {
    private_connection: PrimField<String>,
}
impl DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityEl {}
impl ToListMappable for DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityEl {
    type O =
        BlockAssignable<DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityEl {
    #[doc = "Required. The resource name (URI) of the private connection."]
    pub private_connection: PrimField<String>,
}
impl BuildDatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityEl {
    pub fn build(
        self,
    ) -> DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityEl {
        DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityEl {
            private_connection: self.private_connection,
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityElRef {
        DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `private_connection` after provisioning.\nRequired. The resource name (URI) of the private connection."]
    pub fn private_connection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_connection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfilePostgresqlElSslEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DatabaseMigrationServiceConnectionProfilePostgresqlElSslEl {
    #[doc = "Set the field `ca_certificate`.\nInput only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate.\nThe replica will use this certificate to verify it's connecting to the right host."]
    pub fn set_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate`.\nInput only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.\nIf this field is used then the 'clientKey' field is mandatory"]
    pub fn set_client_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_key`.\nInput only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'clientCertificate' field is mandatory."]
    pub fn set_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_key = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe current connection profile state. Possible values: [\"SERVER_ONLY\", \"SERVER_CLIENT\", \"REQUIRED\", \"NONE\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfilePostgresqlElSslEl {
    type O = BlockAssignable<DatabaseMigrationServiceConnectionProfilePostgresqlElSslEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfilePostgresqlElSslEl {}
impl BuildDatabaseMigrationServiceConnectionProfilePostgresqlElSslEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfilePostgresqlElSslEl {
        DatabaseMigrationServiceConnectionProfilePostgresqlElSslEl {
            ca_certificate: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfilePostgresqlElSslElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfilePostgresqlElSslElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfilePostgresqlElSslElRef {
        DatabaseMigrationServiceConnectionProfilePostgresqlElSslElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfilePostgresqlElSslElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\nInput only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate.\nThe replica will use this certificate to verify it's connecting to the right host."]
    pub fn ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\nInput only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.\nIf this field is used then the 'clientKey' field is mandatory"]
    pub fn client_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_key` after provisioning.\nInput only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'clientCertificate' field is mandatory."]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe current connection profile state. Possible values: [\"SERVER_ONLY\", \"SERVER_CLIENT\", \"REQUIRED\", \"NONE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfilePostgresqlElDynamic {
    private_connectivity: Option<
        DynamicBlock<DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityEl>,
    >,
    ssl: Option<DynamicBlock<DatabaseMigrationServiceConnectionProfilePostgresqlElSslEl>>,
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfilePostgresqlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alloydb_cluster_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_connectivity:
        Option<Vec<DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl: Option<Vec<DatabaseMigrationServiceConnectionProfilePostgresqlElSslEl>>,
    dynamic: DatabaseMigrationServiceConnectionProfilePostgresqlElDynamic,
}
impl DatabaseMigrationServiceConnectionProfilePostgresqlEl {
    #[doc = "Set the field `alloydb_cluster_id`.\nIf the connection profile is an AlloyDB instance, use this field to provide the AlloyDB cluster ID."]
    pub fn set_alloydb_cluster_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.alloydb_cluster_id = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_sql_id`.\nIf the connection profile is a Cloud SQL database, use this field to provide the Cloud SQL instance ID."]
    pub fn set_cloud_sql_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_sql_id = Some(v.into());
        self
    }
    #[doc = "Set the field `database`.\nThe name of the specific database within the host."]
    pub fn set_database(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database = Some(v.into());
        self
    }
    #[doc = "Set the field `host`.\nThe IP or hostname of the source MySQL database."]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `password`.\nInput only. The password for the user that Database Migration Service will be using to connect to the database.\nThis field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe network port of the source MySQL database."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\nThe username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
    #[doc = "Set the field `private_connectivity`.\n"]
    pub fn set_private_connectivity(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.private_connectivity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.private_connectivity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ssl`.\n"]
    pub fn set_ssl(
        mut self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceConnectionProfilePostgresqlElSslEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssl = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfilePostgresqlEl {
    type O = BlockAssignable<DatabaseMigrationServiceConnectionProfilePostgresqlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfilePostgresqlEl {}
impl BuildDatabaseMigrationServiceConnectionProfilePostgresqlEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfilePostgresqlEl {
        DatabaseMigrationServiceConnectionProfilePostgresqlEl {
            alloydb_cluster_id: core::default::Default::default(),
            cloud_sql_id: core::default::Default::default(),
            database: core::default::Default::default(),
            host: core::default::Default::default(),
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            username: core::default::Default::default(),
            private_connectivity: core::default::Default::default(),
            ssl: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfilePostgresqlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfilePostgresqlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfilePostgresqlElRef {
        DatabaseMigrationServiceConnectionProfilePostgresqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfilePostgresqlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alloydb_cluster_id` after provisioning.\nIf the connection profile is an AlloyDB instance, use this field to provide the AlloyDB cluster ID."]
    pub fn alloydb_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alloydb_cluster_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_sql_id` after provisioning.\nIf the connection profile is a Cloud SQL database, use this field to provide the Cloud SQL instance ID."]
    pub fn cloud_sql_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_id", self.base))
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe name of the specific database within the host."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe IP or hostname of the source MySQL database."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `network_architecture` after provisioning.\nOutput only. If the source is a Cloud SQL database, this field indicates the network architecture it's associated with."]
    pub fn network_architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_architecture", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nInput only. The password for the user that Database Migration Service will be using to connect to the database.\nThis field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `password_set` after provisioning.\nOutput only. Indicates If this connection profile password is stored."]
    pub fn password_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_set", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe network port of the source MySQL database."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
    #[doc = "Get a reference to the value of field `private_connectivity` after provisioning.\n"]
    pub fn private_connectivity(
        &self,
    ) -> ListRef<DatabaseMigrationServiceConnectionProfilePostgresqlElPrivateConnectivityElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_connectivity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> ListRef<DatabaseMigrationServiceConnectionProfilePostgresqlElSslElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssl", self.base))
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfileTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DatabaseMigrationServiceConnectionProfileTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceConnectionProfileTimeoutsEl {
    type O = BlockAssignable<DatabaseMigrationServiceConnectionProfileTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceConnectionProfileTimeoutsEl {}
impl BuildDatabaseMigrationServiceConnectionProfileTimeoutsEl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfileTimeoutsEl {
        DatabaseMigrationServiceConnectionProfileTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceConnectionProfileTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceConnectionProfileTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceConnectionProfileTimeoutsElRef {
        DatabaseMigrationServiceConnectionProfileTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceConnectionProfileTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfileDynamic {
    alloydb: Option<DynamicBlock<DatabaseMigrationServiceConnectionProfileAlloydbEl>>,
    cloudsql: Option<DynamicBlock<DatabaseMigrationServiceConnectionProfileCloudsqlEl>>,
    mysql: Option<DynamicBlock<DatabaseMigrationServiceConnectionProfileMysqlEl>>,
    oracle: Option<DynamicBlock<DatabaseMigrationServiceConnectionProfileOracleEl>>,
    postgresql: Option<DynamicBlock<DatabaseMigrationServiceConnectionProfilePostgresqlEl>>,
}
