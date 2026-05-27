use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ColabNotebookExecutionData {
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
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_user: Option<PrimField<String>>,
    gcs_output_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notebook_execution_job_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notebook_runtime_template_resource_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_environment_spec: Option<Vec<ColabNotebookExecutionCustomEnvironmentSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataform_repository_source: Option<Vec<ColabNotebookExecutionDataformRepositorySourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_notebook_source: Option<Vec<ColabNotebookExecutionDirectNotebookSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_notebook_source: Option<Vec<ColabNotebookExecutionGcsNotebookSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ColabNotebookExecutionTimeoutsEl>,
    dynamic: ColabNotebookExecutionDynamic,
}
struct ColabNotebookExecution_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ColabNotebookExecutionData>,
}
#[derive(Clone)]
pub struct ColabNotebookExecution(Rc<ColabNotebookExecution_>);
impl ColabNotebookExecution {
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
    #[doc = "Set the field `execution_timeout`.\nMax running time of the execution job in seconds (default 86400s / 24 hrs)."]
    pub fn set_execution_timeout(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().execution_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_user`.\nThe user email to run the execution as."]
    pub fn set_execution_user(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().execution_user = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `notebook_execution_job_id`.\nUser specified ID for the Notebook Execution Job"]
    pub fn set_notebook_execution_job_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().notebook_execution_job_id = Some(v.into());
        self
    }
    #[doc = "Set the field `notebook_runtime_template_resource_name`.\nThe NotebookRuntimeTemplate to source compute configuration from."]
    pub fn set_notebook_runtime_template_resource_name(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .notebook_runtime_template_resource_name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nThe service account to run the execution as."]
    pub fn set_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_environment_spec`.\n"]
    pub fn set_custom_environment_spec(
        self,
        v: impl Into<BlockAssignable<ColabNotebookExecutionCustomEnvironmentSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_environment_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_environment_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dataform_repository_source`.\n"]
    pub fn set_dataform_repository_source(
        self,
        v: impl Into<BlockAssignable<ColabNotebookExecutionDataformRepositorySourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dataform_repository_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dataform_repository_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `direct_notebook_source`.\n"]
    pub fn set_direct_notebook_source(
        self,
        v: impl Into<BlockAssignable<ColabNotebookExecutionDirectNotebookSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().direct_notebook_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.direct_notebook_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gcs_notebook_source`.\n"]
    pub fn set_gcs_notebook_source(
        self,
        v: impl Into<BlockAssignable<ColabNotebookExecutionGcsNotebookSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gcs_notebook_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gcs_notebook_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ColabNotebookExecutionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Notebook Execution."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_timeout` after provisioning.\nMax running time of the execution job in seconds (default 86400s / 24 hrs)."]
    pub fn execution_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_user` after provisioning.\nThe user email to run the execution as."]
    pub fn execution_user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_user", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_output_uri` after provisioning.\nThe Cloud Storage location to upload the result to. Format:'gs://bucket-name'"]
    pub fn gcs_output_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcs_output_uri", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `notebook_execution_job_id` after provisioning.\nUser specified ID for the Notebook Execution Job"]
    pub fn notebook_execution_job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebook_execution_job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notebook_runtime_template_resource_name` after provisioning.\nThe NotebookRuntimeTemplate to source compute configuration from."]
    pub fn notebook_runtime_template_resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.notebook_runtime_template_resource_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account to run the execution as."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_environment_spec` after provisioning.\n"]
    pub fn custom_environment_spec(
        &self,
    ) -> ListRef<ColabNotebookExecutionCustomEnvironmentSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_environment_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataform_repository_source` after provisioning.\n"]
    pub fn dataform_repository_source(
        &self,
    ) -> ListRef<ColabNotebookExecutionDataformRepositorySourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataform_repository_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `direct_notebook_source` after provisioning.\n"]
    pub fn direct_notebook_source(
        &self,
    ) -> ListRef<ColabNotebookExecutionDirectNotebookSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.direct_notebook_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_notebook_source` after provisioning.\n"]
    pub fn gcs_notebook_source(&self) -> ListRef<ColabNotebookExecutionGcsNotebookSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_notebook_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabNotebookExecutionTimeoutsElRef {
        ColabNotebookExecutionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ColabNotebookExecution {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ColabNotebookExecution {}
impl ToListMappable for ColabNotebookExecution {
    type O = ListRef<ColabNotebookExecutionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ColabNotebookExecution_ {
    fn extract_resource_type(&self) -> String {
        "google_colab_notebook_execution".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildColabNotebookExecution {
    pub tf_id: String,
    #[doc = "Required. The display name of the Notebook Execution."]
    pub display_name: PrimField<String>,
    #[doc = "The Cloud Storage location to upload the result to. Format:'gs://bucket-name'"]
    pub gcs_output_uri: PrimField<String>,
    #[doc = "The location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub location: PrimField<String>,
}
impl BuildColabNotebookExecution {
    pub fn build(self, stack: &mut Stack) -> ColabNotebookExecution {
        let out = ColabNotebookExecution(Rc::new(ColabNotebookExecution_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ColabNotebookExecutionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                execution_timeout: core::default::Default::default(),
                execution_user: core::default::Default::default(),
                gcs_output_uri: self.gcs_output_uri,
                id: core::default::Default::default(),
                location: self.location,
                notebook_execution_job_id: core::default::Default::default(),
                notebook_runtime_template_resource_name: core::default::Default::default(),
                project: core::default::Default::default(),
                service_account: core::default::Default::default(),
                custom_environment_spec: core::default::Default::default(),
                dataform_repository_source: core::default::Default::default(),
                direct_notebook_source: core::default::Default::default(),
                gcs_notebook_source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ColabNotebookExecutionRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabNotebookExecutionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ColabNotebookExecutionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Notebook Execution."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_timeout` after provisioning.\nMax running time of the execution job in seconds (default 86400s / 24 hrs)."]
    pub fn execution_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_user` after provisioning.\nThe user email to run the execution as."]
    pub fn execution_user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_user", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_output_uri` after provisioning.\nThe Cloud Storage location to upload the result to. Format:'gs://bucket-name'"]
    pub fn gcs_output_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcs_output_uri", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `notebook_execution_job_id` after provisioning.\nUser specified ID for the Notebook Execution Job"]
    pub fn notebook_execution_job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebook_execution_job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notebook_runtime_template_resource_name` after provisioning.\nThe NotebookRuntimeTemplate to source compute configuration from."]
    pub fn notebook_runtime_template_resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.notebook_runtime_template_resource_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account to run the execution as."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_environment_spec` after provisioning.\n"]
    pub fn custom_environment_spec(
        &self,
    ) -> ListRef<ColabNotebookExecutionCustomEnvironmentSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_environment_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataform_repository_source` after provisioning.\n"]
    pub fn dataform_repository_source(
        &self,
    ) -> ListRef<ColabNotebookExecutionDataformRepositorySourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataform_repository_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `direct_notebook_source` after provisioning.\n"]
    pub fn direct_notebook_source(
        &self,
    ) -> ListRef<ColabNotebookExecutionDirectNotebookSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.direct_notebook_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_notebook_source` after provisioning.\n"]
    pub fn gcs_notebook_source(&self) -> ListRef<ColabNotebookExecutionGcsNotebookSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_notebook_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabNotebookExecutionTimeoutsElRef {
        ColabNotebookExecutionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
}
impl ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecEl {
    #[doc = "Set the field `accelerator_count`.\nThe number of accelerators used by the runtime."]
    pub fn set_accelerator_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.accelerator_count = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_type`.\nThe type of hardware accelerator used by the runtime. If specified, acceleratorCount must also be specified."]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe Compute Engine machine type selected for the runtime."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
}
impl ToListMappable for ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecEl {
    type O = BlockAssignable<ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabNotebookExecutionCustomEnvironmentSpecElMachineSpecEl {}
impl BuildColabNotebookExecutionCustomEnvironmentSpecElMachineSpecEl {
    pub fn build(self) -> ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecEl {
        ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecEl {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}
pub struct ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecElRef {
        ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of accelerators used by the runtime."]
    pub fn accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\nThe type of hardware accelerator used by the runtime. If specified, acceleratorCount must also be specified."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe Compute Engine machine type selected for the runtime."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_internet_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
}
impl ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecEl {
    #[doc = "Set the field `enable_internet_access`.\nEnable public internet access for the runtime."]
    pub fn set_enable_internet_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_internet_access = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe name of the VPC that this runtime is in."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThe name of the subnetwork that this runtime is in."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
}
impl ToListMappable for ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecEl {
    type O = BlockAssignable<ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecEl {}
impl BuildColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecEl {
    pub fn build(self) -> ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecEl {
        ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecEl {
            enable_internet_access: core::default::Default::default(),
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}
pub struct ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecElRef {
        ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_internet_access` after provisioning.\nEnable public internet access for the runtime."]
    pub fn enable_internet_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_internet_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC that this runtime is in."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe name of the subnetwork that this runtime is in."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}
#[derive(Serialize)]
pub struct ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
}
impl ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecEl {
    #[doc = "Set the field `disk_size_gb`.\nThe disk size of the runtime in GB. If specified, the diskType must also be specified. The minimum size is 10GB and the maximum is 65536GB."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\nThe type of the persistent disk."]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
}
impl ToListMappable for ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecEl {
    type O = BlockAssignable<ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecEl {}
impl BuildColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecEl {
    pub fn build(self) -> ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecEl {
        ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecEl {
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
        }
    }
}
pub struct ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecElRef {
        ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nThe disk size of the runtime in GB. If specified, the diskType must also be specified. The minimum size is 10GB and the maximum is 65536GB."]
    pub fn disk_size_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nThe type of the persistent disk."]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct ColabNotebookExecutionCustomEnvironmentSpecElDynamic {
    machine_spec: Option<DynamicBlock<ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecEl>>,
    network_spec: Option<DynamicBlock<ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecEl>>,
    persistent_disk_spec:
        Option<DynamicBlock<ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecEl>>,
}
#[derive(Serialize)]
pub struct ColabNotebookExecutionCustomEnvironmentSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_spec: Option<Vec<ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_spec: Option<Vec<ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    persistent_disk_spec:
        Option<Vec<ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecEl>>,
    dynamic: ColabNotebookExecutionCustomEnvironmentSpecElDynamic,
}
impl ColabNotebookExecutionCustomEnvironmentSpecEl {
    #[doc = "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        mut self,
        v: impl Into<BlockAssignable<ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.machine_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.machine_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_spec`.\n"]
    pub fn set_network_spec(
        mut self,
        v: impl Into<BlockAssignable<ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `persistent_disk_spec`.\n"]
    pub fn set_persistent_disk_spec(
        mut self,
        v: impl Into<BlockAssignable<ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.persistent_disk_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.persistent_disk_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ColabNotebookExecutionCustomEnvironmentSpecEl {
    type O = BlockAssignable<ColabNotebookExecutionCustomEnvironmentSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabNotebookExecutionCustomEnvironmentSpecEl {}
impl BuildColabNotebookExecutionCustomEnvironmentSpecEl {
    pub fn build(self) -> ColabNotebookExecutionCustomEnvironmentSpecEl {
        ColabNotebookExecutionCustomEnvironmentSpecEl {
            machine_spec: core::default::Default::default(),
            network_spec: core::default::Default::default(),
            persistent_disk_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ColabNotebookExecutionCustomEnvironmentSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabNotebookExecutionCustomEnvironmentSpecElRef {
    fn new(shared: StackShared, base: String) -> ColabNotebookExecutionCustomEnvironmentSpecElRef {
        ColabNotebookExecutionCustomEnvironmentSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabNotebookExecutionCustomEnvironmentSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(
        &self,
    ) -> ListRef<ColabNotebookExecutionCustomEnvironmentSpecElMachineSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.base))
    }
    #[doc = "Get a reference to the value of field `network_spec` after provisioning.\n"]
    pub fn network_spec(
        &self,
    ) -> ListRef<ColabNotebookExecutionCustomEnvironmentSpecElNetworkSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_spec", self.base))
    }
    #[doc = "Get a reference to the value of field `persistent_disk_spec` after provisioning.\n"]
    pub fn persistent_disk_spec(
        &self,
    ) -> ListRef<ColabNotebookExecutionCustomEnvironmentSpecElPersistentDiskSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persistent_disk_spec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ColabNotebookExecutionDataformRepositorySourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_sha: Option<PrimField<String>>,
    dataform_repository_resource_name: PrimField<String>,
}
impl ColabNotebookExecutionDataformRepositorySourceEl {
    #[doc = "Set the field `commit_sha`.\nThe commit SHA to read repository with. If unset, the file will be read at HEAD."]
    pub fn set_commit_sha(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit_sha = Some(v.into());
        self
    }
}
impl ToListMappable for ColabNotebookExecutionDataformRepositorySourceEl {
    type O = BlockAssignable<ColabNotebookExecutionDataformRepositorySourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabNotebookExecutionDataformRepositorySourceEl {
    #[doc = "The resource name of the Dataform Repository."]
    pub dataform_repository_resource_name: PrimField<String>,
}
impl BuildColabNotebookExecutionDataformRepositorySourceEl {
    pub fn build(self) -> ColabNotebookExecutionDataformRepositorySourceEl {
        ColabNotebookExecutionDataformRepositorySourceEl {
            commit_sha: core::default::Default::default(),
            dataform_repository_resource_name: self.dataform_repository_resource_name,
        }
    }
}
pub struct ColabNotebookExecutionDataformRepositorySourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabNotebookExecutionDataformRepositorySourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ColabNotebookExecutionDataformRepositorySourceElRef {
        ColabNotebookExecutionDataformRepositorySourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabNotebookExecutionDataformRepositorySourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `commit_sha` after provisioning.\nThe commit SHA to read repository with. If unset, the file will be read at HEAD."]
    pub fn commit_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }
    #[doc = "Get a reference to the value of field `dataform_repository_resource_name` after provisioning.\nThe resource name of the Dataform Repository."]
    pub fn dataform_repository_resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataform_repository_resource_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ColabNotebookExecutionDirectNotebookSourceEl {
    content: PrimField<String>,
}
impl ColabNotebookExecutionDirectNotebookSourceEl {}
impl ToListMappable for ColabNotebookExecutionDirectNotebookSourceEl {
    type O = BlockAssignable<ColabNotebookExecutionDirectNotebookSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabNotebookExecutionDirectNotebookSourceEl {
    #[doc = "The base64-encoded contents of the input notebook file."]
    pub content: PrimField<String>,
}
impl BuildColabNotebookExecutionDirectNotebookSourceEl {
    pub fn build(self) -> ColabNotebookExecutionDirectNotebookSourceEl {
        ColabNotebookExecutionDirectNotebookSourceEl {
            content: self.content,
        }
    }
}
pub struct ColabNotebookExecutionDirectNotebookSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabNotebookExecutionDirectNotebookSourceElRef {
    fn new(shared: StackShared, base: String) -> ColabNotebookExecutionDirectNotebookSourceElRef {
        ColabNotebookExecutionDirectNotebookSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabNotebookExecutionDirectNotebookSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe base64-encoded contents of the input notebook file."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
}
#[derive(Serialize)]
pub struct ColabNotebookExecutionGcsNotebookSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    uri: PrimField<String>,
}
impl ColabNotebookExecutionGcsNotebookSourceEl {
    #[doc = "Set the field `generation`.\nThe version of the Cloud Storage object to read. If unset, the current version of the object is read. See https://cloud.google.com/storage/docs/metadata#generation-number."]
    pub fn set_generation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.generation = Some(v.into());
        self
    }
}
impl ToListMappable for ColabNotebookExecutionGcsNotebookSourceEl {
    type O = BlockAssignable<ColabNotebookExecutionGcsNotebookSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabNotebookExecutionGcsNotebookSourceEl {
    #[doc = "The Cloud Storage uri pointing to the ipynb file."]
    pub uri: PrimField<String>,
}
impl BuildColabNotebookExecutionGcsNotebookSourceEl {
    pub fn build(self) -> ColabNotebookExecutionGcsNotebookSourceEl {
        ColabNotebookExecutionGcsNotebookSourceEl {
            generation: core::default::Default::default(),
            uri: self.uri,
        }
    }
}
pub struct ColabNotebookExecutionGcsNotebookSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabNotebookExecutionGcsNotebookSourceElRef {
    fn new(shared: StackShared, base: String) -> ColabNotebookExecutionGcsNotebookSourceElRef {
        ColabNotebookExecutionGcsNotebookSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabNotebookExecutionGcsNotebookSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nThe version of the Cloud Storage object to read. If unset, the current version of the object is read. See https://cloud.google.com/storage/docs/metadata#generation-number."]
    pub fn generation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe Cloud Storage uri pointing to the ipynb file."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct ColabNotebookExecutionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ColabNotebookExecutionTimeoutsEl {
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
}
impl ToListMappable for ColabNotebookExecutionTimeoutsEl {
    type O = BlockAssignable<ColabNotebookExecutionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabNotebookExecutionTimeoutsEl {}
impl BuildColabNotebookExecutionTimeoutsEl {
    pub fn build(self) -> ColabNotebookExecutionTimeoutsEl {
        ColabNotebookExecutionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ColabNotebookExecutionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabNotebookExecutionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ColabNotebookExecutionTimeoutsElRef {
        ColabNotebookExecutionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabNotebookExecutionTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct ColabNotebookExecutionDynamic {
    custom_environment_spec: Option<DynamicBlock<ColabNotebookExecutionCustomEnvironmentSpecEl>>,
    dataform_repository_source:
        Option<DynamicBlock<ColabNotebookExecutionDataformRepositorySourceEl>>,
    direct_notebook_source: Option<DynamicBlock<ColabNotebookExecutionDirectNotebookSourceEl>>,
    gcs_notebook_source: Option<DynamicBlock<ColabNotebookExecutionGcsNotebookSourceEl>>,
}
