use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DatastreamConnectionProfileData {
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
    create_without_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_profile: Option<Vec<DatastreamConnectionProfileBigqueryProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_ssh_connectivity: Option<Vec<DatastreamConnectionProfileForwardSshConnectivityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_profile: Option<Vec<DatastreamConnectionProfileGcsProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mongodb_profile: Option<Vec<DatastreamConnectionProfileMongodbProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mysql_profile: Option<Vec<DatastreamConnectionProfileMysqlProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle_profile: Option<Vec<DatastreamConnectionProfileOracleProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postgresql_profile: Option<Vec<DatastreamConnectionProfilePostgresqlProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_connectivity: Option<Vec<DatastreamConnectionProfilePrivateConnectivityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_server_profile: Option<Vec<DatastreamConnectionProfileSqlServerProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DatastreamConnectionProfileTimeoutsEl>,
    dynamic: DatastreamConnectionProfileDynamic,
}
struct DatastreamConnectionProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatastreamConnectionProfileData>,
}
#[derive(Clone)]
pub struct DatastreamConnectionProfile(Rc<DatastreamConnectionProfile_>);
impl DatastreamConnectionProfile {
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
    #[doc = "Set the field `create_without_validation`.\nCreate the connection profile without validating it."]
    pub fn set_create_without_validation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_without_validation = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `bigquery_profile`.\n"]
    pub fn set_bigquery_profile(
        self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfileBigqueryProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bigquery_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bigquery_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `forward_ssh_connectivity`.\n"]
    pub fn set_forward_ssh_connectivity(
        self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfileForwardSshConnectivityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().forward_ssh_connectivity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.forward_ssh_connectivity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcs_profile`.\n"]
    pub fn set_gcs_profile(
        self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfileGcsProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gcs_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gcs_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mongodb_profile`.\n"]
    pub fn set_mongodb_profile(
        self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfileMongodbProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mongodb_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mongodb_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mysql_profile`.\n"]
    pub fn set_mysql_profile(
        self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfileMysqlProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mysql_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mysql_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oracle_profile`.\n"]
    pub fn set_oracle_profile(
        self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfileOracleProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oracle_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oracle_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `postgresql_profile`.\n"]
    pub fn set_postgresql_profile(
        self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfilePostgresqlProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().postgresql_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.postgresql_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `private_connectivity`.\n"]
    pub fn set_private_connectivity(
        self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfilePrivateConnectivityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_connectivity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_connectivity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sql_server_profile`.\n"]
    pub fn set_sql_server_profile(
        self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfileSqlServerProfileEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sql_server_profile = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sql_server_profile = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DatastreamConnectionProfileTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `connection_profile_id` after provisioning.\nThe connection profile identifier."]
    pub fn connection_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_profile_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_without_validation` after provisioning.\nCreate the connection profile without validating it."]
    pub fn create_without_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_without_validation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location this connection profile is located in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource's name."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_profile` after provisioning.\n"]
    pub fn bigquery_profile(&self) -> ListRef<DatastreamConnectionProfileBigqueryProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `forward_ssh_connectivity` after provisioning.\n"]
    pub fn forward_ssh_connectivity(
        &self,
    ) -> ListRef<DatastreamConnectionProfileForwardSshConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forward_ssh_connectivity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_profile` after provisioning.\n"]
    pub fn gcs_profile(&self) -> ListRef<DatastreamConnectionProfileGcsProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mongodb_profile` after provisioning.\n"]
    pub fn mongodb_profile(&self) -> ListRef<DatastreamConnectionProfileMongodbProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mongodb_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mysql_profile` after provisioning.\n"]
    pub fn mysql_profile(&self) -> ListRef<DatastreamConnectionProfileMysqlProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oracle_profile` after provisioning.\n"]
    pub fn oracle_profile(&self) -> ListRef<DatastreamConnectionProfileOracleProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `postgresql_profile` after provisioning.\n"]
    pub fn postgresql_profile(&self) -> ListRef<DatastreamConnectionProfilePostgresqlProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_connectivity` after provisioning.\n"]
    pub fn private_connectivity(
        &self,
    ) -> ListRef<DatastreamConnectionProfilePrivateConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_connectivity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sql_server_profile` after provisioning.\n"]
    pub fn sql_server_profile(&self) -> ListRef<DatastreamConnectionProfileSqlServerProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sql_server_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatastreamConnectionProfileTimeoutsElRef {
        DatastreamConnectionProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DatastreamConnectionProfile {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DatastreamConnectionProfile {}
impl ToListMappable for DatastreamConnectionProfile {
    type O = ListRef<DatastreamConnectionProfileRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DatastreamConnectionProfile_ {
    fn extract_resource_type(&self) -> String {
        "google_datastream_connection_profile".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDatastreamConnectionProfile {
    pub tf_id: String,
    #[doc = "The connection profile identifier."]
    pub connection_profile_id: PrimField<String>,
    #[doc = "Display name."]
    pub display_name: PrimField<String>,
    #[doc = "The name of the location this connection profile is located in."]
    pub location: PrimField<String>,
}
impl BuildDatastreamConnectionProfile {
    pub fn build(self, stack: &mut Stack) -> DatastreamConnectionProfile {
        let out = DatastreamConnectionProfile(Rc::new(DatastreamConnectionProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatastreamConnectionProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                connection_profile_id: self.connection_profile_id,
                create_without_validation: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                bigquery_profile: core::default::Default::default(),
                forward_ssh_connectivity: core::default::Default::default(),
                gcs_profile: core::default::Default::default(),
                mongodb_profile: core::default::Default::default(),
                mysql_profile: core::default::Default::default(),
                oracle_profile: core::default::Default::default(),
                postgresql_profile: core::default::Default::default(),
                private_connectivity: core::default::Default::default(),
                sql_server_profile: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DatastreamConnectionProfileRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DatastreamConnectionProfileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_profile_id` after provisioning.\nThe connection profile identifier."]
    pub fn connection_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_profile_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_without_validation` after provisioning.\nCreate the connection profile without validating it."]
    pub fn create_without_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_without_validation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location this connection profile is located in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource's name."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_profile` after provisioning.\n"]
    pub fn bigquery_profile(&self) -> ListRef<DatastreamConnectionProfileBigqueryProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `forward_ssh_connectivity` after provisioning.\n"]
    pub fn forward_ssh_connectivity(
        &self,
    ) -> ListRef<DatastreamConnectionProfileForwardSshConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.forward_ssh_connectivity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_profile` after provisioning.\n"]
    pub fn gcs_profile(&self) -> ListRef<DatastreamConnectionProfileGcsProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mongodb_profile` after provisioning.\n"]
    pub fn mongodb_profile(&self) -> ListRef<DatastreamConnectionProfileMongodbProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mongodb_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mysql_profile` after provisioning.\n"]
    pub fn mysql_profile(&self) -> ListRef<DatastreamConnectionProfileMysqlProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mysql_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oracle_profile` after provisioning.\n"]
    pub fn oracle_profile(&self) -> ListRef<DatastreamConnectionProfileOracleProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `postgresql_profile` after provisioning.\n"]
    pub fn postgresql_profile(&self) -> ListRef<DatastreamConnectionProfilePostgresqlProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgresql_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_connectivity` after provisioning.\n"]
    pub fn private_connectivity(
        &self,
    ) -> ListRef<DatastreamConnectionProfilePrivateConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_connectivity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sql_server_profile` after provisioning.\n"]
    pub fn sql_server_profile(&self) -> ListRef<DatastreamConnectionProfileSqlServerProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sql_server_profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatastreamConnectionProfileTimeoutsElRef {
        DatastreamConnectionProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileBigqueryProfileEl {}
impl DatastreamConnectionProfileBigqueryProfileEl {}
impl ToListMappable for DatastreamConnectionProfileBigqueryProfileEl {
    type O = BlockAssignable<DatastreamConnectionProfileBigqueryProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileBigqueryProfileEl {}
impl BuildDatastreamConnectionProfileBigqueryProfileEl {
    pub fn build(self) -> DatastreamConnectionProfileBigqueryProfileEl {
        DatastreamConnectionProfileBigqueryProfileEl {}
    }
}
pub struct DatastreamConnectionProfileBigqueryProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileBigqueryProfileElRef {
    fn new(shared: StackShared, base: String) -> DatastreamConnectionProfileBigqueryProfileElRef {
        DatastreamConnectionProfileBigqueryProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileBigqueryProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileForwardSshConnectivityEl {
    hostname: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<PrimField<String>>,
    username: PrimField<String>,
}
impl DatastreamConnectionProfileForwardSshConnectivityEl {
    #[doc = "Set the field `password`.\nSSH password."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nPort for the SSH tunnel."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `private_key`.\nSSH private key."]
    pub fn set_private_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_key = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamConnectionProfileForwardSshConnectivityEl {
    type O = BlockAssignable<DatastreamConnectionProfileForwardSshConnectivityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileForwardSshConnectivityEl {
    #[doc = "Hostname for the SSH tunnel."]
    pub hostname: PrimField<String>,
    #[doc = "Username for the SSH tunnel."]
    pub username: PrimField<String>,
}
impl BuildDatastreamConnectionProfileForwardSshConnectivityEl {
    pub fn build(self) -> DatastreamConnectionProfileForwardSshConnectivityEl {
        DatastreamConnectionProfileForwardSshConnectivityEl {
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            private_key: core::default::Default::default(),
            username: self.username,
        }
    }
}
pub struct DatastreamConnectionProfileForwardSshConnectivityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileForwardSshConnectivityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamConnectionProfileForwardSshConnectivityElRef {
        DatastreamConnectionProfileForwardSshConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileForwardSshConnectivityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname for the SSH tunnel."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nSSH password."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the SSH tunnel."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `private_key` after provisioning.\nSSH private key."]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the SSH tunnel."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileGcsProfileEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_path: Option<PrimField<String>>,
}
impl DatastreamConnectionProfileGcsProfileEl {
    #[doc = "Set the field `root_path`.\nThe root path inside the Cloud Storage bucket."]
    pub fn set_root_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.root_path = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamConnectionProfileGcsProfileEl {
    type O = BlockAssignable<DatastreamConnectionProfileGcsProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileGcsProfileEl {
    #[doc = "The Cloud Storage bucket name."]
    pub bucket: PrimField<String>,
}
impl BuildDatastreamConnectionProfileGcsProfileEl {
    pub fn build(self) -> DatastreamConnectionProfileGcsProfileEl {
        DatastreamConnectionProfileGcsProfileEl {
            bucket: self.bucket,
            root_path: core::default::Default::default(),
        }
    }
}
pub struct DatastreamConnectionProfileGcsProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileGcsProfileElRef {
    fn new(shared: StackShared, base: String) -> DatastreamConnectionProfileGcsProfileElRef {
        DatastreamConnectionProfileGcsProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileGcsProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe Cloud Storage bucket name."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `root_path` after provisioning.\nThe root path inside the Cloud Storage bucket."]
    pub fn root_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_path", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileMongodbProfileElHostAddressesEl {
    hostname: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DatastreamConnectionProfileMongodbProfileElHostAddressesEl {
    #[doc = "Set the field `port`.\nPort for the connection."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamConnectionProfileMongodbProfileElHostAddressesEl {
    type O = BlockAssignable<DatastreamConnectionProfileMongodbProfileElHostAddressesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileMongodbProfileElHostAddressesEl {
    #[doc = "Hostname for the connection."]
    pub hostname: PrimField<String>,
}
impl BuildDatastreamConnectionProfileMongodbProfileElHostAddressesEl {
    pub fn build(self) -> DatastreamConnectionProfileMongodbProfileElHostAddressesEl {
        DatastreamConnectionProfileMongodbProfileElHostAddressesEl {
            hostname: self.hostname,
            port: core::default::Default::default(),
        }
    }
}
pub struct DatastreamConnectionProfileMongodbProfileElHostAddressesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileMongodbProfileElHostAddressesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamConnectionProfileMongodbProfileElHostAddressesElRef {
        DatastreamConnectionProfileMongodbProfileElHostAddressesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileMongodbProfileElHostAddressesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname for the connection."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the connection."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatEl {}
impl DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatEl {}
impl ToListMappable for DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatEl {
    type O = BlockAssignable<DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileMongodbProfileElSrvConnectionFormatEl {}
impl BuildDatastreamConnectionProfileMongodbProfileElSrvConnectionFormatEl {
    pub fn build(self) -> DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatEl {
        DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatEl {}
    }
}
pub struct DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatElRef {
        DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileMongodbProfileElSslConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager_stored_client_key: Option<PrimField<String>>,
}
impl DatastreamConnectionProfileMongodbProfileElSslConfigEl {
    #[doc = "Set the field `ca_certificate`.\nPEM-encoded certificate of the CA that signed the source database\nserver's certificate."]
    pub fn set_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate`.\nPEM-encoded certificate that will be used by the replica to\nauthenticate against the source database server. If this field\nis used then the 'clientKey' and the 'caCertificate' fields are\nmandatory."]
    pub fn set_client_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_key`.\nPEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'client_certificate' and the\n'ca_certificate' fields are mandatory."]
    pub fn set_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_key = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_manager_stored_client_key`.\nA reference to a Secret Manager resource name storing the\nPEM-encoded private key. Mutually exclusive with clientKey."]
    pub fn set_secret_manager_stored_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_manager_stored_client_key = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamConnectionProfileMongodbProfileElSslConfigEl {
    type O = BlockAssignable<DatastreamConnectionProfileMongodbProfileElSslConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileMongodbProfileElSslConfigEl {}
impl BuildDatastreamConnectionProfileMongodbProfileElSslConfigEl {
    pub fn build(self) -> DatastreamConnectionProfileMongodbProfileElSslConfigEl {
        DatastreamConnectionProfileMongodbProfileElSslConfigEl {
            ca_certificate: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
            secret_manager_stored_client_key: core::default::Default::default(),
        }
    }
}
pub struct DatastreamConnectionProfileMongodbProfileElSslConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileMongodbProfileElSslConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamConnectionProfileMongodbProfileElSslConfigElRef {
        DatastreamConnectionProfileMongodbProfileElSslConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileMongodbProfileElSslConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\nPEM-encoded certificate of the CA that signed the source database\nserver's certificate."]
    pub fn ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ca_certificate_set` after provisioning.\nIndicates whether the clientKey field is set."]
    pub fn ca_certificate_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\nPEM-encoded certificate that will be used by the replica to\nauthenticate against the source database server. If this field\nis used then the 'clientKey' and the 'caCertificate' fields are\nmandatory."]
    pub fn client_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate_set` after provisioning.\nIndicates whether the clientCertificate field is set."]
    pub fn client_certificate_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_key` after provisioning.\nPEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'client_certificate' and the\n'ca_certificate' fields are mandatory."]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }
    #[doc = "Get a reference to the value of field `client_key_set` after provisioning.\nIndicates whether the clientKey field is set."]
    pub fn client_key_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_key_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_manager_stored_client_key` after provisioning.\nA reference to a Secret Manager resource name storing the\nPEM-encoded private key. Mutually exclusive with clientKey."]
    pub fn secret_manager_stored_client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_manager_stored_client_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_connection: Option<PrimField<bool>>,
}
impl DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatEl {
    #[doc = "Set the field `direct_connection`.\nSpecifies whether the client connects directly to the\nhost[:port] in the connection URI."]
    pub fn set_direct_connection(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.direct_connection = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatEl {
    type O = BlockAssignable<DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileMongodbProfileElStandardConnectionFormatEl {}
impl BuildDatastreamConnectionProfileMongodbProfileElStandardConnectionFormatEl {
    pub fn build(self) -> DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatEl {
        DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatEl {
            direct_connection: core::default::Default::default(),
        }
    }
}
pub struct DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatElRef {
        DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `direct_connection` after provisioning.\nSpecifies whether the client connects directly to the\nhost[:port] in the connection URI."]
    pub fn direct_connection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.direct_connection", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamConnectionProfileMongodbProfileElDynamic {
    host_addresses:
        Option<DynamicBlock<DatastreamConnectionProfileMongodbProfileElHostAddressesEl>>,
    srv_connection_format:
        Option<DynamicBlock<DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatEl>>,
    ssl_config: Option<DynamicBlock<DatastreamConnectionProfileMongodbProfileElSslConfigEl>>,
    standard_connection_format:
        Option<DynamicBlock<DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatEl>>,
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileMongodbProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_set: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager_stored_password: Option<PrimField<String>>,
    username: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_addresses: Option<Vec<DatastreamConnectionProfileMongodbProfileElHostAddressesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    srv_connection_format:
        Option<Vec<DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_config: Option<Vec<DatastreamConnectionProfileMongodbProfileElSslConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard_connection_format:
        Option<Vec<DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatEl>>,
    dynamic: DatastreamConnectionProfileMongodbProfileElDynamic,
}
impl DatastreamConnectionProfileMongodbProfileEl {
    #[doc = "Set the field `password`.\nPassword for the MongoDB connection. Mutually exclusive with\nsecretManagerStoredPassword."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_set`.\nName of the replica set."]
    pub fn set_replica_set(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replica_set = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_manager_stored_password`.\nA reference to a Secret Manager resource name storing the MongoDB\nconnection password. Mutually exclusive with password."]
    pub fn set_secret_manager_stored_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_manager_stored_password = Some(v.into());
        self
    }
    #[doc = "Set the field `host_addresses`.\n"]
    pub fn set_host_addresses(
        mut self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfileMongodbProfileElHostAddressesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.host_addresses = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.host_addresses = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `srv_connection_format`.\n"]
    pub fn set_srv_connection_format(
        mut self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.srv_connection_format = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.srv_connection_format = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        mut self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfileMongodbProfileElSslConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssl_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `standard_connection_format`.\n"]
    pub fn set_standard_connection_format(
        mut self,
        v: impl Into<
            BlockAssignable<DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.standard_connection_format = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.standard_connection_format = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamConnectionProfileMongodbProfileEl {
    type O = BlockAssignable<DatastreamConnectionProfileMongodbProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileMongodbProfileEl {
    #[doc = "Username for the MongoDB connection."]
    pub username: PrimField<String>,
}
impl BuildDatastreamConnectionProfileMongodbProfileEl {
    pub fn build(self) -> DatastreamConnectionProfileMongodbProfileEl {
        DatastreamConnectionProfileMongodbProfileEl {
            password: core::default::Default::default(),
            replica_set: core::default::Default::default(),
            secret_manager_stored_password: core::default::Default::default(),
            username: self.username,
            host_addresses: core::default::Default::default(),
            srv_connection_format: core::default::Default::default(),
            ssl_config: core::default::Default::default(),
            standard_connection_format: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamConnectionProfileMongodbProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileMongodbProfileElRef {
    fn new(shared: StackShared, base: String) -> DatastreamConnectionProfileMongodbProfileElRef {
        DatastreamConnectionProfileMongodbProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileMongodbProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for the MongoDB connection. Mutually exclusive with\nsecretManagerStoredPassword."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `replica_set` after provisioning.\nName of the replica set."]
    pub fn replica_set(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replica_set", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_manager_stored_password` after provisioning.\nA reference to a Secret Manager resource name storing the MongoDB\nconnection password. Mutually exclusive with password."]
    pub fn secret_manager_stored_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_manager_stored_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the MongoDB connection."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
    #[doc = "Get a reference to the value of field `host_addresses` after provisioning.\n"]
    pub fn host_addresses(
        &self,
    ) -> ListRef<DatastreamConnectionProfileMongodbProfileElHostAddressesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.host_addresses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `srv_connection_format` after provisioning.\n"]
    pub fn srv_connection_format(
        &self,
    ) -> ListRef<DatastreamConnectionProfileMongodbProfileElSrvConnectionFormatElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.srv_connection_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> ListRef<DatastreamConnectionProfileMongodbProfileElSslConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.base))
    }
    #[doc = "Get a reference to the value of field `standard_connection_format` after provisioning.\n"]
    pub fn standard_connection_format(
        &self,
    ) -> ListRef<DatastreamConnectionProfileMongodbProfileElStandardConnectionFormatElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.standard_connection_format", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileMysqlProfileElSslConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
}
impl DatastreamConnectionProfileMysqlProfileElSslConfigEl {
    #[doc = "Set the field `ca_certificate`.\nPEM-encoded certificate of the CA that signed the source database\nserver's certificate."]
    pub fn set_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate`.\nPEM-encoded certificate that will be used by the replica to\nauthenticate against the source database server. If this field\nis used then the 'clientKey' and the 'caCertificate' fields are\nmandatory."]
    pub fn set_client_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_key`.\nPEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'client_certificate' and the\n'ca_certificate' fields are mandatory."]
    pub fn set_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_key = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamConnectionProfileMysqlProfileElSslConfigEl {
    type O = BlockAssignable<DatastreamConnectionProfileMysqlProfileElSslConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileMysqlProfileElSslConfigEl {}
impl BuildDatastreamConnectionProfileMysqlProfileElSslConfigEl {
    pub fn build(self) -> DatastreamConnectionProfileMysqlProfileElSslConfigEl {
        DatastreamConnectionProfileMysqlProfileElSslConfigEl {
            ca_certificate: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
        }
    }
}
pub struct DatastreamConnectionProfileMysqlProfileElSslConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileMysqlProfileElSslConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamConnectionProfileMysqlProfileElSslConfigElRef {
        DatastreamConnectionProfileMysqlProfileElSslConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileMysqlProfileElSslConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\nPEM-encoded certificate of the CA that signed the source database\nserver's certificate."]
    pub fn ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ca_certificate_set` after provisioning.\nIndicates whether the clientKey field is set."]
    pub fn ca_certificate_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\nPEM-encoded certificate that will be used by the replica to\nauthenticate against the source database server. If this field\nis used then the 'clientKey' and the 'caCertificate' fields are\nmandatory."]
    pub fn client_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate_set` after provisioning.\nIndicates whether the clientCertificate field is set."]
    pub fn client_certificate_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_key` after provisioning.\nPEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'client_certificate' and the\n'ca_certificate' fields are mandatory."]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }
    #[doc = "Get a reference to the value of field `client_key_set` after provisioning.\nIndicates whether the clientKey field is set."]
    pub fn client_key_set(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_key_set", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamConnectionProfileMysqlProfileElDynamic {
    ssl_config: Option<DynamicBlock<DatastreamConnectionProfileMysqlProfileElSslConfigEl>>,
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileMysqlProfileEl {
    hostname: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager_stored_password: Option<PrimField<String>>,
    username: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_config: Option<Vec<DatastreamConnectionProfileMysqlProfileElSslConfigEl>>,
    dynamic: DatastreamConnectionProfileMysqlProfileElDynamic,
}
impl DatastreamConnectionProfileMysqlProfileEl {
    #[doc = "Set the field `password`.\nPassword for the MySQL connection."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nPort for the MySQL connection."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_manager_stored_password`.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn set_secret_manager_stored_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_manager_stored_password = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        mut self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfileMysqlProfileElSslConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssl_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamConnectionProfileMysqlProfileEl {
    type O = BlockAssignable<DatastreamConnectionProfileMysqlProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileMysqlProfileEl {
    #[doc = "Hostname for the MySQL connection."]
    pub hostname: PrimField<String>,
    #[doc = "Username for the MySQL connection."]
    pub username: PrimField<String>,
}
impl BuildDatastreamConnectionProfileMysqlProfileEl {
    pub fn build(self) -> DatastreamConnectionProfileMysqlProfileEl {
        DatastreamConnectionProfileMysqlProfileEl {
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            secret_manager_stored_password: core::default::Default::default(),
            username: self.username,
            ssl_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamConnectionProfileMysqlProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileMysqlProfileElRef {
    fn new(shared: StackShared, base: String) -> DatastreamConnectionProfileMysqlProfileElRef {
        DatastreamConnectionProfileMysqlProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileMysqlProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname for the MySQL connection."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for the MySQL connection."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the MySQL connection."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_manager_stored_password` after provisioning.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn secret_manager_stored_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_manager_stored_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the MySQL connection."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> ListRef<DatastreamConnectionProfileMysqlProfileElSslConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileOracleProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_attributes: Option<RecField<PrimField<String>>>,
    database_service: PrimField<String>,
    hostname: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager_stored_password: Option<PrimField<String>>,
    username: PrimField<String>,
}
impl DatastreamConnectionProfileOracleProfileEl {
    #[doc = "Set the field `connection_attributes`.\nConnection string attributes"]
    pub fn set_connection_attributes(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.connection_attributes = Some(v.into());
        self
    }
    #[doc = "Set the field `password`.\nPassword for the Oracle connection."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nPort for the Oracle connection."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_manager_stored_password`.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn set_secret_manager_stored_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_manager_stored_password = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamConnectionProfileOracleProfileEl {
    type O = BlockAssignable<DatastreamConnectionProfileOracleProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileOracleProfileEl {
    #[doc = "Database for the Oracle connection."]
    pub database_service: PrimField<String>,
    #[doc = "Hostname for the Oracle connection."]
    pub hostname: PrimField<String>,
    #[doc = "Username for the Oracle connection."]
    pub username: PrimField<String>,
}
impl BuildDatastreamConnectionProfileOracleProfileEl {
    pub fn build(self) -> DatastreamConnectionProfileOracleProfileEl {
        DatastreamConnectionProfileOracleProfileEl {
            connection_attributes: core::default::Default::default(),
            database_service: self.database_service,
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            secret_manager_stored_password: core::default::Default::default(),
            username: self.username,
        }
    }
}
pub struct DatastreamConnectionProfileOracleProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileOracleProfileElRef {
    fn new(shared: StackShared, base: String) -> DatastreamConnectionProfileOracleProfileElRef {
        DatastreamConnectionProfileOracleProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileOracleProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_attributes` after provisioning.\nConnection string attributes"]
    pub fn connection_attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.connection_attributes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database_service` after provisioning.\nDatabase for the Oracle connection."]
    pub fn database_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname for the Oracle connection."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for the Oracle connection."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the Oracle connection."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_manager_stored_password` after provisioning.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn secret_manager_stored_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_manager_stored_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the Oracle connection."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationEl {
    ca_certificate: PrimField<String>,
    client_certificate: PrimField<String>,
    client_key: PrimField<String>,
}
impl DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationEl {}
impl ToListMappable
    for DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationEl
{
    type O = BlockAssignable<
        DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationEl
{
    #[doc = "PEM-encoded server root CA certificate."]
    pub ca_certificate: PrimField<String>,
    #[doc = "PEM-encoded certificate used by the source database to authenticate the\nclient identity (i.e., the Datastream's identity). This certificate is\nsigned by either a root certificate trusted by the server or one or more\nintermediate certificates (which is stored with the leaf certificate) to\nlink to this certificate to the trusted root certificate."]
    pub client_certificate: PrimField<String>,
    #[doc = "PEM-encoded private key associated with the client certificate.\nThis value will be used during the SSL/TLS handshake, allowing\nthe PostgreSQL server to authenticate the client's identity,\ni.e. identity of the stream."]
    pub client_key: PrimField<String>,
}
impl BuildDatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationEl {
    pub fn build(
        self,
    ) -> DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationEl
    {
        DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationEl {
            ca_certificate: self.ca_certificate,
            client_certificate: self.client_certificate,
            client_key: self.client_key,
        }
    }
}
pub struct DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationElRef
    {
        DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\nPEM-encoded server root CA certificate."]
    pub fn ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\nPEM-encoded certificate used by the source database to authenticate the\nclient identity (i.e., the Datastream's identity). This certificate is\nsigned by either a root certificate trusted by the server or one or more\nintermediate certificates (which is stored with the leaf certificate) to\nlink to this certificate to the trusted root certificate."]
    pub fn client_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_key` after provisioning.\nPEM-encoded private key associated with the client certificate.\nThis value will be used during the SSL/TLS handshake, allowing\nthe PostgreSQL server to authenticate the client's identity,\ni.e. identity of the stream."]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationEl {
    ca_certificate: PrimField<String>,
}
impl DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationEl {}
impl ToListMappable
    for DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationEl
{
    type O = BlockAssignable<
        DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationEl {
    #[doc = "PEM-encoded server root CA certificate."]
    pub ca_certificate: PrimField<String>,
}
impl BuildDatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationEl {
    pub fn build(
        self,
    ) -> DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationEl {
        DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationEl {
            ca_certificate: self.ca_certificate,
        }
    }
}
pub struct DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationElRef {
        DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certificate` after provisioning.\nPEM-encoded server root CA certificate."]
    pub fn ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ca_certificate", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamConnectionProfilePostgresqlProfileElSslConfigElDynamic {
    server_and_client_verification: Option<
        DynamicBlock<
            DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationEl,
        >,
    >,
    server_verification: Option<
        DynamicBlock<DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationEl>,
    >,
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfilePostgresqlProfileElSslConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    server_and_client_verification: Option<
        Vec<DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_verification:
        Option<Vec<DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationEl>>,
    dynamic: DatastreamConnectionProfilePostgresqlProfileElSslConfigElDynamic,
}
impl DatastreamConnectionProfilePostgresqlProfileElSslConfigEl {
    #[doc = "Set the field `server_and_client_verification`.\n"]
    pub fn set_server_and_client_verification(
        mut self,
        v : impl Into < BlockAssignable < DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.server_and_client_verification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.server_and_client_verification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `server_verification`.\n"]
    pub fn set_server_verification(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.server_verification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.server_verification = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamConnectionProfilePostgresqlProfileElSslConfigEl {
    type O = BlockAssignable<DatastreamConnectionProfilePostgresqlProfileElSslConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfilePostgresqlProfileElSslConfigEl {}
impl BuildDatastreamConnectionProfilePostgresqlProfileElSslConfigEl {
    pub fn build(self) -> DatastreamConnectionProfilePostgresqlProfileElSslConfigEl {
        DatastreamConnectionProfilePostgresqlProfileElSslConfigEl {
            server_and_client_verification: core::default::Default::default(),
            server_verification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamConnectionProfilePostgresqlProfileElSslConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfilePostgresqlProfileElSslConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamConnectionProfilePostgresqlProfileElSslConfigElRef {
        DatastreamConnectionProfilePostgresqlProfileElSslConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfilePostgresqlProfileElSslConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `server_and_client_verification` after provisioning.\n"]
    pub fn server_and_client_verification(
        &self,
    ) -> ListRef<
        DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerAndClientVerificationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_and_client_verification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `server_verification` after provisioning.\n"]
    pub fn server_verification(
        &self,
    ) -> ListRef<DatastreamConnectionProfilePostgresqlProfileElSslConfigElServerVerificationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_verification", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatastreamConnectionProfilePostgresqlProfileElDynamic {
    ssl_config: Option<DynamicBlock<DatastreamConnectionProfilePostgresqlProfileElSslConfigEl>>,
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfilePostgresqlProfileEl {
    database: PrimField<String>,
    hostname: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager_stored_password: Option<PrimField<String>>,
    username: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_config: Option<Vec<DatastreamConnectionProfilePostgresqlProfileElSslConfigEl>>,
    dynamic: DatastreamConnectionProfilePostgresqlProfileElDynamic,
}
impl DatastreamConnectionProfilePostgresqlProfileEl {
    #[doc = "Set the field `password`.\nPassword for the PostgreSQL connection."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nPort for the PostgreSQL connection."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_manager_stored_password`.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn set_secret_manager_stored_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_manager_stored_password = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        mut self,
        v: impl Into<BlockAssignable<DatastreamConnectionProfilePostgresqlProfileElSslConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssl_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatastreamConnectionProfilePostgresqlProfileEl {
    type O = BlockAssignable<DatastreamConnectionProfilePostgresqlProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfilePostgresqlProfileEl {
    #[doc = "Database for the PostgreSQL connection."]
    pub database: PrimField<String>,
    #[doc = "Hostname for the PostgreSQL connection."]
    pub hostname: PrimField<String>,
    #[doc = "Username for the PostgreSQL connection."]
    pub username: PrimField<String>,
}
impl BuildDatastreamConnectionProfilePostgresqlProfileEl {
    pub fn build(self) -> DatastreamConnectionProfilePostgresqlProfileEl {
        DatastreamConnectionProfilePostgresqlProfileEl {
            database: self.database,
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            secret_manager_stored_password: core::default::Default::default(),
            username: self.username,
            ssl_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatastreamConnectionProfilePostgresqlProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfilePostgresqlProfileElRef {
    fn new(shared: StackShared, base: String) -> DatastreamConnectionProfilePostgresqlProfileElRef {
        DatastreamConnectionProfilePostgresqlProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfilePostgresqlProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase for the PostgreSQL connection."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname for the PostgreSQL connection."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for the PostgreSQL connection."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the PostgreSQL connection."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_manager_stored_password` after provisioning.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn secret_manager_stored_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_manager_stored_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the PostgreSQL connection."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(
        &self,
    ) -> ListRef<DatastreamConnectionProfilePostgresqlProfileElSslConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfilePrivateConnectivityEl {
    private_connection: PrimField<String>,
}
impl DatastreamConnectionProfilePrivateConnectivityEl {}
impl ToListMappable for DatastreamConnectionProfilePrivateConnectivityEl {
    type O = BlockAssignable<DatastreamConnectionProfilePrivateConnectivityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfilePrivateConnectivityEl {
    #[doc = "A reference to a private connection resource. Format: 'projects/{project}/locations/{location}/privateConnections/{name}'"]
    pub private_connection: PrimField<String>,
}
impl BuildDatastreamConnectionProfilePrivateConnectivityEl {
    pub fn build(self) -> DatastreamConnectionProfilePrivateConnectivityEl {
        DatastreamConnectionProfilePrivateConnectivityEl {
            private_connection: self.private_connection,
        }
    }
}
pub struct DatastreamConnectionProfilePrivateConnectivityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfilePrivateConnectivityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatastreamConnectionProfilePrivateConnectivityElRef {
        DatastreamConnectionProfilePrivateConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfilePrivateConnectivityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `private_connection` after provisioning.\nA reference to a private connection resource. Format: 'projects/{project}/locations/{location}/privateConnections/{name}'"]
    pub fn private_connection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_connection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileSqlServerProfileEl {
    database: PrimField<String>,
    hostname: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager_stored_password: Option<PrimField<String>>,
    username: PrimField<String>,
}
impl DatastreamConnectionProfileSqlServerProfileEl {
    #[doc = "Set the field `password`.\nPassword for the SQL Server connection."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nPort for the SQL Server connection."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_manager_stored_password`.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn set_secret_manager_stored_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_manager_stored_password = Some(v.into());
        self
    }
}
impl ToListMappable for DatastreamConnectionProfileSqlServerProfileEl {
    type O = BlockAssignable<DatastreamConnectionProfileSqlServerProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileSqlServerProfileEl {
    #[doc = "Database for the SQL Server connection."]
    pub database: PrimField<String>,
    #[doc = "Hostname for the SQL Server connection."]
    pub hostname: PrimField<String>,
    #[doc = "Username for the SQL Server connection."]
    pub username: PrimField<String>,
}
impl BuildDatastreamConnectionProfileSqlServerProfileEl {
    pub fn build(self) -> DatastreamConnectionProfileSqlServerProfileEl {
        DatastreamConnectionProfileSqlServerProfileEl {
            database: self.database,
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            secret_manager_stored_password: core::default::Default::default(),
            username: self.username,
        }
    }
}
pub struct DatastreamConnectionProfileSqlServerProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileSqlServerProfileElRef {
    fn new(shared: StackShared, base: String) -> DatastreamConnectionProfileSqlServerProfileElRef {
        DatastreamConnectionProfileSqlServerProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileSqlServerProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase for the SQL Server connection."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname for the SQL Server connection."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for the SQL Server connection."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the SQL Server connection."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_manager_stored_password` after provisioning.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn secret_manager_stored_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_manager_stored_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the SQL Server connection."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct DatastreamConnectionProfileTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DatastreamConnectionProfileTimeoutsEl {
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
impl ToListMappable for DatastreamConnectionProfileTimeoutsEl {
    type O = BlockAssignable<DatastreamConnectionProfileTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatastreamConnectionProfileTimeoutsEl {}
impl BuildDatastreamConnectionProfileTimeoutsEl {
    pub fn build(self) -> DatastreamConnectionProfileTimeoutsEl {
        DatastreamConnectionProfileTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DatastreamConnectionProfileTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatastreamConnectionProfileTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DatastreamConnectionProfileTimeoutsElRef {
        DatastreamConnectionProfileTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatastreamConnectionProfileTimeoutsElRef {
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
struct DatastreamConnectionProfileDynamic {
    bigquery_profile: Option<DynamicBlock<DatastreamConnectionProfileBigqueryProfileEl>>,
    forward_ssh_connectivity:
        Option<DynamicBlock<DatastreamConnectionProfileForwardSshConnectivityEl>>,
    gcs_profile: Option<DynamicBlock<DatastreamConnectionProfileGcsProfileEl>>,
    mongodb_profile: Option<DynamicBlock<DatastreamConnectionProfileMongodbProfileEl>>,
    mysql_profile: Option<DynamicBlock<DatastreamConnectionProfileMysqlProfileEl>>,
    oracle_profile: Option<DynamicBlock<DatastreamConnectionProfileOracleProfileEl>>,
    postgresql_profile: Option<DynamicBlock<DatastreamConnectionProfilePostgresqlProfileEl>>,
    private_connectivity: Option<DynamicBlock<DatastreamConnectionProfilePrivateConnectivityEl>>,
    sql_server_profile: Option<DynamicBlock<DatastreamConnectionProfileSqlServerProfileEl>>,
}
