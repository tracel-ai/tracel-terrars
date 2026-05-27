use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DatabaseMigrationServiceMigrationJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    destination: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dump_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dump_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    migration_job_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    source: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dump_flags: Option<Vec<DatabaseMigrationServiceMigrationJobDumpFlagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    objects_config: Option<Vec<DatabaseMigrationServiceMigrationJobObjectsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_config: Option<Vec<DatabaseMigrationServiceMigrationJobPerformanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postgres_homogeneous_config:
        Option<Vec<DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reverse_ssh_connectivity:
        Option<Vec<DatabaseMigrationServiceMigrationJobReverseSshConnectivityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_ip_connectivity: Option<Vec<DatabaseMigrationServiceMigrationJobStaticIpConnectivityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DatabaseMigrationServiceMigrationJobTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_peering_connectivity:
        Option<Vec<DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityEl>>,
    dynamic: DatabaseMigrationServiceMigrationJobDynamic,
}
struct DatabaseMigrationServiceMigrationJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatabaseMigrationServiceMigrationJobData>,
}
#[derive(Clone)]
pub struct DatabaseMigrationServiceMigrationJob(Rc<DatabaseMigrationServiceMigrationJob_>);
impl DatabaseMigrationServiceMigrationJob {
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
    #[doc = "Set the field `display_name`.\nThe migration job display name."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `dump_path`.\nThe path to the dump file in Google Cloud Storage,\nin the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]).\nThis field and the \"dump_flags\" field are mutually exclusive."]
    pub fn set_dump_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dump_path = Some(v.into());
        self
    }
    #[doc = "Set the field `dump_type`.\nThe type of the data dump. Supported for MySQL to CloudSQL for MySQL\nmigrations only. Possible values: [\"LOGICAL\", \"PHYSICAL\"]"]
    pub fn set_dump_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dump_type = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location where the migration job should reside."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `dump_flags`.\n"]
    pub fn set_dump_flags(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceMigrationJobDumpFlagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dump_flags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dump_flags = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `objects_config`.\n"]
    pub fn set_objects_config(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceMigrationJobObjectsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().objects_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.objects_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `performance_config`.\n"]
    pub fn set_performance_config(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceMigrationJobPerformanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().performance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.performance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `postgres_homogeneous_config`.\n"]
    pub fn set_postgres_homogeneous_config(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().postgres_homogeneous_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.postgres_homogeneous_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `reverse_ssh_connectivity`.\n"]
    pub fn set_reverse_ssh_connectivity(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceMigrationJobReverseSshConnectivityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reverse_ssh_connectivity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reverse_ssh_connectivity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `static_ip_connectivity`.\n"]
    pub fn set_static_ip_connectivity(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceMigrationJobStaticIpConnectivityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().static_ip_connectivity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.static_ip_connectivity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<DatabaseMigrationServiceMigrationJobTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_peering_connectivity`.\n"]
    pub fn set_vpc_peering_connectivity(
        self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_peering_connectivity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_peering_connectivity = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the resource was created. A timestamp in RFC3339 UTC 'Zulu' format, accurate to nanoseconds. Example: '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\nThe name of the destination connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{destinationConnectionProfile}."]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe migration job display name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dump_path` after provisioning.\nThe path to the dump file in Google Cloud Storage,\nin the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]).\nThis field and the \"dump_flags\" field are mutually exclusive."]
    pub fn dump_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dump_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dump_type` after provisioning.\nThe type of the data dump. Supported for MySQL to CloudSQL for MySQL\nmigrations only. Possible values: [\"LOGICAL\", \"PHYSICAL\"]"]
    pub fn dump_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dump_type", self.extract_ref()),
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
    pub fn error(&self) -> ListRef<DatabaseMigrationServiceMigrationJobErrorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the migration job should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `migration_job_id` after provisioning.\nThe ID of the migration job."]
    pub fn migration_job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.migration_job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this migration job resource in the form of projects/{project}/locations/{location}/migrationJobs/{migrationJob}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `phase` after provisioning.\nThe current migration job phase."]
    pub fn phase(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.phase", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\nThe name of the source connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{sourceConnectionProfile}."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current migration job state."]
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the migration job. Possible values: [\"ONE_TIME\", \"CONTINUOUS\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dump_flags` after provisioning.\n"]
    pub fn dump_flags(&self) -> ListRef<DatabaseMigrationServiceMigrationJobDumpFlagsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dump_flags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `objects_config` after provisioning.\n"]
    pub fn objects_config(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobObjectsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.objects_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_config` after provisioning.\n"]
    pub fn performance_config(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobPerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.performance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `postgres_homogeneous_config` after provisioning.\n"]
    pub fn postgres_homogeneous_config(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgres_homogeneous_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reverse_ssh_connectivity` after provisioning.\n"]
    pub fn reverse_ssh_connectivity(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobReverseSshConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reverse_ssh_connectivity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `static_ip_connectivity` after provisioning.\n"]
    pub fn static_ip_connectivity(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobStaticIpConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.static_ip_connectivity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseMigrationServiceMigrationJobTimeoutsElRef {
        DatabaseMigrationServiceMigrationJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_peering_connectivity` after provisioning.\n"]
    pub fn vpc_peering_connectivity(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_peering_connectivity", self.extract_ref()),
        )
    }
}
impl Referable for DatabaseMigrationServiceMigrationJob {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DatabaseMigrationServiceMigrationJob {}
impl ToListMappable for DatabaseMigrationServiceMigrationJob {
    type O = ListRef<DatabaseMigrationServiceMigrationJobRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DatabaseMigrationServiceMigrationJob_ {
    fn extract_resource_type(&self) -> String {
        "google_database_migration_service_migration_job".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJob {
    pub tf_id: String,
    #[doc = "The name of the destination connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{destinationConnectionProfile}."]
    pub destination: PrimField<String>,
    #[doc = "The ID of the migration job."]
    pub migration_job_id: PrimField<String>,
    #[doc = "The name of the source connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{sourceConnectionProfile}."]
    pub source: PrimField<String>,
    #[doc = "The type of the migration job. Possible values: [\"ONE_TIME\", \"CONTINUOUS\"]"]
    pub type_: PrimField<String>,
}
impl BuildDatabaseMigrationServiceMigrationJob {
    pub fn build(self, stack: &mut Stack) -> DatabaseMigrationServiceMigrationJob {
        let out =
            DatabaseMigrationServiceMigrationJob(Rc::new(DatabaseMigrationServiceMigrationJob_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DatabaseMigrationServiceMigrationJobData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    destination: self.destination,
                    display_name: core::default::Default::default(),
                    dump_path: core::default::Default::default(),
                    dump_type: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: core::default::Default::default(),
                    migration_job_id: self.migration_job_id,
                    project: core::default::Default::default(),
                    source: self.source,
                    type_: self.type_,
                    dump_flags: core::default::Default::default(),
                    objects_config: core::default::Default::default(),
                    performance_config: core::default::Default::default(),
                    postgres_homogeneous_config: core::default::Default::default(),
                    reverse_ssh_connectivity: core::default::Default::default(),
                    static_ip_connectivity: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    vpc_peering_connectivity: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DatabaseMigrationServiceMigrationJobRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DatabaseMigrationServiceMigrationJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the resource was created. A timestamp in RFC3339 UTC 'Zulu' format, accurate to nanoseconds. Example: '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\nThe name of the destination connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{destinationConnectionProfile}."]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe migration job display name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dump_path` after provisioning.\nThe path to the dump file in Google Cloud Storage,\nin the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]).\nThis field and the \"dump_flags\" field are mutually exclusive."]
    pub fn dump_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dump_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dump_type` after provisioning.\nThe type of the data dump. Supported for MySQL to CloudSQL for MySQL\nmigrations only. Possible values: [\"LOGICAL\", \"PHYSICAL\"]"]
    pub fn dump_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dump_type", self.extract_ref()),
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
    pub fn error(&self) -> ListRef<DatabaseMigrationServiceMigrationJobErrorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the migration job should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `migration_job_id` after provisioning.\nThe ID of the migration job."]
    pub fn migration_job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.migration_job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this migration job resource in the form of projects/{project}/locations/{location}/migrationJobs/{migrationJob}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `phase` after provisioning.\nThe current migration job phase."]
    pub fn phase(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.phase", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\nThe name of the source connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{sourceConnectionProfile}."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current migration job state."]
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the migration job. Possible values: [\"ONE_TIME\", \"CONTINUOUS\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dump_flags` after provisioning.\n"]
    pub fn dump_flags(&self) -> ListRef<DatabaseMigrationServiceMigrationJobDumpFlagsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dump_flags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `objects_config` after provisioning.\n"]
    pub fn objects_config(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobObjectsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.objects_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_config` after provisioning.\n"]
    pub fn performance_config(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobPerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.performance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `postgres_homogeneous_config` after provisioning.\n"]
    pub fn postgres_homogeneous_config(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postgres_homogeneous_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reverse_ssh_connectivity` after provisioning.\n"]
    pub fn reverse_ssh_connectivity(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobReverseSshConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reverse_ssh_connectivity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `static_ip_connectivity` after provisioning.\n"]
    pub fn static_ip_connectivity(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobStaticIpConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.static_ip_connectivity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseMigrationServiceMigrationJobTimeoutsElRef {
        DatabaseMigrationServiceMigrationJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_peering_connectivity` after provisioning.\n"]
    pub fn vpc_peering_connectivity(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_peering_connectivity", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJobErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<ListField<RecField<PrimField<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl DatabaseMigrationServiceMigrationJobErrorEl {
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
impl ToListMappable for DatabaseMigrationServiceMigrationJobErrorEl {
    type O = BlockAssignable<DatabaseMigrationServiceMigrationJobErrorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJobErrorEl {}
impl BuildDatabaseMigrationServiceMigrationJobErrorEl {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJobErrorEl {
        DatabaseMigrationServiceMigrationJobErrorEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceMigrationJobErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobErrorElRef {
    fn new(shared: StackShared, base: String) -> DatabaseMigrationServiceMigrationJobErrorElRef {
        DatabaseMigrationServiceMigrationJobErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceMigrationJobErrorElRef {
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
pub struct DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsEl {
    #[doc = "Set the field `name`.\nThe name of the flag"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe vale of the flag"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsEl {
    type O = BlockAssignable<DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsEl {}
impl BuildDatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsEl {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsEl {
        DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsElRef {
        DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the flag"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe vale of the flag"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceMigrationJobDumpFlagsElDynamic {
    dump_flags: Option<DynamicBlock<DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsEl>>,
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJobDumpFlagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dump_flags: Option<Vec<DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsEl>>,
    dynamic: DatabaseMigrationServiceMigrationJobDumpFlagsElDynamic,
}
impl DatabaseMigrationServiceMigrationJobDumpFlagsEl {
    #[doc = "Set the field `dump_flags`.\n"]
    pub fn set_dump_flags(
        mut self,
        v: impl Into<BlockAssignable<DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dump_flags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dump_flags = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceMigrationJobDumpFlagsEl {
    type O = BlockAssignable<DatabaseMigrationServiceMigrationJobDumpFlagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJobDumpFlagsEl {}
impl BuildDatabaseMigrationServiceMigrationJobDumpFlagsEl {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJobDumpFlagsEl {
        DatabaseMigrationServiceMigrationJobDumpFlagsEl {
            dump_flags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceMigrationJobDumpFlagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobDumpFlagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceMigrationJobDumpFlagsElRef {
        DatabaseMigrationServiceMigrationJobDumpFlagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceMigrationJobDumpFlagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dump_flags` after provisioning.\n"]
    pub fn dump_flags(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobDumpFlagsElDumpFlagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dump_flags", self.base))
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierEl { # [doc = "Set the field `database`.\nThe database name. Required only if the object uses\na database name as part of its unique identifier."] pub fn set_database (mut self , v : impl Into < PrimField < String > >) -> Self { self . database = Some (v . into ()) ; self } # [doc = "Set the field `schema`.\nThe schema name. Required only if the object uses\na schema name as part of its unique identifier."] pub fn set_schema (mut self , v : impl Into < PrimField < String > >) -> Self { self . schema = Some (v . into ()) ; self } # [doc = "Set the field `table`.\nThe table name. Required only if the object is a level\nbelow database or schema."] pub fn set_table (mut self , v : impl Into < PrimField < String > >) -> Self { self . table = Some (v . into ()) ; self } }
impl ToListMappable for DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierEl { type O = BlockAssignable < DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierEl
{
    #[doc = "The category of the migration job object: 'DATABASE',\n'SCHEMA', or 'TABLE'. Possible values: [\"DATABASE\", \"SCHEMA\", \"TABLE\"]"]
    pub type_: PrimField<String>,
}
impl BuildDatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierEl { pub fn build (self) -> DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierEl { DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierEl { database : core :: default :: Default :: default () , schema : core :: default :: Default :: default () , table : core :: default :: Default :: default () , type_ : self . type_ , } } }
pub struct DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierElRef { fn new (shared : StackShared , base : String) -> DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierElRef { DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierElRef { shared : shared , base : base . to_string () , } } }
impl DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `database` after provisioning.\nThe database name. Required only if the object uses\na database name as part of its unique identifier."] pub fn database (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.database" , self . base)) } # [doc = "Get a reference to the value of field `schema` after provisioning.\nThe schema name. Required only if the object uses\na schema name as part of its unique identifier."] pub fn schema (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.schema" , self . base)) } # [doc = "Get a reference to the value of field `table` after provisioning.\nThe table name. Required only if the object is a level\nbelow database or schema."] pub fn table (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.table" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\nThe category of the migration job object: 'DATABASE',\n'SCHEMA', or 'TABLE'. Possible values: [\"DATABASE\", \"SCHEMA\", \"TABLE\"]"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElDynamic { object_identifier : Option < DynamicBlock < DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierEl >> , }
#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsEl { # [serde (skip_serializing_if = "Option::is_none")] object_identifier : Option < Vec < DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierEl > > , dynamic : DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElDynamic , }
impl DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsEl {
    #[doc = "Set the field `object_identifier`.\n"]
    pub fn set_object_identifier(
        mut self,
        v : impl Into < BlockAssignable < DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.object_identifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.object_identifier = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsEl
{
    type O = BlockAssignable<
        DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsEl
{}
impl BuildDatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsEl {
    pub fn build(
        self,
    ) -> DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsEl
    {
        DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsEl {
            object_identifier: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElRef
    {
        DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `object_identifier` after provisioning.\n"]    pub fn object_identifier (& self) -> ListRef < DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElObjectIdentifierElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_identifier", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElDynamic {
    object_configs: Option<
        DynamicBlock<
            DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    objects_selection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_configs: Option<
        Vec<
            DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsEl,
        >,
    >,
    dynamic: DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElDynamic,
}
impl DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigEl {
    #[doc = "Set the field `objects_selection_type`.\nThe objects selection type of the migration job. When set to\n'SPECIFIED_OBJECTS', only the objects listed in 'objectConfigs' are\nmigrated. When set to 'ALL_OBJECTS', all objects available on the\nsource are migrated. Possible values: [\"ALL_OBJECTS\", \"SPECIFIED_OBJECTS\"]"]
    pub fn set_objects_selection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.objects_selection_type = Some(v.into());
        self
    }
    #[doc = "Set the field `object_configs`.\n"]
    pub fn set_object_configs(
        mut self,
        v : impl Into < BlockAssignable < DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.object_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.object_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigEl {
    type O =
        BlockAssignable<DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigEl {}
impl BuildDatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigEl {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigEl {
        DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigEl {
            objects_selection_type: core::default::Default::default(),
            object_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElRef {
        DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `objects_selection_type` after provisioning.\nThe objects selection type of the migration job. When set to\n'SPECIFIED_OBJECTS', only the objects listed in 'objectConfigs' are\nmigrated. When set to 'ALL_OBJECTS', all objects available on the\nsource are migrated. Possible values: [\"ALL_OBJECTS\", \"SPECIFIED_OBJECTS\"]"]
    pub fn objects_selection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.objects_selection_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `object_configs` after provisioning.\n"]
    pub fn object_configs(
        &self,
    ) -> ListRef<
        DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElObjectConfigsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.object_configs", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceMigrationJobObjectsConfigElDynamic {
    source_objects_config: Option<
        DynamicBlock<DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJobObjectsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_objects_config:
        Option<Vec<DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigEl>>,
    dynamic: DatabaseMigrationServiceMigrationJobObjectsConfigElDynamic,
}
impl DatabaseMigrationServiceMigrationJobObjectsConfigEl {
    #[doc = "Set the field `source_objects_config`.\n"]
    pub fn set_source_objects_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_objects_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_objects_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceMigrationJobObjectsConfigEl {
    type O = BlockAssignable<DatabaseMigrationServiceMigrationJobObjectsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJobObjectsConfigEl {}
impl BuildDatabaseMigrationServiceMigrationJobObjectsConfigEl {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJobObjectsConfigEl {
        DatabaseMigrationServiceMigrationJobObjectsConfigEl {
            source_objects_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceMigrationJobObjectsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobObjectsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceMigrationJobObjectsConfigElRef {
        DatabaseMigrationServiceMigrationJobObjectsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceMigrationJobObjectsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_objects_config` after provisioning.\n"]
    pub fn source_objects_config(
        &self,
    ) -> ListRef<DatabaseMigrationServiceMigrationJobObjectsConfigElSourceObjectsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_objects_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJobPerformanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dump_parallel_level: Option<PrimField<String>>,
}
impl DatabaseMigrationServiceMigrationJobPerformanceConfigEl {
    #[doc = "Set the field `dump_parallel_level`.\nInitial dump parallelism level. Possible values: [\"MIN\", \"OPTIMAL\", \"MAX\"]"]
    pub fn set_dump_parallel_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dump_parallel_level = Some(v.into());
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceMigrationJobPerformanceConfigEl {
    type O = BlockAssignable<DatabaseMigrationServiceMigrationJobPerformanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJobPerformanceConfigEl {}
impl BuildDatabaseMigrationServiceMigrationJobPerformanceConfigEl {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJobPerformanceConfigEl {
        DatabaseMigrationServiceMigrationJobPerformanceConfigEl {
            dump_parallel_level: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceMigrationJobPerformanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobPerformanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceMigrationJobPerformanceConfigElRef {
        DatabaseMigrationServiceMigrationJobPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceMigrationJobPerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dump_parallel_level` after provisioning.\nInitial dump parallelism level. Possible values: [\"MIN\", \"OPTIMAL\", \"MAX\"]"]
    pub fn dump_parallel_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dump_parallel_level", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigEl {
    is_native_logical: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_additional_subscriptions: Option<PrimField<f64>>,
}
impl DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigEl {
    #[doc = "Set the field `max_additional_subscriptions`.\nMaximum number of additional subscriptions to use for the migration job."]
    pub fn set_max_additional_subscriptions(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_additional_subscriptions = Some(v.into());
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigEl {
    type O = BlockAssignable<DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigEl {
    #[doc = "Whether the migration uses native logical replication."]
    pub is_native_logical: PrimField<bool>,
}
impl BuildDatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigEl {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigEl {
        DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigEl {
            is_native_logical: self.is_native_logical,
            max_additional_subscriptions: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigElRef {
        DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_native_logical` after provisioning.\nWhether the migration uses native logical replication."]
    pub fn is_native_logical(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_native_logical", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_additional_subscriptions` after provisioning.\nMaximum number of additional subscriptions to use for the migration job."]
    pub fn max_additional_subscriptions(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_additional_subscriptions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJobReverseSshConnectivityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    vm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc: Option<PrimField<String>>,
}
impl DatabaseMigrationServiceMigrationJobReverseSshConnectivityEl {
    #[doc = "Set the field `vm`.\nThe name of the virtual machine (Compute Engine) used as the bastion server\nfor the SSH tunnel."]
    pub fn set_vm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vm = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_ip`.\nThe IP of the virtual machine (Compute Engine) used as the bastion server\nfor the SSH tunnel."]
    pub fn set_vm_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vm_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_port`.\nThe forwarding port of the virtual machine (Compute Engine) used as the\nbastion server for the SSH tunnel."]
    pub fn set_vm_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vm_port = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc`.\nThe name of the VPC to peer with the Cloud SQL private network."]
    pub fn set_vpc(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc = Some(v.into());
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceMigrationJobReverseSshConnectivityEl {
    type O = BlockAssignable<DatabaseMigrationServiceMigrationJobReverseSshConnectivityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJobReverseSshConnectivityEl {}
impl BuildDatabaseMigrationServiceMigrationJobReverseSshConnectivityEl {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJobReverseSshConnectivityEl {
        DatabaseMigrationServiceMigrationJobReverseSshConnectivityEl {
            vm: core::default::Default::default(),
            vm_ip: core::default::Default::default(),
            vm_port: core::default::Default::default(),
            vpc: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceMigrationJobReverseSshConnectivityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobReverseSshConnectivityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceMigrationJobReverseSshConnectivityElRef {
        DatabaseMigrationServiceMigrationJobReverseSshConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceMigrationJobReverseSshConnectivityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `vm` after provisioning.\nThe name of the virtual machine (Compute Engine) used as the bastion server\nfor the SSH tunnel."]
    pub fn vm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vm", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_ip` after provisioning.\nThe IP of the virtual machine (Compute Engine) used as the bastion server\nfor the SSH tunnel."]
    pub fn vm_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vm_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_port` after provisioning.\nThe forwarding port of the virtual machine (Compute Engine) used as the\nbastion server for the SSH tunnel."]
    pub fn vm_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vm_port", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc` after provisioning.\nThe name of the VPC to peer with the Cloud SQL private network."]
    pub fn vpc(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJobStaticIpConnectivityEl {}
impl DatabaseMigrationServiceMigrationJobStaticIpConnectivityEl {}
impl ToListMappable for DatabaseMigrationServiceMigrationJobStaticIpConnectivityEl {
    type O = BlockAssignable<DatabaseMigrationServiceMigrationJobStaticIpConnectivityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJobStaticIpConnectivityEl {}
impl BuildDatabaseMigrationServiceMigrationJobStaticIpConnectivityEl {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJobStaticIpConnectivityEl {
        DatabaseMigrationServiceMigrationJobStaticIpConnectivityEl {}
    }
}
pub struct DatabaseMigrationServiceMigrationJobStaticIpConnectivityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobStaticIpConnectivityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceMigrationJobStaticIpConnectivityElRef {
        DatabaseMigrationServiceMigrationJobStaticIpConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceMigrationJobStaticIpConnectivityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJobTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DatabaseMigrationServiceMigrationJobTimeoutsEl {
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
impl ToListMappable for DatabaseMigrationServiceMigrationJobTimeoutsEl {
    type O = BlockAssignable<DatabaseMigrationServiceMigrationJobTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJobTimeoutsEl {}
impl BuildDatabaseMigrationServiceMigrationJobTimeoutsEl {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJobTimeoutsEl {
        DatabaseMigrationServiceMigrationJobTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceMigrationJobTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DatabaseMigrationServiceMigrationJobTimeoutsElRef {
        DatabaseMigrationServiceMigrationJobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceMigrationJobTimeoutsElRef {
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
#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc: Option<PrimField<String>>,
}
impl DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityEl {
    #[doc = "Set the field `vpc`.\nThe name of the VPC network to peer with the Cloud SQL private network."]
    pub fn set_vpc(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc = Some(v.into());
        self
    }
}
impl ToListMappable for DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityEl {
    type O = BlockAssignable<DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatabaseMigrationServiceMigrationJobVpcPeeringConnectivityEl {}
impl BuildDatabaseMigrationServiceMigrationJobVpcPeeringConnectivityEl {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityEl {
        DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityEl {
            vpc: core::default::Default::default(),
        }
    }
}
pub struct DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityElRef {
        DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `vpc` after provisioning.\nThe name of the VPC network to peer with the Cloud SQL private network."]
    pub fn vpc(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatabaseMigrationServiceMigrationJobDynamic {
    dump_flags: Option<DynamicBlock<DatabaseMigrationServiceMigrationJobDumpFlagsEl>>,
    objects_config: Option<DynamicBlock<DatabaseMigrationServiceMigrationJobObjectsConfigEl>>,
    performance_config:
        Option<DynamicBlock<DatabaseMigrationServiceMigrationJobPerformanceConfigEl>>,
    postgres_homogeneous_config:
        Option<DynamicBlock<DatabaseMigrationServiceMigrationJobPostgresHomogeneousConfigEl>>,
    reverse_ssh_connectivity:
        Option<DynamicBlock<DatabaseMigrationServiceMigrationJobReverseSshConnectivityEl>>,
    static_ip_connectivity:
        Option<DynamicBlock<DatabaseMigrationServiceMigrationJobStaticIpConnectivityEl>>,
    vpc_peering_connectivity:
        Option<DynamicBlock<DatabaseMigrationServiceMigrationJobVpcPeeringConnectivityEl>>,
}
