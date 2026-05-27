use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppEngineStandardAppVersionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_engine_apis: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_service_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inbound_services: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    noop_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    runtime: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_api_version: Option<PrimField<String>>,
    service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threadsafe: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_scaling: Option<Vec<AppEngineStandardAppVersionAutomaticScalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_scaling: Option<Vec<AppEngineStandardAppVersionBasicScalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment: Option<Vec<AppEngineStandardAppVersionDeploymentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entrypoint: Option<Vec<AppEngineStandardAppVersionEntrypointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handlers: Option<Vec<AppEngineStandardAppVersionHandlersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    libraries: Option<Vec<AppEngineStandardAppVersionLibrariesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_scaling: Option<Vec<AppEngineStandardAppVersionManualScalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AppEngineStandardAppVersionTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_access_connector: Option<Vec<AppEngineStandardAppVersionVpcAccessConnectorEl>>,
    dynamic: AppEngineStandardAppVersionDynamic,
}
struct AppEngineStandardAppVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppEngineStandardAppVersionData>,
}
#[derive(Clone)]
pub struct AppEngineStandardAppVersion(Rc<AppEngineStandardAppVersion_>);
impl AppEngineStandardAppVersion {
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
    #[doc = "Set the field `app_engine_apis`.\nAllows App Engine second generation runtimes to access the legacy bundled services."]
    pub fn set_app_engine_apis(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().app_engine_apis = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_service_on_destroy`.\nIf set to 'true', the service will be deleted if it is the last version."]
    pub fn set_delete_service_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_service_on_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `env_variables`.\nEnvironment variables available to the application."]
    pub fn set_env_variables(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().env_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `inbound_services`.\nA list of the types of messages that this application is able to receive. Possible values: [\"INBOUND_SERVICE_MAIL\", \"INBOUND_SERVICE_MAIL_BOUNCE\", \"INBOUND_SERVICE_XMPP_ERROR\", \"INBOUND_SERVICE_XMPP_MESSAGE\", \"INBOUND_SERVICE_XMPP_SUBSCRIBE\", \"INBOUND_SERVICE_XMPP_PRESENCE\", \"INBOUND_SERVICE_CHANNEL_PRESENCE\", \"INBOUND_SERVICE_WARMUP\"]"]
    pub fn set_inbound_services(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().inbound_services = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_class`.\nInstance class that is used to run this version. Valid values are\nAutomaticScaling: F1, F2, F4, F4_1G\nBasicScaling or ManualScaling: B1, B2, B4, B4_1G, B8\nDefaults to F1 for AutomaticScaling and B2 for ManualScaling and BasicScaling. If no scaling is specified, AutomaticScaling is chosen."]
    pub fn set_instance_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_class = Some(v.into());
        self
    }
    #[doc = "Set the field `noop_on_destroy`.\nIf set to 'true', the application version will not be deleted."]
    pub fn set_noop_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().noop_on_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `runtime_api_version`.\nThe version of the API in the given runtime environment.\nPlease see the app.yaml reference for valid values at 'https://cloud.google.com/appengine/docs/standard/<language>/config/appref'\\\nSubstitute '<language>' with 'python', 'java', 'php', 'ruby', 'go' or 'nodejs'."]
    pub fn set_runtime_api_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().runtime_api_version = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nThe identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default if this field is neither provided in app.yaml file nor through CLI flag."]
    pub fn set_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `threadsafe`.\nWhether multiple requests can be dispatched to this version at once."]
    pub fn set_threadsafe(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().threadsafe = Some(v.into());
        self
    }
    #[doc = "Set the field `version_id`.\nRelative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names,\"default\", \"latest\", and any name with the prefix \"ah-\"."]
    pub fn set_version_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_id = Some(v.into());
        self
    }
    #[doc = "Set the field `automatic_scaling`.\n"]
    pub fn set_automatic_scaling(
        self,
        v: impl Into<BlockAssignable<AppEngineStandardAppVersionAutomaticScalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().automatic_scaling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.automatic_scaling = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `basic_scaling`.\n"]
    pub fn set_basic_scaling(
        self,
        v: impl Into<BlockAssignable<AppEngineStandardAppVersionBasicScalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().basic_scaling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.basic_scaling = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `deployment`.\n"]
    pub fn set_deployment(
        self,
        v: impl Into<BlockAssignable<AppEngineStandardAppVersionDeploymentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `entrypoint`.\n"]
    pub fn set_entrypoint(
        self,
        v: impl Into<BlockAssignable<AppEngineStandardAppVersionEntrypointEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entrypoint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entrypoint = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `handlers`.\n"]
    pub fn set_handlers(
        self,
        v: impl Into<BlockAssignable<AppEngineStandardAppVersionHandlersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().handlers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.handlers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `libraries`.\n"]
    pub fn set_libraries(
        self,
        v: impl Into<BlockAssignable<AppEngineStandardAppVersionLibrariesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().libraries = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.libraries = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `manual_scaling`.\n"]
    pub fn set_manual_scaling(
        self,
        v: impl Into<BlockAssignable<AppEngineStandardAppVersionManualScalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().manual_scaling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.manual_scaling = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AppEngineStandardAppVersionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_access_connector`.\n"]
    pub fn set_vpc_access_connector(
        self,
        v: impl Into<BlockAssignable<AppEngineStandardAppVersionVpcAccessConnectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_access_connector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_access_connector = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `app_engine_apis` after provisioning.\nAllows App Engine second generation runtimes to access the legacy bundled services."]
    pub fn app_engine_apis(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_engine_apis", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_service_on_destroy` after provisioning.\nIf set to 'true', the service will be deleted if it is the last version."]
    pub fn delete_service_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_service_on_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `env_variables` after provisioning.\nEnvironment variables available to the application."]
    pub fn env_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.env_variables", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `inbound_services` after provisioning.\nA list of the types of messages that this application is able to receive. Possible values: [\"INBOUND_SERVICE_MAIL\", \"INBOUND_SERVICE_MAIL_BOUNCE\", \"INBOUND_SERVICE_XMPP_ERROR\", \"INBOUND_SERVICE_XMPP_MESSAGE\", \"INBOUND_SERVICE_XMPP_SUBSCRIBE\", \"INBOUND_SERVICE_XMPP_PRESENCE\", \"INBOUND_SERVICE_CHANNEL_PRESENCE\", \"INBOUND_SERVICE_WARMUP\"]"]
    pub fn inbound_services(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.inbound_services", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_class` after provisioning.\nInstance class that is used to run this version. Valid values are\nAutomaticScaling: F1, F2, F4, F4_1G\nBasicScaling or ManualScaling: B1, B2, B4, B4_1G, B8\nDefaults to F1 for AutomaticScaling and B2 for ManualScaling and BasicScaling. If no scaling is specified, AutomaticScaling is chosen."]
    pub fn instance_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nFull path to the Version resource in the API. Example, \"v1\"."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `noop_on_destroy` after provisioning.\nIf set to 'true', the application version will not be deleted."]
    pub fn noop_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.noop_on_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime` after provisioning.\nDesired runtime. Example python27."]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_api_version` after provisioning.\nThe version of the API in the given runtime environment.\nPlease see the app.yaml reference for valid values at 'https://cloud.google.com/appengine/docs/standard/<language>/config/appref'\\\nSubstitute '<language>' with 'python', 'java', 'php', 'ruby', 'go' or 'nodejs'."]
    pub fn runtime_api_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_api_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nAppEngine service resource"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default if this field is neither provided in app.yaml file nor through CLI flag."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `threadsafe` after provisioning.\nWhether multiple requests can be dispatched to this version at once."]
    pub fn threadsafe(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threadsafe", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_id` after provisioning.\nRelative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names,\"default\", \"latest\", and any name with the prefix \"ah-\"."]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automatic_scaling` after provisioning.\n"]
    pub fn automatic_scaling(&self) -> ListRef<AppEngineStandardAppVersionAutomaticScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automatic_scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `basic_scaling` after provisioning.\n"]
    pub fn basic_scaling(&self) -> ListRef<AppEngineStandardAppVersionBasicScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.basic_scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> ListRef<AppEngineStandardAppVersionDeploymentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entrypoint` after provisioning.\n"]
    pub fn entrypoint(&self) -> ListRef<AppEngineStandardAppVersionEntrypointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entrypoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `handlers` after provisioning.\n"]
    pub fn handlers(&self) -> ListRef<AppEngineStandardAppVersionHandlersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.handlers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `libraries` after provisioning.\n"]
    pub fn libraries(&self) -> ListRef<AppEngineStandardAppVersionLibrariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.libraries", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `manual_scaling` after provisioning.\n"]
    pub fn manual_scaling(&self) -> ListRef<AppEngineStandardAppVersionManualScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.manual_scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineStandardAppVersionTimeoutsElRef {
        AppEngineStandardAppVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_access_connector` after provisioning.\n"]
    pub fn vpc_access_connector(
        &self,
    ) -> ListRef<AppEngineStandardAppVersionVpcAccessConnectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_access_connector", self.extract_ref()),
        )
    }
}
impl Referable for AppEngineStandardAppVersion {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppEngineStandardAppVersion {}
impl ToListMappable for AppEngineStandardAppVersion {
    type O = ListRef<AppEngineStandardAppVersionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppEngineStandardAppVersion_ {
    fn extract_resource_type(&self) -> String {
        "google_app_engine_standard_app_version".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppEngineStandardAppVersion {
    pub tf_id: String,
    #[doc = "Desired runtime. Example python27."]
    pub runtime: PrimField<String>,
    #[doc = "AppEngine service resource"]
    pub service: PrimField<String>,
}
impl BuildAppEngineStandardAppVersion {
    pub fn build(self, stack: &mut Stack) -> AppEngineStandardAppVersion {
        let out = AppEngineStandardAppVersion(Rc::new(AppEngineStandardAppVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppEngineStandardAppVersionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_engine_apis: core::default::Default::default(),
                delete_service_on_destroy: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                env_variables: core::default::Default::default(),
                id: core::default::Default::default(),
                inbound_services: core::default::Default::default(),
                instance_class: core::default::Default::default(),
                noop_on_destroy: core::default::Default::default(),
                project: core::default::Default::default(),
                runtime: self.runtime,
                runtime_api_version: core::default::Default::default(),
                service: self.service,
                service_account: core::default::Default::default(),
                threadsafe: core::default::Default::default(),
                version_id: core::default::Default::default(),
                automatic_scaling: core::default::Default::default(),
                basic_scaling: core::default::Default::default(),
                deployment: core::default::Default::default(),
                entrypoint: core::default::Default::default(),
                handlers: core::default::Default::default(),
                libraries: core::default::Default::default(),
                manual_scaling: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_access_connector: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AppEngineStandardAppVersionRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppEngineStandardAppVersionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_engine_apis` after provisioning.\nAllows App Engine second generation runtimes to access the legacy bundled services."]
    pub fn app_engine_apis(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_engine_apis", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_service_on_destroy` after provisioning.\nIf set to 'true', the service will be deleted if it is the last version."]
    pub fn delete_service_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_service_on_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `env_variables` after provisioning.\nEnvironment variables available to the application."]
    pub fn env_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.env_variables", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `inbound_services` after provisioning.\nA list of the types of messages that this application is able to receive. Possible values: [\"INBOUND_SERVICE_MAIL\", \"INBOUND_SERVICE_MAIL_BOUNCE\", \"INBOUND_SERVICE_XMPP_ERROR\", \"INBOUND_SERVICE_XMPP_MESSAGE\", \"INBOUND_SERVICE_XMPP_SUBSCRIBE\", \"INBOUND_SERVICE_XMPP_PRESENCE\", \"INBOUND_SERVICE_CHANNEL_PRESENCE\", \"INBOUND_SERVICE_WARMUP\"]"]
    pub fn inbound_services(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.inbound_services", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_class` after provisioning.\nInstance class that is used to run this version. Valid values are\nAutomaticScaling: F1, F2, F4, F4_1G\nBasicScaling or ManualScaling: B1, B2, B4, B4_1G, B8\nDefaults to F1 for AutomaticScaling and B2 for ManualScaling and BasicScaling. If no scaling is specified, AutomaticScaling is chosen."]
    pub fn instance_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nFull path to the Version resource in the API. Example, \"v1\"."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `noop_on_destroy` after provisioning.\nIf set to 'true', the application version will not be deleted."]
    pub fn noop_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.noop_on_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime` after provisioning.\nDesired runtime. Example python27."]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_api_version` after provisioning.\nThe version of the API in the given runtime environment.\nPlease see the app.yaml reference for valid values at 'https://cloud.google.com/appengine/docs/standard/<language>/config/appref'\\\nSubstitute '<language>' with 'python', 'java', 'php', 'ruby', 'go' or 'nodejs'."]
    pub fn runtime_api_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_api_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nAppEngine service resource"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default if this field is neither provided in app.yaml file nor through CLI flag."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `threadsafe` after provisioning.\nWhether multiple requests can be dispatched to this version at once."]
    pub fn threadsafe(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threadsafe", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_id` after provisioning.\nRelative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names,\"default\", \"latest\", and any name with the prefix \"ah-\"."]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automatic_scaling` after provisioning.\n"]
    pub fn automatic_scaling(&self) -> ListRef<AppEngineStandardAppVersionAutomaticScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automatic_scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `basic_scaling` after provisioning.\n"]
    pub fn basic_scaling(&self) -> ListRef<AppEngineStandardAppVersionBasicScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.basic_scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> ListRef<AppEngineStandardAppVersionDeploymentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entrypoint` after provisioning.\n"]
    pub fn entrypoint(&self) -> ListRef<AppEngineStandardAppVersionEntrypointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entrypoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `handlers` after provisioning.\n"]
    pub fn handlers(&self) -> ListRef<AppEngineStandardAppVersionHandlersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.handlers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `libraries` after provisioning.\n"]
    pub fn libraries(&self) -> ListRef<AppEngineStandardAppVersionLibrariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.libraries", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `manual_scaling` after provisioning.\n"]
    pub fn manual_scaling(&self) -> ListRef<AppEngineStandardAppVersionManualScalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.manual_scaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineStandardAppVersionTimeoutsElRef {
        AppEngineStandardAppVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_access_connector` after provisioning.\n"]
    pub fn vpc_access_connector(
        &self,
    ) -> ListRef<AppEngineStandardAppVersionVpcAccessConnectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_access_connector", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_cpu_utilization: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_throughput_utilization: Option<PrimField<f64>>,
}
impl AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {
    #[doc = "Set the field `max_instances`.\nMaximum number of instances to run for this version. Set to zero to disable maxInstances configuration.\n\n**Note:** Starting from March 2025, App Engine sets the maxInstances default for standard environment deployments to 20. This change doesn't impact existing apps. To override the default, specify a new value between 0 and 2147483647, and deploy a new version or redeploy over an existing version. To disable the maxInstances default configuration setting, specify the maximum permitted value 2147483647."]
    pub fn set_max_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `min_instances`.\nMinimum number of instances to run for this version. Set to zero to disable minInstances configuration."]
    pub fn set_min_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `target_cpu_utilization`.\nTarget CPU utilization ratio to maintain when scaling. Should be a value in the range [0.50, 0.95], zero, or a negative value."]
    pub fn set_target_cpu_utilization(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_cpu_utilization = Some(v.into());
        self
    }
    #[doc = "Set the field `target_throughput_utilization`.\nTarget throughput utilization ratio to maintain when scaling. Should be a value in the range [0.50, 0.95], zero, or a negative value."]
    pub fn set_target_throughput_utilization(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_throughput_utilization = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {
    type O =
        BlockAssignable<AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {}
impl BuildAppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {
    pub fn build(self) -> AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {
        AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {
            max_instances: core::default::Default::default(),
            min_instances: core::default::Default::default(),
            target_cpu_utilization: core::default::Default::default(),
            target_throughput_utilization: core::default::Default::default(),
        }
    }
}
pub struct AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsElRef {
        AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_instances` after provisioning.\nMaximum number of instances to run for this version. Set to zero to disable maxInstances configuration.\n\n**Note:** Starting from March 2025, App Engine sets the maxInstances default for standard environment deployments to 20. This change doesn't impact existing apps. To override the default, specify a new value between 0 and 2147483647, and deploy a new version or redeploy over an existing version. To disable the maxInstances default configuration setting, specify the maximum permitted value 2147483647."]
    pub fn max_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_instances` after provisioning.\nMinimum number of instances to run for this version. Set to zero to disable minInstances configuration."]
    pub fn min_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_cpu_utilization` after provisioning.\nTarget CPU utilization ratio to maintain when scaling. Should be a value in the range [0.50, 0.95], zero, or a negative value."]
    pub fn target_cpu_utilization(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_cpu_utilization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_throughput_utilization` after provisioning.\nTarget throughput utilization ratio to maintain when scaling. Should be a value in the range [0.50, 0.95], zero, or a negative value."]
    pub fn target_throughput_utilization(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_throughput_utilization", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AppEngineStandardAppVersionAutomaticScalingElDynamic {
    standard_scheduler_settings: Option<
        DynamicBlock<AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl>,
    >,
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionAutomaticScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent_requests: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_idle_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pending_latency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_idle_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_pending_latency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard_scheduler_settings:
        Option<Vec<AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl>>,
    dynamic: AppEngineStandardAppVersionAutomaticScalingElDynamic,
}
impl AppEngineStandardAppVersionAutomaticScalingEl {
    #[doc = "Set the field `max_concurrent_requests`.\nNumber of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.\n\nDefaults to a runtime-specific value."]
    pub fn set_max_concurrent_requests(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_concurrent_requests = Some(v.into());
        self
    }
    #[doc = "Set the field `max_idle_instances`.\nMaximum number of idle instances that should be maintained for this version."]
    pub fn set_max_idle_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_idle_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `max_pending_latency`.\nMaximum amount of time that a request should wait in the pending queue before starting a new instance to handle it.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_max_pending_latency(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_pending_latency = Some(v.into());
        self
    }
    #[doc = "Set the field `min_idle_instances`.\nMinimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service."]
    pub fn set_min_idle_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_idle_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `min_pending_latency`.\nMinimum amount of time a request should wait in the pending queue before starting a new instance to handle it.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_min_pending_latency(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_pending_latency = Some(v.into());
        self
    }
    #[doc = "Set the field `standard_scheduler_settings`.\n"]
    pub fn set_standard_scheduler_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.standard_scheduler_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.standard_scheduler_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppEngineStandardAppVersionAutomaticScalingEl {
    type O = BlockAssignable<AppEngineStandardAppVersionAutomaticScalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionAutomaticScalingEl {}
impl BuildAppEngineStandardAppVersionAutomaticScalingEl {
    pub fn build(self) -> AppEngineStandardAppVersionAutomaticScalingEl {
        AppEngineStandardAppVersionAutomaticScalingEl {
            max_concurrent_requests: core::default::Default::default(),
            max_idle_instances: core::default::Default::default(),
            max_pending_latency: core::default::Default::default(),
            min_idle_instances: core::default::Default::default(),
            min_pending_latency: core::default::Default::default(),
            standard_scheduler_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppEngineStandardAppVersionAutomaticScalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionAutomaticScalingElRef {
    fn new(shared: StackShared, base: String) -> AppEngineStandardAppVersionAutomaticScalingElRef {
        AppEngineStandardAppVersionAutomaticScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionAutomaticScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_concurrent_requests` after provisioning.\nNumber of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.\n\nDefaults to a runtime-specific value."]
    pub fn max_concurrent_requests(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_requests", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_idle_instances` after provisioning.\nMaximum number of idle instances that should be maintained for this version."]
    pub fn max_idle_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_idle_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_pending_latency` after provisioning.\nMaximum amount of time that a request should wait in the pending queue before starting a new instance to handle it.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn max_pending_latency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pending_latency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_idle_instances` after provisioning.\nMinimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service."]
    pub fn min_idle_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_idle_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_pending_latency` after provisioning.\nMinimum amount of time a request should wait in the pending queue before starting a new instance to handle it.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn min_pending_latency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_pending_latency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `standard_scheduler_settings` after provisioning.\n"]
    pub fn standard_scheduler_settings(
        &self,
    ) -> ListRef<AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.standard_scheduler_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionBasicScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout: Option<PrimField<String>>,
    max_instances: PrimField<f64>,
}
impl AppEngineStandardAppVersionBasicScalingEl {
    #[doc = "Set the field `idle_timeout`.\nDuration of time after the last request that an instance must wait before the instance is shut down.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\". Defaults to 900s."]
    pub fn set_idle_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.idle_timeout = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineStandardAppVersionBasicScalingEl {
    type O = BlockAssignable<AppEngineStandardAppVersionBasicScalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionBasicScalingEl {
    #[doc = "Maximum number of instances to create for this version. Must be in the range [1.0, 200.0]."]
    pub max_instances: PrimField<f64>,
}
impl BuildAppEngineStandardAppVersionBasicScalingEl {
    pub fn build(self) -> AppEngineStandardAppVersionBasicScalingEl {
        AppEngineStandardAppVersionBasicScalingEl {
            idle_timeout: core::default::Default::default(),
            max_instances: self.max_instances,
        }
    }
}
pub struct AppEngineStandardAppVersionBasicScalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionBasicScalingElRef {
    fn new(shared: StackShared, base: String) -> AppEngineStandardAppVersionBasicScalingElRef {
        AppEngineStandardAppVersionBasicScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionBasicScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\nDuration of time after the last request that an instance must wait before the instance is shut down.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\". Defaults to 900s."]
    pub fn idle_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `max_instances` after provisioning.\nMaximum number of instances to create for this version. Must be in the range [1.0, 200.0]."]
    pub fn max_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instances", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionDeploymentElFilesEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha1_sum: Option<PrimField<String>>,
    source_url: PrimField<String>,
}
impl AppEngineStandardAppVersionDeploymentElFilesEl {
    #[doc = "Set the field `sha1_sum`.\nSHA1 checksum of the file"]
    pub fn set_sha1_sum(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha1_sum = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineStandardAppVersionDeploymentElFilesEl {
    type O = BlockAssignable<AppEngineStandardAppVersionDeploymentElFilesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionDeploymentElFilesEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = "Source URL"]
    pub source_url: PrimField<String>,
}
impl BuildAppEngineStandardAppVersionDeploymentElFilesEl {
    pub fn build(self) -> AppEngineStandardAppVersionDeploymentElFilesEl {
        AppEngineStandardAppVersionDeploymentElFilesEl {
            name: self.name,
            sha1_sum: core::default::Default::default(),
            source_url: self.source_url,
        }
    }
}
pub struct AppEngineStandardAppVersionDeploymentElFilesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionDeploymentElFilesElRef {
    fn new(shared: StackShared, base: String) -> AppEngineStandardAppVersionDeploymentElFilesElRef {
        AppEngineStandardAppVersionDeploymentElFilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionDeploymentElFilesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `sha1_sum` after provisioning.\nSHA1 checksum of the file"]
    pub fn sha1_sum(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha1_sum", self.base))
    }
    #[doc = "Get a reference to the value of field `source_url` after provisioning.\nSource URL"]
    pub fn source_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_url", self.base))
    }
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionDeploymentElZipEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    files_count: Option<PrimField<f64>>,
    source_url: PrimField<String>,
}
impl AppEngineStandardAppVersionDeploymentElZipEl {
    #[doc = "Set the field `files_count`.\nfiles count"]
    pub fn set_files_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.files_count = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineStandardAppVersionDeploymentElZipEl {
    type O = BlockAssignable<AppEngineStandardAppVersionDeploymentElZipEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionDeploymentElZipEl {
    #[doc = "Source URL"]
    pub source_url: PrimField<String>,
}
impl BuildAppEngineStandardAppVersionDeploymentElZipEl {
    pub fn build(self) -> AppEngineStandardAppVersionDeploymentElZipEl {
        AppEngineStandardAppVersionDeploymentElZipEl {
            files_count: core::default::Default::default(),
            source_url: self.source_url,
        }
    }
}
pub struct AppEngineStandardAppVersionDeploymentElZipElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionDeploymentElZipElRef {
    fn new(shared: StackShared, base: String) -> AppEngineStandardAppVersionDeploymentElZipElRef {
        AppEngineStandardAppVersionDeploymentElZipElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionDeploymentElZipElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `files_count` after provisioning.\nfiles count"]
    pub fn files_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.files_count", self.base))
    }
    #[doc = "Get a reference to the value of field `source_url` after provisioning.\nSource URL"]
    pub fn source_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_url", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppEngineStandardAppVersionDeploymentElDynamic {
    files: Option<DynamicBlock<AppEngineStandardAppVersionDeploymentElFilesEl>>,
    zip: Option<DynamicBlock<AppEngineStandardAppVersionDeploymentElZipEl>>,
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionDeploymentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<Vec<AppEngineStandardAppVersionDeploymentElFilesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip: Option<Vec<AppEngineStandardAppVersionDeploymentElZipEl>>,
    dynamic: AppEngineStandardAppVersionDeploymentElDynamic,
}
impl AppEngineStandardAppVersionDeploymentEl {
    #[doc = "Set the field `files`.\n"]
    pub fn set_files(
        mut self,
        v: impl Into<BlockAssignable<AppEngineStandardAppVersionDeploymentElFilesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.files = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.files = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `zip`.\n"]
    pub fn set_zip(
        mut self,
        v: impl Into<BlockAssignable<AppEngineStandardAppVersionDeploymentElZipEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.zip = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.zip = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppEngineStandardAppVersionDeploymentEl {
    type O = BlockAssignable<AppEngineStandardAppVersionDeploymentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionDeploymentEl {}
impl BuildAppEngineStandardAppVersionDeploymentEl {
    pub fn build(self) -> AppEngineStandardAppVersionDeploymentEl {
        AppEngineStandardAppVersionDeploymentEl {
            files: core::default::Default::default(),
            zip: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppEngineStandardAppVersionDeploymentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionDeploymentElRef {
    fn new(shared: StackShared, base: String) -> AppEngineStandardAppVersionDeploymentElRef {
        AppEngineStandardAppVersionDeploymentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionDeploymentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `zip` after provisioning.\n"]
    pub fn zip(&self) -> ListRef<AppEngineStandardAppVersionDeploymentElZipElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zip", self.base))
    }
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionEntrypointEl {
    shell: PrimField<String>,
}
impl AppEngineStandardAppVersionEntrypointEl {}
impl ToListMappable for AppEngineStandardAppVersionEntrypointEl {
    type O = BlockAssignable<AppEngineStandardAppVersionEntrypointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionEntrypointEl {
    #[doc = "The format should be a shell command that can be fed to bash -c."]
    pub shell: PrimField<String>,
}
impl BuildAppEngineStandardAppVersionEntrypointEl {
    pub fn build(self) -> AppEngineStandardAppVersionEntrypointEl {
        AppEngineStandardAppVersionEntrypointEl { shell: self.shell }
    }
}
pub struct AppEngineStandardAppVersionEntrypointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionEntrypointElRef {
    fn new(shared: StackShared, base: String) -> AppEngineStandardAppVersionEntrypointElRef {
        AppEngineStandardAppVersionEntrypointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionEntrypointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `shell` after provisioning.\nThe format should be a shell command that can be fed to bash -c."]
    pub fn shell(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shell", self.base))
    }
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionHandlersElScriptEl {
    script_path: PrimField<String>,
}
impl AppEngineStandardAppVersionHandlersElScriptEl {}
impl ToListMappable for AppEngineStandardAppVersionHandlersElScriptEl {
    type O = BlockAssignable<AppEngineStandardAppVersionHandlersElScriptEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionHandlersElScriptEl {
    #[doc = "Path to the script from the application root directory."]
    pub script_path: PrimField<String>,
}
impl BuildAppEngineStandardAppVersionHandlersElScriptEl {
    pub fn build(self) -> AppEngineStandardAppVersionHandlersElScriptEl {
        AppEngineStandardAppVersionHandlersElScriptEl {
            script_path: self.script_path,
        }
    }
}
pub struct AppEngineStandardAppVersionHandlersElScriptElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionHandlersElScriptElRef {
    fn new(shared: StackShared, base: String) -> AppEngineStandardAppVersionHandlersElScriptElRef {
        AppEngineStandardAppVersionHandlersElScriptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionHandlersElScriptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `script_path` after provisioning.\nPath to the script from the application root directory."]
    pub fn script_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.script_path", self.base))
    }
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionHandlersElStaticFilesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_readable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_headers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mime_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_matching_file: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_path_regex: Option<PrimField<String>>,
}
impl AppEngineStandardAppVersionHandlersElStaticFilesEl {
    #[doc = "Set the field `application_readable`.\nWhether files should also be uploaded as code data. By default, files declared in static file handlers are uploaded as\nstatic data and are only served to end users; they cannot be read by the application. If enabled, uploads are charged\nagainst both your code and static data storage resource quotas."]
    pub fn set_application_readable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.application_readable = Some(v.into());
        self
    }
    #[doc = "Set the field `expiration`.\nTime a static file served by this handler should be cached by web proxies and browsers.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example \"3.5s\"."]
    pub fn set_expiration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expiration = Some(v.into());
        self
    }
    #[doc = "Set the field `http_headers`.\nHTTP headers to use for all responses from these URLs.\nAn object containing a list of \"key:value\" value pairs.\"."]
    pub fn set_http_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.http_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `mime_type`.\nMIME type used to serve all files served by this handler.\nDefaults to file-specific MIME types, which are derived from each file's filename extension."]
    pub fn set_mime_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mime_type = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nPath to the static files matched by the URL pattern, from the application root directory. The path can refer to text matched in groupings in the URL pattern."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `require_matching_file`.\nWhether this handler should match the request if the file referenced by the handler does not exist."]
    pub fn set_require_matching_file(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_matching_file = Some(v.into());
        self
    }
    #[doc = "Set the field `upload_path_regex`.\nRegular expression that matches the file paths for all files that should be referenced by this handler."]
    pub fn set_upload_path_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.upload_path_regex = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineStandardAppVersionHandlersElStaticFilesEl {
    type O = BlockAssignable<AppEngineStandardAppVersionHandlersElStaticFilesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionHandlersElStaticFilesEl {}
impl BuildAppEngineStandardAppVersionHandlersElStaticFilesEl {
    pub fn build(self) -> AppEngineStandardAppVersionHandlersElStaticFilesEl {
        AppEngineStandardAppVersionHandlersElStaticFilesEl {
            application_readable: core::default::Default::default(),
            expiration: core::default::Default::default(),
            http_headers: core::default::Default::default(),
            mime_type: core::default::Default::default(),
            path: core::default::Default::default(),
            require_matching_file: core::default::Default::default(),
            upload_path_regex: core::default::Default::default(),
        }
    }
}
pub struct AppEngineStandardAppVersionHandlersElStaticFilesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionHandlersElStaticFilesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppEngineStandardAppVersionHandlersElStaticFilesElRef {
        AppEngineStandardAppVersionHandlersElStaticFilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionHandlersElStaticFilesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `application_readable` after provisioning.\nWhether files should also be uploaded as code data. By default, files declared in static file handlers are uploaded as\nstatic data and are only served to end users; they cannot be read by the application. If enabled, uploads are charged\nagainst both your code and static data storage resource quotas."]
    pub fn application_readable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_readable", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `expiration` after provisioning.\nTime a static file served by this handler should be cached by web proxies and browsers.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example \"3.5s\"."]
    pub fn expiration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiration", self.base))
    }
    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\nHTTP headers to use for all responses from these URLs.\nAn object containing a list of \"key:value\" value pairs.\"."]
    pub fn http_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
    #[doc = "Get a reference to the value of field `mime_type` after provisioning.\nMIME type used to serve all files served by this handler.\nDefaults to file-specific MIME types, which are derived from each file's filename extension."]
    pub fn mime_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mime_type", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath to the static files matched by the URL pattern, from the application root directory. The path can refer to text matched in groupings in the URL pattern."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `require_matching_file` after provisioning.\nWhether this handler should match the request if the file referenced by the handler does not exist."]
    pub fn require_matching_file(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_matching_file", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upload_path_regex` after provisioning.\nRegular expression that matches the file paths for all files that should be referenced by this handler."]
    pub fn upload_path_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.upload_path_regex", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AppEngineStandardAppVersionHandlersElDynamic {
    script: Option<DynamicBlock<AppEngineStandardAppVersionHandlersElScriptEl>>,
    static_files: Option<DynamicBlock<AppEngineStandardAppVersionHandlersElStaticFilesEl>>,
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionHandlersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_fail_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_http_response_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script: Option<Vec<AppEngineStandardAppVersionHandlersElScriptEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_files: Option<Vec<AppEngineStandardAppVersionHandlersElStaticFilesEl>>,
    dynamic: AppEngineStandardAppVersionHandlersElDynamic,
}
impl AppEngineStandardAppVersionHandlersEl {
    #[doc = "Set the field `auth_fail_action`.\nActions to take when the user is not logged in. Possible values: [\"AUTH_FAIL_ACTION_REDIRECT\", \"AUTH_FAIL_ACTION_UNAUTHORIZED\"]"]
    pub fn set_auth_fail_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_fail_action = Some(v.into());
        self
    }
    #[doc = "Set the field `login`.\nMethods to restrict access to a URL based on login status. Possible values: [\"LOGIN_OPTIONAL\", \"LOGIN_ADMIN\", \"LOGIN_REQUIRED\"]"]
    pub fn set_login(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.login = Some(v.into());
        self
    }
    #[doc = "Set the field `redirect_http_response_code`.\n30x code to use when performing redirects for the secure field. Possible values: [\"REDIRECT_HTTP_RESPONSE_CODE_301\", \"REDIRECT_HTTP_RESPONSE_CODE_302\", \"REDIRECT_HTTP_RESPONSE_CODE_303\", \"REDIRECT_HTTP_RESPONSE_CODE_307\"]"]
    pub fn set_redirect_http_response_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_http_response_code = Some(v.into());
        self
    }
    #[doc = "Set the field `security_level`.\nSecurity (HTTPS) enforcement for this URL. Possible values: [\"SECURE_DEFAULT\", \"SECURE_NEVER\", \"SECURE_OPTIONAL\", \"SECURE_ALWAYS\"]"]
    pub fn set_security_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_level = Some(v.into());
        self
    }
    #[doc = "Set the field `url_regex`.\nURL prefix. Uses regular expression syntax, which means regexp special characters must be escaped, but should not contain groupings.\nAll URLs that begin with this prefix are handled by this handler, using the portion of the URL after the prefix as part of the file path."]
    pub fn set_url_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `script`.\n"]
    pub fn set_script(
        mut self,
        v: impl Into<BlockAssignable<AppEngineStandardAppVersionHandlersElScriptEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.script = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.script = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `static_files`.\n"]
    pub fn set_static_files(
        mut self,
        v: impl Into<BlockAssignable<AppEngineStandardAppVersionHandlersElStaticFilesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.static_files = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.static_files = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppEngineStandardAppVersionHandlersEl {
    type O = BlockAssignable<AppEngineStandardAppVersionHandlersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionHandlersEl {}
impl BuildAppEngineStandardAppVersionHandlersEl {
    pub fn build(self) -> AppEngineStandardAppVersionHandlersEl {
        AppEngineStandardAppVersionHandlersEl {
            auth_fail_action: core::default::Default::default(),
            login: core::default::Default::default(),
            redirect_http_response_code: core::default::Default::default(),
            security_level: core::default::Default::default(),
            url_regex: core::default::Default::default(),
            script: core::default::Default::default(),
            static_files: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppEngineStandardAppVersionHandlersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionHandlersElRef {
    fn new(shared: StackShared, base: String) -> AppEngineStandardAppVersionHandlersElRef {
        AppEngineStandardAppVersionHandlersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionHandlersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_fail_action` after provisioning.\nActions to take when the user is not logged in. Possible values: [\"AUTH_FAIL_ACTION_REDIRECT\", \"AUTH_FAIL_ACTION_UNAUTHORIZED\"]"]
    pub fn auth_fail_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auth_fail_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `login` after provisioning.\nMethods to restrict access to a URL based on login status. Possible values: [\"LOGIN_OPTIONAL\", \"LOGIN_ADMIN\", \"LOGIN_REQUIRED\"]"]
    pub fn login(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login", self.base))
    }
    #[doc = "Get a reference to the value of field `redirect_http_response_code` after provisioning.\n30x code to use when performing redirects for the secure field. Possible values: [\"REDIRECT_HTTP_RESPONSE_CODE_301\", \"REDIRECT_HTTP_RESPONSE_CODE_302\", \"REDIRECT_HTTP_RESPONSE_CODE_303\", \"REDIRECT_HTTP_RESPONSE_CODE_307\"]"]
    pub fn redirect_http_response_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redirect_http_response_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_level` after provisioning.\nSecurity (HTTPS) enforcement for this URL. Possible values: [\"SECURE_DEFAULT\", \"SECURE_NEVER\", \"SECURE_OPTIONAL\", \"SECURE_ALWAYS\"]"]
    pub fn security_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `url_regex` after provisioning.\nURL prefix. Uses regular expression syntax, which means regexp special characters must be escaped, but should not contain groupings.\nAll URLs that begin with this prefix are handled by this handler, using the portion of the URL after the prefix as part of the file path."]
    pub fn url_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `script` after provisioning.\n"]
    pub fn script(&self) -> ListRef<AppEngineStandardAppVersionHandlersElScriptElRef> {
        ListRef::new(self.shared().clone(), format!("{}.script", self.base))
    }
    #[doc = "Get a reference to the value of field `static_files` after provisioning.\n"]
    pub fn static_files(&self) -> ListRef<AppEngineStandardAppVersionHandlersElStaticFilesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.static_files", self.base))
    }
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionLibrariesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl AppEngineStandardAppVersionLibrariesEl {
    #[doc = "Set the field `name`.\nName of the library. Example \"django\"."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nVersion of the library to select, or \"latest\"."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineStandardAppVersionLibrariesEl {
    type O = BlockAssignable<AppEngineStandardAppVersionLibrariesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionLibrariesEl {}
impl BuildAppEngineStandardAppVersionLibrariesEl {
    pub fn build(self) -> AppEngineStandardAppVersionLibrariesEl {
        AppEngineStandardAppVersionLibrariesEl {
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct AppEngineStandardAppVersionLibrariesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionLibrariesElRef {
    fn new(shared: StackShared, base: String) -> AppEngineStandardAppVersionLibrariesElRef {
        AppEngineStandardAppVersionLibrariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionLibrariesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the library. Example \"django\"."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the library to select, or \"latest\"."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionManualScalingEl {
    instances: PrimField<f64>,
}
impl AppEngineStandardAppVersionManualScalingEl {}
impl ToListMappable for AppEngineStandardAppVersionManualScalingEl {
    type O = BlockAssignable<AppEngineStandardAppVersionManualScalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionManualScalingEl {
    #[doc = "Number of instances to assign to the service at the start.\n\n**Note:** When managing the number of instances at runtime through the App Engine Admin API or the (now deprecated) Python 2\nModules API set_num_instances() you must use 'lifecycle.ignore_changes = [\"manual_scaling\"[0].instances]' to prevent drift detection."]
    pub instances: PrimField<f64>,
}
impl BuildAppEngineStandardAppVersionManualScalingEl {
    pub fn build(self) -> AppEngineStandardAppVersionManualScalingEl {
        AppEngineStandardAppVersionManualScalingEl {
            instances: self.instances,
        }
    }
}
pub struct AppEngineStandardAppVersionManualScalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionManualScalingElRef {
    fn new(shared: StackShared, base: String) -> AppEngineStandardAppVersionManualScalingElRef {
        AppEngineStandardAppVersionManualScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionManualScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\nNumber of instances to assign to the service at the start.\n\n**Note:** When managing the number of instances at runtime through the App Engine Admin API or the (now deprecated) Python 2\nModules API set_num_instances() you must use 'lifecycle.ignore_changes = [\"manual_scaling\"[0].instances]' to prevent drift detection."]
    pub fn instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}
#[derive(Serialize)]
pub struct AppEngineStandardAppVersionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AppEngineStandardAppVersionTimeoutsEl {
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
impl ToListMappable for AppEngineStandardAppVersionTimeoutsEl {
    type O = BlockAssignable<AppEngineStandardAppVersionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionTimeoutsEl {}
impl BuildAppEngineStandardAppVersionTimeoutsEl {
    pub fn build(self) -> AppEngineStandardAppVersionTimeoutsEl {
        AppEngineStandardAppVersionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AppEngineStandardAppVersionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AppEngineStandardAppVersionTimeoutsElRef {
        AppEngineStandardAppVersionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionTimeoutsElRef {
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
pub struct AppEngineStandardAppVersionVpcAccessConnectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_setting: Option<PrimField<String>>,
    name: PrimField<String>,
}
impl AppEngineStandardAppVersionVpcAccessConnectorEl {
    #[doc = "Set the field `egress_setting`.\nThe egress setting for the connector, controlling what traffic is diverted through it."]
    pub fn set_egress_setting(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.egress_setting = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineStandardAppVersionVpcAccessConnectorEl {
    type O = BlockAssignable<AppEngineStandardAppVersionVpcAccessConnectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineStandardAppVersionVpcAccessConnectorEl {
    #[doc = "Full Serverless VPC Access Connector name e.g. /projects/my-project/locations/us-central1/connectors/c1."]
    pub name: PrimField<String>,
}
impl BuildAppEngineStandardAppVersionVpcAccessConnectorEl {
    pub fn build(self) -> AppEngineStandardAppVersionVpcAccessConnectorEl {
        AppEngineStandardAppVersionVpcAccessConnectorEl {
            egress_setting: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct AppEngineStandardAppVersionVpcAccessConnectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineStandardAppVersionVpcAccessConnectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppEngineStandardAppVersionVpcAccessConnectorElRef {
        AppEngineStandardAppVersionVpcAccessConnectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineStandardAppVersionVpcAccessConnectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `egress_setting` after provisioning.\nThe egress setting for the connector, controlling what traffic is diverted through it."]
    pub fn egress_setting(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.egress_setting", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nFull Serverless VPC Access Connector name e.g. /projects/my-project/locations/us-central1/connectors/c1."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppEngineStandardAppVersionDynamic {
    automatic_scaling: Option<DynamicBlock<AppEngineStandardAppVersionAutomaticScalingEl>>,
    basic_scaling: Option<DynamicBlock<AppEngineStandardAppVersionBasicScalingEl>>,
    deployment: Option<DynamicBlock<AppEngineStandardAppVersionDeploymentEl>>,
    entrypoint: Option<DynamicBlock<AppEngineStandardAppVersionEntrypointEl>>,
    handlers: Option<DynamicBlock<AppEngineStandardAppVersionHandlersEl>>,
    libraries: Option<DynamicBlock<AppEngineStandardAppVersionLibrariesEl>>,
    manual_scaling: Option<DynamicBlock<AppEngineStandardAppVersionManualScalingEl>>,
    vpc_access_connector: Option<DynamicBlock<AppEngineStandardAppVersionVpcAccessConnectorEl>>,
}
