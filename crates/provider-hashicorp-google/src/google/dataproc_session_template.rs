use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataprocSessionTemplateData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_config: Option<Vec<DataprocSessionTemplateEnvironmentConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_session: Option<Vec<DataprocSessionTemplateJupyterSessionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_config: Option<Vec<DataprocSessionTemplateRuntimeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_connect_session: Option<Vec<DataprocSessionTemplateSparkConnectSessionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataprocSessionTemplateTimeoutsEl>,
    dynamic: DataprocSessionTemplateDynamic,
}
struct DataprocSessionTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataprocSessionTemplateData>,
}
#[derive(Clone)]
pub struct DataprocSessionTemplate(Rc<DataprocSessionTemplate_>);
impl DataprocSessionTemplate {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels to associate with this session template.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location in which the session template will be created in."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `environment_config`.\n"]
    pub fn set_environment_config(
        self,
        v: impl Into<BlockAssignable<DataprocSessionTemplateEnvironmentConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().environment_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.environment_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `jupyter_session`.\n"]
    pub fn set_jupyter_session(
        self,
        v: impl Into<BlockAssignable<DataprocSessionTemplateJupyterSessionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().jupyter_session = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.jupyter_session = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `runtime_config`.\n"]
    pub fn set_runtime_config(
        self,
        v: impl Into<BlockAssignable<DataprocSessionTemplateRuntimeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark_connect_session`.\n"]
    pub fn set_spark_connect_session(
        self,
        v: impl Into<BlockAssignable<DataprocSessionTemplateSparkConnectSessionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_connect_session = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_connect_session = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataprocSessionTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the session template was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator` after provisioning.\nThe email address of the user who created the session template."]
    pub fn creator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this session template.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location in which the session template will be created in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the session template in the following format:\nprojects/{project}/locations/{location}/sessionTemplates/{template_id}"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the session template was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nA session template UUID (Unique Universal Identifier). The service generates this value when it creates the session template."]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uuid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment_config` after provisioning.\n"]
    pub fn environment_config(&self) -> ListRef<DataprocSessionTemplateEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.environment_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `jupyter_session` after provisioning.\n"]
    pub fn jupyter_session(&self) -> ListRef<DataprocSessionTemplateJupyterSessionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jupyter_session", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(&self) -> ListRef<DataprocSessionTemplateRuntimeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_connect_session` after provisioning.\n"]
    pub fn spark_connect_session(
        &self,
    ) -> ListRef<DataprocSessionTemplateSparkConnectSessionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_connect_session", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocSessionTemplateTimeoutsElRef {
        DataprocSessionTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataprocSessionTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataprocSessionTemplate {}
impl ToListMappable for DataprocSessionTemplate {
    type O = ListRef<DataprocSessionTemplateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataprocSessionTemplate_ {
    fn extract_resource_type(&self) -> String {
        "google_dataproc_session_template".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataprocSessionTemplate {
    pub tf_id: String,
    #[doc = "The resource name of the session template in the following format:\nprojects/{project}/locations/{location}/sessionTemplates/{template_id}"]
    pub name: PrimField<String>,
}
impl BuildDataprocSessionTemplate {
    pub fn build(self, stack: &mut Stack) -> DataprocSessionTemplate {
        let out = DataprocSessionTemplate(Rc::new(DataprocSessionTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataprocSessionTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                environment_config: core::default::Default::default(),
                jupyter_session: core::default::Default::default(),
                runtime_config: core::default::Default::default(),
                spark_connect_session: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataprocSessionTemplateRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocSessionTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataprocSessionTemplateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the session template was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator` after provisioning.\nThe email address of the user who created the session template."]
    pub fn creator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this session template.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location in which the session template will be created in."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the session template in the following format:\nprojects/{project}/locations/{location}/sessionTemplates/{template_id}"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the session template was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nA session template UUID (Unique Universal Identifier). The service generates this value when it creates the session template."]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.uuid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment_config` after provisioning.\n"]
    pub fn environment_config(&self) -> ListRef<DataprocSessionTemplateEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.environment_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `jupyter_session` after provisioning.\n"]
    pub fn jupyter_session(&self) -> ListRef<DataprocSessionTemplateJupyterSessionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jupyter_session", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(&self) -> ListRef<DataprocSessionTemplateRuntimeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_connect_session` after provisioning.\n"]
    pub fn spark_connect_session(
        &self,
    ) -> ListRef<DataprocSessionTemplateSparkConnectSessionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_connect_session", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocSessionTemplateTimeoutsElRef {
        DataprocSessionTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_workload_authentication_type: Option<PrimField<String>>,
}
impl DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {
    #[doc = "Set the field `user_workload_authentication_type`.\nAuthentication type for the user workload running in containers. Possible values: [\"SERVICE_ACCOUNT\", \"END_USER_CREDENTIALS\"]"]
    pub fn set_user_workload_authentication_type(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.user_workload_authentication_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigEl
{
    type O = BlockAssignable<
        DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {
}
impl BuildDataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {
    pub fn build(
        self,
    ) -> DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {
        DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigEl {
            user_workload_authentication_type: core::default::Default::default(),
        }
    }
}
pub struct DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigElRef {
        DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_workload_authentication_type` after provisioning.\nAuthentication type for the user workload running in containers. Possible values: [\"SERVICE_ACCOUNT\", \"END_USER_CREDENTIALS\"]"]
    pub fn user_workload_authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_workload_authentication_type", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocSessionTemplateEnvironmentConfigElExecutionConfigElDynamic {
    authentication_config: Option<
        DynamicBlock<
            DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataprocSessionTemplateEnvironmentConfigElExecutionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    staging_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_config: Option<
        Vec<DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigEl>,
    >,
    dynamic: DataprocSessionTemplateEnvironmentConfigElExecutionConfigElDynamic,
}
impl DataprocSessionTemplateEnvironmentConfigElExecutionConfigEl {
    #[doc = "Set the field `idle_ttl`.\nThe duration to keep the session alive while it's idling.\nExceeding this threshold causes the session to terminate. Minimum value is 10 minutes; maximum value is 14 day.\nDefaults to 1 hour if not set. If both ttl and idleTtl are specified for an interactive session, the conditions\nare treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or when ttl has\nbeen exceeded, whichever occurs first."]
    pub fn set_idle_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.idle_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\nThe Cloud KMS key to use for encryption."]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tags`.\nTags used for network traffic control."]
    pub fn set_network_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.network_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nService account that used to execute workload."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `staging_bucket`.\nA Cloud Storage bucket used to stage workload dependencies, config files, and store\nworkload output and other ephemeral data, such as Spark history files. If you do not specify a staging bucket,\nCloud Dataproc will determine a Cloud Storage location according to the region where your workload is running,\nand then create and manage project-level, per-location staging and temporary buckets.\nThis field requires a Cloud Storage bucket name, not a gs://... URI to a Cloud Storage bucket."]
    pub fn set_staging_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.staging_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork_uri`.\nSubnetwork configuration for workload execution."]
    pub fn set_subnetwork_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\nThe duration after which the workload will be terminated.\nWhen the workload exceeds this duration, it will be unconditionally terminated without waiting for ongoing\nwork to finish. If ttl is not specified for a session workload, the workload will be allowed to run until it\nexits naturally (or run forever without exiting). If ttl is not specified for an interactive session,\nit defaults to 24 hours. If ttl is not specified for a batch that uses 2.1+ runtime version, it defaults to 4 hours.\nMinimum value is 10 minutes; maximum value is 14 days. If both ttl and idleTtl are specified (for an interactive session),\nthe conditions are treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or\nwhen ttl has been exceeded, whichever occurs first."]
    pub fn set_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `authentication_config`.\n"]
    pub fn set_authentication_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authentication_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocSessionTemplateEnvironmentConfigElExecutionConfigEl {
    type O = BlockAssignable<DataprocSessionTemplateEnvironmentConfigElExecutionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocSessionTemplateEnvironmentConfigElExecutionConfigEl {}
impl BuildDataprocSessionTemplateEnvironmentConfigElExecutionConfigEl {
    pub fn build(self) -> DataprocSessionTemplateEnvironmentConfigElExecutionConfigEl {
        DataprocSessionTemplateEnvironmentConfigElExecutionConfigEl {
            idle_ttl: core::default::Default::default(),
            kms_key: core::default::Default::default(),
            network_tags: core::default::Default::default(),
            service_account: core::default::Default::default(),
            staging_bucket: core::default::Default::default(),
            subnetwork_uri: core::default::Default::default(),
            ttl: core::default::Default::default(),
            authentication_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocSessionTemplateEnvironmentConfigElExecutionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocSessionTemplateEnvironmentConfigElExecutionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocSessionTemplateEnvironmentConfigElExecutionConfigElRef {
        DataprocSessionTemplateEnvironmentConfigElExecutionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocSessionTemplateEnvironmentConfigElExecutionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `idle_ttl` after provisioning.\nThe duration to keep the session alive while it's idling.\nExceeding this threshold causes the session to terminate. Minimum value is 10 minutes; maximum value is 14 day.\nDefaults to 1 hour if not set. If both ttl and idleTtl are specified for an interactive session, the conditions\nare treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or when ttl has\nbeen exceeded, whichever occurs first."]
    pub fn idle_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS key to use for encryption."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
    #[doc = "Get a reference to the value of field `network_tags` after provisioning.\nTags used for network traffic control."]
    pub fn network_tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nService account that used to execute workload."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `staging_bucket` after provisioning.\nA Cloud Storage bucket used to stage workload dependencies, config files, and store\nworkload output and other ephemeral data, such as Spark history files. If you do not specify a staging bucket,\nCloud Dataproc will determine a Cloud Storage location according to the region where your workload is running,\nand then create and manage project-level, per-location staging and temporary buckets.\nThis field requires a Cloud Storage bucket name, not a gs://... URI to a Cloud Storage bucket."]
    pub fn staging_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.staging_bucket", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork_uri` after provisioning.\nSubnetwork configuration for workload execution."]
    pub fn subnetwork_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe duration after which the workload will be terminated.\nWhen the workload exceeds this duration, it will be unconditionally terminated without waiting for ongoing\nwork to finish. If ttl is not specified for a session workload, the workload will be allowed to run until it\nexits naturally (or run forever without exiting). If ttl is not specified for an interactive session,\nit defaults to 24 hours. If ttl is not specified for a batch that uses 2.1+ runtime version, it defaults to 4 hours.\nMinimum value is 10 minutes; maximum value is 14 days. If both ttl and idleTtl are specified (for an interactive session),\nthe conditions are treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or\nwhen ttl has been exceeded, whichever occurs first."]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `authentication_config` after provisioning.\n"]
    pub fn authentication_config(
        &self,
    ) -> ListRef<DataprocSessionTemplateEnvironmentConfigElExecutionConfigElAuthenticationConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authentication_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataproc_cluster: Option<PrimField<String>>,
}
impl DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl {
    #[doc = "Set the field `dataproc_cluster`.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the workload."]
    pub fn set_dataproc_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataproc_cluster = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl
{
    type O = BlockAssignable<
        DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl
{}
impl BuildDataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl {
    pub fn build(
        self,
    ) -> DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl
    {
        DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl {
            dataproc_cluster: core::default::Default::default(),
        }
    }
}
pub struct DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigElRef
    {
        DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataproc_cluster` after provisioning.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the workload."]
    pub fn dataproc_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataproc_cluster", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElDynamic {
    spark_history_server_config: Option<
        DynamicBlock<
            DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metastore_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_history_server_config: Option<
        Vec<
            DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl,
        >,
    >,
    dynamic: DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElDynamic,
}
impl DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigEl {
    #[doc = "Set the field `metastore_service`.\nResource name of an existing Dataproc Metastore service."]
    pub fn set_metastore_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metastore_service = Some(v.into());
        self
    }
    #[doc = "Set the field `spark_history_server_config`.\n"]
    pub fn set_spark_history_server_config(
        mut self,
        v : impl Into < BlockAssignable < DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spark_history_server_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_history_server_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigEl {
    type O = BlockAssignable<DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocSessionTemplateEnvironmentConfigElPeripheralsConfigEl {}
impl BuildDataprocSessionTemplateEnvironmentConfigElPeripheralsConfigEl {
    pub fn build(self) -> DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigEl {
        DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigEl {
            metastore_service: core::default::Default::default(),
            spark_history_server_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElRef {
        DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metastore_service` after provisioning.\nResource name of an existing Dataproc Metastore service."]
    pub fn metastore_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metastore_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spark_history_server_config` after provisioning.\n"]
    pub fn spark_history_server_config(
        &self,
    ) -> ListRef<
        DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElSparkHistoryServerConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_history_server_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocSessionTemplateEnvironmentConfigElDynamic {
    execution_config:
        Option<DynamicBlock<DataprocSessionTemplateEnvironmentConfigElExecutionConfigEl>>,
    peripherals_config:
        Option<DynamicBlock<DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocSessionTemplateEnvironmentConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_config: Option<Vec<DataprocSessionTemplateEnvironmentConfigElExecutionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peripherals_config: Option<Vec<DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigEl>>,
    dynamic: DataprocSessionTemplateEnvironmentConfigElDynamic,
}
impl DataprocSessionTemplateEnvironmentConfigEl {
    #[doc = "Set the field `execution_config`.\n"]
    pub fn set_execution_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocSessionTemplateEnvironmentConfigElExecutionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.execution_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.execution_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `peripherals_config`.\n"]
    pub fn set_peripherals_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.peripherals_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.peripherals_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocSessionTemplateEnvironmentConfigEl {
    type O = BlockAssignable<DataprocSessionTemplateEnvironmentConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocSessionTemplateEnvironmentConfigEl {}
impl BuildDataprocSessionTemplateEnvironmentConfigEl {
    pub fn build(self) -> DataprocSessionTemplateEnvironmentConfigEl {
        DataprocSessionTemplateEnvironmentConfigEl {
            execution_config: core::default::Default::default(),
            peripherals_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocSessionTemplateEnvironmentConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocSessionTemplateEnvironmentConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocSessionTemplateEnvironmentConfigElRef {
        DataprocSessionTemplateEnvironmentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocSessionTemplateEnvironmentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `execution_config` after provisioning.\n"]
    pub fn execution_config(
        &self,
    ) -> ListRef<DataprocSessionTemplateEnvironmentConfigElExecutionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.execution_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peripherals_config` after provisioning.\n"]
    pub fn peripherals_config(
        &self,
    ) -> ListRef<DataprocSessionTemplateEnvironmentConfigElPeripheralsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.peripherals_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocSessionTemplateJupyterSessionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel: Option<PrimField<String>>,
}
impl DataprocSessionTemplateJupyterSessionEl {
    #[doc = "Set the field `display_name`.\nDisplay name, shown in the Jupyter kernelspec card."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `kernel`.\nKernel to be used with Jupyter interactive session. Possible values: [\"PYTHON\", \"SCALA\"]"]
    pub fn set_kernel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kernel = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocSessionTemplateJupyterSessionEl {
    type O = BlockAssignable<DataprocSessionTemplateJupyterSessionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocSessionTemplateJupyterSessionEl {}
impl BuildDataprocSessionTemplateJupyterSessionEl {
    pub fn build(self) -> DataprocSessionTemplateJupyterSessionEl {
        DataprocSessionTemplateJupyterSessionEl {
            display_name: core::default::Default::default(),
            kernel: core::default::Default::default(),
        }
    }
}
pub struct DataprocSessionTemplateJupyterSessionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocSessionTemplateJupyterSessionElRef {
    fn new(shared: StackShared, base: String) -> DataprocSessionTemplateJupyterSessionElRef {
        DataprocSessionTemplateJupyterSessionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocSessionTemplateJupyterSessionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name, shown in the Jupyter kernelspec card."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `kernel` after provisioning.\nKernel to be used with Jupyter interactive session. Possible values: [\"PYTHON\", \"SCALA\"]"]
    pub fn kernel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kernel", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocSessionTemplateRuntimeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataprocSessionTemplateRuntimeConfigEl {
    #[doc = "Set the field `container_image`.\nOptional custom container image for the job runtime environment. If not specified, a default container image will be used."]
    pub fn set_container_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_image = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nA mapping of property names to values, which are used to configure workload execution."]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nVersion of the session runtime."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocSessionTemplateRuntimeConfigEl {
    type O = BlockAssignable<DataprocSessionTemplateRuntimeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocSessionTemplateRuntimeConfigEl {}
impl BuildDataprocSessionTemplateRuntimeConfigEl {
    pub fn build(self) -> DataprocSessionTemplateRuntimeConfigEl {
        DataprocSessionTemplateRuntimeConfigEl {
            container_image: core::default::Default::default(),
            properties: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataprocSessionTemplateRuntimeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocSessionTemplateRuntimeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocSessionTemplateRuntimeConfigElRef {
        DataprocSessionTemplateRuntimeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocSessionTemplateRuntimeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_image` after provisioning.\nOptional custom container image for the job runtime environment. If not specified, a default container image will be used."]
    pub fn container_image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_image", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `effective_properties` after provisioning.\nA mapping of property names to values, which are used to configure workload execution."]
    pub fn effective_properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values, which are used to configure workload execution."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the session runtime."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocSessionTemplateSparkConnectSessionEl {}
impl DataprocSessionTemplateSparkConnectSessionEl {}
impl ToListMappable for DataprocSessionTemplateSparkConnectSessionEl {
    type O = BlockAssignable<DataprocSessionTemplateSparkConnectSessionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocSessionTemplateSparkConnectSessionEl {}
impl BuildDataprocSessionTemplateSparkConnectSessionEl {
    pub fn build(self) -> DataprocSessionTemplateSparkConnectSessionEl {
        DataprocSessionTemplateSparkConnectSessionEl {}
    }
}
pub struct DataprocSessionTemplateSparkConnectSessionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocSessionTemplateSparkConnectSessionElRef {
    fn new(shared: StackShared, base: String) -> DataprocSessionTemplateSparkConnectSessionElRef {
        DataprocSessionTemplateSparkConnectSessionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocSessionTemplateSparkConnectSessionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataprocSessionTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataprocSessionTemplateTimeoutsEl {
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
impl ToListMappable for DataprocSessionTemplateTimeoutsEl {
    type O = BlockAssignable<DataprocSessionTemplateTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocSessionTemplateTimeoutsEl {}
impl BuildDataprocSessionTemplateTimeoutsEl {
    pub fn build(self) -> DataprocSessionTemplateTimeoutsEl {
        DataprocSessionTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataprocSessionTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocSessionTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataprocSessionTemplateTimeoutsElRef {
        DataprocSessionTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocSessionTemplateTimeoutsElRef {
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
struct DataprocSessionTemplateDynamic {
    environment_config: Option<DynamicBlock<DataprocSessionTemplateEnvironmentConfigEl>>,
    jupyter_session: Option<DynamicBlock<DataprocSessionTemplateJupyterSessionEl>>,
    runtime_config: Option<DynamicBlock<DataprocSessionTemplateRuntimeConfigEl>>,
    spark_connect_session: Option<DynamicBlock<DataprocSessionTemplateSparkConnectSessionEl>>,
}
