use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MwaaEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    airflow_configuration_options: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    airflow_version: Option<PrimField<String>>,
    dag_s3_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_management: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_class: Option<PrimField<String>>,
    execution_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_webservers: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_workers: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_webservers: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_workers: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plugins_s3_object_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plugins_s3_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requirements_s3_object_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requirements_s3_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedulers: Option<PrimField<f64>>,
    source_bucket_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_script_s3_object_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_script_s3_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webserver_access_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_window_start: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_replacement_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_configuration: Option<Vec<MwaaEnvironmentLoggingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configuration: Option<Vec<MwaaEnvironmentNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MwaaEnvironmentTimeoutsEl>,
    dynamic: MwaaEnvironmentDynamic,
}

struct MwaaEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MwaaEnvironmentData>,
}

#[derive(Clone)]
pub struct MwaaEnvironment(Rc<MwaaEnvironment_>);

impl MwaaEnvironment {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `airflow_configuration_options`.\n"]
    pub fn set_airflow_configuration_options(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().airflow_configuration_options = Some(v.into());
        self
    }

    #[doc = "Set the field `airflow_version`.\n"]
    pub fn set_airflow_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().airflow_version = Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_management`.\n"]
    pub fn set_endpoint_management(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().endpoint_management = Some(v.into());
        self
    }

    #[doc = "Set the field `environment_class`.\n"]
    pub fn set_environment_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().environment_class = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key = Some(v.into());
        self
    }

    #[doc = "Set the field `max_webservers`.\n"]
    pub fn set_max_webservers(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_webservers = Some(v.into());
        self
    }

    #[doc = "Set the field `max_workers`.\n"]
    pub fn set_max_workers(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_workers = Some(v.into());
        self
    }

    #[doc = "Set the field `min_webservers`.\n"]
    pub fn set_min_webservers(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_webservers = Some(v.into());
        self
    }

    #[doc = "Set the field `min_workers`.\n"]
    pub fn set_min_workers(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_workers = Some(v.into());
        self
    }

    #[doc = "Set the field `plugins_s3_object_version`.\n"]
    pub fn set_plugins_s3_object_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().plugins_s3_object_version = Some(v.into());
        self
    }

    #[doc = "Set the field `plugins_s3_path`.\n"]
    pub fn set_plugins_s3_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().plugins_s3_path = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `requirements_s3_object_version`.\n"]
    pub fn set_requirements_s3_object_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().requirements_s3_object_version = Some(v.into());
        self
    }

    #[doc = "Set the field `requirements_s3_path`.\n"]
    pub fn set_requirements_s3_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().requirements_s3_path = Some(v.into());
        self
    }

    #[doc = "Set the field `schedulers`.\n"]
    pub fn set_schedulers(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().schedulers = Some(v.into());
        self
    }

    #[doc = "Set the field `startup_script_s3_object_version`.\n"]
    pub fn set_startup_script_s3_object_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().startup_script_s3_object_version = Some(v.into());
        self
    }

    #[doc = "Set the field `startup_script_s3_path`.\n"]
    pub fn set_startup_script_s3_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().startup_script_s3_path = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `webserver_access_mode`.\n"]
    pub fn set_webserver_access_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().webserver_access_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_maintenance_window_start`.\n"]
    pub fn set_weekly_maintenance_window_start(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().weekly_maintenance_window_start = Some(v.into());
        self
    }

    #[doc = "Set the field `worker_replacement_strategy`.\n"]
    pub fn set_worker_replacement_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().worker_replacement_strategy = Some(v.into());
        self
    }

    #[doc = "Set the field `logging_configuration`.\n"]
    pub fn set_logging_configuration(
        self,
        v: impl Into<BlockAssignable<MwaaEnvironmentLoggingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl Into<BlockAssignable<MwaaEnvironmentNetworkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MwaaEnvironmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `airflow_configuration_options` after provisioning.\n"]
    pub fn airflow_configuration_options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.airflow_configuration_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `airflow_version` after provisioning.\n"]
    pub fn airflow_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.airflow_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dag_s3_path` after provisioning.\n"]
    pub fn dag_s3_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dag_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_vpc_endpoint_service` after provisioning.\n"]
    pub fn database_vpc_endpoint_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_vpc_endpoint_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_management` after provisioning.\n"]
    pub fn endpoint_management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_class` after provisioning.\n"]
    pub fn environment_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> ListRef<MwaaEnvironmentLastUpdatedElRef> {
        ListRef::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_webservers` after provisioning.\n"]
    pub fn max_webservers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_webservers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_workers` after provisioning.\n"]
    pub fn max_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_webservers` after provisioning.\n"]
    pub fn min_webservers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_webservers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_workers` after provisioning.\n"]
    pub fn min_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plugins_s3_object_version` after provisioning.\n"]
    pub fn plugins_s3_object_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plugins_s3_object_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plugins_s3_path` after provisioning.\n"]
    pub fn plugins_s3_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plugins_s3_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requirements_s3_object_version` after provisioning.\n"]
    pub fn requirements_s3_object_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.requirements_s3_object_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requirements_s3_path` after provisioning.\n"]
    pub fn requirements_s3_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.requirements_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedulers` after provisioning.\n"]
    pub fn schedulers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedulers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_bucket_arn` after provisioning.\n"]
    pub fn source_bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_bucket_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `startup_script_s3_object_version` after provisioning.\n"]
    pub fn startup_script_s3_object_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.startup_script_s3_object_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `startup_script_s3_path` after provisioning.\n"]
    pub fn startup_script_s3_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.startup_script_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webserver_access_mode` after provisioning.\n"]
    pub fn webserver_access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webserver_access_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webserver_url` after provisioning.\n"]
    pub fn webserver_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webserver_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webserver_vpc_endpoint_service` after provisioning.\n"]
    pub fn webserver_vpc_endpoint_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webserver_vpc_endpoint_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_window_start` after provisioning.\n"]
    pub fn weekly_maintenance_window_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.weekly_maintenance_window_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_replacement_strategy` after provisioning.\n"]
    pub fn worker_replacement_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_replacement_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> ListRef<MwaaEnvironmentLoggingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<MwaaEnvironmentNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MwaaEnvironmentTimeoutsElRef {
        MwaaEnvironmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for MwaaEnvironment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for MwaaEnvironment { }

impl ToListMappable for MwaaEnvironment {
    type O = ListRef<MwaaEnvironmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for MwaaEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "aws_mwaa_environment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMwaaEnvironment {
    pub tf_id: String,
    #[doc = ""]
    pub dag_s3_path: PrimField<String>,
    #[doc = ""]
    pub execution_role_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub source_bucket_arn: PrimField<String>,
}

impl BuildMwaaEnvironment {
    pub fn build(self, stack: &mut Stack) -> MwaaEnvironment {
        let out = MwaaEnvironment(Rc::new(MwaaEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MwaaEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                airflow_configuration_options: core::default::Default::default(),
                airflow_version: core::default::Default::default(),
                dag_s3_path: self.dag_s3_path,
                endpoint_management: core::default::Default::default(),
                environment_class: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                id: core::default::Default::default(),
                kms_key: core::default::Default::default(),
                max_webservers: core::default::Default::default(),
                max_workers: core::default::Default::default(),
                min_webservers: core::default::Default::default(),
                min_workers: core::default::Default::default(),
                name: self.name,
                plugins_s3_object_version: core::default::Default::default(),
                plugins_s3_path: core::default::Default::default(),
                region: core::default::Default::default(),
                requirements_s3_object_version: core::default::Default::default(),
                requirements_s3_path: core::default::Default::default(),
                schedulers: core::default::Default::default(),
                source_bucket_arn: self.source_bucket_arn,
                startup_script_s3_object_version: core::default::Default::default(),
                startup_script_s3_path: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                webserver_access_mode: core::default::Default::default(),
                weekly_maintenance_window_start: core::default::Default::default(),
                worker_replacement_strategy: core::default::Default::default(),
                logging_configuration: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MwaaEnvironmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for MwaaEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl MwaaEnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `airflow_configuration_options` after provisioning.\n"]
    pub fn airflow_configuration_options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.airflow_configuration_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `airflow_version` after provisioning.\n"]
    pub fn airflow_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.airflow_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dag_s3_path` after provisioning.\n"]
    pub fn dag_s3_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dag_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_vpc_endpoint_service` after provisioning.\n"]
    pub fn database_vpc_endpoint_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_vpc_endpoint_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_management` after provisioning.\n"]
    pub fn endpoint_management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_class` after provisioning.\n"]
    pub fn environment_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> ListRef<MwaaEnvironmentLastUpdatedElRef> {
        ListRef::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_webservers` after provisioning.\n"]
    pub fn max_webservers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_webservers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_workers` after provisioning.\n"]
    pub fn max_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_webservers` after provisioning.\n"]
    pub fn min_webservers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_webservers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_workers` after provisioning.\n"]
    pub fn min_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plugins_s3_object_version` after provisioning.\n"]
    pub fn plugins_s3_object_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plugins_s3_object_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plugins_s3_path` after provisioning.\n"]
    pub fn plugins_s3_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plugins_s3_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requirements_s3_object_version` after provisioning.\n"]
    pub fn requirements_s3_object_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.requirements_s3_object_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requirements_s3_path` after provisioning.\n"]
    pub fn requirements_s3_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.requirements_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedulers` after provisioning.\n"]
    pub fn schedulers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedulers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_bucket_arn` after provisioning.\n"]
    pub fn source_bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_bucket_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `startup_script_s3_object_version` after provisioning.\n"]
    pub fn startup_script_s3_object_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.startup_script_s3_object_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `startup_script_s3_path` after provisioning.\n"]
    pub fn startup_script_s3_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.startup_script_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webserver_access_mode` after provisioning.\n"]
    pub fn webserver_access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webserver_access_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webserver_url` after provisioning.\n"]
    pub fn webserver_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webserver_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webserver_vpc_endpoint_service` after provisioning.\n"]
    pub fn webserver_vpc_endpoint_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webserver_vpc_endpoint_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_window_start` after provisioning.\n"]
    pub fn weekly_maintenance_window_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.weekly_maintenance_window_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_replacement_strategy` after provisioning.\n"]
    pub fn worker_replacement_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_replacement_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> ListRef<MwaaEnvironmentLoggingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<MwaaEnvironmentNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MwaaEnvironmentTimeoutsElRef {
        MwaaEnvironmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironmentLastUpdatedElErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<PrimField<String>>,
}

impl MwaaEnvironmentLastUpdatedElErrorEl {
    #[doc = "Set the field `error_code`.\n"]
    pub fn set_error_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_code = Some(v.into());
        self
    }

    #[doc = "Set the field `error_message`.\n"]
    pub fn set_error_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_message = Some(v.into());
        self
    }
}

impl ToListMappable for MwaaEnvironmentLastUpdatedElErrorEl {
    type O = BlockAssignable<MwaaEnvironmentLastUpdatedElErrorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLastUpdatedElErrorEl {}

impl BuildMwaaEnvironmentLastUpdatedElErrorEl {
    pub fn build(self) -> MwaaEnvironmentLastUpdatedElErrorEl {
        MwaaEnvironmentLastUpdatedElErrorEl {
            error_code: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLastUpdatedElErrorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MwaaEnvironmentLastUpdatedElErrorElRef {
    fn new(shared: StackShared, base: String) -> MwaaEnvironmentLastUpdatedElErrorElRef {
        MwaaEnvironmentLastUpdatedElErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLastUpdatedElErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_code` after provisioning.\n"]
    pub fn error_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_code", self.base))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironmentLastUpdatedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ListField<MwaaEnvironmentLastUpdatedElErrorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl MwaaEnvironmentLastUpdatedEl {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc = "Set the field `error`.\n"]
    pub fn set_error(mut self, v: impl Into<ListField<MwaaEnvironmentLastUpdatedElErrorEl>>) -> Self {
        self.error = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for MwaaEnvironmentLastUpdatedEl {
    type O = BlockAssignable<MwaaEnvironmentLastUpdatedEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLastUpdatedEl {}

impl BuildMwaaEnvironmentLastUpdatedEl {
    pub fn build(self) -> MwaaEnvironmentLastUpdatedEl {
        MwaaEnvironmentLastUpdatedEl {
            created_at: core::default::Default::default(),
            error: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLastUpdatedElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MwaaEnvironmentLastUpdatedElRef {
    fn new(shared: StackShared, base: String) -> MwaaEnvironmentLastUpdatedElRef {
        MwaaEnvironmentLastUpdatedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLastUpdatedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> ListRef<MwaaEnvironmentLastUpdatedElErrorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
}

impl MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_level = Some(v.into());
        self
    }
}

impl ToListMappable for MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {
    type O = BlockAssignable<MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {}

impl BuildMwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {
    pub fn build(self) -> MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {
        MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLoggingConfigurationElDagProcessingLogsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MwaaEnvironmentLoggingConfigurationElDagProcessingLogsElRef {
    fn new(shared: StackShared, base: String) -> MwaaEnvironmentLoggingConfigurationElDagProcessingLogsElRef {
        MwaaEnvironmentLoggingConfigurationElDagProcessingLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLoggingConfigurationElDagProcessingLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_log_group_arn` after provisioning.\n"]
    pub fn cloud_watch_log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
}

impl MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_level = Some(v.into());
        self
    }
}

impl ToListMappable for MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {
    type O = BlockAssignable<MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {}

impl BuildMwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {
    pub fn build(self) -> MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {
        MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLoggingConfigurationElSchedulerLogsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MwaaEnvironmentLoggingConfigurationElSchedulerLogsElRef {
    fn new(shared: StackShared, base: String) -> MwaaEnvironmentLoggingConfigurationElSchedulerLogsElRef {
        MwaaEnvironmentLoggingConfigurationElSchedulerLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLoggingConfigurationElSchedulerLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_log_group_arn` after provisioning.\n"]
    pub fn cloud_watch_log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironmentLoggingConfigurationElTaskLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
}

impl MwaaEnvironmentLoggingConfigurationElTaskLogsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_level = Some(v.into());
        self
    }
}

impl ToListMappable for MwaaEnvironmentLoggingConfigurationElTaskLogsEl {
    type O = BlockAssignable<MwaaEnvironmentLoggingConfigurationElTaskLogsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLoggingConfigurationElTaskLogsEl {}

impl BuildMwaaEnvironmentLoggingConfigurationElTaskLogsEl {
    pub fn build(self) -> MwaaEnvironmentLoggingConfigurationElTaskLogsEl {
        MwaaEnvironmentLoggingConfigurationElTaskLogsEl {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLoggingConfigurationElTaskLogsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MwaaEnvironmentLoggingConfigurationElTaskLogsElRef {
    fn new(shared: StackShared, base: String) -> MwaaEnvironmentLoggingConfigurationElTaskLogsElRef {
        MwaaEnvironmentLoggingConfigurationElTaskLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLoggingConfigurationElTaskLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_log_group_arn` after provisioning.\n"]
    pub fn cloud_watch_log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironmentLoggingConfigurationElWebserverLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
}

impl MwaaEnvironmentLoggingConfigurationElWebserverLogsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_level = Some(v.into());
        self
    }
}

impl ToListMappable for MwaaEnvironmentLoggingConfigurationElWebserverLogsEl {
    type O = BlockAssignable<MwaaEnvironmentLoggingConfigurationElWebserverLogsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLoggingConfigurationElWebserverLogsEl {}

impl BuildMwaaEnvironmentLoggingConfigurationElWebserverLogsEl {
    pub fn build(self) -> MwaaEnvironmentLoggingConfigurationElWebserverLogsEl {
        MwaaEnvironmentLoggingConfigurationElWebserverLogsEl {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLoggingConfigurationElWebserverLogsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MwaaEnvironmentLoggingConfigurationElWebserverLogsElRef {
    fn new(shared: StackShared, base: String) -> MwaaEnvironmentLoggingConfigurationElWebserverLogsElRef {
        MwaaEnvironmentLoggingConfigurationElWebserverLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLoggingConfigurationElWebserverLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_log_group_arn` after provisioning.\n"]
    pub fn cloud_watch_log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironmentLoggingConfigurationElWorkerLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
}

impl MwaaEnvironmentLoggingConfigurationElWorkerLogsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_level = Some(v.into());
        self
    }
}

impl ToListMappable for MwaaEnvironmentLoggingConfigurationElWorkerLogsEl {
    type O = BlockAssignable<MwaaEnvironmentLoggingConfigurationElWorkerLogsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLoggingConfigurationElWorkerLogsEl {}

impl BuildMwaaEnvironmentLoggingConfigurationElWorkerLogsEl {
    pub fn build(self) -> MwaaEnvironmentLoggingConfigurationElWorkerLogsEl {
        MwaaEnvironmentLoggingConfigurationElWorkerLogsEl {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLoggingConfigurationElWorkerLogsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MwaaEnvironmentLoggingConfigurationElWorkerLogsElRef {
    fn new(shared: StackShared, base: String) -> MwaaEnvironmentLoggingConfigurationElWorkerLogsElRef {
        MwaaEnvironmentLoggingConfigurationElWorkerLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLoggingConfigurationElWorkerLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_log_group_arn` after provisioning.\n"]
    pub fn cloud_watch_log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct MwaaEnvironmentLoggingConfigurationElDynamic {
    dag_processing_logs: Option<DynamicBlock<MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl>>,
    scheduler_logs: Option<DynamicBlock<MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl>>,
    task_logs: Option<DynamicBlock<MwaaEnvironmentLoggingConfigurationElTaskLogsEl>>,
    webserver_logs: Option<DynamicBlock<MwaaEnvironmentLoggingConfigurationElWebserverLogsEl>>,
    worker_logs: Option<DynamicBlock<MwaaEnvironmentLoggingConfigurationElWorkerLogsEl>>,
}

#[derive(Serialize)]
pub struct MwaaEnvironmentLoggingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dag_processing_logs: Option<Vec<MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduler_logs: Option<Vec<MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_logs: Option<Vec<MwaaEnvironmentLoggingConfigurationElTaskLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webserver_logs: Option<Vec<MwaaEnvironmentLoggingConfigurationElWebserverLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_logs: Option<Vec<MwaaEnvironmentLoggingConfigurationElWorkerLogsEl>>,
    dynamic: MwaaEnvironmentLoggingConfigurationElDynamic,
}

impl MwaaEnvironmentLoggingConfigurationEl {
    #[doc = "Set the field `dag_processing_logs`.\n"]
    pub fn set_dag_processing_logs(
        mut self,
        v: impl Into<BlockAssignable<MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dag_processing_logs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dag_processing_logs = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduler_logs`.\n"]
    pub fn set_scheduler_logs(
        mut self,
        v: impl Into<BlockAssignable<MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scheduler_logs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduler_logs = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `task_logs`.\n"]
    pub fn set_task_logs(
        mut self,
        v: impl Into<BlockAssignable<MwaaEnvironmentLoggingConfigurationElTaskLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.task_logs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.task_logs = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `webserver_logs`.\n"]
    pub fn set_webserver_logs(
        mut self,
        v: impl Into<BlockAssignable<MwaaEnvironmentLoggingConfigurationElWebserverLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.webserver_logs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.webserver_logs = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `worker_logs`.\n"]
    pub fn set_worker_logs(
        mut self,
        v: impl Into<BlockAssignable<MwaaEnvironmentLoggingConfigurationElWorkerLogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.worker_logs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.worker_logs = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for MwaaEnvironmentLoggingConfigurationEl {
    type O = BlockAssignable<MwaaEnvironmentLoggingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLoggingConfigurationEl {}

impl BuildMwaaEnvironmentLoggingConfigurationEl {
    pub fn build(self) -> MwaaEnvironmentLoggingConfigurationEl {
        MwaaEnvironmentLoggingConfigurationEl {
            dag_processing_logs: core::default::Default::default(),
            scheduler_logs: core::default::Default::default(),
            task_logs: core::default::Default::default(),
            webserver_logs: core::default::Default::default(),
            worker_logs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLoggingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MwaaEnvironmentLoggingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> MwaaEnvironmentLoggingConfigurationElRef {
        MwaaEnvironmentLoggingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLoggingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dag_processing_logs` after provisioning.\n"]
    pub fn dag_processing_logs(&self) -> ListRef<MwaaEnvironmentLoggingConfigurationElDagProcessingLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dag_processing_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduler_logs` after provisioning.\n"]
    pub fn scheduler_logs(&self) -> ListRef<MwaaEnvironmentLoggingConfigurationElSchedulerLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduler_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `task_logs` after provisioning.\n"]
    pub fn task_logs(&self) -> ListRef<MwaaEnvironmentLoggingConfigurationElTaskLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.task_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `webserver_logs` after provisioning.\n"]
    pub fn webserver_logs(&self) -> ListRef<MwaaEnvironmentLoggingConfigurationElWebserverLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.webserver_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_logs` after provisioning.\n"]
    pub fn worker_logs(&self) -> ListRef<MwaaEnvironmentLoggingConfigurationElWorkerLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.worker_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironmentNetworkConfigurationEl {
    security_group_ids: SetField<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
}

impl MwaaEnvironmentNetworkConfigurationEl { }

impl ToListMappable for MwaaEnvironmentNetworkConfigurationEl {
    type O = BlockAssignable<MwaaEnvironmentNetworkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentNetworkConfigurationEl {
    #[doc = ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}

impl BuildMwaaEnvironmentNetworkConfigurationEl {
    pub fn build(self) -> MwaaEnvironmentNetworkConfigurationEl {
        MwaaEnvironmentNetworkConfigurationEl {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

pub struct MwaaEnvironmentNetworkConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MwaaEnvironmentNetworkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> MwaaEnvironmentNetworkConfigurationElRef {
        MwaaEnvironmentNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl MwaaEnvironmentTimeoutsEl {
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

impl ToListMappable for MwaaEnvironmentTimeoutsEl {
    type O = BlockAssignable<MwaaEnvironmentTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentTimeoutsEl {}

impl BuildMwaaEnvironmentTimeoutsEl {
    pub fn build(self) -> MwaaEnvironmentTimeoutsEl {
        MwaaEnvironmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MwaaEnvironmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MwaaEnvironmentTimeoutsElRef {
        MwaaEnvironmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentTimeoutsElRef {
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
struct MwaaEnvironmentDynamic {
    logging_configuration: Option<DynamicBlock<MwaaEnvironmentLoggingConfigurationEl>>,
    network_configuration: Option<DynamicBlock<MwaaEnvironmentNetworkConfigurationEl>>,
}
