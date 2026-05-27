use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct WorkbenchInstanceData {
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
    desired_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_proxy_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_managed_euc: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_third_party_identity: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_owners: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gce_setup: Option<Vec<WorkbenchInstanceGceSetupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<WorkbenchInstanceTimeoutsEl>,
    dynamic: WorkbenchInstanceDynamic,
}
struct WorkbenchInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WorkbenchInstanceData>,
}
#[derive(Clone)]
pub struct WorkbenchInstance(Rc<WorkbenchInstance_>);
impl WorkbenchInstance {
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
    #[doc = "Set the field `desired_state`.\nDesired state of the Workbench Instance. Set this field to 'ACTIVE' to start the Instance, and 'STOPPED' to stop the Instance."]
    pub fn set_desired_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().desired_state = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_proxy_access`.\nOptional. If true, the workbench instance will not register with the proxy."]
    pub fn set_disable_proxy_access(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_proxy_access = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_managed_euc`.\nFlag to enable managed end user credentials for the instance."]
    pub fn set_enable_managed_euc(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_managed_euc = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_third_party_identity`.\nFlag that specifies that a notebook can be accessed with third party\nidentity provider."]
    pub fn set_enable_third_party_identity(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_third_party_identity = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_id`.\nRequired. User-defined unique ID of this instance."]
    pub fn set_instance_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_id = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_owners`.\n'Optional. Input only. The owner of this instance after creation. Format:\n'alias@example.com' Currently supports one owner only. If not specified, all of\nthe service account users of your VM instance''s service account can use the instance.\nIf specified, sets the access mode to 'Single user'. For more details, see\nhttps://cloud.google.com/vertex-ai/docs/workbench/instances/manage-access-jupyterlab'"]
    pub fn set_instance_owners(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().instance_owners = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Labels to apply to this instance. These can be later modified\nby the UpdateInstance method.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `gce_setup`.\n"]
    pub fn set_gce_setup(self, v: impl Into<BlockAssignable<WorkbenchInstanceGceSetupEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gce_setup = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gce_setup = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<WorkbenchInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nAn RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.\nThe milliseconds portion (\".SSS\") is optional."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator` after provisioning.\nOutput only. Email address of entity that sent original CreateInstance request."]
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
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Workbench Instance. Set this field to 'ACTIVE' to start the Instance, and 'STOPPED' to stop the Instance."]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_proxy_access` after provisioning.\nOptional. If true, the workbench instance will not register with the proxy."]
    pub fn disable_proxy_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_proxy_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_managed_euc` after provisioning.\nFlag to enable managed end user credentials for the instance."]
    pub fn enable_managed_euc(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_managed_euc", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_third_party_identity` after provisioning.\nFlag that specifies that a notebook can be accessed with third party\nidentity provider."]
    pub fn enable_third_party_identity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_third_party_identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_info` after provisioning.\n'Output only. Additional information about instance health. Example:\nhealthInfo\": { \"docker_proxy_agent_status\": \"1\", \"docker_status\": \"1\", \"jupyterlab_api_status\":\n\"-1\", \"jupyterlab_status\": \"-1\", \"updated\": \"2020-10-18 09:40:03.573409\" }'"]
    pub fn health_info(&self) -> ListRef<WorkbenchInstanceHealthInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_state` after provisioning.\nOutput only. Instance health_state."]
    pub fn health_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nRequired. User-defined unique ID of this instance."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_owners` after provisioning.\n'Optional. Input only. The owner of this instance after creation. Format:\n'alias@example.com' Currently supports one owner only. If not specified, all of\nthe service account users of your VM instance''s service account can use the instance.\nIf specified, sets the access mode to 'Single user'. For more details, see\nhttps://cloud.google.com/vertex-ai/docs/workbench/instances/manage-access-jupyterlab'"]
    pub fn instance_owners(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_owners", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to apply to this instance. These can be later modified\nby the UpdateInstance method.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this workbench instance. Format: 'projects/{project_id}/locations/{location}/instances/{instance_id}'"]
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
    #[doc = "Get a reference to the value of field `proxy_uri` after provisioning.\nOutput only. The proxy endpoint that is used to access the Jupyter notebook."]
    pub fn proxy_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.proxy_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of this instance."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nAn RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.\nThe milliseconds portion (\".SSS\") is optional."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade_history` after provisioning.\nOutput only. The upgrade history of this instance."]
    pub fn upgrade_history(&self) -> ListRef<WorkbenchInstanceUpgradeHistoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrade_history", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gce_setup` after provisioning.\n"]
    pub fn gce_setup(&self) -> ListRef<WorkbenchInstanceGceSetupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gce_setup", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkbenchInstanceTimeoutsElRef {
        WorkbenchInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for WorkbenchInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for WorkbenchInstance {}
impl ToListMappable for WorkbenchInstance {
    type O = ListRef<WorkbenchInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for WorkbenchInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_workbench_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildWorkbenchInstance {
    pub tf_id: String,
    #[doc = "Part of 'parent'. See documentation of 'projectsId'."]
    pub location: PrimField<String>,
    #[doc = "The name of this workbench instance. Format: 'projects/{project_id}/locations/{location}/instances/{instance_id}'"]
    pub name: PrimField<String>,
}
impl BuildWorkbenchInstance {
    pub fn build(self, stack: &mut Stack) -> WorkbenchInstance {
        let out = WorkbenchInstance(Rc::new(WorkbenchInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WorkbenchInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                disable_proxy_access: core::default::Default::default(),
                enable_managed_euc: core::default::Default::default(),
                enable_third_party_identity: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: core::default::Default::default(),
                instance_owners: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                gce_setup: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct WorkbenchInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl WorkbenchInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nAn RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.\nThe milliseconds portion (\".SSS\") is optional."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator` after provisioning.\nOutput only. Email address of entity that sent original CreateInstance request."]
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
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Workbench Instance. Set this field to 'ACTIVE' to start the Instance, and 'STOPPED' to stop the Instance."]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_proxy_access` after provisioning.\nOptional. If true, the workbench instance will not register with the proxy."]
    pub fn disable_proxy_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_proxy_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_managed_euc` after provisioning.\nFlag to enable managed end user credentials for the instance."]
    pub fn enable_managed_euc(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_managed_euc", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_third_party_identity` after provisioning.\nFlag that specifies that a notebook can be accessed with third party\nidentity provider."]
    pub fn enable_third_party_identity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_third_party_identity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_info` after provisioning.\n'Output only. Additional information about instance health. Example:\nhealthInfo\": { \"docker_proxy_agent_status\": \"1\", \"docker_status\": \"1\", \"jupyterlab_api_status\":\n\"-1\", \"jupyterlab_status\": \"-1\", \"updated\": \"2020-10-18 09:40:03.573409\" }'"]
    pub fn health_info(&self) -> ListRef<WorkbenchInstanceHealthInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_state` after provisioning.\nOutput only. Instance health_state."]
    pub fn health_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nRequired. User-defined unique ID of this instance."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_owners` after provisioning.\n'Optional. Input only. The owner of this instance after creation. Format:\n'alias@example.com' Currently supports one owner only. If not specified, all of\nthe service account users of your VM instance''s service account can use the instance.\nIf specified, sets the access mode to 'Single user'. For more details, see\nhttps://cloud.google.com/vertex-ai/docs/workbench/instances/manage-access-jupyterlab'"]
    pub fn instance_owners(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_owners", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to apply to this instance. These can be later modified\nby the UpdateInstance method.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this workbench instance. Format: 'projects/{project_id}/locations/{location}/instances/{instance_id}'"]
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
    #[doc = "Get a reference to the value of field `proxy_uri` after provisioning.\nOutput only. The proxy endpoint that is used to access the Jupyter notebook."]
    pub fn proxy_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.proxy_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of this instance."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nAn RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.\nThe milliseconds portion (\".SSS\") is optional."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade_history` after provisioning.\nOutput only. The upgrade history of this instance."]
    pub fn upgrade_history(&self) -> ListRef<WorkbenchInstanceUpgradeHistoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrade_history", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gce_setup` after provisioning.\n"]
    pub fn gce_setup(&self) -> ListRef<WorkbenchInstanceGceSetupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gce_setup", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkbenchInstanceTimeoutsElRef {
        WorkbenchInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceHealthInfoEl {}
impl WorkbenchInstanceHealthInfoEl {}
impl ToListMappable for WorkbenchInstanceHealthInfoEl {
    type O = BlockAssignable<WorkbenchInstanceHealthInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceHealthInfoEl {}
impl BuildWorkbenchInstanceHealthInfoEl {
    pub fn build(self) -> WorkbenchInstanceHealthInfoEl {
        WorkbenchInstanceHealthInfoEl {}
    }
}
pub struct WorkbenchInstanceHealthInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceHealthInfoElRef {
    fn new(shared: StackShared, base: String) -> WorkbenchInstanceHealthInfoElRef {
        WorkbenchInstanceHealthInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceHealthInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceUpgradeHistoryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    framework: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_image: Option<PrimField<String>>,
}
impl WorkbenchInstanceUpgradeHistoryEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }
    #[doc = "Set the field `container_image`.\n"]
    pub fn set_container_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_image = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `framework`.\n"]
    pub fn set_framework(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.framework = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot`.\n"]
    pub fn set_snapshot(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `target_version`.\n"]
    pub fn set_target_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_version = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_image`.\n"]
    pub fn set_vm_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vm_image = Some(v.into());
        self
    }
}
impl ToListMappable for WorkbenchInstanceUpgradeHistoryEl {
    type O = BlockAssignable<WorkbenchInstanceUpgradeHistoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceUpgradeHistoryEl {}
impl BuildWorkbenchInstanceUpgradeHistoryEl {
    pub fn build(self) -> WorkbenchInstanceUpgradeHistoryEl {
        WorkbenchInstanceUpgradeHistoryEl {
            action: core::default::Default::default(),
            container_image: core::default::Default::default(),
            create_time: core::default::Default::default(),
            framework: core::default::Default::default(),
            snapshot: core::default::Default::default(),
            state: core::default::Default::default(),
            target_version: core::default::Default::default(),
            version: core::default::Default::default(),
            vm_image: core::default::Default::default(),
        }
    }
}
pub struct WorkbenchInstanceUpgradeHistoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceUpgradeHistoryElRef {
    fn new(shared: StackShared, base: String) -> WorkbenchInstanceUpgradeHistoryElRef {
        WorkbenchInstanceUpgradeHistoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceUpgradeHistoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_image", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `framework` after provisioning.\n"]
    pub fn framework(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.framework", self.base))
    }
    #[doc = "Get a reference to the value of field `snapshot` after provisioning.\n"]
    pub fn snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `target_version` after provisioning.\n"]
    pub fn target_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_image` after provisioning.\n"]
    pub fn vm_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vm_image", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElAcceleratorConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    core_count: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl WorkbenchInstanceGceSetupElAcceleratorConfigsEl {
    #[doc = "Set the field `core_count`.\nOptional. Count of cores of this accelerator."]
    pub fn set_core_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.core_count = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nOptional. Type of this accelerator. Possible values: [\"NVIDIA_TESLA_P100\", \"NVIDIA_TESLA_V100\", \"NVIDIA_TESLA_P4\", \"NVIDIA_TESLA_T4\", \"NVIDIA_TESLA_A100\", \"NVIDIA_A100_80GB\", \"NVIDIA_L4\", \"NVIDIA_H100_80GB\", \"NVIDIA_H100_MEGA_80GB\", \"NVIDIA_H200_141GB\", \"NVIDIA_B200\", \"NVIDIA_TESLA_T4_VWS\", \"NVIDIA_TESLA_P100_VWS\", \"NVIDIA_TESLA_P4_VWS\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for WorkbenchInstanceGceSetupElAcceleratorConfigsEl {
    type O = BlockAssignable<WorkbenchInstanceGceSetupElAcceleratorConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceGceSetupElAcceleratorConfigsEl {}
impl BuildWorkbenchInstanceGceSetupElAcceleratorConfigsEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElAcceleratorConfigsEl {
        WorkbenchInstanceGceSetupElAcceleratorConfigsEl {
            core_count: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct WorkbenchInstanceGceSetupElAcceleratorConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceGceSetupElAcceleratorConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkbenchInstanceGceSetupElAcceleratorConfigsElRef {
        WorkbenchInstanceGceSetupElAcceleratorConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceGceSetupElAcceleratorConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `core_count` after provisioning.\nOptional. Count of cores of this accelerator."]
    pub fn core_count(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_count", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional. Type of this accelerator. Possible values: [\"NVIDIA_TESLA_P100\", \"NVIDIA_TESLA_V100\", \"NVIDIA_TESLA_P4\", \"NVIDIA_TESLA_T4\", \"NVIDIA_TESLA_A100\", \"NVIDIA_A100_80GB\", \"NVIDIA_L4\", \"NVIDIA_H100_80GB\", \"NVIDIA_H100_MEGA_80GB\", \"NVIDIA_H200_141GB\", \"NVIDIA_B200\", \"NVIDIA_TESLA_T4_VWS\", \"NVIDIA_TESLA_P100_VWS\", \"NVIDIA_TESLA_P4_VWS\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElBootDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
}
impl WorkbenchInstanceGceSetupElBootDiskEl {
    #[doc = "Set the field `disk_encryption`.\nOptional. Input only. Disk encryption method used on the boot and\ndata disks, defaults to GMEK. Possible values: [\"GMEK\", \"CMEK\"]"]
    pub fn set_disk_encryption(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_encryption = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\nOptional. The size of the boot disk in GB attached to this instance,\nup to a maximum of 64000 GB (64 TB). If not specified, this defaults to the\nrecommended value of 150GB."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\nOptional. Indicates the type of the disk. Possible values: [\"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\", \"HYPERDISK_BALANCED\", \"HYPERDISK_BALANCED_HIGH_AVAILABILITY\", \"HYPERDISK_ML\"]"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\n'Optional. The KMS key used to encrypt the disks, only\napplicable if disk_encryption is CMEK. Format: 'projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}'\nLearn more about using your own encryption keys.'"]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
}
impl ToListMappable for WorkbenchInstanceGceSetupElBootDiskEl {
    type O = BlockAssignable<WorkbenchInstanceGceSetupElBootDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceGceSetupElBootDiskEl {}
impl BuildWorkbenchInstanceGceSetupElBootDiskEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElBootDiskEl {
        WorkbenchInstanceGceSetupElBootDiskEl {
            disk_encryption: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}
pub struct WorkbenchInstanceGceSetupElBootDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceGceSetupElBootDiskElRef {
    fn new(shared: StackShared, base: String) -> WorkbenchInstanceGceSetupElBootDiskElRef {
        WorkbenchInstanceGceSetupElBootDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceGceSetupElBootDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_encryption` after provisioning.\nOptional. Input only. Disk encryption method used on the boot and\ndata disks, defaults to GMEK. Possible values: [\"GMEK\", \"CMEK\"]"]
    pub fn disk_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_encryption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nOptional. The size of the boot disk in GB attached to this instance,\nup to a maximum of 64000 GB (64 TB). If not specified, this defaults to the\nrecommended value of 150GB."]
    pub fn disk_size_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nOptional. Indicates the type of the disk. Possible values: [\"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\", \"HYPERDISK_BALANCED\", \"HYPERDISK_BALANCED_HIGH_AVAILABILITY\", \"HYPERDISK_ML\"]"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n'Optional. The KMS key used to encrypt the disks, only\napplicable if disk_encryption is CMEK. Format: 'projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}'\nLearn more about using your own encryption keys.'"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElConfidentialInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_type: Option<PrimField<String>>,
}
impl WorkbenchInstanceGceSetupElConfidentialInstanceConfigEl {
    #[doc = "Set the field `confidential_instance_type`.\nDefines the type of technology used by the confidential instance. Possible values: [\"SEV\"]"]
    pub fn set_confidential_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confidential_instance_type = Some(v.into());
        self
    }
}
impl ToListMappable for WorkbenchInstanceGceSetupElConfidentialInstanceConfigEl {
    type O = BlockAssignable<WorkbenchInstanceGceSetupElConfidentialInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceGceSetupElConfidentialInstanceConfigEl {}
impl BuildWorkbenchInstanceGceSetupElConfidentialInstanceConfigEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElConfidentialInstanceConfigEl {
        WorkbenchInstanceGceSetupElConfidentialInstanceConfigEl {
            confidential_instance_type: core::default::Default::default(),
        }
    }
}
pub struct WorkbenchInstanceGceSetupElConfidentialInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceGceSetupElConfidentialInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkbenchInstanceGceSetupElConfidentialInstanceConfigElRef {
        WorkbenchInstanceGceSetupElConfidentialInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceGceSetupElConfidentialInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confidential_instance_type` after provisioning.\nDefines the type of technology used by the confidential instance. Possible values: [\"SEV\"]"]
    pub fn confidential_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confidential_instance_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElContainerImageEl {
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl WorkbenchInstanceGceSetupElContainerImageEl {
    #[doc = "Set the field `tag`.\nThe tag of the container image. If not specified, this defaults to the latest tag."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for WorkbenchInstanceGceSetupElContainerImageEl {
    type O = BlockAssignable<WorkbenchInstanceGceSetupElContainerImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceGceSetupElContainerImageEl {
    #[doc = "The path to the container image repository.\nFor example: gcr.io/{project_id}/{imageName}"]
    pub repository: PrimField<String>,
}
impl BuildWorkbenchInstanceGceSetupElContainerImageEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElContainerImageEl {
        WorkbenchInstanceGceSetupElContainerImageEl {
            repository: self.repository,
            tag: core::default::Default::default(),
        }
    }
}
pub struct WorkbenchInstanceGceSetupElContainerImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceGceSetupElContainerImageElRef {
    fn new(shared: StackShared, base: String) -> WorkbenchInstanceGceSetupElContainerImageElRef {
        WorkbenchInstanceGceSetupElContainerImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceGceSetupElContainerImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nThe path to the container image repository.\nFor example: gcr.io/{project_id}/{imageName}"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nThe tag of the container image. If not specified, this defaults to the latest tag."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElDataDisksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
}
impl WorkbenchInstanceGceSetupElDataDisksEl {
    #[doc = "Set the field `disk_encryption`.\nOptional. Input only. Disk encryption method used on the boot\nand data disks, defaults to GMEK. Possible values: [\"GMEK\", \"CMEK\"]"]
    pub fn set_disk_encryption(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_encryption = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\nOptional. The size of the disk in GB attached to this VM instance,\nup to a maximum of 64000 GB (64 TB). If not specified, this defaults to\n100."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\nOptional. Input only. Indicates the type of the disk. Possible values: [\"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\", \"HYPERDISK_BALANCED\", \"HYPERDISK_EXTREME\", \"HYPERDISK_THROUGHPUT\", \"HYPERDISK_BALANCED_HIGH_AVAILABILITY\", \"HYPERDISK_ML\"]"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\n'Optional. The KMS key used to encrypt the disks,\nonly applicable if disk_encryption is CMEK. Format: 'projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}'\nLearn more about using your own encryption keys.'"]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
}
impl ToListMappable for WorkbenchInstanceGceSetupElDataDisksEl {
    type O = BlockAssignable<WorkbenchInstanceGceSetupElDataDisksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceGceSetupElDataDisksEl {}
impl BuildWorkbenchInstanceGceSetupElDataDisksEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElDataDisksEl {
        WorkbenchInstanceGceSetupElDataDisksEl {
            disk_encryption: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}
pub struct WorkbenchInstanceGceSetupElDataDisksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceGceSetupElDataDisksElRef {
    fn new(shared: StackShared, base: String) -> WorkbenchInstanceGceSetupElDataDisksElRef {
        WorkbenchInstanceGceSetupElDataDisksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceGceSetupElDataDisksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_encryption` after provisioning.\nOptional. Input only. Disk encryption method used on the boot\nand data disks, defaults to GMEK. Possible values: [\"GMEK\", \"CMEK\"]"]
    pub fn disk_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_encryption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nOptional. The size of the disk in GB attached to this VM instance,\nup to a maximum of 64000 GB (64 TB). If not specified, this defaults to\n100."]
    pub fn disk_size_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nOptional. Input only. Indicates the type of the disk. Possible values: [\"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\", \"HYPERDISK_BALANCED\", \"HYPERDISK_EXTREME\", \"HYPERDISK_THROUGHPUT\", \"HYPERDISK_BALANCED_HIGH_AVAILABILITY\", \"HYPERDISK_ML\"]"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n'Optional. The KMS key used to encrypt the disks,\nonly applicable if disk_encryption is CMEK. Format: 'projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}'\nLearn more about using your own encryption keys.'"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsEl {
    external_ip: PrimField<String>,
}
impl WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsEl {}
impl ToListMappable for WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsEl {
    type O = BlockAssignable<WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsEl {
    #[doc = "An external IP address associated with this instance. Specify an unused\nstatic external IP address available to the project or leave this field\nundefined to use an IP from a shared ephemeral IP address pool. If you\nspecify a static external IP address, it must live in the same region as\nthe zone of the instance."]
    pub external_ip: PrimField<String>,
}
impl BuildWorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsEl {
        WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsEl {
            external_ip: self.external_ip,
        }
    }
}
pub struct WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsElRef {
        WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_ip` after provisioning.\nAn external IP address associated with this instance. Specify an unused\nstatic external IP address available to the project or leave this field\nundefined to use an IP from a shared ephemeral IP address pool. If you\nspecify a static external IP address, it must live in the same region as\nthe zone of the instance."]
    pub fn external_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_ip", self.base))
    }
}
#[derive(Serialize, Default)]
struct WorkbenchInstanceGceSetupElNetworkInterfacesElDynamic {
    access_configs:
        Option<DynamicBlock<WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsEl>>,
}
#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElNetworkInterfacesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nic_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_configs: Option<Vec<WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsEl>>,
    dynamic: WorkbenchInstanceGceSetupElNetworkInterfacesElDynamic,
}
impl WorkbenchInstanceGceSetupElNetworkInterfacesEl {
    #[doc = "Set the field `network`.\nOptional. The name of the VPC that this VM instance is in."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `nic_type`.\nOptional. The type of vNIC to be used on this interface. This\nmay be gVNIC or VirtioNet. Possible values: [\"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn set_nic_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nic_type = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet`.\nOptional. The name of the subnet that this VM instance is in."]
    pub fn set_subnet(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet = Some(v.into());
        self
    }
    #[doc = "Set the field `access_configs`.\n"]
    pub fn set_access_configs(
        mut self,
        v: impl Into<BlockAssignable<WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for WorkbenchInstanceGceSetupElNetworkInterfacesEl {
    type O = BlockAssignable<WorkbenchInstanceGceSetupElNetworkInterfacesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceGceSetupElNetworkInterfacesEl {}
impl BuildWorkbenchInstanceGceSetupElNetworkInterfacesEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElNetworkInterfacesEl {
        WorkbenchInstanceGceSetupElNetworkInterfacesEl {
            network: core::default::Default::default(),
            nic_type: core::default::Default::default(),
            subnet: core::default::Default::default(),
            access_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WorkbenchInstanceGceSetupElNetworkInterfacesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceGceSetupElNetworkInterfacesElRef {
    fn new(shared: StackShared, base: String) -> WorkbenchInstanceGceSetupElNetworkInterfacesElRef {
        WorkbenchInstanceGceSetupElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceGceSetupElNetworkInterfacesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nOptional. The name of the VPC that this VM instance is in."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `nic_type` after provisioning.\nOptional. The type of vNIC to be used on this interface. This\nmay be gVNIC or VirtioNet. Possible values: [\"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn nic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nic_type", self.base))
    }
    #[doc = "Get a reference to the value of field `subnet` after provisioning.\nOptional. The name of the subnet that this VM instance is in."]
    pub fn subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
    #[doc = "Get a reference to the value of field `access_configs` after provisioning.\n"]
    pub fn access_configs(
        &self,
    ) -> ListRef<WorkbenchInstanceGceSetupElNetworkInterfacesElAccessConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElReservationAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consume_reservation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl WorkbenchInstanceGceSetupElReservationAffinityEl {
    #[doc = "Set the field `consume_reservation_type`.\nSpecifies the type of reservation from which this instance can consume resources:\nRESERVATION_ANY (default), RESERVATION_SPECIFIC, or RESERVATION_NONE. Possible values: [\"RESERVATION_NONE\", \"RESERVATION_ANY\", \"RESERVATION_SPECIFIC\"]"]
    pub fn set_consume_reservation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consume_reservation_type = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\nCorresponds to the label key of a reservation resource. To target a\nRESERVATION_SPECIFIC by name, use compute.googleapis.com/reservation-name\nas the key and specify the name of your reservation as its value."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\nCorresponds to the label values of a reservation resource. This can be\neither a name to a reservation in the same project or\n\"projects/different-project/reservations/some-reservation-name\"\nto target a shared reservation in the same zone but in a different project."]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for WorkbenchInstanceGceSetupElReservationAffinityEl {
    type O = BlockAssignable<WorkbenchInstanceGceSetupElReservationAffinityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceGceSetupElReservationAffinityEl {}
impl BuildWorkbenchInstanceGceSetupElReservationAffinityEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElReservationAffinityEl {
        WorkbenchInstanceGceSetupElReservationAffinityEl {
            consume_reservation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct WorkbenchInstanceGceSetupElReservationAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceGceSetupElReservationAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkbenchInstanceGceSetupElReservationAffinityElRef {
        WorkbenchInstanceGceSetupElReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceGceSetupElReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consume_reservation_type` after provisioning.\nSpecifies the type of reservation from which this instance can consume resources:\nRESERVATION_ANY (default), RESERVATION_SPECIFIC, or RESERVATION_NONE. Possible values: [\"RESERVATION_NONE\", \"RESERVATION_ANY\", \"RESERVATION_SPECIFIC\"]"]
    pub fn consume_reservation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consume_reservation_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nCorresponds to the label key of a reservation resource. To target a\nRESERVATION_SPECIFIC by name, use compute.googleapis.com/reservation-name\nas the key and specify the name of your reservation as its value."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nCorresponds to the label values of a reservation resource. This can be\neither a name to a reservation in the same project or\n\"projects/different-project/reservations/some-reservation-name\"\nto target a shared reservation in the same zone but in a different project."]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElServiceAccountsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
}
impl WorkbenchInstanceGceSetupElServiceAccountsEl {
    #[doc = "Set the field `email`.\nOptional. Email address of the service account."]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
}
impl ToListMappable for WorkbenchInstanceGceSetupElServiceAccountsEl {
    type O = BlockAssignable<WorkbenchInstanceGceSetupElServiceAccountsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceGceSetupElServiceAccountsEl {}
impl BuildWorkbenchInstanceGceSetupElServiceAccountsEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElServiceAccountsEl {
        WorkbenchInstanceGceSetupElServiceAccountsEl {
            email: core::default::Default::default(),
        }
    }
}
pub struct WorkbenchInstanceGceSetupElServiceAccountsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceGceSetupElServiceAccountsElRef {
    fn new(shared: StackShared, base: String) -> WorkbenchInstanceGceSetupElServiceAccountsElRef {
        WorkbenchInstanceGceSetupElServiceAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceGceSetupElServiceAccountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nOptional. Email address of the service account."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nOutput only. The list of scopes to be made available for this\nservice account. Set by the CLH to https://www.googleapis.com/auth/cloud-platform"]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_vtpm: Option<PrimField<bool>>,
}
impl WorkbenchInstanceGceSetupElShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\nOptional. Defines whether the VM instance has integrity monitoring\nenabled. Enables monitoring and attestation of the boot integrity of the VM\ninstance. The attestation is performed against the integrity policy baseline.\nThis baseline is initially derived from the implicitly trusted boot image\nwhen the VM instance is created. Enabled by default."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_secure_boot`.\nOptional. Defines whether the VM instance has Secure Boot enabled.\nSecure Boot helps ensure that the system only runs authentic software by verifying\nthe digital signature of all boot components, and halting the boot process\nif signature verification fails. Disabled by default."]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_vtpm`.\nOptional. Defines whether the VM instance has the vTPM enabled.\nEnabled by default."]
    pub fn set_enable_vtpm(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_vtpm = Some(v.into());
        self
    }
}
impl ToListMappable for WorkbenchInstanceGceSetupElShieldedInstanceConfigEl {
    type O = BlockAssignable<WorkbenchInstanceGceSetupElShieldedInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceGceSetupElShieldedInstanceConfigEl {}
impl BuildWorkbenchInstanceGceSetupElShieldedInstanceConfigEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElShieldedInstanceConfigEl {
        WorkbenchInstanceGceSetupElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}
pub struct WorkbenchInstanceGceSetupElShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceGceSetupElShieldedInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkbenchInstanceGceSetupElShieldedInstanceConfigElRef {
        WorkbenchInstanceGceSetupElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceGceSetupElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nOptional. Defines whether the VM instance has integrity monitoring\nenabled. Enables monitoring and attestation of the boot integrity of the VM\ninstance. The attestation is performed against the integrity policy baseline.\nThis baseline is initially derived from the implicitly trusted boot image\nwhen the VM instance is created. Enabled by default."]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_integrity_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\nOptional. Defines whether the VM instance has Secure Boot enabled.\nSecure Boot helps ensure that the system only runs authentic software by verifying\nthe digital signature of all boot components, and halting the boot process\nif signature verification fails. Disabled by default."]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_vtpm` after provisioning.\nOptional. Defines whether the VM instance has the vTPM enabled.\nEnabled by default."]
    pub fn enable_vtpm(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElVmImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
impl WorkbenchInstanceGceSetupElVmImageEl {
    #[doc = "Set the field `family`.\nOptional. Use this VM image family to find the image; the newest\nimage in this family will be used."]
    pub fn set_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.family = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nOptional. Use VM image name to find the image."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe name of the Google Cloud project that this VM image belongs to.\nFormat: {project_id}"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
}
impl ToListMappable for WorkbenchInstanceGceSetupElVmImageEl {
    type O = BlockAssignable<WorkbenchInstanceGceSetupElVmImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceGceSetupElVmImageEl {}
impl BuildWorkbenchInstanceGceSetupElVmImageEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElVmImageEl {
        WorkbenchInstanceGceSetupElVmImageEl {
            family: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}
pub struct WorkbenchInstanceGceSetupElVmImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceGceSetupElVmImageElRef {
    fn new(shared: StackShared, base: String) -> WorkbenchInstanceGceSetupElVmImageElRef {
        WorkbenchInstanceGceSetupElVmImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceGceSetupElVmImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `family` after provisioning.\nOptional. Use this VM image family to find the image; the newest\nimage in this family will be used."]
    pub fn family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.family", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOptional. Use VM image name to find the image."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe name of the Google Cloud project that this VM image belongs to.\nFormat: {project_id}"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}
#[derive(Serialize, Default)]
struct WorkbenchInstanceGceSetupElDynamic {
    accelerator_configs: Option<DynamicBlock<WorkbenchInstanceGceSetupElAcceleratorConfigsEl>>,
    boot_disk: Option<DynamicBlock<WorkbenchInstanceGceSetupElBootDiskEl>>,
    confidential_instance_config:
        Option<DynamicBlock<WorkbenchInstanceGceSetupElConfidentialInstanceConfigEl>>,
    container_image: Option<DynamicBlock<WorkbenchInstanceGceSetupElContainerImageEl>>,
    data_disks: Option<DynamicBlock<WorkbenchInstanceGceSetupElDataDisksEl>>,
    network_interfaces: Option<DynamicBlock<WorkbenchInstanceGceSetupElNetworkInterfacesEl>>,
    reservation_affinity: Option<DynamicBlock<WorkbenchInstanceGceSetupElReservationAffinityEl>>,
    service_accounts: Option<DynamicBlock<WorkbenchInstanceGceSetupElServiceAccountsEl>>,
    shielded_instance_config:
        Option<DynamicBlock<WorkbenchInstanceGceSetupElShieldedInstanceConfigEl>>,
    vm_image: Option<DynamicBlock<WorkbenchInstanceGceSetupElVmImageEl>>,
}
#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ip_forwarding: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_configs: Option<Vec<WorkbenchInstanceGceSetupElAcceleratorConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk: Option<Vec<WorkbenchInstanceGceSetupElBootDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_config:
        Option<Vec<WorkbenchInstanceGceSetupElConfidentialInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_image: Option<Vec<WorkbenchInstanceGceSetupElContainerImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_disks: Option<Vec<WorkbenchInstanceGceSetupElDataDisksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interfaces: Option<Vec<WorkbenchInstanceGceSetupElNetworkInterfacesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_affinity: Option<Vec<WorkbenchInstanceGceSetupElReservationAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_accounts: Option<Vec<WorkbenchInstanceGceSetupElServiceAccountsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config: Option<Vec<WorkbenchInstanceGceSetupElShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_image: Option<Vec<WorkbenchInstanceGceSetupElVmImageEl>>,
    dynamic: WorkbenchInstanceGceSetupElDynamic,
}
impl WorkbenchInstanceGceSetupEl {
    #[doc = "Set the field `disable_public_ip`.\nOptional. If true, no external IP will be assigned to this VM instance."]
    pub fn set_disable_public_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_public_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_ip_forwarding`.\nOptional. Flag to enable ip forwarding or not, default false/off.\nhttps://cloud.google.com/vpc/docs/using-routes#canipforward"]
    pub fn set_enable_ip_forwarding(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ip_forwarding = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nOptional. The machine type of the VM instance. https://cloud.google.com/compute/docs/machine-resource"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\nOptional. Custom metadata to apply to this instance."]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nOptional. The Compute Engine tags to add to instance (see [Tagging\ninstances](https://cloud.google.com/compute/docs/label-or-tag-resources#tags))."]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_configs`.\n"]
    pub fn set_accelerator_configs(
        mut self,
        v: impl Into<BlockAssignable<WorkbenchInstanceGceSetupElAcceleratorConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerator_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(
        mut self,
        v: impl Into<BlockAssignable<WorkbenchInstanceGceSetupElBootDiskEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.boot_disk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.boot_disk = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `confidential_instance_config`.\n"]
    pub fn set_confidential_instance_config(
        mut self,
        v: impl Into<BlockAssignable<WorkbenchInstanceGceSetupElConfidentialInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.confidential_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.confidential_instance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `container_image`.\n"]
    pub fn set_container_image(
        mut self,
        v: impl Into<BlockAssignable<WorkbenchInstanceGceSetupElContainerImageEl>>,
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
    #[doc = "Set the field `data_disks`.\n"]
    pub fn set_data_disks(
        mut self,
        v: impl Into<BlockAssignable<WorkbenchInstanceGceSetupElDataDisksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_disks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_disks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_interfaces`.\n"]
    pub fn set_network_interfaces(
        mut self,
        v: impl Into<BlockAssignable<WorkbenchInstanceGceSetupElNetworkInterfacesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_interfaces = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interfaces = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v: impl Into<BlockAssignable<WorkbenchInstanceGceSetupElReservationAffinityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.reservation_affinity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.reservation_affinity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_accounts`.\n"]
    pub fn set_service_accounts(
        mut self,
        v: impl Into<BlockAssignable<WorkbenchInstanceGceSetupElServiceAccountsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_accounts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_accounts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl Into<BlockAssignable<WorkbenchInstanceGceSetupElShieldedInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.shielded_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vm_image`.\n"]
    pub fn set_vm_image(
        mut self,
        v: impl Into<BlockAssignable<WorkbenchInstanceGceSetupElVmImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vm_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vm_image = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for WorkbenchInstanceGceSetupEl {
    type O = BlockAssignable<WorkbenchInstanceGceSetupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceGceSetupEl {}
impl BuildWorkbenchInstanceGceSetupEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupEl {
        WorkbenchInstanceGceSetupEl {
            disable_public_ip: core::default::Default::default(),
            enable_ip_forwarding: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            metadata: core::default::Default::default(),
            tags: core::default::Default::default(),
            accelerator_configs: core::default::Default::default(),
            boot_disk: core::default::Default::default(),
            confidential_instance_config: core::default::Default::default(),
            container_image: core::default::Default::default(),
            data_disks: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            service_accounts: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            vm_image: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WorkbenchInstanceGceSetupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceGceSetupElRef {
    fn new(shared: StackShared, base: String) -> WorkbenchInstanceGceSetupElRef {
        WorkbenchInstanceGceSetupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceGceSetupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_public_ip` after provisioning.\nOptional. If true, no external IP will be assigned to this VM instance."]
    pub fn disable_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_public_ip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_ip_forwarding` after provisioning.\nOptional. Flag to enable ip forwarding or not, default false/off.\nhttps://cloud.google.com/vpc/docs/using-routes#canipforward"]
    pub fn enable_ip_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ip_forwarding", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nOptional. The machine type of the VM instance. https://cloud.google.com/compute/docs/machine-resource"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nOptional. Custom metadata to apply to this instance."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nOptional. The Compute Engine tags to add to instance (see [Tagging\ninstances](https://cloud.google.com/compute/docs/label-or-tag-resources#tags))."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `accelerator_configs` after provisioning.\n"]
    pub fn accelerator_configs(
        &self,
    ) -> ListRef<WorkbenchInstanceGceSetupElAcceleratorConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> ListRef<WorkbenchInstanceGceSetupElBootDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }
    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(
        &self,
    ) -> ListRef<WorkbenchInstanceGceSetupElConfidentialInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_instance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(&self) -> ListRef<WorkbenchInstanceGceSetupElContainerImageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_image", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_disks` after provisioning.\n"]
    pub fn data_disks(&self) -> ListRef<WorkbenchInstanceGceSetupElDataDisksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_disks", self.base))
    }
    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(&self) -> ListRef<WorkbenchInstanceGceSetupElNetworkInterfacesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interfaces", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(
        &self,
    ) -> ListRef<WorkbenchInstanceGceSetupElReservationAffinityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_affinity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_accounts` after provisioning.\n"]
    pub fn service_accounts(&self) -> ListRef<WorkbenchInstanceGceSetupElServiceAccountsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_accounts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<WorkbenchInstanceGceSetupElShieldedInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vm_image` after provisioning.\n"]
    pub fn vm_image(&self) -> ListRef<WorkbenchInstanceGceSetupElVmImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vm_image", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkbenchInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl WorkbenchInstanceTimeoutsEl {
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
impl ToListMappable for WorkbenchInstanceTimeoutsEl {
    type O = BlockAssignable<WorkbenchInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkbenchInstanceTimeoutsEl {}
impl BuildWorkbenchInstanceTimeoutsEl {
    pub fn build(self) -> WorkbenchInstanceTimeoutsEl {
        WorkbenchInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct WorkbenchInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkbenchInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> WorkbenchInstanceTimeoutsElRef {
        WorkbenchInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkbenchInstanceTimeoutsElRef {
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
struct WorkbenchInstanceDynamic {
    gce_setup: Option<DynamicBlock<WorkbenchInstanceGceSetupEl>>,
}
