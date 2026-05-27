use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataflowJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_experiments: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_streaming_engine: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_configuration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_workers: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_wait_on_job_termination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    temp_gcs_location: PrimField<String>,
    template_gcs_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transform_name_mapping: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataflowJobTimeoutsEl>,
}
struct DataflowJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataflowJobData>,
}
#[derive(Clone)]
pub struct DataflowJob(Rc<DataflowJob_>);
impl DataflowJob {
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
    #[doc = "Set the field `additional_experiments`.\nList of experiments that should be used by the job. An example value is [\"enable_stackdriver_agent_metrics\"]."]
    pub fn set_additional_experiments(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().additional_experiments = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_streaming_engine`.\nIndicates if the job should use the streaming engine feature."]
    pub fn set_enable_streaming_engine(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_streaming_engine = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_configuration`.\nThe configuration for VM IPs. Options are \"WORKER_IP_PUBLIC\" or \"WORKER_IP_PRIVATE\"."]
    pub fn set_ip_configuration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\nThe name for the Cloud KMS key for the job. Key format is: projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY"]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. NOTE: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe machine type to use for the job."]
    pub fn set_machine_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `max_workers`.\nThe number of workers permitted to work on the job. More workers may improve processing speed at additional cost."]
    pub fn set_max_workers(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_workers = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe network to which VMs will be assigned. If it is not provided, \"default\" will be used."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `on_delete`.\nOne of \"drain\" or \"cancel\". Specifies behavior of deletion during terraform destroy."]
    pub fn set_on_delete(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().on_delete = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\nKey/Value pairs to be passed to the Dataflow job (as used in the template)."]
    pub fn set_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project in which the resource belongs."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region in which the created job should run."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\nThe Service Account email used to create the job."]
    pub fn set_service_account_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_wait_on_job_termination`.\nIf true, treat DRAINING and CANCELLING as terminal job states and do not wait for further changes before removing from terraform state and moving on. WARNING: this will lead to job name conflicts if you do not ensure that the job names are different, e.g. by embedding a release ID or by using a random_id."]
    pub fn set_skip_wait_on_job_termination(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_wait_on_job_termination = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThe subnetwork to which VMs will be assigned. Should be of the form \"regions/REGION/subnetworks/SUBNETWORK\"."]
    pub fn set_subnetwork(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `transform_name_mapping`.\nOnly applicable when updating a pipeline. Map of transform name prefixes of the job to be replaced with the corresponding name prefixes of the new job."]
    pub fn set_transform_name_mapping(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().transform_name_mapping = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe zone in which the created job should run. If it is not provided, the provider zone is used."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataflowJobTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `additional_experiments` after provisioning.\nList of experiments that should be used by the job. An example value is [\"enable_stackdriver_agent_metrics\"]."]
    pub fn additional_experiments(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_experiments", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `enable_streaming_engine` after provisioning.\nIndicates if the job should use the streaming engine feature."]
    pub fn enable_streaming_engine(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_streaming_engine", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_configuration` after provisioning.\nThe configuration for VM IPs. Options are \"WORKER_IP_PUBLIC\" or \"WORKER_IP_PRIVATE\"."]
    pub fn ip_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_id` after provisioning.\nThe unique ID of this job."]
    pub fn job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe name for the Cloud KMS key for the job. Key format is: projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. NOTE: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to use for the job."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.machine_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_workers` after provisioning.\nThe number of workers permitted to work on the job. More workers may improve processing speed at additional cost."]
    pub fn max_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_workers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique name for the resource, required by Dataflow."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe network to which VMs will be assigned. If it is not provided, \"default\" will be used."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_delete` after provisioning.\nOne of \"drain\" or \"cancel\". Specifies behavior of deletion during terraform destroy."]
    pub fn on_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\nKey/Value pairs to be passed to the Dataflow job (as used in the template)."]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region in which the created job should run."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nThe Service Account email used to create the job."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_wait_on_job_termination` after provisioning.\nIf true, treat DRAINING and CANCELLING as terminal job states and do not wait for further changes before removing from terraform state and moving on. WARNING: this will lead to job name conflicts if you do not ensure that the job names are different, e.g. by embedding a release ID or by using a random_id."]
    pub fn skip_wait_on_job_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_wait_on_job_termination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the resource, selected from the JobState enum."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe subnetwork to which VMs will be assigned. Should be of the form \"regions/REGION/subnetworks/SUBNETWORK\"."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `temp_gcs_location` after provisioning.\nA writeable location on Google Cloud Storage for the Dataflow job to dump its temporary data."]
    pub fn temp_gcs_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.temp_gcs_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_gcs_path` after provisioning.\nThe Google Cloud Storage path to the Dataflow job template."]
    pub fn template_gcs_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_gcs_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transform_name_mapping` after provisioning.\nOnly applicable when updating a pipeline. Map of transform name prefixes of the job to be replaced with the corresponding name prefixes of the new job."]
    pub fn transform_name_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.transform_name_mapping", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of this job, selected from the JobType enum."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone in which the created job should run. If it is not provided, the provider zone is used."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataflowJobTimeoutsElRef {
        DataflowJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataflowJob {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataflowJob {}
impl ToListMappable for DataflowJob {
    type O = ListRef<DataflowJobRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataflowJob_ {
    fn extract_resource_type(&self) -> String {
        "google_dataflow_job".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataflowJob {
    pub tf_id: String,
    #[doc = "A unique name for the resource, required by Dataflow."]
    pub name: PrimField<String>,
    #[doc = "A writeable location on Google Cloud Storage for the Dataflow job to dump its temporary data."]
    pub temp_gcs_location: PrimField<String>,
    #[doc = "The Google Cloud Storage path to the Dataflow job template."]
    pub template_gcs_path: PrimField<String>,
}
impl BuildDataflowJob {
    pub fn build(self, stack: &mut Stack) -> DataflowJob {
        let out = DataflowJob(Rc::new(DataflowJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataflowJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                additional_experiments: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                enable_streaming_engine: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_configuration: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                machine_type: core::default::Default::default(),
                max_workers: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                on_delete: core::default::Default::default(),
                parameters: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                service_account_email: core::default::Default::default(),
                skip_wait_on_job_termination: core::default::Default::default(),
                subnetwork: core::default::Default::default(),
                temp_gcs_location: self.temp_gcs_location,
                template_gcs_path: self.template_gcs_path,
                transform_name_mapping: core::default::Default::default(),
                zone: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataflowJobRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataflowJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataflowJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_experiments` after provisioning.\nList of experiments that should be used by the job. An example value is [\"enable_stackdriver_agent_metrics\"]."]
    pub fn additional_experiments(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_experiments", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `enable_streaming_engine` after provisioning.\nIndicates if the job should use the streaming engine feature."]
    pub fn enable_streaming_engine(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_streaming_engine", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_configuration` after provisioning.\nThe configuration for VM IPs. Options are \"WORKER_IP_PUBLIC\" or \"WORKER_IP_PRIVATE\"."]
    pub fn ip_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `job_id` after provisioning.\nThe unique ID of this job."]
    pub fn job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe name for the Cloud KMS key for the job. Key format is: projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. NOTE: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to use for the job."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.machine_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_workers` after provisioning.\nThe number of workers permitted to work on the job. More workers may improve processing speed at additional cost."]
    pub fn max_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_workers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique name for the resource, required by Dataflow."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe network to which VMs will be assigned. If it is not provided, \"default\" will be used."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `on_delete` after provisioning.\nOne of \"drain\" or \"cancel\". Specifies behavior of deletion during terraform destroy."]
    pub fn on_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\nKey/Value pairs to be passed to the Dataflow job (as used in the template)."]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region in which the created job should run."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\nThe Service Account email used to create the job."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_wait_on_job_termination` after provisioning.\nIf true, treat DRAINING and CANCELLING as terminal job states and do not wait for further changes before removing from terraform state and moving on. WARNING: this will lead to job name conflicts if you do not ensure that the job names are different, e.g. by embedding a release ID or by using a random_id."]
    pub fn skip_wait_on_job_termination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_wait_on_job_termination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the resource, selected from the JobState enum."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe subnetwork to which VMs will be assigned. Should be of the form \"regions/REGION/subnetworks/SUBNETWORK\"."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `temp_gcs_location` after provisioning.\nA writeable location on Google Cloud Storage for the Dataflow job to dump its temporary data."]
    pub fn temp_gcs_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.temp_gcs_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_gcs_path` after provisioning.\nThe Google Cloud Storage path to the Dataflow job template."]
    pub fn template_gcs_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_gcs_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transform_name_mapping` after provisioning.\nOnly applicable when updating a pipeline. Map of transform name prefixes of the job to be replaced with the corresponding name prefixes of the new job."]
    pub fn transform_name_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.transform_name_mapping", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of this job, selected from the JobType enum."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone in which the created job should run. If it is not provided, the provider zone is used."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataflowJobTimeoutsElRef {
        DataflowJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataflowJobTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataflowJobTimeoutsEl {
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for DataflowJobTimeoutsEl {
    type O = BlockAssignable<DataflowJobTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataflowJobTimeoutsEl {}
impl BuildDataflowJobTimeoutsEl {
    pub fn build(self) -> DataflowJobTimeoutsEl {
        DataflowJobTimeoutsEl {
            update: core::default::Default::default(),
        }
    }
}
pub struct DataflowJobTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataflowJobTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataflowJobTimeoutsElRef {
        DataflowJobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataflowJobTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
