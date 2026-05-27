use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DeveloperConnectInsightsConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_hub_application: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    insights_config_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_configs: Option<Vec<DeveloperConnectInsightsConfigArtifactConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_projects: Option<Vec<DeveloperConnectInsightsConfigTargetProjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DeveloperConnectInsightsConfigTimeoutsEl>,
    dynamic: DeveloperConnectInsightsConfigDynamic,
}
struct DeveloperConnectInsightsConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DeveloperConnectInsightsConfigData>,
}
#[derive(Clone)]
pub struct DeveloperConnectInsightsConfig(Rc<DeveloperConnectInsightsConfig_>);
impl DeveloperConnectInsightsConfig {
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
    #[doc = "Set the field `annotations`.\nUser specified annotations. See https://google.aip.dev/148#annotations\nfor more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `app_hub_application`.\nThe name of the App Hub Application.\nFormat:\nprojects/{project}/locations/{location}/applications/{application}"]
    pub fn set_app_hub_application(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().app_hub_application = Some(v.into());
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
    #[doc = "Set the field `labels`.\nSet of labels associated with an InsightsConfig.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `artifact_configs`.\n"]
    pub fn set_artifact_configs(
        self,
        v: impl Into<BlockAssignable<DeveloperConnectInsightsConfigArtifactConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().artifact_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.artifact_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target_projects`.\n"]
    pub fn set_target_projects(
        self,
        v: impl Into<BlockAssignable<DeveloperConnectInsightsConfigTargetProjectsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_projects = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_projects = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DeveloperConnectInsightsConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser specified annotations. See https://google.aip.dev/148#annotations\nfor more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app_hub_application` after provisioning.\nThe name of the App Hub Application.\nFormat:\nprojects/{project}/locations/{location}/applications/{application}"]
    pub fn app_hub_application(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_hub_application", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create timestamp"]
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
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `errors` after provisioning.\nAny errors that occurred while setting up the InsightsConfig.\nEach error will be in the format: 'field_name: error_message', e.g.\nGetAppHubApplication: Permission denied while getting App Hub\napplication. Please grant permissions to the P4SA."]
    pub fn errors(&self) -> ListRef<DeveloperConnectInsightsConfigErrorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.errors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `insights_config_id` after provisioning.\nID of the requesting InsightsConfig."]
    pub fn insights_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.insights_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with an InsightsConfig.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the InsightsConfig.\nFormat:\nprojects/{project}/locations/{location}/insightsConfigs/{insightsConfig}"]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nReconciling (https://google.aip.dev/128#reconciliation).\nSet to true if the current state of InsightsConfig does not match the\nuser's intended state, and the service is actively updating the resource to\nreconcile them. This can happen due to user-triggered updates or\nsystem actions like failover or maintenance."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_configs` after provisioning.\nThe runtime configurations where the application is deployed."]
    pub fn runtime_configs(&self) -> ListRef<DeveloperConnectInsightsConfigRuntimeConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the InsightsConfig.\nPossible values:\nSTATE_UNSPECIFIED\nPENDING\nCOMPLETE\nERROR"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update timestamp"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `artifact_configs` after provisioning.\n"]
    pub fn artifact_configs(&self) -> ListRef<DeveloperConnectInsightsConfigArtifactConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.artifact_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_projects` after provisioning.\n"]
    pub fn target_projects(&self) -> ListRef<DeveloperConnectInsightsConfigTargetProjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_projects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeveloperConnectInsightsConfigTimeoutsElRef {
        DeveloperConnectInsightsConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DeveloperConnectInsightsConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DeveloperConnectInsightsConfig {}
impl ToListMappable for DeveloperConnectInsightsConfig {
    type O = ListRef<DeveloperConnectInsightsConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DeveloperConnectInsightsConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_developer_connect_insights_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDeveloperConnectInsightsConfig {
    pub tf_id: String,
    #[doc = "ID of the requesting InsightsConfig."]
    pub insights_config_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildDeveloperConnectInsightsConfig {
    pub fn build(self, stack: &mut Stack) -> DeveloperConnectInsightsConfig {
        let out = DeveloperConnectInsightsConfig(Rc::new(DeveloperConnectInsightsConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DeveloperConnectInsightsConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                app_hub_application: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                insights_config_id: self.insights_config_id,
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                artifact_configs: core::default::Default::default(),
                target_projects: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DeveloperConnectInsightsConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectInsightsConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DeveloperConnectInsightsConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser specified annotations. See https://google.aip.dev/148#annotations\nfor more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app_hub_application` after provisioning.\nThe name of the App Hub Application.\nFormat:\nprojects/{project}/locations/{location}/applications/{application}"]
    pub fn app_hub_application(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_hub_application", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create timestamp"]
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
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `errors` after provisioning.\nAny errors that occurred while setting up the InsightsConfig.\nEach error will be in the format: 'field_name: error_message', e.g.\nGetAppHubApplication: Permission denied while getting App Hub\napplication. Please grant permissions to the P4SA."]
    pub fn errors(&self) -> ListRef<DeveloperConnectInsightsConfigErrorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.errors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `insights_config_id` after provisioning.\nID of the requesting InsightsConfig."]
    pub fn insights_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.insights_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with an InsightsConfig.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the InsightsConfig.\nFormat:\nprojects/{project}/locations/{location}/insightsConfigs/{insightsConfig}"]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nReconciling (https://google.aip.dev/128#reconciliation).\nSet to true if the current state of InsightsConfig does not match the\nuser's intended state, and the service is actively updating the resource to\nreconcile them. This can happen due to user-triggered updates or\nsystem actions like failover or maintenance."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_configs` after provisioning.\nThe runtime configurations where the application is deployed."]
    pub fn runtime_configs(&self) -> ListRef<DeveloperConnectInsightsConfigRuntimeConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the InsightsConfig.\nPossible values:\nSTATE_UNSPECIFIED\nPENDING\nCOMPLETE\nERROR"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update timestamp"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `artifact_configs` after provisioning.\n"]
    pub fn artifact_configs(&self) -> ListRef<DeveloperConnectInsightsConfigArtifactConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.artifact_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_projects` after provisioning.\n"]
    pub fn target_projects(&self) -> ListRef<DeveloperConnectInsightsConfigTargetProjectsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_projects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeveloperConnectInsightsConfigTimeoutsElRef {
        DeveloperConnectInsightsConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfigErrorsElDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    detail_message: Option<PrimField<String>>,
}
impl DeveloperConnectInsightsConfigErrorsElDetailsEl {
    #[doc = "Set the field `detail_message`.\n"]
    pub fn set_detail_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.detail_message = Some(v.into());
        self
    }
}
impl ToListMappable for DeveloperConnectInsightsConfigErrorsElDetailsEl {
    type O = BlockAssignable<DeveloperConnectInsightsConfigErrorsElDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectInsightsConfigErrorsElDetailsEl {}
impl BuildDeveloperConnectInsightsConfigErrorsElDetailsEl {
    pub fn build(self) -> DeveloperConnectInsightsConfigErrorsElDetailsEl {
        DeveloperConnectInsightsConfigErrorsElDetailsEl {
            detail_message: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectInsightsConfigErrorsElDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectInsightsConfigErrorsElDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectInsightsConfigErrorsElDetailsElRef {
        DeveloperConnectInsightsConfigErrorsElDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectInsightsConfigErrorsElDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `detail_message` after provisioning.\n"]
    pub fn detail_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detail_message", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfigErrorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<ListField<DeveloperConnectInsightsConfigErrorsElDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl DeveloperConnectInsightsConfigErrorsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl Into<ListField<DeveloperConnectInsightsConfigErrorsElDetailsEl>>,
    ) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for DeveloperConnectInsightsConfigErrorsEl {
    type O = BlockAssignable<DeveloperConnectInsightsConfigErrorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectInsightsConfigErrorsEl {}
impl BuildDeveloperConnectInsightsConfigErrorsEl {
    pub fn build(self) -> DeveloperConnectInsightsConfigErrorsEl {
        DeveloperConnectInsightsConfigErrorsEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectInsightsConfigErrorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectInsightsConfigErrorsElRef {
    fn new(shared: StackShared, base: String) -> DeveloperConnectInsightsConfigErrorsElRef {
        DeveloperConnectInsightsConfigErrorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectInsightsConfigErrorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<DeveloperConnectInsightsConfigErrorsElDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    criticality: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload: Option<PrimField<String>>,
}
impl DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadEl {
    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.criticality = Some(v.into());
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment = Some(v.into());
        self
    }
    #[doc = "Set the field `workload`.\n"]
    pub fn set_workload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workload = Some(v.into());
        self
    }
}
impl ToListMappable for DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadEl {
    type O = BlockAssignable<DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadEl {}
impl BuildDeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadEl {
    pub fn build(self) -> DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadEl {
        DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadEl {
            criticality: core::default::Default::default(),
            environment: core::default::Default::default(),
            workload: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadElRef {
        DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `criticality` after provisioning.\n"]
    pub fn criticality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.criticality", self.base))
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }
    #[doc = "Get a reference to the value of field `workload` after provisioning.\n"]
    pub fn workload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workload", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment: Option<PrimField<String>>,
}
impl DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadEl {
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `deployment`.\n"]
    pub fn set_deployment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deployment = Some(v.into());
        self
    }
}
impl ToListMappable for DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadEl {
    type O = BlockAssignable<DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadEl {}
impl BuildDeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadEl {
    pub fn build(self) -> DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadEl {
        DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadEl {
            cluster: core::default::Default::default(),
            deployment: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadElRef {
        DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfigRuntimeConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_hub_workload:
        Option<ListField<DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_workload: Option<ListField<DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DeveloperConnectInsightsConfigRuntimeConfigsEl {
    #[doc = "Set the field `app_hub_workload`.\n"]
    pub fn set_app_hub_workload(
        mut self,
        v: impl Into<ListField<DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadEl>>,
    ) -> Self {
        self.app_hub_workload = Some(v.into());
        self
    }
    #[doc = "Set the field `gke_workload`.\n"]
    pub fn set_gke_workload(
        mut self,
        v: impl Into<ListField<DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadEl>>,
    ) -> Self {
        self.gke_workload = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for DeveloperConnectInsightsConfigRuntimeConfigsEl {
    type O = BlockAssignable<DeveloperConnectInsightsConfigRuntimeConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectInsightsConfigRuntimeConfigsEl {}
impl BuildDeveloperConnectInsightsConfigRuntimeConfigsEl {
    pub fn build(self) -> DeveloperConnectInsightsConfigRuntimeConfigsEl {
        DeveloperConnectInsightsConfigRuntimeConfigsEl {
            app_hub_workload: core::default::Default::default(),
            gke_workload: core::default::Default::default(),
            state: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectInsightsConfigRuntimeConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectInsightsConfigRuntimeConfigsElRef {
    fn new(shared: StackShared, base: String) -> DeveloperConnectInsightsConfigRuntimeConfigsElRef {
        DeveloperConnectInsightsConfigRuntimeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectInsightsConfigRuntimeConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_hub_workload` after provisioning.\n"]
    pub fn app_hub_workload(
        &self,
    ) -> ListRef<DeveloperConnectInsightsConfigRuntimeConfigsElAppHubWorkloadElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_hub_workload", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gke_workload` after provisioning.\n"]
    pub fn gke_workload(
        &self,
    ) -> ListRef<DeveloperConnectInsightsConfigRuntimeConfigsElGkeWorkloadElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gke_workload", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisEl {
    project_id: PrimField<String>,
}
impl DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisEl {}
impl ToListMappable for DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisEl {
    type O =
        BlockAssignable<DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisEl {
    #[doc = "The project id of the project where the provenance is stored."]
    pub project_id: PrimField<String>,
}
impl BuildDeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisEl {
    pub fn build(self) -> DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisEl {
        DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisEl {
            project_id: self.project_id,
        }
    }
}
pub struct DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisElRef {
        DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe project id of the project where the provenance is stored."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryEl {
    artifact_registry_package: PrimField<String>,
    project_id: PrimField<String>,
}
impl DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryEl {}
impl ToListMappable for DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryEl {
    type O =
        BlockAssignable<DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryEl {
    #[doc = "The name of the artifact registry package."]
    pub artifact_registry_package: PrimField<String>,
    #[doc = "The host project of Artifact Registry."]
    pub project_id: PrimField<String>,
}
impl BuildDeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryEl {
    pub fn build(self) -> DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryEl {
        DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryEl {
            artifact_registry_package: self.artifact_registry_package,
            project_id: self.project_id,
        }
    }
}
pub struct DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryElRef {
        DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `artifact_registry_package` after provisioning.\nThe name of the artifact registry package."]
    pub fn artifact_registry_package(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.artifact_registry_package", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe host project of Artifact Registry."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct DeveloperConnectInsightsConfigArtifactConfigsElDynamic {
    google_artifact_analysis: Option<
        DynamicBlock<DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisEl>,
    >,
    google_artifact_registry: Option<
        DynamicBlock<DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryEl>,
    >,
}
#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfigArtifactConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_artifact_analysis:
        Option<Vec<DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_artifact_registry:
        Option<Vec<DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryEl>>,
    dynamic: DeveloperConnectInsightsConfigArtifactConfigsElDynamic,
}
impl DeveloperConnectInsightsConfigArtifactConfigsEl {
    #[doc = "Set the field `uri`.\nThe URI of the artifact that is deployed.\ne.g. 'us-docker.pkg.dev/my-project/my-repo/image'.\nThe URI does not include the tag / digest because it captures a lineage of\nartifacts."]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
    #[doc = "Set the field `google_artifact_analysis`.\n"]
    pub fn set_google_artifact_analysis(
        mut self,
        v: impl Into<
            BlockAssignable<
                DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_artifact_analysis = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_artifact_analysis = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `google_artifact_registry`.\n"]
    pub fn set_google_artifact_registry(
        mut self,
        v: impl Into<
            BlockAssignable<
                DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_artifact_registry = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_artifact_registry = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DeveloperConnectInsightsConfigArtifactConfigsEl {
    type O = BlockAssignable<DeveloperConnectInsightsConfigArtifactConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectInsightsConfigArtifactConfigsEl {}
impl BuildDeveloperConnectInsightsConfigArtifactConfigsEl {
    pub fn build(self) -> DeveloperConnectInsightsConfigArtifactConfigsEl {
        DeveloperConnectInsightsConfigArtifactConfigsEl {
            uri: core::default::Default::default(),
            google_artifact_analysis: core::default::Default::default(),
            google_artifact_registry: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DeveloperConnectInsightsConfigArtifactConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectInsightsConfigArtifactConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DeveloperConnectInsightsConfigArtifactConfigsElRef {
        DeveloperConnectInsightsConfigArtifactConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectInsightsConfigArtifactConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe URI of the artifact that is deployed.\ne.g. 'us-docker.pkg.dev/my-project/my-repo/image'.\nThe URI does not include the tag / digest because it captures a lineage of\nartifacts."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
    #[doc = "Get a reference to the value of field `google_artifact_analysis` after provisioning.\n"]
    pub fn google_artifact_analysis(
        &self,
    ) -> ListRef<DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactAnalysisElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_artifact_analysis", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `google_artifact_registry` after provisioning.\n"]
    pub fn google_artifact_registry(
        &self,
    ) -> ListRef<DeveloperConnectInsightsConfigArtifactConfigsElGoogleArtifactRegistryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_artifact_registry", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfigTargetProjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    project_ids: Option<ListField<PrimField<String>>>,
}
impl DeveloperConnectInsightsConfigTargetProjectsEl {
    #[doc = "Set the field `project_ids`.\nThe project IDs. Format {project}."]
    pub fn set_project_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.project_ids = Some(v.into());
        self
    }
}
impl ToListMappable for DeveloperConnectInsightsConfigTargetProjectsEl {
    type O = BlockAssignable<DeveloperConnectInsightsConfigTargetProjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectInsightsConfigTargetProjectsEl {}
impl BuildDeveloperConnectInsightsConfigTargetProjectsEl {
    pub fn build(self) -> DeveloperConnectInsightsConfigTargetProjectsEl {
        DeveloperConnectInsightsConfigTargetProjectsEl {
            project_ids: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectInsightsConfigTargetProjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectInsightsConfigTargetProjectsElRef {
    fn new(shared: StackShared, base: String) -> DeveloperConnectInsightsConfigTargetProjectsElRef {
        DeveloperConnectInsightsConfigTargetProjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectInsightsConfigTargetProjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project_ids` after provisioning.\nThe project IDs. Format {project}."]
    pub fn project_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.project_ids", self.base))
    }
}
#[derive(Serialize)]
pub struct DeveloperConnectInsightsConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DeveloperConnectInsightsConfigTimeoutsEl {
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
impl ToListMappable for DeveloperConnectInsightsConfigTimeoutsEl {
    type O = BlockAssignable<DeveloperConnectInsightsConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeveloperConnectInsightsConfigTimeoutsEl {}
impl BuildDeveloperConnectInsightsConfigTimeoutsEl {
    pub fn build(self) -> DeveloperConnectInsightsConfigTimeoutsEl {
        DeveloperConnectInsightsConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DeveloperConnectInsightsConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeveloperConnectInsightsConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DeveloperConnectInsightsConfigTimeoutsElRef {
        DeveloperConnectInsightsConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeveloperConnectInsightsConfigTimeoutsElRef {
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
struct DeveloperConnectInsightsConfigDynamic {
    artifact_configs: Option<DynamicBlock<DeveloperConnectInsightsConfigArtifactConfigsEl>>,
    target_projects: Option<DynamicBlock<DeveloperConnectInsightsConfigTargetProjectsEl>>,
}
