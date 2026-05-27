use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataplexTaskData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lake: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_spec: Option<Vec<DataplexTaskExecutionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notebook: Option<Vec<DataplexTaskNotebookEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark: Option<Vec<DataplexTaskSparkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataplexTaskTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_spec: Option<Vec<DataplexTaskTriggerSpecEl>>,
    dynamic: DataplexTaskDynamic,
}
struct DataplexTask_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataplexTaskData>,
}
#[derive(Clone)]
pub struct DataplexTask(Rc<DataplexTask_>);
impl DataplexTask {
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
    #[doc = "Set the field `description`.\nUser-provided description of the task."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser friendly display name."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-defined labels for the task.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `lake`.\nThe lake in which the task will be created in."]
    pub fn set_lake(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().lake = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location in which the task will be created in."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `task_id`.\nThe task Id of the task."]
    pub fn set_task_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().task_id = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_spec`.\n"]
    pub fn set_execution_spec(
        self,
        v: impl Into<BlockAssignable<DataplexTaskExecutionSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `notebook`.\n"]
    pub fn set_notebook(self, v: impl Into<BlockAssignable<DataplexTaskNotebookEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notebook = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notebook = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark`.\n"]
    pub fn set_spark(self, v: impl Into<BlockAssignable<DataplexTaskSparkEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataplexTaskTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `trigger_spec`.\n"]
    pub fn set_trigger_spec(
        self,
        v: impl Into<BlockAssignable<DataplexTaskTriggerSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trigger_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trigger_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the task was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the task."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser friendly display name."]
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
    #[doc = "Get a reference to the value of field `execution_status` after provisioning.\nConfiguration for the cluster"]
    pub fn execution_status(&self) -> ListRef<DataplexTaskExecutionStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the task.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lake` after provisioning.\nThe lake in which the task will be created in."]
    pub fn lake(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lake", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location in which the task will be created in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the task, of the form: projects/{project_number}/locations/{locationId}/lakes/{lakeId}/ tasks/{name}."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the task."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `task_id` after provisioning.\nThe task Id of the task."]
    pub fn task_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem generated globally unique ID for the task. This ID will be different if the task is deleted and re-created with the same name."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the task was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_spec` after provisioning.\n"]
    pub fn execution_spec(&self) -> ListRef<DataplexTaskExecutionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notebook` after provisioning.\n"]
    pub fn notebook(&self) -> ListRef<DataplexTaskNotebookElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notebook", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> ListRef<DataplexTaskSparkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexTaskTimeoutsElRef {
        DataplexTaskTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_spec` after provisioning.\n"]
    pub fn trigger_spec(&self) -> ListRef<DataplexTaskTriggerSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_spec", self.extract_ref()),
        )
    }
}
impl Referable for DataplexTask {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataplexTask {}
impl ToListMappable for DataplexTask {
    type O = ListRef<DataplexTaskRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataplexTask_ {
    fn extract_resource_type(&self) -> String {
        "google_dataplex_task".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataplexTask {
    pub tf_id: String,
}
impl BuildDataplexTask {
    pub fn build(self, stack: &mut Stack) -> DataplexTask {
        let out = DataplexTask(Rc::new(DataplexTask_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataplexTaskData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                lake: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                task_id: core::default::Default::default(),
                execution_spec: core::default::Default::default(),
                notebook: core::default::Default::default(),
                spark: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                trigger_spec: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataplexTaskRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataplexTaskRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the task was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the task."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser friendly display name."]
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
    #[doc = "Get a reference to the value of field `execution_status` after provisioning.\nConfiguration for the cluster"]
    pub fn execution_status(&self) -> ListRef<DataplexTaskExecutionStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the task.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lake` after provisioning.\nThe lake in which the task will be created in."]
    pub fn lake(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lake", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location in which the task will be created in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the task, of the form: projects/{project_number}/locations/{locationId}/lakes/{lakeId}/ tasks/{name}."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the task."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `task_id` after provisioning.\nThe task Id of the task."]
    pub fn task_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem generated globally unique ID for the task. This ID will be different if the task is deleted and re-created with the same name."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the task was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_spec` after provisioning.\n"]
    pub fn execution_spec(&self) -> ListRef<DataplexTaskExecutionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notebook` after provisioning.\n"]
    pub fn notebook(&self) -> ListRef<DataplexTaskNotebookElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notebook", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> ListRef<DataplexTaskSparkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexTaskTimeoutsElRef {
        DataplexTaskTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_spec` after provisioning.\n"]
    pub fn trigger_spec(&self) -> ListRef<DataplexTaskTriggerSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_spec", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexTaskExecutionStatusElLatestJobEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_job: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
}
impl DataplexTaskExecutionStatusElLatestJobEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_count`.\n"]
    pub fn set_retry_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retry_count = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
    #[doc = "Set the field `service_job`.\n"]
    pub fn set_service_job(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_job = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexTaskExecutionStatusElLatestJobEl {
    type O = BlockAssignable<DataplexTaskExecutionStatusElLatestJobEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskExecutionStatusElLatestJobEl {}
impl BuildDataplexTaskExecutionStatusElLatestJobEl {
    pub fn build(self) -> DataplexTaskExecutionStatusElLatestJobEl {
        DataplexTaskExecutionStatusElLatestJobEl {
            end_time: core::default::Default::default(),
            message: core::default::Default::default(),
            name: core::default::Default::default(),
            retry_count: core::default::Default::default(),
            service: core::default::Default::default(),
            service_job: core::default::Default::default(),
            start_time: core::default::Default::default(),
            state: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}
pub struct DataplexTaskExecutionStatusElLatestJobElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskExecutionStatusElLatestJobElRef {
    fn new(shared: StackShared, base: String) -> DataplexTaskExecutionStatusElLatestJobElRef {
        DataplexTaskExecutionStatusElLatestJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskExecutionStatusElLatestJobElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `retry_count` after provisioning.\n"]
    pub fn retry_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry_count", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `service_job` after provisioning.\n"]
    pub fn service_job(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_job", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexTaskExecutionStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    latest_job: Option<ListField<DataplexTaskExecutionStatusElLatestJobEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataplexTaskExecutionStatusEl {
    #[doc = "Set the field `latest_job`.\n"]
    pub fn set_latest_job(
        mut self,
        v: impl Into<ListField<DataplexTaskExecutionStatusElLatestJobEl>>,
    ) -> Self {
        self.latest_job = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexTaskExecutionStatusEl {
    type O = BlockAssignable<DataplexTaskExecutionStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskExecutionStatusEl {}
impl BuildDataplexTaskExecutionStatusEl {
    pub fn build(self) -> DataplexTaskExecutionStatusEl {
        DataplexTaskExecutionStatusEl {
            latest_job: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataplexTaskExecutionStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskExecutionStatusElRef {
    fn new(shared: StackShared, base: String) -> DataplexTaskExecutionStatusElRef {
        DataplexTaskExecutionStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskExecutionStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `latest_job` after provisioning.\n"]
    pub fn latest_job(&self) -> ListRef<DataplexTaskExecutionStatusElLatestJobElRef> {
        ListRef::new(self.shared().clone(), format!("{}.latest_job", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexTaskExecutionSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_job_execution_lifetime: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    service_account: PrimField<String>,
}
impl DataplexTaskExecutionSpecEl {
    #[doc = "Set the field `args`.\nThe arguments to pass to the task. The args can use placeholders of the format ${placeholder} as part of key/value string. These will be interpolated before passing the args to the driver. Currently supported placeholders: - ${taskId} - ${job_time} To pass positional args, set the key as TASK_ARGS. The value should be a comma-separated string of all the positional arguments. To use a delimiter other than comma, refer to https://cloud.google.com/sdk/gcloud/reference/topic/escaping. In case of other keys being present in the args, then TASK_ARGS will be passed as the last argument. An object containing a list of 'key': value pairs. Example: { 'name': 'wrench', 'mass': '1.3kg', 'count': '3' }."]
    pub fn set_args(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\nThe Cloud KMS key to use for encryption, of the form: projects/{project_number}/locations/{locationId}/keyRings/{key-ring-name}/cryptoKeys/{key-name}."]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `max_job_execution_lifetime`.\nThe maximum duration after which the job execution is expired. A duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'."]
    pub fn set_max_job_execution_lifetime(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_job_execution_lifetime = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project in which jobs are run. By default, the project containing the Lake is used. If a project is provided, the ExecutionSpec.service_account must belong to this project."]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexTaskExecutionSpecEl {
    type O = BlockAssignable<DataplexTaskExecutionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskExecutionSpecEl {
    #[doc = "Service account to use to execute a task. If not provided, the default Compute service account for the project is used."]
    pub service_account: PrimField<String>,
}
impl BuildDataplexTaskExecutionSpecEl {
    pub fn build(self) -> DataplexTaskExecutionSpecEl {
        DataplexTaskExecutionSpecEl {
            args: core::default::Default::default(),
            kms_key: core::default::Default::default(),
            max_job_execution_lifetime: core::default::Default::default(),
            project: core::default::Default::default(),
            service_account: self.service_account,
        }
    }
}
pub struct DataplexTaskExecutionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskExecutionSpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexTaskExecutionSpecElRef {
        DataplexTaskExecutionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskExecutionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the task. The args can use placeholders of the format ${placeholder} as part of key/value string. These will be interpolated before passing the args to the driver. Currently supported placeholders: - ${taskId} - ${job_time} To pass positional args, set the key as TASK_ARGS. The value should be a comma-separated string of all the positional arguments. To use a delimiter other than comma, refer to https://cloud.google.com/sdk/gcloud/reference/topic/escaping. In case of other keys being present in the args, then TASK_ARGS will be passed as the last argument. An object containing a list of 'key': value pairs. Example: { 'name': 'wrench', 'mass': '1.3kg', 'count': '3' }."]
    pub fn args(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS key to use for encryption, of the form: projects/{project_number}/locations/{locationId}/keyRings/{key-ring-name}/cryptoKeys/{key-name}."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
    #[doc = "Get a reference to the value of field `max_job_execution_lifetime` after provisioning.\nThe maximum duration after which the job execution is expired. A duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'."]
    pub fn max_job_execution_lifetime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_job_execution_lifetime", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project in which jobs are run. By default, the project containing the Lake is used. If a project is provided, the ExecutionSpec.service_account must belong to this project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nService account to use to execute a task. If not provided, the default Compute service account for the project is used."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexTaskNotebookElInfrastructureSpecElBatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    executors_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_executors_count: Option<PrimField<f64>>,
}
impl DataplexTaskNotebookElInfrastructureSpecElBatchEl {
    #[doc = "Set the field `executors_count`.\nTotal number of job executors. Executor Count should be between 2 and 100. [Default=2]"]
    pub fn set_executors_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.executors_count = Some(v.into());
        self
    }
    #[doc = "Set the field `max_executors_count`.\nMax configurable executors. If maxExecutorsCount > executorsCount, then auto-scaling is enabled. Max Executor Count should be between 2 and 1000. [Default=1000]"]
    pub fn set_max_executors_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_executors_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexTaskNotebookElInfrastructureSpecElBatchEl {
    type O = BlockAssignable<DataplexTaskNotebookElInfrastructureSpecElBatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskNotebookElInfrastructureSpecElBatchEl {}
impl BuildDataplexTaskNotebookElInfrastructureSpecElBatchEl {
    pub fn build(self) -> DataplexTaskNotebookElInfrastructureSpecElBatchEl {
        DataplexTaskNotebookElInfrastructureSpecElBatchEl {
            executors_count: core::default::Default::default(),
            max_executors_count: core::default::Default::default(),
        }
    }
}
pub struct DataplexTaskNotebookElInfrastructureSpecElBatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskNotebookElInfrastructureSpecElBatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexTaskNotebookElInfrastructureSpecElBatchElRef {
        DataplexTaskNotebookElInfrastructureSpecElBatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskNotebookElInfrastructureSpecElBatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `executors_count` after provisioning.\nTotal number of job executors. Executor Count should be between 2 and 100. [Default=2]"]
    pub fn executors_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.executors_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_executors_count` after provisioning.\nMax configurable executors. If maxExecutorsCount > executorsCount, then auto-scaling is enabled. Max Executor Count should be between 2 and 1000. [Default=1000]"]
    pub fn max_executors_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_executors_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexTaskNotebookElInfrastructureSpecElContainerImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    java_jars: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_packages: Option<ListField<PrimField<String>>>,
}
impl DataplexTaskNotebookElInfrastructureSpecElContainerImageEl {
    #[doc = "Set the field `image`.\nContainer image to use."]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }
    #[doc = "Set the field `java_jars`.\nA list of Java JARS to add to the classpath. Valid input includes Cloud Storage URIs to Jar binaries. For example, gs://bucket-name/my/path/to/file.jar"]
    pub fn set_java_jars(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.java_jars = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nOverride to common configuration of open source components installed on the Dataproc cluster. The properties to set on daemon config files. Property keys are specified in prefix:property format, for example core:hadoop.tmp.dir. For more information, see Cluster properties."]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `python_packages`.\nA list of python packages to be installed. Valid formats include Cloud Storage URI to a PIP installable library. For example, gs://bucket-name/my/path/to/lib.tar.gz"]
    pub fn set_python_packages(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.python_packages = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexTaskNotebookElInfrastructureSpecElContainerImageEl {
    type O = BlockAssignable<DataplexTaskNotebookElInfrastructureSpecElContainerImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskNotebookElInfrastructureSpecElContainerImageEl {}
impl BuildDataplexTaskNotebookElInfrastructureSpecElContainerImageEl {
    pub fn build(self) -> DataplexTaskNotebookElInfrastructureSpecElContainerImageEl {
        DataplexTaskNotebookElInfrastructureSpecElContainerImageEl {
            image: core::default::Default::default(),
            java_jars: core::default::Default::default(),
            properties: core::default::Default::default(),
            python_packages: core::default::Default::default(),
        }
    }
}
pub struct DataplexTaskNotebookElInfrastructureSpecElContainerImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskNotebookElInfrastructureSpecElContainerImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexTaskNotebookElInfrastructureSpecElContainerImageElRef {
        DataplexTaskNotebookElInfrastructureSpecElContainerImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskNotebookElInfrastructureSpecElContainerImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\nContainer image to use."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `java_jars` after provisioning.\nA list of Java JARS to add to the classpath. Valid input includes Cloud Storage URIs to Jar binaries. For example, gs://bucket-name/my/path/to/file.jar"]
    pub fn java_jars(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.java_jars", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nOverride to common configuration of open source components installed on the Dataproc cluster. The properties to set on daemon config files. Property keys are specified in prefix:property format, for example core:hadoop.tmp.dir. For more information, see Cluster properties."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `python_packages` after provisioning.\nA list of python packages to be installed. Valid formats include Cloud Storage URI to a PIP installable library. For example, gs://bucket-name/my/path/to/lib.tar.gz"]
    pub fn python_packages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.python_packages", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_network: Option<PrimField<String>>,
}
impl DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {
    #[doc = "Set the field `network`.\nThe Cloud VPC network in which the job is run. By default, the Cloud VPC network named Default within the project is used."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tags`.\nList of network tags to apply to the job."]
    pub fn set_network_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.network_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `sub_network`.\nThe Cloud VPC sub-network in which the job is run."]
    pub fn set_sub_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sub_network = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {
    type O = BlockAssignable<DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {}
impl BuildDataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {
    pub fn build(self) -> DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {
        DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {
            network: core::default::Default::default(),
            network_tags: core::default::Default::default(),
            sub_network: core::default::Default::default(),
        }
    }
}
pub struct DataplexTaskNotebookElInfrastructureSpecElVpcNetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskNotebookElInfrastructureSpecElVpcNetworkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexTaskNotebookElInfrastructureSpecElVpcNetworkElRef {
        DataplexTaskNotebookElInfrastructureSpecElVpcNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskNotebookElInfrastructureSpecElVpcNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe Cloud VPC network in which the job is run. By default, the Cloud VPC network named Default within the project is used."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `network_tags` after provisioning.\nList of network tags to apply to the job."]
    pub fn network_tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }
    #[doc = "Get a reference to the value of field `sub_network` after provisioning.\nThe Cloud VPC sub-network in which the job is run."]
    pub fn sub_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sub_network", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexTaskNotebookElInfrastructureSpecElDynamic {
    batch: Option<DynamicBlock<DataplexTaskNotebookElInfrastructureSpecElBatchEl>>,
    container_image:
        Option<DynamicBlock<DataplexTaskNotebookElInfrastructureSpecElContainerImageEl>>,
    vpc_network: Option<DynamicBlock<DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl>>,
}
#[derive(Serialize)]
pub struct DataplexTaskNotebookElInfrastructureSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch: Option<Vec<DataplexTaskNotebookElInfrastructureSpecElBatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_image: Option<Vec<DataplexTaskNotebookElInfrastructureSpecElContainerImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_network: Option<Vec<DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl>>,
    dynamic: DataplexTaskNotebookElInfrastructureSpecElDynamic,
}
impl DataplexTaskNotebookElInfrastructureSpecEl {
    #[doc = "Set the field `batch`.\n"]
    pub fn set_batch(
        mut self,
        v: impl Into<BlockAssignable<DataplexTaskNotebookElInfrastructureSpecElBatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.batch = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.batch = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `container_image`.\n"]
    pub fn set_container_image(
        mut self,
        v: impl Into<BlockAssignable<DataplexTaskNotebookElInfrastructureSpecElContainerImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_image = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_network`.\n"]
    pub fn set_vpc_network(
        mut self,
        v: impl Into<BlockAssignable<DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_network = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_network = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexTaskNotebookElInfrastructureSpecEl {
    type O = BlockAssignable<DataplexTaskNotebookElInfrastructureSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskNotebookElInfrastructureSpecEl {}
impl BuildDataplexTaskNotebookElInfrastructureSpecEl {
    pub fn build(self) -> DataplexTaskNotebookElInfrastructureSpecEl {
        DataplexTaskNotebookElInfrastructureSpecEl {
            batch: core::default::Default::default(),
            container_image: core::default::Default::default(),
            vpc_network: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexTaskNotebookElInfrastructureSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskNotebookElInfrastructureSpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexTaskNotebookElInfrastructureSpecElRef {
        DataplexTaskNotebookElInfrastructureSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskNotebookElInfrastructureSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch` after provisioning.\n"]
    pub fn batch(&self) -> ListRef<DataplexTaskNotebookElInfrastructureSpecElBatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.batch", self.base))
    }
    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(
        &self,
    ) -> ListRef<DataplexTaskNotebookElInfrastructureSpecElContainerImageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_image", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_network` after provisioning.\n"]
    pub fn vpc_network(
        &self,
    ) -> ListRef<DataplexTaskNotebookElInfrastructureSpecElVpcNetworkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_network", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexTaskNotebookElDynamic {
    infrastructure_spec: Option<DynamicBlock<DataplexTaskNotebookElInfrastructureSpecEl>>,
}
#[derive(Serialize)]
pub struct DataplexTaskNotebookEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_uris: Option<ListField<PrimField<String>>>,
    notebook: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    infrastructure_spec: Option<Vec<DataplexTaskNotebookElInfrastructureSpecEl>>,
    dynamic: DataplexTaskNotebookElDynamic,
}
impl DataplexTaskNotebookEl {
    #[doc = "Set the field `archive_uris`.\nCloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.archive_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `file_uris`.\nCloud Storage URIs of files to be placed in the working directory of each executor."]
    pub fn set_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `infrastructure_spec`.\n"]
    pub fn set_infrastructure_spec(
        mut self,
        v: impl Into<BlockAssignable<DataplexTaskNotebookElInfrastructureSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.infrastructure_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.infrastructure_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexTaskNotebookEl {
    type O = BlockAssignable<DataplexTaskNotebookEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskNotebookEl {
    #[doc = "Path to input notebook. This can be the Cloud Storage URI of the notebook file or the path to a Notebook Content. The execution args are accessible as environment variables (TASK_key=value)."]
    pub notebook: PrimField<String>,
}
impl BuildDataplexTaskNotebookEl {
    pub fn build(self) -> DataplexTaskNotebookEl {
        DataplexTaskNotebookEl {
            archive_uris: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            notebook: self.notebook,
            infrastructure_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexTaskNotebookElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskNotebookElRef {
    fn new(shared: StackShared, base: String) -> DataplexTaskNotebookElRef {
        DataplexTaskNotebookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskNotebookElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_uris` after provisioning.\nCloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `file_uris` after provisioning.\nCloud Storage URIs of files to be placed in the working directory of each executor."]
    pub fn file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `notebook` after provisioning.\nPath to input notebook. This can be the Cloud Storage URI of the notebook file or the path to a Notebook Content. The execution args are accessible as environment variables (TASK_key=value)."]
    pub fn notebook(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notebook", self.base))
    }
    #[doc = "Get a reference to the value of field `infrastructure_spec` after provisioning.\n"]
    pub fn infrastructure_spec(&self) -> ListRef<DataplexTaskNotebookElInfrastructureSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.infrastructure_spec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexTaskSparkElInfrastructureSpecElBatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    executors_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_executors_count: Option<PrimField<f64>>,
}
impl DataplexTaskSparkElInfrastructureSpecElBatchEl {
    #[doc = "Set the field `executors_count`.\nTotal number of job executors. Executor Count should be between 2 and 100. [Default=2]"]
    pub fn set_executors_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.executors_count = Some(v.into());
        self
    }
    #[doc = "Set the field `max_executors_count`.\nMax configurable executors. If maxExecutorsCount > executorsCount, then auto-scaling is enabled. Max Executor Count should be between 2 and 1000. [Default=1000]"]
    pub fn set_max_executors_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_executors_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexTaskSparkElInfrastructureSpecElBatchEl {
    type O = BlockAssignable<DataplexTaskSparkElInfrastructureSpecElBatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskSparkElInfrastructureSpecElBatchEl {}
impl BuildDataplexTaskSparkElInfrastructureSpecElBatchEl {
    pub fn build(self) -> DataplexTaskSparkElInfrastructureSpecElBatchEl {
        DataplexTaskSparkElInfrastructureSpecElBatchEl {
            executors_count: core::default::Default::default(),
            max_executors_count: core::default::Default::default(),
        }
    }
}
pub struct DataplexTaskSparkElInfrastructureSpecElBatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskSparkElInfrastructureSpecElBatchElRef {
    fn new(shared: StackShared, base: String) -> DataplexTaskSparkElInfrastructureSpecElBatchElRef {
        DataplexTaskSparkElInfrastructureSpecElBatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskSparkElInfrastructureSpecElBatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `executors_count` after provisioning.\nTotal number of job executors. Executor Count should be between 2 and 100. [Default=2]"]
    pub fn executors_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.executors_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_executors_count` after provisioning.\nMax configurable executors. If maxExecutorsCount > executorsCount, then auto-scaling is enabled. Max Executor Count should be between 2 and 1000. [Default=1000]"]
    pub fn max_executors_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_executors_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexTaskSparkElInfrastructureSpecElContainerImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    java_jars: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_packages: Option<ListField<PrimField<String>>>,
}
impl DataplexTaskSparkElInfrastructureSpecElContainerImageEl {
    #[doc = "Set the field `image`.\nContainer image to use."]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }
    #[doc = "Set the field `java_jars`.\nA list of Java JARS to add to the classpath. Valid input includes Cloud Storage URIs to Jar binaries. For example, gs://bucket-name/my/path/to/file.jar"]
    pub fn set_java_jars(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.java_jars = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nOverride to common configuration of open source components installed on the Dataproc cluster. The properties to set on daemon config files. Property keys are specified in prefix:property format, for example core:hadoop.tmp.dir. For more information, see Cluster properties."]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `python_packages`.\nA list of python packages to be installed. Valid formats include Cloud Storage URI to a PIP installable library. For example, gs://bucket-name/my/path/to/lib.tar.gz"]
    pub fn set_python_packages(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.python_packages = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexTaskSparkElInfrastructureSpecElContainerImageEl {
    type O = BlockAssignable<DataplexTaskSparkElInfrastructureSpecElContainerImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskSparkElInfrastructureSpecElContainerImageEl {}
impl BuildDataplexTaskSparkElInfrastructureSpecElContainerImageEl {
    pub fn build(self) -> DataplexTaskSparkElInfrastructureSpecElContainerImageEl {
        DataplexTaskSparkElInfrastructureSpecElContainerImageEl {
            image: core::default::Default::default(),
            java_jars: core::default::Default::default(),
            properties: core::default::Default::default(),
            python_packages: core::default::Default::default(),
        }
    }
}
pub struct DataplexTaskSparkElInfrastructureSpecElContainerImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskSparkElInfrastructureSpecElContainerImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexTaskSparkElInfrastructureSpecElContainerImageElRef {
        DataplexTaskSparkElInfrastructureSpecElContainerImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskSparkElInfrastructureSpecElContainerImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\nContainer image to use."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `java_jars` after provisioning.\nA list of Java JARS to add to the classpath. Valid input includes Cloud Storage URIs to Jar binaries. For example, gs://bucket-name/my/path/to/file.jar"]
    pub fn java_jars(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.java_jars", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nOverride to common configuration of open source components installed on the Dataproc cluster. The properties to set on daemon config files. Property keys are specified in prefix:property format, for example core:hadoop.tmp.dir. For more information, see Cluster properties."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `python_packages` after provisioning.\nA list of python packages to be installed. Valid formats include Cloud Storage URI to a PIP installable library. For example, gs://bucket-name/my/path/to/lib.tar.gz"]
    pub fn python_packages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.python_packages", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_network: Option<PrimField<String>>,
}
impl DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {
    #[doc = "Set the field `network`.\nThe Cloud VPC network in which the job is run. By default, the Cloud VPC network named Default within the project is used."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tags`.\nList of network tags to apply to the job."]
    pub fn set_network_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.network_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `sub_network`.\nThe Cloud VPC sub-network in which the job is run."]
    pub fn set_sub_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sub_network = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {
    type O = BlockAssignable<DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {}
impl BuildDataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {
    pub fn build(self) -> DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {
        DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {
            network: core::default::Default::default(),
            network_tags: core::default::Default::default(),
            sub_network: core::default::Default::default(),
        }
    }
}
pub struct DataplexTaskSparkElInfrastructureSpecElVpcNetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskSparkElInfrastructureSpecElVpcNetworkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataplexTaskSparkElInfrastructureSpecElVpcNetworkElRef {
        DataplexTaskSparkElInfrastructureSpecElVpcNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskSparkElInfrastructureSpecElVpcNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe Cloud VPC network in which the job is run. By default, the Cloud VPC network named Default within the project is used."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `network_tags` after provisioning.\nList of network tags to apply to the job."]
    pub fn network_tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }
    #[doc = "Get a reference to the value of field `sub_network` after provisioning.\nThe Cloud VPC sub-network in which the job is run."]
    pub fn sub_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sub_network", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexTaskSparkElInfrastructureSpecElDynamic {
    batch: Option<DynamicBlock<DataplexTaskSparkElInfrastructureSpecElBatchEl>>,
    container_image: Option<DynamicBlock<DataplexTaskSparkElInfrastructureSpecElContainerImageEl>>,
    vpc_network: Option<DynamicBlock<DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl>>,
}
#[derive(Serialize)]
pub struct DataplexTaskSparkElInfrastructureSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch: Option<Vec<DataplexTaskSparkElInfrastructureSpecElBatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_image: Option<Vec<DataplexTaskSparkElInfrastructureSpecElContainerImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_network: Option<Vec<DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl>>,
    dynamic: DataplexTaskSparkElInfrastructureSpecElDynamic,
}
impl DataplexTaskSparkElInfrastructureSpecEl {
    #[doc = "Set the field `batch`.\n"]
    pub fn set_batch(
        mut self,
        v: impl Into<BlockAssignable<DataplexTaskSparkElInfrastructureSpecElBatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.batch = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.batch = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `container_image`.\n"]
    pub fn set_container_image(
        mut self,
        v: impl Into<BlockAssignable<DataplexTaskSparkElInfrastructureSpecElContainerImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_image = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_network`.\n"]
    pub fn set_vpc_network(
        mut self,
        v: impl Into<BlockAssignable<DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_network = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_network = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexTaskSparkElInfrastructureSpecEl {
    type O = BlockAssignable<DataplexTaskSparkElInfrastructureSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskSparkElInfrastructureSpecEl {}
impl BuildDataplexTaskSparkElInfrastructureSpecEl {
    pub fn build(self) -> DataplexTaskSparkElInfrastructureSpecEl {
        DataplexTaskSparkElInfrastructureSpecEl {
            batch: core::default::Default::default(),
            container_image: core::default::Default::default(),
            vpc_network: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexTaskSparkElInfrastructureSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskSparkElInfrastructureSpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexTaskSparkElInfrastructureSpecElRef {
        DataplexTaskSparkElInfrastructureSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskSparkElInfrastructureSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch` after provisioning.\n"]
    pub fn batch(&self) -> ListRef<DataplexTaskSparkElInfrastructureSpecElBatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.batch", self.base))
    }
    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(
        &self,
    ) -> ListRef<DataplexTaskSparkElInfrastructureSpecElContainerImageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_image", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_network` after provisioning.\n"]
    pub fn vpc_network(&self) -> ListRef<DataplexTaskSparkElInfrastructureSpecElVpcNetworkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_network", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexTaskSparkElDynamic {
    infrastructure_spec: Option<DynamicBlock<DataplexTaskSparkElInfrastructureSpecEl>>,
}
#[derive(Serialize)]
pub struct DataplexTaskSparkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    main_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    main_jar_file_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_script_file: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_script_file: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    infrastructure_spec: Option<Vec<DataplexTaskSparkElInfrastructureSpecEl>>,
    dynamic: DataplexTaskSparkElDynamic,
}
impl DataplexTaskSparkEl {
    #[doc = "Set the field `archive_uris`.\nCloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.archive_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `file_uris`.\nCloud Storage URIs of files to be placed in the working directory of each executor."]
    pub fn set_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `main_class`.\nThe name of the driver's main class. The jar file that contains the class must be in the default CLASSPATH or specified in jar_file_uris. The execution args are passed in as a sequence of named process arguments (--key=value)."]
    pub fn set_main_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_class = Some(v.into());
        self
    }
    #[doc = "Set the field `main_jar_file_uri`.\nThe Cloud Storage URI of the jar file that contains the main class. The execution args are passed in as a sequence of named process arguments (--key=value)."]
    pub fn set_main_jar_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_jar_file_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `python_script_file`.\nThe Gcloud Storage URI of the main Python file to use as the driver. Must be a .py file. The execution args are passed in as a sequence of named process arguments (--key=value)."]
    pub fn set_python_script_file(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_script_file = Some(v.into());
        self
    }
    #[doc = "Set the field `sql_script`.\nThe query text. The execution args are used to declare a set of script variables (set key='value';)."]
    pub fn set_sql_script(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sql_script = Some(v.into());
        self
    }
    #[doc = "Set the field `sql_script_file`.\nA reference to a query file. This can be the Cloud Storage URI of the query file or it can the path to a SqlScript Content. The execution args are used to declare a set of script variables (set key='value';)."]
    pub fn set_sql_script_file(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sql_script_file = Some(v.into());
        self
    }
    #[doc = "Set the field `infrastructure_spec`.\n"]
    pub fn set_infrastructure_spec(
        mut self,
        v: impl Into<BlockAssignable<DataplexTaskSparkElInfrastructureSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.infrastructure_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.infrastructure_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexTaskSparkEl {
    type O = BlockAssignable<DataplexTaskSparkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskSparkEl {}
impl BuildDataplexTaskSparkEl {
    pub fn build(self) -> DataplexTaskSparkEl {
        DataplexTaskSparkEl {
            archive_uris: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_jar_file_uri: core::default::Default::default(),
            python_script_file: core::default::Default::default(),
            sql_script: core::default::Default::default(),
            sql_script_file: core::default::Default::default(),
            infrastructure_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexTaskSparkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskSparkElRef {
    fn new(shared: StackShared, base: String) -> DataplexTaskSparkElRef {
        DataplexTaskSparkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskSparkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_uris` after provisioning.\nCloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `file_uris` after provisioning.\nCloud Storage URIs of files to be placed in the working directory of each executor."]
    pub fn file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `main_class` after provisioning.\nThe name of the driver's main class. The jar file that contains the class must be in the default CLASSPATH or specified in jar_file_uris. The execution args are passed in as a sequence of named process arguments (--key=value)."]
    pub fn main_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }
    #[doc = "Get a reference to the value of field `main_jar_file_uri` after provisioning.\nThe Cloud Storage URI of the jar file that contains the main class. The execution args are passed in as a sequence of named process arguments (--key=value)."]
    pub fn main_jar_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_jar_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `python_script_file` after provisioning.\nThe Gcloud Storage URI of the main Python file to use as the driver. Must be a .py file. The execution args are passed in as a sequence of named process arguments (--key=value)."]
    pub fn python_script_file(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.python_script_file", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sql_script` after provisioning.\nThe query text. The execution args are used to declare a set of script variables (set key='value';)."]
    pub fn sql_script(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql_script", self.base))
    }
    #[doc = "Get a reference to the value of field `sql_script_file` after provisioning.\nA reference to a query file. This can be the Cloud Storage URI of the query file or it can the path to a SqlScript Content. The execution args are used to declare a set of script variables (set key='value';)."]
    pub fn sql_script_file(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_script_file", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `infrastructure_spec` after provisioning.\n"]
    pub fn infrastructure_spec(&self) -> ListRef<DataplexTaskSparkElInfrastructureSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.infrastructure_spec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexTaskTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataplexTaskTimeoutsEl {
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
impl ToListMappable for DataplexTaskTimeoutsEl {
    type O = BlockAssignable<DataplexTaskTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskTimeoutsEl {}
impl BuildDataplexTaskTimeoutsEl {
    pub fn build(self) -> DataplexTaskTimeoutsEl {
        DataplexTaskTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataplexTaskTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataplexTaskTimeoutsElRef {
        DataplexTaskTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskTimeoutsElRef {
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
pub struct DataplexTaskTriggerSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl DataplexTaskTriggerSpecEl {
    #[doc = "Set the field `disabled`.\nPrevent the task from executing. This does not cancel already running tasks. It is intended to temporarily disable RECURRING tasks."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `max_retries`.\nNumber of retry attempts before aborting. Set to zero to never attempt to retry a failed task."]
    pub fn set_max_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_retries = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule`.\nCron schedule (https://en.wikipedia.org/wiki/Cron) for running tasks periodically. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: 'CRON_TZ=${IANA_TIME_ZONE}' or 'TZ=${IANA_TIME_ZONE}'. The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, CRON_TZ=America/New_York 1 * * * *, or TZ=America/New_York 1 * * * *. This field is required for RECURRING tasks."]
    pub fn set_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\nThe first run of the task will be after this time. If not specified, the task will run shortly after being submitted if ON_DEMAND and based on the schedule if RECURRING."]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexTaskTriggerSpecEl {
    type O = BlockAssignable<DataplexTaskTriggerSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexTaskTriggerSpecEl {
    #[doc = "Trigger type of the user-specified Task Possible values: [\"ON_DEMAND\", \"RECURRING\"]"]
    pub type_: PrimField<String>,
}
impl BuildDataplexTaskTriggerSpecEl {
    pub fn build(self) -> DataplexTaskTriggerSpecEl {
        DataplexTaskTriggerSpecEl {
            disabled: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            schedule: core::default::Default::default(),
            start_time: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct DataplexTaskTriggerSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexTaskTriggerSpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexTaskTriggerSpecElRef {
        DataplexTaskTriggerSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexTaskTriggerSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nPrevent the task from executing. This does not cancel already running tasks. It is intended to temporarily disable RECURRING tasks."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\nNumber of retry attempts before aborting. Set to zero to never attempt to retry a failed task."]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nCron schedule (https://en.wikipedia.org/wiki/Cron) for running tasks periodically. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: 'CRON_TZ=${IANA_TIME_ZONE}' or 'TZ=${IANA_TIME_ZONE}'. The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, CRON_TZ=America/New_York 1 * * * *, or TZ=America/New_York 1 * * * *. This field is required for RECURRING tasks."]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThe first run of the task will be after this time. If not specified, the task will run shortly after being submitted if ON_DEMAND and based on the schedule if RECURRING."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nTrigger type of the user-specified Task Possible values: [\"ON_DEMAND\", \"RECURRING\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexTaskDynamic {
    execution_spec: Option<DynamicBlock<DataplexTaskExecutionSpecEl>>,
    notebook: Option<DynamicBlock<DataplexTaskNotebookEl>>,
    spark: Option<DynamicBlock<DataplexTaskSparkEl>>,
    trigger_spec: Option<DynamicBlock<DataplexTaskTriggerSpecEl>>,
}
