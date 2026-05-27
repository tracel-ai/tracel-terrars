use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudfunctionsFunctionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_memory_mb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_worker_pool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    docker_registry: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    docker_repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_point: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_trigger_security_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_trigger_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instances: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    runtime: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_archive_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_archive_object: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_http: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_connector: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_connector_egress_settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_update_policy: Option<Vec<CloudfunctionsFunctionAutomaticUpdatePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_trigger: Option<Vec<CloudfunctionsFunctionEventTriggerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_deploy_update_policy: Option<Vec<CloudfunctionsFunctionOnDeployUpdatePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_environment_variables: Option<Vec<CloudfunctionsFunctionSecretEnvironmentVariablesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_volumes: Option<Vec<CloudfunctionsFunctionSecretVolumesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_repository: Option<Vec<CloudfunctionsFunctionSourceRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudfunctionsFunctionTimeoutsEl>,
    dynamic: CloudfunctionsFunctionDynamic,
}
struct CloudfunctionsFunction_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfunctionsFunctionData>,
}
#[derive(Clone)]
pub struct CloudfunctionsFunction(Rc<CloudfunctionsFunction_>);
impl CloudfunctionsFunction {
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
    #[doc = "Set the field `available_memory_mb`.\nMemory (in MB), available to the function. Default value is 256. Possible values include 128, 256, 512, 1024, etc."]
    pub fn set_available_memory_mb(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().available_memory_mb = Some(v.into());
        self
    }
    #[doc = "Set the field `build_environment_variables`.\n A set of key/value environment variable pairs available during build time."]
    pub fn set_build_environment_variables(
        self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().build_environment_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `build_service_account`.\nThe fully-qualified name of the service account to be used for the build step of deploying this function"]
    pub fn set_build_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().build_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `build_worker_pool`.\nName of the Cloud Build Custom Worker Pool that should be used to build the function."]
    pub fn set_build_worker_pool(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().build_worker_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nDescription of the function."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `docker_registry`.\nDocker Registry to use for storing the function's Docker images. Allowed values are ARTIFACT_REGISTRY (default) and CONTAINER_REGISTRY."]
    pub fn set_docker_registry(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().docker_registry = Some(v.into());
        self
    }
    #[doc = "Set the field `docker_repository`.\nUser managed repository created in Artifact Registry optionally with a customer managed encryption key. If specified, deployments will use Artifact Registry for storing images built with Cloud Build."]
    pub fn set_docker_repository(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().docker_repository = Some(v.into());
        self
    }
    #[doc = "Set the field `entry_point`.\nName of the function that will be executed when the Google Cloud Function is triggered."]
    pub fn set_entry_point(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().entry_point = Some(v.into());
        self
    }
    #[doc = "Set the field `environment_variables`.\nA set of key/value environment variable pairs to assign to the function."]
    pub fn set_environment_variables(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().environment_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `https_trigger_security_level`.\nThe security level for the function. Defaults to SECURE_OPTIONAL. Valid only if trigger_http is used."]
    pub fn set_https_trigger_security_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().https_trigger_security_level = Some(v.into());
        self
    }
    #[doc = "Set the field `https_trigger_url`.\nURL which triggers function execution. Returned only if trigger_http is used."]
    pub fn set_https_trigger_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().https_trigger_url = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress_settings`.\nString value that controls what traffic can reach the function. Allowed values are ALLOW_ALL and ALLOW_INTERNAL_ONLY. Changes to this field will recreate the cloud function."]
    pub fn set_ingress_settings(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ingress_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources."]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nA set of key/value label pairs to assign to the function. Label keys must follow the requirements at https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels#requirements.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `max_instances`.\nThe limit on the maximum number of function instances that may coexist at a given time."]
    pub fn set_max_instances(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `min_instances`.\nThe limit on the minimum number of function instances that may coexist at a given time."]
    pub fn set_min_instances(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nProject of the function. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion of function. If it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\n If provided, the self-provided service account to run the function with."]
    pub fn set_service_account_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `source_archive_bucket`.\nThe GCS bucket containing the zip archive which contains the function."]
    pub fn set_source_archive_bucket(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_archive_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `source_archive_object`.\nThe source archive object (file) in archive bucket."]
    pub fn set_source_archive_object(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_archive_object = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\nTimeout (in seconds) for the function. Default value is 60 seconds. Cannot be more than 540 seconds."]
    pub fn set_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `trigger_http`.\nBoolean variable. Any HTTP request (of a supported type) to the endpoint will trigger function execution. Supported HTTP request types are: POST, PUT, GET, DELETE, and OPTIONS. Endpoint is returned as https_trigger_url. Cannot be used with trigger_bucket and trigger_topic."]
    pub fn set_trigger_http(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().trigger_http = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_connector`.\nThe VPC Network Connector that this cloud function can connect to. It can be either the fully-qualified URI, or the short name of the network connector resource. The format of this field is projects/*/locations/*/connectors/*."]
    pub fn set_vpc_connector(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_connector = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_connector_egress_settings`.\nThe egress settings for the connector, controlling what traffic is diverted through it. Allowed values are ALL_TRAFFIC and PRIVATE_RANGES_ONLY. Defaults to PRIVATE_RANGES_ONLY. If unset, this field preserves the previously set value."]
    pub fn set_vpc_connector_egress_settings(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_connector_egress_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `automatic_update_policy`.\n"]
    pub fn set_automatic_update_policy(
        self,
        v: impl Into<BlockAssignable<CloudfunctionsFunctionAutomaticUpdatePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().automatic_update_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.automatic_update_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `event_trigger`.\n"]
    pub fn set_event_trigger(
        self,
        v: impl Into<BlockAssignable<CloudfunctionsFunctionEventTriggerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_trigger = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_trigger = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `on_deploy_update_policy`.\n"]
    pub fn set_on_deploy_update_policy(
        self,
        v: impl Into<BlockAssignable<CloudfunctionsFunctionOnDeployUpdatePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().on_deploy_update_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.on_deploy_update_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secret_environment_variables`.\n"]
    pub fn set_secret_environment_variables(
        self,
        v: impl Into<BlockAssignable<CloudfunctionsFunctionSecretEnvironmentVariablesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secret_environment_variables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .secret_environment_variables = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secret_volumes`.\n"]
    pub fn set_secret_volumes(
        self,
        v: impl Into<BlockAssignable<CloudfunctionsFunctionSecretVolumesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secret_volumes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secret_volumes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_repository`.\n"]
    pub fn set_source_repository(
        self,
        v: impl Into<BlockAssignable<CloudfunctionsFunctionSourceRepositoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_repository = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudfunctionsFunctionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `available_memory_mb` after provisioning.\nMemory (in MB), available to the function. Default value is 256. Possible values include 128, 256, 512, 1024, etc."]
    pub fn available_memory_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_memory_mb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_environment_variables` after provisioning.\n A set of key/value environment variable pairs available during build time."]
    pub fn build_environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.build_environment_variables", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_service_account` after provisioning.\nThe fully-qualified name of the service account to be used for the build step of deploying this function"]
    pub fn build_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_worker_pool` after provisioning.\nName of the Cloud Build Custom Worker Pool that should be used to build the function."]
    pub fn build_worker_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_worker_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the function."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `docker_registry` after provisioning.\nDocker Registry to use for storing the function's Docker images. Allowed values are ARTIFACT_REGISTRY (default) and CONTAINER_REGISTRY."]
    pub fn docker_registry(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.docker_registry", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `docker_repository` after provisioning.\nUser managed repository created in Artifact Registry optionally with a customer managed encryption key. If specified, deployments will use Artifact Registry for storing images built with Cloud Build."]
    pub fn docker_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.docker_repository", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\nName of the function that will be executed when the Google Cloud Function is triggered."]
    pub fn entry_point(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_point", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\nA set of key/value environment variable pairs to assign to the function."]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.environment_variables", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `https_trigger_security_level` after provisioning.\nThe security level for the function. Defaults to SECURE_OPTIONAL. Valid only if trigger_http is used."]
    pub fn https_trigger_security_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.https_trigger_security_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `https_trigger_url` after provisioning.\nURL which triggers function execution. Returned only if trigger_http is used."]
    pub fn https_trigger_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.https_trigger_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingress_settings` after provisioning.\nString value that controls what traffic can reach the function. Allowed values are ALLOW_ALL and ALLOW_INTERNAL_ONLY. Changes to this field will recreate the cloud function."]
    pub fn ingress_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingress_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the function. Label keys must follow the requirements at https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels#requirements.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_instances` after provisioning.\nThe limit on the maximum number of function instances that may coexist at a given time."]
    pub fn max_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_instances` after provisioning.\nThe limit on the minimum number of function instances that may coexist at a given time."]
    pub fn min_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the function. Function names must be unique globally."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject of the function. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion of function. If it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime` after provisioning.\nThe runtime in which the function is going to run. Eg. \"nodejs20\", \"python37\", \"go111\"."]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n If provided, the self-provided service account to run the function with."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_archive_bucket` after provisioning.\nThe GCS bucket containing the zip archive which contains the function."]
    pub fn source_archive_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_archive_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_archive_object` after provisioning.\nThe source archive object (file) in archive bucket."]
    pub fn source_archive_object(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_archive_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nDescribes the current stage of a deployment."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nTimeout (in seconds) for the function. Default value is 60 seconds. Cannot be more than 540 seconds."]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_http` after provisioning.\nBoolean variable. Any HTTP request (of a supported type) to the endpoint will trigger function execution. Supported HTTP request types are: POST, PUT, GET, DELETE, and OPTIONS. Endpoint is returned as https_trigger_url. Cannot be used with trigger_bucket and trigger_topic."]
    pub fn trigger_http(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_http", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_id` after provisioning.\nThe version identifier of the Cloud Function. Each deployment attempt results in a new version of a function being created."]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_connector` after provisioning.\nThe VPC Network Connector that this cloud function can connect to. It can be either the fully-qualified URI, or the short name of the network connector resource. The format of this field is projects/*/locations/*/connectors/*."]
    pub fn vpc_connector(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_connector", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_connector_egress_settings` after provisioning.\nThe egress settings for the connector, controlling what traffic is diverted through it. Allowed values are ALL_TRAFFIC and PRIVATE_RANGES_ONLY. Defaults to PRIVATE_RANGES_ONLY. If unset, this field preserves the previously set value."]
    pub fn vpc_connector_egress_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_connector_egress_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automatic_update_policy` after provisioning.\n"]
    pub fn automatic_update_policy(
        &self,
    ) -> ListRef<CloudfunctionsFunctionAutomaticUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automatic_update_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_trigger` after provisioning.\n"]
    pub fn event_trigger(&self) -> ListRef<CloudfunctionsFunctionEventTriggerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_trigger", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_deploy_update_policy` after provisioning.\n"]
    pub fn on_deploy_update_policy(
        &self,
    ) -> ListRef<CloudfunctionsFunctionOnDeployUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_deploy_update_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_environment_variables` after provisioning.\n"]
    pub fn secret_environment_variables(
        &self,
    ) -> ListRef<CloudfunctionsFunctionSecretEnvironmentVariablesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_environment_variables", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_volumes` after provisioning.\n"]
    pub fn secret_volumes(&self) -> ListRef<CloudfunctionsFunctionSecretVolumesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_volumes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_repository` after provisioning.\n"]
    pub fn source_repository(&self) -> ListRef<CloudfunctionsFunctionSourceRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_repository", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudfunctionsFunctionTimeoutsElRef {
        CloudfunctionsFunctionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudfunctionsFunction {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudfunctionsFunction {}
impl ToListMappable for CloudfunctionsFunction {
    type O = ListRef<CloudfunctionsFunctionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudfunctionsFunction_ {
    fn extract_resource_type(&self) -> String {
        "google_cloudfunctions_function".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudfunctionsFunction {
    pub tf_id: String,
    #[doc = "A user-defined name of the function. Function names must be unique globally."]
    pub name: PrimField<String>,
    #[doc = "The runtime in which the function is going to run. Eg. \"nodejs20\", \"python37\", \"go111\"."]
    pub runtime: PrimField<String>,
}
impl BuildCloudfunctionsFunction {
    pub fn build(self, stack: &mut Stack) -> CloudfunctionsFunction {
        let out = CloudfunctionsFunction(Rc::new(CloudfunctionsFunction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudfunctionsFunctionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                available_memory_mb: core::default::Default::default(),
                build_environment_variables: core::default::Default::default(),
                build_service_account: core::default::Default::default(),
                build_worker_pool: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                docker_registry: core::default::Default::default(),
                docker_repository: core::default::Default::default(),
                entry_point: core::default::Default::default(),
                environment_variables: core::default::Default::default(),
                https_trigger_security_level: core::default::Default::default(),
                https_trigger_url: core::default::Default::default(),
                id: core::default::Default::default(),
                ingress_settings: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                max_instances: core::default::Default::default(),
                min_instances: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                runtime: self.runtime,
                service_account_email: core::default::Default::default(),
                source_archive_bucket: core::default::Default::default(),
                source_archive_object: core::default::Default::default(),
                timeout: core::default::Default::default(),
                trigger_http: core::default::Default::default(),
                vpc_connector: core::default::Default::default(),
                vpc_connector_egress_settings: core::default::Default::default(),
                automatic_update_policy: core::default::Default::default(),
                event_trigger: core::default::Default::default(),
                on_deploy_update_policy: core::default::Default::default(),
                secret_environment_variables: core::default::Default::default(),
                secret_volumes: core::default::Default::default(),
                source_repository: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudfunctionsFunctionRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfunctionsFunctionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudfunctionsFunctionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `available_memory_mb` after provisioning.\nMemory (in MB), available to the function. Default value is 256. Possible values include 128, 256, 512, 1024, etc."]
    pub fn available_memory_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_memory_mb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_environment_variables` after provisioning.\n A set of key/value environment variable pairs available during build time."]
    pub fn build_environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.build_environment_variables", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_service_account` after provisioning.\nThe fully-qualified name of the service account to be used for the build step of deploying this function"]
    pub fn build_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build_worker_pool` after provisioning.\nName of the Cloud Build Custom Worker Pool that should be used to build the function."]
    pub fn build_worker_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.build_worker_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the function."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `docker_registry` after provisioning.\nDocker Registry to use for storing the function's Docker images. Allowed values are ARTIFACT_REGISTRY (default) and CONTAINER_REGISTRY."]
    pub fn docker_registry(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.docker_registry", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `docker_repository` after provisioning.\nUser managed repository created in Artifact Registry optionally with a customer managed encryption key. If specified, deployments will use Artifact Registry for storing images built with Cloud Build."]
    pub fn docker_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.docker_repository", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\nName of the function that will be executed when the Google Cloud Function is triggered."]
    pub fn entry_point(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_point", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\nA set of key/value environment variable pairs to assign to the function."]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.environment_variables", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `https_trigger_security_level` after provisioning.\nThe security level for the function. Defaults to SECURE_OPTIONAL. Valid only if trigger_http is used."]
    pub fn https_trigger_security_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.https_trigger_security_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `https_trigger_url` after provisioning.\nURL which triggers function execution. Returned only if trigger_http is used."]
    pub fn https_trigger_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.https_trigger_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingress_settings` after provisioning.\nString value that controls what traffic can reach the function. Allowed values are ALLOW_ALL and ALLOW_INTERNAL_ONLY. Changes to this field will recreate the cloud function."]
    pub fn ingress_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingress_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the function. Label keys must follow the requirements at https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels#requirements.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_instances` after provisioning.\nThe limit on the maximum number of function instances that may coexist at a given time."]
    pub fn max_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_instances` after provisioning.\nThe limit on the minimum number of function instances that may coexist at a given time."]
    pub fn min_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instances", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the function. Function names must be unique globally."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject of the function. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion of function. If it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime` after provisioning.\nThe runtime in which the function is going to run. Eg. \"nodejs20\", \"python37\", \"go111\"."]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n If provided, the self-provided service account to run the function with."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_archive_bucket` after provisioning.\nThe GCS bucket containing the zip archive which contains the function."]
    pub fn source_archive_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_archive_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_archive_object` after provisioning.\nThe source archive object (file) in archive bucket."]
    pub fn source_archive_object(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_archive_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nDescribes the current stage of a deployment."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nTimeout (in seconds) for the function. Default value is 60 seconds. Cannot be more than 540 seconds."]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_http` after provisioning.\nBoolean variable. Any HTTP request (of a supported type) to the endpoint will trigger function execution. Supported HTTP request types are: POST, PUT, GET, DELETE, and OPTIONS. Endpoint is returned as https_trigger_url. Cannot be used with trigger_bucket and trigger_topic."]
    pub fn trigger_http(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_http", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_id` after provisioning.\nThe version identifier of the Cloud Function. Each deployment attempt results in a new version of a function being created."]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_connector` after provisioning.\nThe VPC Network Connector that this cloud function can connect to. It can be either the fully-qualified URI, or the short name of the network connector resource. The format of this field is projects/*/locations/*/connectors/*."]
    pub fn vpc_connector(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_connector", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_connector_egress_settings` after provisioning.\nThe egress settings for the connector, controlling what traffic is diverted through it. Allowed values are ALL_TRAFFIC and PRIVATE_RANGES_ONLY. Defaults to PRIVATE_RANGES_ONLY. If unset, this field preserves the previously set value."]
    pub fn vpc_connector_egress_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_connector_egress_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automatic_update_policy` after provisioning.\n"]
    pub fn automatic_update_policy(
        &self,
    ) -> ListRef<CloudfunctionsFunctionAutomaticUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automatic_update_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_trigger` after provisioning.\n"]
    pub fn event_trigger(&self) -> ListRef<CloudfunctionsFunctionEventTriggerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_trigger", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_deploy_update_policy` after provisioning.\n"]
    pub fn on_deploy_update_policy(
        &self,
    ) -> ListRef<CloudfunctionsFunctionOnDeployUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.on_deploy_update_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_environment_variables` after provisioning.\n"]
    pub fn secret_environment_variables(
        &self,
    ) -> ListRef<CloudfunctionsFunctionSecretEnvironmentVariablesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_environment_variables", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_volumes` after provisioning.\n"]
    pub fn secret_volumes(&self) -> ListRef<CloudfunctionsFunctionSecretVolumesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_volumes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_repository` after provisioning.\n"]
    pub fn source_repository(&self) -> ListRef<CloudfunctionsFunctionSourceRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_repository", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudfunctionsFunctionTimeoutsElRef {
        CloudfunctionsFunctionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfunctionsFunctionAutomaticUpdatePolicyEl {}
impl CloudfunctionsFunctionAutomaticUpdatePolicyEl {}
impl ToListMappable for CloudfunctionsFunctionAutomaticUpdatePolicyEl {
    type O = BlockAssignable<CloudfunctionsFunctionAutomaticUpdatePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctionsFunctionAutomaticUpdatePolicyEl {}
impl BuildCloudfunctionsFunctionAutomaticUpdatePolicyEl {
    pub fn build(self) -> CloudfunctionsFunctionAutomaticUpdatePolicyEl {
        CloudfunctionsFunctionAutomaticUpdatePolicyEl {}
    }
}
pub struct CloudfunctionsFunctionAutomaticUpdatePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfunctionsFunctionAutomaticUpdatePolicyElRef {
    fn new(shared: StackShared, base: String) -> CloudfunctionsFunctionAutomaticUpdatePolicyElRef {
        CloudfunctionsFunctionAutomaticUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfunctionsFunctionAutomaticUpdatePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct CloudfunctionsFunctionEventTriggerElFailurePolicyEl {
    retry: PrimField<bool>,
}
impl CloudfunctionsFunctionEventTriggerElFailurePolicyEl {}
impl ToListMappable for CloudfunctionsFunctionEventTriggerElFailurePolicyEl {
    type O = BlockAssignable<CloudfunctionsFunctionEventTriggerElFailurePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctionsFunctionEventTriggerElFailurePolicyEl {
    #[doc = "Whether the function should be retried on failure. Defaults to false."]
    pub retry: PrimField<bool>,
}
impl BuildCloudfunctionsFunctionEventTriggerElFailurePolicyEl {
    pub fn build(self) -> CloudfunctionsFunctionEventTriggerElFailurePolicyEl {
        CloudfunctionsFunctionEventTriggerElFailurePolicyEl { retry: self.retry }
    }
}
pub struct CloudfunctionsFunctionEventTriggerElFailurePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfunctionsFunctionEventTriggerElFailurePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfunctionsFunctionEventTriggerElFailurePolicyElRef {
        CloudfunctionsFunctionEventTriggerElFailurePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfunctionsFunctionEventTriggerElFailurePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `retry` after provisioning.\nWhether the function should be retried on failure. Defaults to false."]
    pub fn retry(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudfunctionsFunctionEventTriggerElDynamic {
    failure_policy: Option<DynamicBlock<CloudfunctionsFunctionEventTriggerElFailurePolicyEl>>,
}
#[derive(Serialize)]
pub struct CloudfunctionsFunctionEventTriggerEl {
    event_type: PrimField<String>,
    resource: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_policy: Option<Vec<CloudfunctionsFunctionEventTriggerElFailurePolicyEl>>,
    dynamic: CloudfunctionsFunctionEventTriggerElDynamic,
}
impl CloudfunctionsFunctionEventTriggerEl {
    #[doc = "Set the field `failure_policy`.\n"]
    pub fn set_failure_policy(
        mut self,
        v: impl Into<BlockAssignable<CloudfunctionsFunctionEventTriggerElFailurePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.failure_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.failure_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfunctionsFunctionEventTriggerEl {
    type O = BlockAssignable<CloudfunctionsFunctionEventTriggerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctionsFunctionEventTriggerEl {
    #[doc = "The type of event to observe. For example: \"google.storage.object.finalize\". See the documentation on calling Cloud Functions for a full reference of accepted triggers."]
    pub event_type: PrimField<String>,
    #[doc = "The name or partial URI of the resource from which to observe events. For example, \"myBucket\" or \"projects/my-project/topics/my-topic\""]
    pub resource: PrimField<String>,
}
impl BuildCloudfunctionsFunctionEventTriggerEl {
    pub fn build(self) -> CloudfunctionsFunctionEventTriggerEl {
        CloudfunctionsFunctionEventTriggerEl {
            event_type: self.event_type,
            resource: self.resource,
            failure_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfunctionsFunctionEventTriggerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfunctionsFunctionEventTriggerElRef {
    fn new(shared: StackShared, base: String) -> CloudfunctionsFunctionEventTriggerElRef {
        CloudfunctionsFunctionEventTriggerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfunctionsFunctionEventTriggerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_type` after provisioning.\nThe type of event to observe. For example: \"google.storage.object.finalize\". See the documentation on calling Cloud Functions for a full reference of accepted triggers."]
    pub fn event_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nThe name or partial URI of the resource from which to observe events. For example, \"myBucket\" or \"projects/my-project/topics/my-topic\""]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
    #[doc = "Get a reference to the value of field `failure_policy` after provisioning.\n"]
    pub fn failure_policy(
        &self,
    ) -> ListRef<CloudfunctionsFunctionEventTriggerElFailurePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.failure_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfunctionsFunctionOnDeployUpdatePolicyEl {}
impl CloudfunctionsFunctionOnDeployUpdatePolicyEl {}
impl ToListMappable for CloudfunctionsFunctionOnDeployUpdatePolicyEl {
    type O = BlockAssignable<CloudfunctionsFunctionOnDeployUpdatePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctionsFunctionOnDeployUpdatePolicyEl {}
impl BuildCloudfunctionsFunctionOnDeployUpdatePolicyEl {
    pub fn build(self) -> CloudfunctionsFunctionOnDeployUpdatePolicyEl {
        CloudfunctionsFunctionOnDeployUpdatePolicyEl {}
    }
}
pub struct CloudfunctionsFunctionOnDeployUpdatePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfunctionsFunctionOnDeployUpdatePolicyElRef {
    fn new(shared: StackShared, base: String) -> CloudfunctionsFunctionOnDeployUpdatePolicyElRef {
        CloudfunctionsFunctionOnDeployUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfunctionsFunctionOnDeployUpdatePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `runtime_version` after provisioning.\nThe runtime version which was used during latest function deployment."]
    pub fn runtime_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfunctionsFunctionSecretEnvironmentVariablesEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    secret: PrimField<String>,
    version: PrimField<String>,
}
impl CloudfunctionsFunctionSecretEnvironmentVariablesEl {
    #[doc = "Set the field `project_id`.\nProject identifier (due to a known limitation, only project number is supported by this field) of the project that contains the secret. If not set, it will be populated with the function's project, assuming that the secret exists in the same project as of the function."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfunctionsFunctionSecretEnvironmentVariablesEl {
    type O = BlockAssignable<CloudfunctionsFunctionSecretEnvironmentVariablesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctionsFunctionSecretEnvironmentVariablesEl {
    #[doc = "Name of the environment variable."]
    pub key: PrimField<String>,
    #[doc = "ID of the secret in secret manager (not the full resource name)."]
    pub secret: PrimField<String>,
    #[doc = "Version of the secret (version number or the string \"latest\"). It is recommended to use a numeric version for secret environment variables as any updates to the secret value is not reflected until new clones start."]
    pub version: PrimField<String>,
}
impl BuildCloudfunctionsFunctionSecretEnvironmentVariablesEl {
    pub fn build(self) -> CloudfunctionsFunctionSecretEnvironmentVariablesEl {
        CloudfunctionsFunctionSecretEnvironmentVariablesEl {
            key: self.key,
            project_id: core::default::Default::default(),
            secret: self.secret,
            version: self.version,
        }
    }
}
pub struct CloudfunctionsFunctionSecretEnvironmentVariablesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfunctionsFunctionSecretEnvironmentVariablesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfunctionsFunctionSecretEnvironmentVariablesElRef {
        CloudfunctionsFunctionSecretEnvironmentVariablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfunctionsFunctionSecretEnvironmentVariablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nName of the environment variable."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject identifier (due to a known limitation, only project number is supported by this field) of the project that contains the secret. If not set, it will be populated with the function's project, assuming that the secret exists in the same project as of the function."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nID of the secret in secret manager (not the full resource name)."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the secret (version number or the string \"latest\"). It is recommended to use a numeric version for secret environment variables as any updates to the secret value is not reflected until new clones start."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfunctionsFunctionSecretVolumesElVersionsEl {
    path: PrimField<String>,
    version: PrimField<String>,
}
impl CloudfunctionsFunctionSecretVolumesElVersionsEl {}
impl ToListMappable for CloudfunctionsFunctionSecretVolumesElVersionsEl {
    type O = BlockAssignable<CloudfunctionsFunctionSecretVolumesElVersionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctionsFunctionSecretVolumesElVersionsEl {
    #[doc = "Relative path of the file under the mount path where the secret value for this version will be fetched and made available. For example, setting the mount_path as \"/etc/secrets\" and path as \"/secret_foo\" would mount the secret value file at \"/etc/secrets/secret_foo\"."]
    pub path: PrimField<String>,
    #[doc = "Version of the secret (version number or the string \"latest\"). It is preferable to use \"latest\" version with secret volumes as secret value changes are reflected immediately."]
    pub version: PrimField<String>,
}
impl BuildCloudfunctionsFunctionSecretVolumesElVersionsEl {
    pub fn build(self) -> CloudfunctionsFunctionSecretVolumesElVersionsEl {
        CloudfunctionsFunctionSecretVolumesElVersionsEl {
            path: self.path,
            version: self.version,
        }
    }
}
pub struct CloudfunctionsFunctionSecretVolumesElVersionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfunctionsFunctionSecretVolumesElVersionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfunctionsFunctionSecretVolumesElVersionsElRef {
        CloudfunctionsFunctionSecretVolumesElVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfunctionsFunctionSecretVolumesElVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nRelative path of the file under the mount path where the secret value for this version will be fetched and made available. For example, setting the mount_path as \"/etc/secrets\" and path as \"/secret_foo\" would mount the secret value file at \"/etc/secrets/secret_foo\"."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the secret (version number or the string \"latest\"). It is preferable to use \"latest\" version with secret volumes as secret value changes are reflected immediately."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudfunctionsFunctionSecretVolumesElDynamic {
    versions: Option<DynamicBlock<CloudfunctionsFunctionSecretVolumesElVersionsEl>>,
}
#[derive(Serialize)]
pub struct CloudfunctionsFunctionSecretVolumesEl {
    mount_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versions: Option<Vec<CloudfunctionsFunctionSecretVolumesElVersionsEl>>,
    dynamic: CloudfunctionsFunctionSecretVolumesElDynamic,
}
impl CloudfunctionsFunctionSecretVolumesEl {
    #[doc = "Set the field `project_id`.\nProject identifier (due to a known limitation, only project number is supported by this field) of the project that contains the secret. If not set, it will be populated with the function's project, assuming that the secret exists in the same project as of the function."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `versions`.\n"]
    pub fn set_versions(
        mut self,
        v: impl Into<BlockAssignable<CloudfunctionsFunctionSecretVolumesElVersionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.versions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.versions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfunctionsFunctionSecretVolumesEl {
    type O = BlockAssignable<CloudfunctionsFunctionSecretVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctionsFunctionSecretVolumesEl {
    #[doc = "The path within the container to mount the secret volume. For example, setting the mount_path as \"/etc/secrets\" would mount the secret value files under the \"/etc/secrets\" directory. This directory will also be completely shadowed and unavailable to mount any other secrets. Recommended mount paths: \"/etc/secrets\" Restricted mount paths: \"/cloudsql\", \"/dev/log\", \"/pod\", \"/proc\", \"/var/log\"."]
    pub mount_path: PrimField<String>,
    #[doc = "ID of the secret in secret manager (not the full resource name)."]
    pub secret: PrimField<String>,
}
impl BuildCloudfunctionsFunctionSecretVolumesEl {
    pub fn build(self) -> CloudfunctionsFunctionSecretVolumesEl {
        CloudfunctionsFunctionSecretVolumesEl {
            mount_path: self.mount_path,
            project_id: core::default::Default::default(),
            secret: self.secret,
            versions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfunctionsFunctionSecretVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfunctionsFunctionSecretVolumesElRef {
    fn new(shared: StackShared, base: String) -> CloudfunctionsFunctionSecretVolumesElRef {
        CloudfunctionsFunctionSecretVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfunctionsFunctionSecretVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\nThe path within the container to mount the secret volume. For example, setting the mount_path as \"/etc/secrets\" would mount the secret value files under the \"/etc/secrets\" directory. This directory will also be completely shadowed and unavailable to mount any other secrets. Recommended mount paths: \"/etc/secrets\" Restricted mount paths: \"/cloudsql\", \"/dev/log\", \"/pod\", \"/proc\", \"/var/log\"."]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject identifier (due to a known limitation, only project number is supported by this field) of the project that contains the secret. If not set, it will be populated with the function's project, assuming that the secret exists in the same project as of the function."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\nID of the secret in secret manager (not the full resource name)."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> ListRef<CloudfunctionsFunctionSecretVolumesElVersionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfunctionsFunctionSourceRepositoryEl {
    url: PrimField<String>,
}
impl CloudfunctionsFunctionSourceRepositoryEl {}
impl ToListMappable for CloudfunctionsFunctionSourceRepositoryEl {
    type O = BlockAssignable<CloudfunctionsFunctionSourceRepositoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctionsFunctionSourceRepositoryEl {
    #[doc = "The URL pointing to the hosted repository where the function is defined."]
    pub url: PrimField<String>,
}
impl BuildCloudfunctionsFunctionSourceRepositoryEl {
    pub fn build(self) -> CloudfunctionsFunctionSourceRepositoryEl {
        CloudfunctionsFunctionSourceRepositoryEl { url: self.url }
    }
}
pub struct CloudfunctionsFunctionSourceRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfunctionsFunctionSourceRepositoryElRef {
    fn new(shared: StackShared, base: String) -> CloudfunctionsFunctionSourceRepositoryElRef {
        CloudfunctionsFunctionSourceRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfunctionsFunctionSourceRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deployed_url` after provisioning.\nThe URL pointing to the hosted repository where the function was defined at the time of deployment."]
    pub fn deployed_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployed_url", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL pointing to the hosted repository where the function is defined."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfunctionsFunctionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudfunctionsFunctionTimeoutsEl {
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
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfunctionsFunctionTimeoutsEl {
    type O = BlockAssignable<CloudfunctionsFunctionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfunctionsFunctionTimeoutsEl {}
impl BuildCloudfunctionsFunctionTimeoutsEl {
    pub fn build(self) -> CloudfunctionsFunctionTimeoutsEl {
        CloudfunctionsFunctionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudfunctionsFunctionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfunctionsFunctionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudfunctionsFunctionTimeoutsElRef {
        CloudfunctionsFunctionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfunctionsFunctionTimeoutsElRef {
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
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudfunctionsFunctionDynamic {
    automatic_update_policy: Option<DynamicBlock<CloudfunctionsFunctionAutomaticUpdatePolicyEl>>,
    event_trigger: Option<DynamicBlock<CloudfunctionsFunctionEventTriggerEl>>,
    on_deploy_update_policy: Option<DynamicBlock<CloudfunctionsFunctionOnDeployUpdatePolicyEl>>,
    secret_environment_variables:
        Option<DynamicBlock<CloudfunctionsFunctionSecretEnvironmentVariablesEl>>,
    secret_volumes: Option<DynamicBlock<CloudfunctionsFunctionSecretVolumesEl>>,
    source_repository: Option<DynamicBlock<CloudfunctionsFunctionSourceRepositoryEl>>,
}
