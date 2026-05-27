use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ColabScheduleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_queueing: Option<PrimField<bool>>,
    cron: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_state: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    max_concurrent_run_count: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_run_count: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_notebook_execution_job_request:
        Option<Vec<ColabScheduleCreateNotebookExecutionJobRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ColabScheduleTimeoutsEl>,
    dynamic: ColabScheduleDynamic,
}
struct ColabSchedule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ColabScheduleData>,
}
#[derive(Clone)]
pub struct ColabSchedule(Rc<ColabSchedule_>);
impl ColabSchedule {
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
    #[doc = "Set the field `allow_queueing`.\nWhether new scheduled runs can be queued when max_concurrent_runs limit is reached. If set to true, new runs will be queued instead of skipped. Default to false."]
    pub fn set_allow_queueing(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_queueing = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `desired_state`.\nDesired state of the Colab Schedule. Set this field to 'ACTIVE' to start/resume the schedule, and 'PAUSED' to pause the schedule."]
    pub fn set_desired_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().desired_state = Some(v.into());
        self
    }
    #[doc = "Set the field `end_time`.\nTimestamp after which no new runs can be scheduled. If specified, the schedule will be completed when either end_time is reached or when scheduled_run_count >= max_run_count. Must be in the RFC 3339 (https://www.ietf.org/rfc/rfc3339.txt) format."]
    pub fn set_end_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `max_run_count`.\nMaximum run count of the schedule. If specified, The schedule will be completed when either startedRunCount >= maxRunCount or when endTime is reached. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified."]
    pub fn set_max_run_count(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().max_run_count = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\nThe timestamp after which the first run can be scheduled. Defaults to the schedule creation time. Must be in the RFC 3339 (https://www.ietf.org/rfc/rfc3339.txt) format."]
    pub fn set_start_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `create_notebook_execution_job_request`.\n"]
    pub fn set_create_notebook_execution_job_request(
        self,
        v: impl Into<BlockAssignable<ColabScheduleCreateNotebookExecutionJobRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .create_notebook_execution_job_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .create_notebook_execution_job_request = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ColabScheduleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `allow_queueing` after provisioning.\nWhether new scheduled runs can be queued when max_concurrent_runs limit is reached. If set to true, new runs will be queued instead of skipped. Default to false."]
    pub fn allow_queueing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_queueing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cron` after provisioning.\nCron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs."]
    pub fn cron(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cron", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Colab Schedule. Set this field to 'ACTIVE' to start/resume the schedule, and 'PAUSED' to pause the schedule."]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Schedule."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nTimestamp after which no new runs can be scheduled. If specified, the schedule will be completed when either end_time is reached or when scheduled_run_count >= max_run_count. Must be in the RFC 3339 (https://www.ietf.org/rfc/rfc3339.txt) format."]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_concurrent_run_count` after provisioning.\nMaximum number of runs that can be started concurrently for this Schedule. This is the limit for starting the scheduled requests and not the execution of the notebook execution jobs created by the requests."]
    pub fn max_concurrent_run_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_run_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_run_count` after provisioning.\nMaximum run count of the schedule. If specified, The schedule will be completed when either startedRunCount >= maxRunCount or when endTime is reached. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified."]
    pub fn max_run_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_run_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Schedule"]
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
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThe timestamp after which the first run can be scheduled. Defaults to the schedule creation time. Must be in the RFC 3339 (https://www.ietf.org/rfc/rfc3339.txt) format."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the schedule."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_notebook_execution_job_request` after provisioning.\n"]
    pub fn create_notebook_execution_job_request(
        &self,
    ) -> ListRef<ColabScheduleCreateNotebookExecutionJobRequestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.create_notebook_execution_job_request",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabScheduleTimeoutsElRef {
        ColabScheduleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ColabSchedule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ColabSchedule {}
impl ToListMappable for ColabSchedule {
    type O = ListRef<ColabScheduleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ColabSchedule_ {
    fn extract_resource_type(&self) -> String {
        "google_colab_schedule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildColabSchedule {
    pub tf_id: String,
    #[doc = "Cron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs."]
    pub cron: PrimField<String>,
    #[doc = "Required. The display name of the Schedule."]
    pub display_name: PrimField<String>,
    #[doc = "The location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub location: PrimField<String>,
    #[doc = "Maximum number of runs that can be started concurrently for this Schedule. This is the limit for starting the scheduled requests and not the execution of the notebook execution jobs created by the requests."]
    pub max_concurrent_run_count: PrimField<String>,
}
impl BuildColabSchedule {
    pub fn build(self, stack: &mut Stack) -> ColabSchedule {
        let out = ColabSchedule(Rc::new(ColabSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ColabScheduleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_queueing: core::default::Default::default(),
                cron: self.cron,
                deletion_policy: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                display_name: self.display_name,
                end_time: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                max_concurrent_run_count: self.max_concurrent_run_count,
                max_run_count: core::default::Default::default(),
                project: core::default::Default::default(),
                start_time: core::default::Default::default(),
                create_notebook_execution_job_request: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ColabScheduleRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabScheduleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ColabScheduleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_queueing` after provisioning.\nWhether new scheduled runs can be queued when max_concurrent_runs limit is reached. If set to true, new runs will be queued instead of skipped. Default to false."]
    pub fn allow_queueing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_queueing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cron` after provisioning.\nCron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs."]
    pub fn cron(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cron", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Colab Schedule. Set this field to 'ACTIVE' to start/resume the schedule, and 'PAUSED' to pause the schedule."]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Schedule."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nTimestamp after which no new runs can be scheduled. If specified, the schedule will be completed when either end_time is reached or when scheduled_run_count >= max_run_count. Must be in the RFC 3339 (https://www.ietf.org/rfc/rfc3339.txt) format."]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_concurrent_run_count` after provisioning.\nMaximum number of runs that can be started concurrently for this Schedule. This is the limit for starting the scheduled requests and not the execution of the notebook execution jobs created by the requests."]
    pub fn max_concurrent_run_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_run_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_run_count` after provisioning.\nMaximum run count of the schedule. If specified, The schedule will be completed when either startedRunCount >= maxRunCount or when endTime is reached. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified."]
    pub fn max_run_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_run_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Schedule"]
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
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThe timestamp after which the first run can be scheduled. Defaults to the schedule creation time. Must be in the RFC 3339 (https://www.ietf.org/rfc/rfc3339.txt) format."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the schedule."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_notebook_execution_job_request` after provisioning.\n"]
    pub fn create_notebook_execution_job_request(
        &self,
    ) -> ListRef<ColabScheduleCreateNotebookExecutionJobRequestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.create_notebook_execution_job_request",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabScheduleTimeoutsElRef {
        ColabScheduleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_sha: Option<PrimField<String>>,
    dataform_repository_resource_name: PrimField<String>,
}
impl
    ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceEl
{
    #[doc = "Set the field `commit_sha`.\nThe commit SHA to read repository with. If unset, the file will be read at HEAD."]
    pub fn set_commit_sha(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit_sha = Some(v.into());
        self
    }
}
impl ToListMappable for ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceEl { type O = BlockAssignable < ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceEl
{
    #[doc = "The resource name of the Dataform Repository."]
    pub dataform_repository_resource_name: PrimField<String>,
}
impl BuildColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceEl { pub fn build (self) -> ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceEl { ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceEl { commit_sha : core :: default :: Default :: default () , dataform_repository_resource_name : self . dataform_repository_resource_name , } } }
pub struct ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceElRef { fn new (shared : StackShared , base : String) -> ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceElRef { ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceElRef { shared : shared , base : base . to_string () , } } }
impl ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `commit_sha` after provisioning.\nThe commit SHA to read repository with. If unset, the file will be read at HEAD."] pub fn commit_sha (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.commit_sha" , self . base)) } # [doc = "Get a reference to the value of field `dataform_repository_resource_name` after provisioning.\nThe resource name of the Dataform Repository."] pub fn dataform_repository_resource_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.dataform_repository_resource_name" , self . base)) } }
#[derive(Serialize)]
pub struct ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceEl {
    #[doc = "Set the field `generation`.\nThe version of the Cloud Storage object to read. If unset, the current version of the object is read. See https://cloud.google.com/storage/docs/metadata#generation-number."]
    pub fn set_generation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.generation = Some(v.into());
        self
    }
}
impl ToListMappable
    for ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceEl
{
    type O = BlockAssignable<
        ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceEl
{
    #[doc = "The Cloud Storage uri pointing to the ipynb file. Format: gs://bucket/notebook_file.ipynb"]
    pub uri: PrimField<String>,
}
impl
    BuildColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceEl
{
    pub fn build(
        self,
    ) -> ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceEl
    {
        ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceEl {
            generation: core::default::Default::default(),
            uri: self.uri,
        }
    }
}
pub struct ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceElRef
    {
        ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceElRef { shared : shared , base : base . to_string () , }
    }
}
impl ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nThe version of the Cloud Storage object to read. If unset, the current version of the object is read. See https://cloud.google.com/storage/docs/metadata#generation-number."]
    pub fn generation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe Cloud Storage uri pointing to the ipynb file. Format: gs://bucket/notebook_file.ipynb"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDynamic { dataform_repository_source : Option < DynamicBlock < ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceEl >> , gcs_notebook_source : Option < DynamicBlock < ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceEl >> , }
#[derive(Serialize)]
pub struct ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobEl { display_name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] execution_timeout : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] execution_user : Option < PrimField < String > > , gcs_output_uri : PrimField < String > , notebook_runtime_template_resource_name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] service_account : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] dataform_repository_source : Option < Vec < ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceEl > > , # [serde (skip_serializing_if = "Option::is_none")] gcs_notebook_source : Option < Vec < ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceEl > > , dynamic : ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDynamic , }
impl ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobEl {
    #[doc = "Set the field `execution_timeout`.\nMax running time of the execution job in seconds (default 86400s / 24 hrs). A duration in seconds with up to nine fractional digits, ending with \"s\". Example: \"3.5s\"."]
    pub fn set_execution_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_user`.\nThe user email to run the execution as."]
    pub fn set_execution_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_user = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nThe service account to run the execution as."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `dataform_repository_source`.\n"]
    pub fn set_dataform_repository_source(
        mut self,
        v : impl Into < BlockAssignable < ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dataform_repository_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dataform_repository_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcs_notebook_source`.\n"]
    pub fn set_gcs_notebook_source(
        mut self,
        v : impl Into < BlockAssignable < ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gcs_notebook_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_notebook_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobEl {
    type O =
        BlockAssignable<ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobEl {
    #[doc = "Required. The display name of the Notebook Execution."]
    pub display_name: PrimField<String>,
    #[doc = "The Cloud Storage location to upload the result to. Format:'gs://bucket-name'"]
    pub gcs_output_uri: PrimField<String>,
    #[doc = "The NotebookRuntimeTemplate to source compute configuration from."]
    pub notebook_runtime_template_resource_name: PrimField<String>,
}
impl BuildColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobEl {
    pub fn build(self) -> ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobEl {
        ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobEl {
            display_name: self.display_name,
            execution_timeout: core::default::Default::default(),
            execution_user: core::default::Default::default(),
            gcs_output_uri: self.gcs_output_uri,
            notebook_runtime_template_resource_name: self.notebook_runtime_template_resource_name,
            service_account: core::default::Default::default(),
            dataform_repository_source: core::default::Default::default(),
            gcs_notebook_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElRef {
        ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Notebook Execution."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `execution_timeout` after provisioning.\nMax running time of the execution job in seconds (default 86400s / 24 hrs). A duration in seconds with up to nine fractional digits, ending with \"s\". Example: \"3.5s\"."]
    pub fn execution_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `execution_user` after provisioning.\nThe user email to run the execution as."]
    pub fn execution_user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_user", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_output_uri` after provisioning.\nThe Cloud Storage location to upload the result to. Format:'gs://bucket-name'"]
    pub fn gcs_output_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcs_output_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `notebook_runtime_template_resource_name` after provisioning.\nThe NotebookRuntimeTemplate to source compute configuration from."]
    pub fn notebook_runtime_template_resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebook_runtime_template_resource_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account to run the execution as."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dataform_repository_source` after provisioning.\n"]    pub fn dataform_repository_source (& self) -> ListRef < ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElDataformRepositorySourceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataform_repository_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_notebook_source` after provisioning.\n"]    pub fn gcs_notebook_source (& self) -> ListRef < ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElGcsNotebookSourceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_notebook_source", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ColabScheduleCreateNotebookExecutionJobRequestElDynamic {
    notebook_execution_job: Option<
        DynamicBlock<ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobEl>,
    >,
}
#[derive(Serialize)]
pub struct ColabScheduleCreateNotebookExecutionJobRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    notebook_execution_job:
        Option<Vec<ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobEl>>,
    dynamic: ColabScheduleCreateNotebookExecutionJobRequestElDynamic,
}
impl ColabScheduleCreateNotebookExecutionJobRequestEl {
    #[doc = "Set the field `notebook_execution_job`.\n"]
    pub fn set_notebook_execution_job(
        mut self,
        v: impl Into<
            BlockAssignable<ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.notebook_execution_job = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.notebook_execution_job = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ColabScheduleCreateNotebookExecutionJobRequestEl {
    type O = BlockAssignable<ColabScheduleCreateNotebookExecutionJobRequestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabScheduleCreateNotebookExecutionJobRequestEl {}
impl BuildColabScheduleCreateNotebookExecutionJobRequestEl {
    pub fn build(self) -> ColabScheduleCreateNotebookExecutionJobRequestEl {
        ColabScheduleCreateNotebookExecutionJobRequestEl {
            notebook_execution_job: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ColabScheduleCreateNotebookExecutionJobRequestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabScheduleCreateNotebookExecutionJobRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ColabScheduleCreateNotebookExecutionJobRequestElRef {
        ColabScheduleCreateNotebookExecutionJobRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabScheduleCreateNotebookExecutionJobRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `notebook_execution_job` after provisioning.\n"]
    pub fn notebook_execution_job(
        &self,
    ) -> ListRef<ColabScheduleCreateNotebookExecutionJobRequestElNotebookExecutionJobElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notebook_execution_job", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ColabScheduleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ColabScheduleTimeoutsEl {
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
impl ToListMappable for ColabScheduleTimeoutsEl {
    type O = BlockAssignable<ColabScheduleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabScheduleTimeoutsEl {}
impl BuildColabScheduleTimeoutsEl {
    pub fn build(self) -> ColabScheduleTimeoutsEl {
        ColabScheduleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ColabScheduleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabScheduleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ColabScheduleTimeoutsElRef {
        ColabScheduleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabScheduleTimeoutsElRef {
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
struct ColabScheduleDynamic {
    create_notebook_execution_job_request:
        Option<DynamicBlock<ColabScheduleCreateNotebookExecutionJobRequestEl>>,
}
