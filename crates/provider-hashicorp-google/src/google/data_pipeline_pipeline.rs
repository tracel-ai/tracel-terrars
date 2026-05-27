use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataPipelinePipelineData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_sources: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduler_service_account_email: Option<PrimField<String>>,
    state: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_info: Option<Vec<DataPipelinePipelineScheduleInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataPipelinePipelineTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload: Option<Vec<DataPipelinePipelineWorkloadEl>>,
    dynamic: DataPipelinePipelineDynamic,
}
struct DataPipelinePipeline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataPipelinePipelineData>,
}
#[derive(Clone)]
pub struct DataPipelinePipeline(Rc<DataPipelinePipeline_>);
impl DataPipelinePipeline {
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
    #[doc = "Set the field `display_name`.\nThe display name of the pipeline. It can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), and underscores (_)."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `pipeline_sources`.\nThe sources of the pipeline (for example, Dataplex). The keys and values are set by the corresponding sources during pipeline creation.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_pipeline_sources(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().pipeline_sources = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nA reference to the region"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `scheduler_service_account_email`.\nOptional. A service account email to be used with the Cloud Scheduler job. If not specified, the default compute engine service account will be used."]
    pub fn set_scheduler_service_account_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().scheduler_service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule_info`.\n"]
    pub fn set_schedule_info(
        self,
        v: impl Into<BlockAssignable<DataPipelinePipelineScheduleInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule_info = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataPipelinePipelineTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `workload`.\n"]
    pub fn set_workload(
        self,
        v: impl Into<BlockAssignable<DataPipelinePipelineWorkloadEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workload = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workload = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the pipeline was initially created. Set by the Data Pipelines service.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the pipeline. It can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), and underscores (_)."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `job_count` after provisioning.\nNumber of jobs."]
    pub fn job_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_update_time` after provisioning.\nThe timestamp when the pipeline was last modified. Set by the Data Pipelines service.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n\"The pipeline name. For example': 'projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID.\"\n\"- PROJECT_ID can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see Identifying projects.\"\n\"LOCATION_ID is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling google.cloud.location.Locations.ListLocations. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in App Engine regions.\"\n\"PIPELINE_ID is the ID of the pipeline. Must be unique for the selected project and location.\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_sources` after provisioning.\nThe sources of the pipeline (for example, Dataplex). The keys and values are set by the corresponding sources during pipeline creation.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn pipeline_sources(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.pipeline_sources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nA reference to the region"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scheduler_service_account_email` after provisioning.\nOptional. A service account email to be used with the Cloud Scheduler job. If not specified, the default compute engine service account will be used."]
    pub fn scheduler_service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduler_service_account_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the pipeline. When the pipeline is created, the state is set to 'PIPELINE_STATE_ACTIVE' by default. State changes can be requested by setting the state to stopping, paused, or resuming. State cannot be changed through pipelines.patch requests.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#state Possible values: [\"STATE_UNSPECIFIED\", \"STATE_RESUMING\", \"STATE_ACTIVE\", \"STATE_STOPPING\", \"STATE_ARCHIVED\", \"STATE_PAUSED\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the pipeline. This field affects the scheduling of the pipeline and the type of metrics to show for the pipeline.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#pipelinetype Possible values: [\"PIPELINE_TYPE_UNSPECIFIED\", \"PIPELINE_TYPE_BATCH\", \"PIPELINE_TYPE_STREAMING\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_info` after provisioning.\n"]
    pub fn schedule_info(&self) -> ListRef<DataPipelinePipelineScheduleInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataPipelinePipelineTimeoutsElRef {
        DataPipelinePipelineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload` after provisioning.\n"]
    pub fn workload(&self) -> ListRef<DataPipelinePipelineWorkloadElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload", self.extract_ref()),
        )
    }
}
impl Referable for DataPipelinePipeline {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataPipelinePipeline {}
impl ToListMappable for DataPipelinePipeline {
    type O = ListRef<DataPipelinePipelineRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataPipelinePipeline_ {
    fn extract_resource_type(&self) -> String {
        "google_data_pipeline_pipeline".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataPipelinePipeline {
    pub tf_id: String,
    #[doc = "\"The pipeline name. For example': 'projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID.\"\n\"- PROJECT_ID can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see Identifying projects.\"\n\"LOCATION_ID is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling google.cloud.location.Locations.ListLocations. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in App Engine regions.\"\n\"PIPELINE_ID is the ID of the pipeline. Must be unique for the selected project and location.\""]
    pub name: PrimField<String>,
    #[doc = "The state of the pipeline. When the pipeline is created, the state is set to 'PIPELINE_STATE_ACTIVE' by default. State changes can be requested by setting the state to stopping, paused, or resuming. State cannot be changed through pipelines.patch requests.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#state Possible values: [\"STATE_UNSPECIFIED\", \"STATE_RESUMING\", \"STATE_ACTIVE\", \"STATE_STOPPING\", \"STATE_ARCHIVED\", \"STATE_PAUSED\"]"]
    pub state: PrimField<String>,
    #[doc = "The type of the pipeline. This field affects the scheduling of the pipeline and the type of metrics to show for the pipeline.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#pipelinetype Possible values: [\"PIPELINE_TYPE_UNSPECIFIED\", \"PIPELINE_TYPE_BATCH\", \"PIPELINE_TYPE_STREAMING\"]"]
    pub type_: PrimField<String>,
}
impl BuildDataPipelinePipeline {
    pub fn build(self, stack: &mut Stack) -> DataPipelinePipeline {
        let out = DataPipelinePipeline(Rc::new(DataPipelinePipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataPipelinePipelineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                pipeline_sources: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                scheduler_service_account_email: core::default::Default::default(),
                state: self.state,
                type_: self.type_,
                schedule_info: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                workload: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataPipelinePipelineRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPipelinePipelineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataPipelinePipelineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the pipeline was initially created. Set by the Data Pipelines service.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the pipeline. It can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), and underscores (_)."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `job_count` after provisioning.\nNumber of jobs."]
    pub fn job_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_update_time` after provisioning.\nThe timestamp when the pipeline was last modified. Set by the Data Pipelines service.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n\"The pipeline name. For example': 'projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID.\"\n\"- PROJECT_ID can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see Identifying projects.\"\n\"LOCATION_ID is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling google.cloud.location.Locations.ListLocations. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in App Engine regions.\"\n\"PIPELINE_ID is the ID of the pipeline. Must be unique for the selected project and location.\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pipeline_sources` after provisioning.\nThe sources of the pipeline (for example, Dataplex). The keys and values are set by the corresponding sources during pipeline creation.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn pipeline_sources(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.pipeline_sources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nA reference to the region"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scheduler_service_account_email` after provisioning.\nOptional. A service account email to be used with the Cloud Scheduler job. If not specified, the default compute engine service account will be used."]
    pub fn scheduler_service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduler_service_account_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the pipeline. When the pipeline is created, the state is set to 'PIPELINE_STATE_ACTIVE' by default. State changes can be requested by setting the state to stopping, paused, or resuming. State cannot be changed through pipelines.patch requests.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#state Possible values: [\"STATE_UNSPECIFIED\", \"STATE_RESUMING\", \"STATE_ACTIVE\", \"STATE_STOPPING\", \"STATE_ARCHIVED\", \"STATE_PAUSED\"]"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the pipeline. This field affects the scheduling of the pipeline and the type of metrics to show for the pipeline.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#pipelinetype Possible values: [\"PIPELINE_TYPE_UNSPECIFIED\", \"PIPELINE_TYPE_BATCH\", \"PIPELINE_TYPE_STREAMING\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_info` after provisioning.\n"]
    pub fn schedule_info(&self) -> ListRef<DataPipelinePipelineScheduleInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schedule_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataPipelinePipelineTimeoutsElRef {
        DataPipelinePipelineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload` after provisioning.\n"]
    pub fn workload(&self) -> ListRef<DataPipelinePipelineWorkloadElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataPipelinePipelineScheduleInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
}
impl DataPipelinePipelineScheduleInfoEl {
    #[doc = "Set the field `schedule`.\nUnix-cron format of the schedule. This information is retrieved from the linked Cloud Scheduler."]
    pub fn set_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `time_zone`.\nTimezone ID. This matches the timezone IDs used by the Cloud Scheduler API. If empty, UTC time is assumed."]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }
}
impl ToListMappable for DataPipelinePipelineScheduleInfoEl {
    type O = BlockAssignable<DataPipelinePipelineScheduleInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPipelinePipelineScheduleInfoEl {}
impl BuildDataPipelinePipelineScheduleInfoEl {
    pub fn build(self) -> DataPipelinePipelineScheduleInfoEl {
        DataPipelinePipelineScheduleInfoEl {
            schedule: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}
pub struct DataPipelinePipelineScheduleInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPipelinePipelineScheduleInfoElRef {
    fn new(shared: StackShared, base: String) -> DataPipelinePipelineScheduleInfoElRef {
        DataPipelinePipelineScheduleInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPipelinePipelineScheduleInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `next_job_time` after provisioning.\nWhen the next Scheduler job is going to run.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn next_job_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_job_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nUnix-cron format of the schedule. This information is retrieved from the linked Cloud Scheduler."]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nTimezone ID. This matches the timezone IDs used by the Cloud Scheduler API. If empty, UTC time is assumed."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize)]
pub struct DataPipelinePipelineTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataPipelinePipelineTimeoutsEl {
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
impl ToListMappable for DataPipelinePipelineTimeoutsEl {
    type O = BlockAssignable<DataPipelinePipelineTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPipelinePipelineTimeoutsEl {}
impl BuildDataPipelinePipelineTimeoutsEl {
    pub fn build(self) -> DataPipelinePipelineTimeoutsEl {
        DataPipelinePipelineTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataPipelinePipelineTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPipelinePipelineTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataPipelinePipelineTimeoutsElRef {
        DataPipelinePipelineTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPipelinePipelineTimeoutsElRef {
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
pub struct DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_experiments: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_user_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_streaming_engine: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flexrs_goal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_configuration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_workers: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_workers: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temp_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentEl {
    #[doc = "Set the field `additional_experiments`.\nAdditional experiment flags for the job."]
    pub fn set_additional_experiments(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.additional_experiments = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_user_labels`.\nAdditional user labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. An object containing a list of key/value pairs.\n'Example: { \"name\": \"wrench\", \"mass\": \"1kg\", \"count\": \"3\" }.'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_additional_user_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.additional_user_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_streaming_engine`.\nWhether to enable Streaming Engine for the job."]
    pub fn set_enable_streaming_engine(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_streaming_engine = Some(v.into());
        self
    }
    #[doc = "Set the field `flexrs_goal`.\nSet FlexRS goal for the job. https://cloud.google.com/dataflow/docs/guides/flexrs\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#FlexResourceSchedulingGoal Possible values: [\"FLEXRS_UNSPECIFIED\", \"FLEXRS_SPEED_OPTIMIZED\", \"FLEXRS_COST_OPTIMIZED\"]"]
    pub fn set_flexrs_goal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flexrs_goal = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_configuration`.\nConfiguration for VM IPs.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#WorkerIPAddressConfiguration Possible values: [\"WORKER_IP_UNSPECIFIED\", \"WORKER_IP_PUBLIC\", \"WORKER_IP_PRIVATE\"]"]
    pub fn set_ip_configuration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\n'Name for the Cloud KMS key for the job. The key format is: projects//locations//keyRings//cryptoKeys/'"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe machine type to use for the job. Defaults to the value from the template if not specified."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `max_workers`.\nThe maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000."]
    pub fn set_max_workers(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_workers = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nNetwork to which VMs will be assigned. If empty or unspecified, the service will use the network \"default\"."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `num_workers`.\nThe initial number of Compute Engine instances for the job."]
    pub fn set_num_workers(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_workers = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\nThe email address of the service account to run the job as."]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nSubnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form \"https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK\" or \"regions/REGION/subnetworks/SUBNETWORK\". If the subnetwork is located in a Shared VPC network, you must use the complete URL."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `temp_location`.\nThe Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://."]
    pub fn set_temp_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.temp_location = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_region`.\nThe Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1\". Mutually exclusive with workerZone. If neither workerRegion nor workerZone is specified, default to the control plane's region."]
    pub fn set_worker_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.worker_region = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_zone`.\nThe Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1-a\". Mutually exclusive with workerRegion. If neither workerRegion nor workerZone is specified, a zone in the control plane's region is chosen based on available capacity. If both workerZone and zone are set, workerZone takes precedence."]
    pub fn set_worker_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.worker_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe Compute Engine availability zone for launching worker instances to run your pipeline. In the future, workerZone will take precedence."]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentEl
{
    type O = BlockAssignable<
        DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentEl
{}
impl
    BuildDataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentEl
{
    pub fn build(
        self,
    ) -> DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentEl
    {
        DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentEl {
            additional_experiments: core::default::Default::default(),
            additional_user_labels: core::default::Default::default(),
            enable_streaming_engine: core::default::Default::default(),
            flexrs_goal: core::default::Default::default(),
            ip_configuration: core::default::Default::default(),
            kms_key_name: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            max_workers: core::default::Default::default(),
            network: core::default::Default::default(),
            num_workers: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            temp_location: core::default::Default::default(),
            worker_region: core::default::Default::default(),
            worker_zone: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentElRef
    {
        DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_experiments` after provisioning.\nAdditional experiment flags for the job."]
    pub fn additional_experiments(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_experiments", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_user_labels` after provisioning.\nAdditional user labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. An object containing a list of key/value pairs.\n'Example: { \"name\": \"wrench\", \"mass\": \"1kg\", \"count\": \"3\" }.'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn additional_user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.additional_user_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_streaming_engine` after provisioning.\nWhether to enable Streaming Engine for the job."]
    pub fn enable_streaming_engine(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_streaming_engine", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `flexrs_goal` after provisioning.\nSet FlexRS goal for the job. https://cloud.google.com/dataflow/docs/guides/flexrs\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#FlexResourceSchedulingGoal Possible values: [\"FLEXRS_UNSPECIFIED\", \"FLEXRS_SPEED_OPTIMIZED\", \"FLEXRS_COST_OPTIMIZED\"]"]
    pub fn flexrs_goal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flexrs_goal", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_configuration` after provisioning.\nConfiguration for VM IPs.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#WorkerIPAddressConfiguration Possible values: [\"WORKER_IP_UNSPECIFIED\", \"WORKER_IP_PUBLIC\", \"WORKER_IP_PRIVATE\"]"]
    pub fn ip_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n'Name for the Cloud KMS key for the job. The key format is: projects//locations//keyRings//cryptoKeys/'"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to use for the job. Defaults to the value from the template if not specified."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `max_workers` after provisioning.\nThe maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000."]
    pub fn max_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_workers", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nNetwork to which VMs will be assigned. If empty or unspecified, the service will use the network \"default\"."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `num_workers` after provisioning.\nThe initial number of Compute Engine instances for the job."]
    pub fn num_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_workers", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nThe email address of the service account to run the job as."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nSubnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form \"https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK\" or \"regions/REGION/subnetworks/SUBNETWORK\". If the subnetwork is located in a Shared VPC network, you must use the complete URL."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `temp_location` after provisioning.\nThe Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://."]
    pub fn temp_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.temp_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `worker_region` after provisioning.\nThe Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1\". Mutually exclusive with workerZone. If neither workerRegion nor workerZone is specified, default to the control plane's region."]
    pub fn worker_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.worker_region", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `worker_zone` after provisioning.\nThe Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1-a\". Mutually exclusive with workerRegion. If neither workerRegion nor workerZone is specified, a zone in the control plane's region is chosen based on available capacity. If both workerZone and zone are set, workerZone takes precedence."]
    pub fn worker_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_zone", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe Compute Engine availability zone for launching worker instances to run your pipeline. In the future, workerZone will take precedence."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElDynamic { environment : Option < DynamicBlock < DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentEl >> , }
#[derive(Serialize)]
pub struct DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterEl { # [serde (skip_serializing_if = "Option::is_none")] container_spec_gcs_path : Option < PrimField < String > > , job_name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] launch_options : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] parameters : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] transform_name_mappings : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] update : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] environment : Option < Vec < DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentEl > > , dynamic : DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElDynamic , }
impl DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterEl {
    #[doc = "Set the field `container_spec_gcs_path`.\nCloud Storage path to a file with a JSON-serialized ContainerSpec as content."]
    pub fn set_container_spec_gcs_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_spec_gcs_path = Some(v.into());
        self
    }
    #[doc = "Set the field `launch_options`.\nLaunch options for this Flex Template job. This is a common set of options across languages and templates. This should not be used to pass job parameters.\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_launch_options(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.launch_options = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\n'The parameters for the Flex Template. Example: {\"numWorkers\":\"5\"}'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `transform_name_mappings`.\n'Use this to pass transform name mappings for streaming update jobs. Example: {\"oldTransformName\":\"newTransformName\",...}'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_transform_name_mappings(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.transform_name_mappings = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nSet this to true if you are sending a request to update a running streaming job. When set, the job name should be the same as the running job."]
    pub fn set_update(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.update = Some(v.into());
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v : impl Into < BlockAssignable < DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.environment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterEl
{
    type O = BlockAssignable<
        DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterEl {
    #[doc = "The job name to use for the created job. For an update job request, the job name should be the same as the existing running job."]
    pub job_name: PrimField<String>,
}
impl BuildDataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterEl {
    pub fn build(
        self,
    ) -> DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterEl {
        DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterEl {
            container_spec_gcs_path: core::default::Default::default(),
            job_name: self.job_name,
            launch_options: core::default::Default::default(),
            parameters: core::default::Default::default(),
            transform_name_mappings: core::default::Default::default(),
            update: core::default::Default::default(),
            environment: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElRef {
        DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_spec_gcs_path` after provisioning.\nCloud Storage path to a file with a JSON-serialized ContainerSpec as content."]
    pub fn container_spec_gcs_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_spec_gcs_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `job_name` after provisioning.\nThe job name to use for the created job. For an update job request, the job name should be the same as the existing running job."]
    pub fn job_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.base))
    }
    #[doc = "Get a reference to the value of field `launch_options` after provisioning.\nLaunch options for this Flex Template job. This is a common set of options across languages and templates. This should not be used to pass job parameters.\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn launch_options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.launch_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n'The parameters for the Flex Template. Example: {\"numWorkers\":\"5\"}'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
    #[doc = "Get a reference to the value of field `transform_name_mappings` after provisioning.\n'Use this to pass transform name mappings for streaming update jobs. Example: {\"oldTransformName\":\"newTransformName\",...}'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn transform_name_mappings(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.transform_name_mappings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nSet this to true if you are sending a request to update a running streaming job. When set, the job name should be the same as the running job."]
    pub fn update(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]    pub fn environment (& self) -> ListRef < DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElEnvironmentElRef >{
        ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElDynamic {
    launch_parameter: Option<
        DynamicBlock<DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterEl>,
    >,
}
#[derive(Serialize)]
pub struct DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestEl {
    location: PrimField<String>,
    project_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validate_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_parameter:
        Option<Vec<DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterEl>>,
    dynamic: DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElDynamic,
}
impl DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestEl {
    #[doc = "Set the field `validate_only`.\nIf true, the request is validated but not actually executed. Defaults to false."]
    pub fn set_validate_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.validate_only = Some(v.into());
        self
    }
    #[doc = "Set the field `launch_parameter`.\n"]
    pub fn set_launch_parameter(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.launch_parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_parameter = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestEl {
    type O = BlockAssignable<DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPipelinePipelineWorkloadElDataflowFlexTemplateRequestEl {
    #[doc = "The regional endpoint to which to direct the request. For example, us-central1, us-west1."]
    pub location: PrimField<String>,
    #[doc = "The ID of the Cloud Platform project that the job belongs to."]
    pub project_id: PrimField<String>,
}
impl BuildDataPipelinePipelineWorkloadElDataflowFlexTemplateRequestEl {
    pub fn build(self) -> DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestEl {
        DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestEl {
            location: self.location,
            project_id: self.project_id,
            validate_only: core::default::Default::default(),
            launch_parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElRef {
        DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe regional endpoint to which to direct the request. For example, us-central1, us-west1."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the Cloud Platform project that the job belongs to."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `validate_only` after provisioning.\nIf true, the request is validated but not actually executed. Defaults to false."]
    pub fn validate_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.validate_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `launch_parameter` after provisioning.\n"]
    pub fn launch_parameter(
        &self,
    ) -> ListRef<DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElLaunchParameterElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_parameter", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_experiments: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_user_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_temp_dir_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_streaming_engine: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_configuration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_workers: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_workers: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temp_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentEl {
    #[doc = "Set the field `additional_experiments`.\nAdditional experiment flags for the job."]
    pub fn set_additional_experiments(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.additional_experiments = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_user_labels`.\nAdditional user labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. An object containing a list of key/value pairs.\n'Example: { \"name\": \"wrench\", \"mass\": \"1kg\", \"count\": \"3\" }.'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_additional_user_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.additional_user_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `bypass_temp_dir_validation`.\nWhether to bypass the safety checks for the job's temporary directory. Use with caution."]
    pub fn set_bypass_temp_dir_validation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bypass_temp_dir_validation = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_streaming_engine`.\nWhether to enable Streaming Engine for the job."]
    pub fn set_enable_streaming_engine(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_streaming_engine = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_configuration`.\nConfiguration for VM IPs.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#WorkerIPAddressConfiguration Possible values: [\"WORKER_IP_UNSPECIFIED\", \"WORKER_IP_PUBLIC\", \"WORKER_IP_PRIVATE\"]"]
    pub fn set_ip_configuration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\n'Name for the Cloud KMS key for the job. The key format is: projects//locations//keyRings//cryptoKeys/'"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe machine type to use for the job. Defaults to the value from the template if not specified."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `max_workers`.\nThe maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000."]
    pub fn set_max_workers(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_workers = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nNetwork to which VMs will be assigned. If empty or unspecified, the service will use the network \"default\"."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `num_workers`.\nThe initial number of Compute Engine instances for the job."]
    pub fn set_num_workers(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_workers = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\nThe email address of the service account to run the job as."]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nSubnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form \"https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK\" or \"regions/REGION/subnetworks/SUBNETWORK\". If the subnetwork is located in a Shared VPC network, you must use the complete URL."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `temp_location`.\nThe Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://."]
    pub fn set_temp_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.temp_location = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_region`.\nThe Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1\". Mutually exclusive with workerZone. If neither workerRegion nor workerZone is specified, default to the control plane's region."]
    pub fn set_worker_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.worker_region = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_zone`.\nThe Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1-a\". Mutually exclusive with workerRegion. If neither workerRegion nor workerZone is specified, a zone in the control plane's region is chosen based on available capacity. If both workerZone and zone are set, workerZone takes precedence."]
    pub fn set_worker_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.worker_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe Compute Engine availability zone for launching worker instances to run your pipeline. In the future, workerZone will take precedence."]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentEl
{
    type O = BlockAssignable < DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentEl
{}
impl BuildDataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentEl { pub fn build (self) -> DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentEl { DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentEl { additional_experiments : core :: default :: Default :: default () , additional_user_labels : core :: default :: Default :: default () , bypass_temp_dir_validation : core :: default :: Default :: default () , enable_streaming_engine : core :: default :: Default :: default () , ip_configuration : core :: default :: Default :: default () , kms_key_name : core :: default :: Default :: default () , machine_type : core :: default :: Default :: default () , max_workers : core :: default :: Default :: default () , network : core :: default :: Default :: default () , num_workers : core :: default :: Default :: default () , service_account_email : core :: default :: Default :: default () , subnetwork : core :: default :: Default :: default () , temp_location : core :: default :: Default :: default () , worker_region : core :: default :: Default :: default () , worker_zone : core :: default :: Default :: default () , zone : core :: default :: Default :: default () , } } }
pub struct DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentElRef { fn new (shared : StackShared , base : String) -> DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentElRef { DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentElRef { shared : shared , base : base . to_string () , } } }
impl
    DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_experiments` after provisioning.\nAdditional experiment flags for the job."]
    pub fn additional_experiments(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_experiments", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_user_labels` after provisioning.\nAdditional user labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. An object containing a list of key/value pairs.\n'Example: { \"name\": \"wrench\", \"mass\": \"1kg\", \"count\": \"3\" }.'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn additional_user_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.additional_user_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bypass_temp_dir_validation` after provisioning.\nWhether to bypass the safety checks for the job's temporary directory. Use with caution."]
    pub fn bypass_temp_dir_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bypass_temp_dir_validation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_streaming_engine` after provisioning.\nWhether to enable Streaming Engine for the job."]
    pub fn enable_streaming_engine(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_streaming_engine", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_configuration` after provisioning.\nConfiguration for VM IPs.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#WorkerIPAddressConfiguration Possible values: [\"WORKER_IP_UNSPECIFIED\", \"WORKER_IP_PUBLIC\", \"WORKER_IP_PRIVATE\"]"]
    pub fn ip_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n'Name for the Cloud KMS key for the job. The key format is: projects//locations//keyRings//cryptoKeys/'"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to use for the job. Defaults to the value from the template if not specified."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `max_workers` after provisioning.\nThe maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000."]
    pub fn max_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_workers", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nNetwork to which VMs will be assigned. If empty or unspecified, the service will use the network \"default\"."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `num_workers` after provisioning.\nThe initial number of Compute Engine instances for the job."]
    pub fn num_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_workers", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nThe email address of the service account to run the job as."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nSubnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form \"https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK\" or \"regions/REGION/subnetworks/SUBNETWORK\". If the subnetwork is located in a Shared VPC network, you must use the complete URL."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `temp_location` after provisioning.\nThe Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://."]
    pub fn temp_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.temp_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `worker_region` after provisioning.\nThe Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1\". Mutually exclusive with workerZone. If neither workerRegion nor workerZone is specified, default to the control plane's region."]
    pub fn worker_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.worker_region", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `worker_zone` after provisioning.\nThe Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1-a\". Mutually exclusive with workerRegion. If neither workerRegion nor workerZone is specified, a zone in the control plane's region is chosen based on available capacity. If both workerZone and zone are set, workerZone takes precedence."]
    pub fn worker_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_zone", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe Compute Engine availability zone for launching worker instances to run your pipeline. In the future, workerZone will take precedence."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElDynamic { environment : Option < DynamicBlock < DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentEl >> , }
#[derive(Serialize)]
pub struct DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersEl { job_name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] parameters : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] transform_name_mapping : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] update : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] environment : Option < Vec < DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentEl > > , dynamic : DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElDynamic , }
impl DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersEl {
    #[doc = "Set the field `parameters`.\nThe runtime parameters to pass to the job.\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `transform_name_mapping`.\nMap of transform name prefixes of the job to be replaced to the corresponding name prefixes of the new job. Only applicable when updating a pipeline.\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_transform_name_mapping(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.transform_name_mapping = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nIf set, replace the existing pipeline with the name specified by jobName with this pipeline, preserving state."]
    pub fn set_update(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.update = Some(v.into());
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v : impl Into < BlockAssignable < DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.environment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersEl
{
    type O = BlockAssignable<
        DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersEl {
    #[doc = "The job name to use for the created job."]
    pub job_name: PrimField<String>,
}
impl BuildDataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersEl {
    pub fn build(
        self,
    ) -> DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersEl {
        DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersEl {
            job_name: self.job_name,
            parameters: core::default::Default::default(),
            transform_name_mapping: core::default::Default::default(),
            update: core::default::Default::default(),
            environment: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElRef {
        DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `job_name` after provisioning.\nThe job name to use for the created job."]
    pub fn job_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.base))
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\nThe runtime parameters to pass to the job.\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
    #[doc = "Get a reference to the value of field `transform_name_mapping` after provisioning.\nMap of transform name prefixes of the job to be replaced to the corresponding name prefixes of the new job. Only applicable when updating a pipeline.\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn transform_name_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.transform_name_mapping", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nIf set, replace the existing pipeline with the name specified by jobName with this pipeline, preserving state."]
    pub fn update(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]    pub fn environment (& self) -> ListRef < DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElEnvironmentElRef >{
        ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElDynamic {
    launch_parameters: Option<
        DynamicBlock<
            DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    project_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validate_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_parameters: Option<
        Vec<DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersEl>,
    >,
    dynamic: DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElDynamic,
}
impl DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestEl {
    #[doc = "Set the field `gcs_path`.\nA Cloud Storage path to the template from which to create the job. Must be a valid Cloud Storage URL, beginning with 'gs://'."]
    pub fn set_gcs_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcs_path = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe regional endpoint to which to direct the request."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `validate_only`.\n"]
    pub fn set_validate_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.validate_only = Some(v.into());
        self
    }
    #[doc = "Set the field `launch_parameters`.\n"]
    pub fn set_launch_parameters(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.launch_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_parameters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestEl {
    type O = BlockAssignable<DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestEl {
    #[doc = "The ID of the Cloud Platform project that the job belongs to."]
    pub project_id: PrimField<String>,
}
impl BuildDataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestEl {
    pub fn build(self) -> DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestEl {
        DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestEl {
            gcs_path: core::default::Default::default(),
            location: core::default::Default::default(),
            project_id: self.project_id,
            validate_only: core::default::Default::default(),
            launch_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElRef {
        DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcs_path` after provisioning.\nA Cloud Storage path to the template from which to create the job. Must be a valid Cloud Storage URL, beginning with 'gs://'."]
    pub fn gcs_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcs_path", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe regional endpoint to which to direct the request."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the Cloud Platform project that the job belongs to."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `validate_only` after provisioning.\n"]
    pub fn validate_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.validate_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `launch_parameters` after provisioning.\n"]
    pub fn launch_parameters(
        &self,
    ) -> ListRef<DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElLaunchParametersElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_parameters", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataPipelinePipelineWorkloadElDynamic {
    dataflow_flex_template_request:
        Option<DynamicBlock<DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestEl>>,
    dataflow_launch_template_request:
        Option<DynamicBlock<DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestEl>>,
}
#[derive(Serialize)]
pub struct DataPipelinePipelineWorkloadEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataflow_flex_template_request:
        Option<Vec<DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataflow_launch_template_request:
        Option<Vec<DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestEl>>,
    dynamic: DataPipelinePipelineWorkloadElDynamic,
}
impl DataPipelinePipelineWorkloadEl {
    #[doc = "Set the field `dataflow_flex_template_request`.\n"]
    pub fn set_dataflow_flex_template_request(
        mut self,
        v: impl Into<BlockAssignable<DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dataflow_flex_template_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dataflow_flex_template_request = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dataflow_launch_template_request`.\n"]
    pub fn set_dataflow_launch_template_request(
        mut self,
        v: impl Into<BlockAssignable<DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dataflow_launch_template_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dataflow_launch_template_request = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataPipelinePipelineWorkloadEl {
    type O = BlockAssignable<DataPipelinePipelineWorkloadEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataPipelinePipelineWorkloadEl {}
impl BuildDataPipelinePipelineWorkloadEl {
    pub fn build(self) -> DataPipelinePipelineWorkloadEl {
        DataPipelinePipelineWorkloadEl {
            dataflow_flex_template_request: core::default::Default::default(),
            dataflow_launch_template_request: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataPipelinePipelineWorkloadElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataPipelinePipelineWorkloadElRef {
    fn new(shared: StackShared, base: String) -> DataPipelinePipelineWorkloadElRef {
        DataPipelinePipelineWorkloadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataPipelinePipelineWorkloadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataflow_flex_template_request` after provisioning.\n"]
    pub fn dataflow_flex_template_request(
        &self,
    ) -> ListRef<DataPipelinePipelineWorkloadElDataflowFlexTemplateRequestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataflow_flex_template_request", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dataflow_launch_template_request` after provisioning.\n"]
    pub fn dataflow_launch_template_request(
        &self,
    ) -> ListRef<DataPipelinePipelineWorkloadElDataflowLaunchTemplateRequestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataflow_launch_template_request", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataPipelinePipelineDynamic {
    schedule_info: Option<DynamicBlock<DataPipelinePipelineScheduleInfoEl>>,
    workload: Option<DynamicBlock<DataPipelinePipelineWorkloadEl>>,
}
