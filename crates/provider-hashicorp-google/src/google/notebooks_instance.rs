use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NotebooksInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_gpu_driver_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    install_gpu_driver: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_owners: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    machine_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nic_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_proxy_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_remove_data_disk: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_startup_script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_config: Option<Vec<NotebooksInstanceAcceleratorConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_image: Option<Vec<NotebooksInstanceContainerImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_affinity: Option<Vec<NotebooksInstanceReservationAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config: Option<Vec<NotebooksInstanceShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NotebooksInstanceTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_image: Option<Vec<NotebooksInstanceVmImageEl>>,
    dynamic: NotebooksInstanceDynamic,
}
struct NotebooksInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NotebooksInstanceData>,
}
#[derive(Clone)]
pub struct NotebooksInstance(Rc<NotebooksInstance_>);
impl NotebooksInstance {
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
    #[doc = "Set the field `boot_disk_size_gb`.\nThe size of the boot disk in GB attached to this instance,\nup to a maximum of 64000 GB (64 TB). The minimum recommended value is 100 GB.\nIf not specified, this defaults to 100."]
    pub fn set_boot_disk_size_gb(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().boot_disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_type`.\nPossible disk types for notebook instances. Possible values: [\"DISK_TYPE_UNSPECIFIED\", \"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\"]"]
    pub fn set_boot_disk_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().boot_disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\nInstance creation time"]
    pub fn set_create_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_gpu_driver_path`.\nSpecify a custom Cloud Storage path where the GPU driver is stored.\nIf not specified, we'll automatically choose from official GPU drivers."]
    pub fn set_custom_gpu_driver_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_gpu_driver_path = Some(v.into());
        self
    }
    #[doc = "Set the field `data_disk_size_gb`.\nThe size of the data disk in GB attached to this instance,\nup to a maximum of 64000 GB (64 TB).\nYou can choose the size of the data disk based on how big your notebooks and data are.\nIf not specified, this defaults to 100."]
    pub fn set_data_disk_size_gb(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().data_disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `data_disk_type`.\nPossible disk types for notebook instances. Possible values: [\"DISK_TYPE_UNSPECIFIED\", \"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\"]"]
    pub fn set_data_disk_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `desired_state`.\nDesired state of the Notebook Instance. Set this field to 'ACTIVE' to start the Instance, and 'STOPPED' to stop the Instance."]
    pub fn set_desired_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().desired_state = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_encryption`.\nDisk encryption method used on the boot and data disks, defaults to GMEK. Possible values: [\"DISK_ENCRYPTION_UNSPECIFIED\", \"GMEK\", \"CMEK\"]"]
    pub fn set_disk_encryption(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().disk_encryption = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `install_gpu_driver`.\nWhether the end user authorizes Google Cloud to install GPU driver\non this instance. If this field is empty or set to false, the GPU driver\nwon't be installed. Only applicable to instances with GPUs."]
    pub fn set_install_gpu_driver(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().install_gpu_driver = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_owners`.\nThe list of owners of this instance after creation.\nFormat: alias@example.com.\nCurrently supports one owner only.\nIf not specified, all of the service account users of\nyour VM instance's service account can use the instance."]
    pub fn set_instance_owners(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().instance_owners = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\nThe KMS key used to encrypt the disks, only applicable if diskEncryption is CMEK.\nFormat: projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}"]
    pub fn set_kms_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels to apply to this instance. These can be later modified by the setLabels method.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\nCustom metadata to apply to this instance.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_metadata(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe name of the VPC that this instance is in.\nFormat: projects/{project_id}/global/networks/{network_id}"]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `nic_type`.\nThe type of vNIC driver. Possible values: [\"UNSPECIFIED_NIC_TYPE\", \"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn set_nic_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().nic_type = Some(v.into());
        self
    }
    #[doc = "Set the field `no_proxy_access`.\nThe notebook instance will not register with the proxy.."]
    pub fn set_no_proxy_access(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().no_proxy_access = Some(v.into());
        self
    }
    #[doc = "Set the field `no_public_ip`.\nNo public IP will be assigned to this instance."]
    pub fn set_no_public_ip(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().no_public_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `no_remove_data_disk`.\nIf true, the data disk will not be auto deleted when deleting the instance."]
    pub fn set_no_remove_data_disk(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().no_remove_data_disk = Some(v.into());
        self
    }
    #[doc = "Set the field `post_startup_script`.\nPath to a Bash script that automatically runs after a\nnotebook instance fully boots up. The path must be a URL\nor Cloud Storage path (gs://path-to-file/file-name)."]
    pub fn set_post_startup_script(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().post_startup_script = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nThe service account on this instance, giving access to other\nGoogle Cloud services. You can use any service account within\nthe same project, but you must have the service account user\npermission to use the instance. If not specified,\nthe Compute Engine default service account is used."]
    pub fn set_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_scopes`.\nOptional. The URIs of service account scopes to be included in Compute Engine instances.\nIf not specified, the following scopes are defined:\n- https://www.googleapis.com/auth/cloud-platform\n- https://www.googleapis.com/auth/userinfo.email"]
    pub fn set_service_account_scopes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().service_account_scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet`.\nThe name of the subnet that this instance is in.\nFormat: projects/{project_id}/regions/{region}/subnetworks/{subnetwork_id}"]
    pub fn set_subnet(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnet = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nThe Compute Engine tags to add to instance."]
    pub fn set_tags(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\nInstance update time."]
    pub fn set_update_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().update_time = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_config`.\n"]
    pub fn set_accelerator_config(
        self,
        v: impl Into<BlockAssignable<NotebooksInstanceAcceleratorConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().accelerator_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.accelerator_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `container_image`.\n"]
    pub fn set_container_image(
        self,
        v: impl Into<BlockAssignable<NotebooksInstanceContainerImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().container_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.container_image = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        self,
        v: impl Into<BlockAssignable<NotebooksInstanceReservationAffinityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reservation_affinity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reservation_affinity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        self,
        v: impl Into<BlockAssignable<NotebooksInstanceShieldedInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shielded_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.shielded_instance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NotebooksInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_image`.\n"]
    pub fn set_vm_image(self, v: impl Into<BlockAssignable<NotebooksInstanceVmImageEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vm_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vm_image = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nThe size of the boot disk in GB attached to this instance,\nup to a maximum of 64000 GB (64 TB). The minimum recommended value is 100 GB.\nIf not specified, this defaults to 100."]
    pub fn boot_disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_type` after provisioning.\nPossible disk types for notebook instances. Possible values: [\"DISK_TYPE_UNSPECIFIED\", \"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\"]"]
    pub fn boot_disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nInstance creation time"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_gpu_driver_path` after provisioning.\nSpecify a custom Cloud Storage path where the GPU driver is stored.\nIf not specified, we'll automatically choose from official GPU drivers."]
    pub fn custom_gpu_driver_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_gpu_driver_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_disk_size_gb` after provisioning.\nThe size of the data disk in GB attached to this instance,\nup to a maximum of 64000 GB (64 TB).\nYou can choose the size of the data disk based on how big your notebooks and data are.\nIf not specified, this defaults to 100."]
    pub fn data_disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_disk_size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_disk_type` after provisioning.\nPossible disk types for notebook instances. Possible values: [\"DISK_TYPE_UNSPECIFIED\", \"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\"]"]
    pub fn data_disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_disk_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Notebook Instance. Set this field to 'ACTIVE' to start the Instance, and 'STOPPED' to stop the Instance."]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_encryption` after provisioning.\nDisk encryption method used on the boot and data disks, defaults to GMEK. Possible values: [\"DISK_ENCRYPTION_UNSPECIFIED\", \"GMEK\", \"CMEK\"]"]
    pub fn disk_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_encryption", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `install_gpu_driver` after provisioning.\nWhether the end user authorizes Google Cloud to install GPU driver\non this instance. If this field is empty or set to false, the GPU driver\nwon't be installed. Only applicable to instances with GPUs."]
    pub fn install_gpu_driver(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.install_gpu_driver", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_owners` after provisioning.\nThe list of owners of this instance after creation.\nFormat: alias@example.com.\nCurrently supports one owner only.\nIf not specified, all of the service account users of\nyour VM instance's service account can use the instance."]
    pub fn instance_owners(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_owners", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the disks, only applicable if diskEncryption is CMEK.\nFormat: projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this instance. These can be later modified by the setLabels method.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nA reference to the zone where the machine resides."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nA reference to a machine type which defines VM kind."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.machine_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata to apply to this instance.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name specified for the Notebook instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC that this instance is in.\nFormat: projects/{project_id}/global/networks/{network_id}"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nic_type` after provisioning.\nThe type of vNIC driver. Possible values: [\"UNSPECIFIED_NIC_TYPE\", \"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn nic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nic_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `no_proxy_access` after provisioning.\nThe notebook instance will not register with the proxy.."]
    pub fn no_proxy_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_proxy_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `no_public_ip` after provisioning.\nNo public IP will be assigned to this instance."]
    pub fn no_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_public_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `no_remove_data_disk` after provisioning.\nIf true, the data disk will not be auto deleted when deleting the instance."]
    pub fn no_remove_data_disk(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_remove_data_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `post_startup_script` after provisioning.\nPath to a Bash script that automatically runs after a\nnotebook instance fully boots up. The path must be a URL\nor Cloud Storage path (gs://path-to-file/file-name)."]
    pub fn post_startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_startup_script", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_uri` after provisioning.\nThe proxy endpoint that is used to access the Jupyter notebook.\nOnly returned when the resource is in a 'PROVISIONED' state. If\nneeded you can utilize 'terraform apply -refresh-only' to await\nthe population of this value."]
    pub fn proxy_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.proxy_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account on this instance, giving access to other\nGoogle Cloud services. You can use any service account within\nthe same project, but you must have the service account user\npermission to use the instance. If not specified,\nthe Compute Engine default service account is used."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_scopes` after provisioning.\nOptional. The URIs of service account scopes to be included in Compute Engine instances.\nIf not specified, the following scopes are defined:\n- https://www.googleapis.com/auth/cloud-platform\n- https://www.googleapis.com/auth/userinfo.email"]
    pub fn service_account_scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account_scopes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of this instance."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet` after provisioning.\nThe name of the subnet that this instance is in.\nFormat: projects/{project_id}/regions/{region}/subnetworks/{subnetwork_id}"]
    pub fn subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nThe Compute Engine tags to add to instance."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nInstance update time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_config` after provisioning.\n"]
    pub fn accelerator_config(&self) -> ListRef<NotebooksInstanceAcceleratorConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(&self) -> ListRef<NotebooksInstanceContainerImageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_image", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> ListRef<NotebooksInstanceReservationAffinityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_affinity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<NotebooksInstanceShieldedInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NotebooksInstanceTimeoutsElRef {
        NotebooksInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vm_image` after provisioning.\n"]
    pub fn vm_image(&self) -> ListRef<NotebooksInstanceVmImageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vm_image", self.extract_ref()),
        )
    }
}
impl Referable for NotebooksInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NotebooksInstance {}
impl ToListMappable for NotebooksInstance {
    type O = ListRef<NotebooksInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NotebooksInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_notebooks_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNotebooksInstance {
    pub tf_id: String,
    #[doc = "A reference to the zone where the machine resides."]
    pub location: PrimField<String>,
    #[doc = "A reference to a machine type which defines VM kind."]
    pub machine_type: PrimField<String>,
    #[doc = "The name specified for the Notebook instance."]
    pub name: PrimField<String>,
}
impl BuildNotebooksInstance {
    pub fn build(self, stack: &mut Stack) -> NotebooksInstance {
        let out = NotebooksInstance(Rc::new(NotebooksInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NotebooksInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                boot_disk_size_gb: core::default::Default::default(),
                boot_disk_type: core::default::Default::default(),
                create_time: core::default::Default::default(),
                custom_gpu_driver_path: core::default::Default::default(),
                data_disk_size_gb: core::default::Default::default(),
                data_disk_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                disk_encryption: core::default::Default::default(),
                id: core::default::Default::default(),
                install_gpu_driver: core::default::Default::default(),
                instance_owners: core::default::Default::default(),
                kms_key: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                machine_type: self.machine_type,
                metadata: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                nic_type: core::default::Default::default(),
                no_proxy_access: core::default::Default::default(),
                no_public_ip: core::default::Default::default(),
                no_remove_data_disk: core::default::Default::default(),
                post_startup_script: core::default::Default::default(),
                project: core::default::Default::default(),
                service_account: core::default::Default::default(),
                service_account_scopes: core::default::Default::default(),
                subnet: core::default::Default::default(),
                tags: core::default::Default::default(),
                update_time: core::default::Default::default(),
                accelerator_config: core::default::Default::default(),
                container_image: core::default::Default::default(),
                reservation_affinity: core::default::Default::default(),
                shielded_instance_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vm_image: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NotebooksInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NotebooksInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nThe size of the boot disk in GB attached to this instance,\nup to a maximum of 64000 GB (64 TB). The minimum recommended value is 100 GB.\nIf not specified, this defaults to 100."]
    pub fn boot_disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_type` after provisioning.\nPossible disk types for notebook instances. Possible values: [\"DISK_TYPE_UNSPECIFIED\", \"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\"]"]
    pub fn boot_disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nInstance creation time"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_gpu_driver_path` after provisioning.\nSpecify a custom Cloud Storage path where the GPU driver is stored.\nIf not specified, we'll automatically choose from official GPU drivers."]
    pub fn custom_gpu_driver_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_gpu_driver_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_disk_size_gb` after provisioning.\nThe size of the data disk in GB attached to this instance,\nup to a maximum of 64000 GB (64 TB).\nYou can choose the size of the data disk based on how big your notebooks and data are.\nIf not specified, this defaults to 100."]
    pub fn data_disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_disk_size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_disk_type` after provisioning.\nPossible disk types for notebook instances. Possible values: [\"DISK_TYPE_UNSPECIFIED\", \"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\"]"]
    pub fn data_disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_disk_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Notebook Instance. Set this field to 'ACTIVE' to start the Instance, and 'STOPPED' to stop the Instance."]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_encryption` after provisioning.\nDisk encryption method used on the boot and data disks, defaults to GMEK. Possible values: [\"DISK_ENCRYPTION_UNSPECIFIED\", \"GMEK\", \"CMEK\"]"]
    pub fn disk_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_encryption", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `install_gpu_driver` after provisioning.\nWhether the end user authorizes Google Cloud to install GPU driver\non this instance. If this field is empty or set to false, the GPU driver\nwon't be installed. Only applicable to instances with GPUs."]
    pub fn install_gpu_driver(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.install_gpu_driver", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_owners` after provisioning.\nThe list of owners of this instance after creation.\nFormat: alias@example.com.\nCurrently supports one owner only.\nIf not specified, all of the service account users of\nyour VM instance's service account can use the instance."]
    pub fn instance_owners(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_owners", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe KMS key used to encrypt the disks, only applicable if diskEncryption is CMEK.\nFormat: projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this instance. These can be later modified by the setLabels method.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nA reference to the zone where the machine resides."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nA reference to a machine type which defines VM kind."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.machine_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata to apply to this instance.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name specified for the Notebook instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC that this instance is in.\nFormat: projects/{project_id}/global/networks/{network_id}"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nic_type` after provisioning.\nThe type of vNIC driver. Possible values: [\"UNSPECIFIED_NIC_TYPE\", \"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn nic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nic_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `no_proxy_access` after provisioning.\nThe notebook instance will not register with the proxy.."]
    pub fn no_proxy_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_proxy_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `no_public_ip` after provisioning.\nNo public IP will be assigned to this instance."]
    pub fn no_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_public_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `no_remove_data_disk` after provisioning.\nIf true, the data disk will not be auto deleted when deleting the instance."]
    pub fn no_remove_data_disk(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_remove_data_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `post_startup_script` after provisioning.\nPath to a Bash script that automatically runs after a\nnotebook instance fully boots up. The path must be a URL\nor Cloud Storage path (gs://path-to-file/file-name)."]
    pub fn post_startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_startup_script", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_uri` after provisioning.\nThe proxy endpoint that is used to access the Jupyter notebook.\nOnly returned when the resource is in a 'PROVISIONED' state. If\nneeded you can utilize 'terraform apply -refresh-only' to await\nthe population of this value."]
    pub fn proxy_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.proxy_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account on this instance, giving access to other\nGoogle Cloud services. You can use any service account within\nthe same project, but you must have the service account user\npermission to use the instance. If not specified,\nthe Compute Engine default service account is used."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_scopes` after provisioning.\nOptional. The URIs of service account scopes to be included in Compute Engine instances.\nIf not specified, the following scopes are defined:\n- https://www.googleapis.com/auth/cloud-platform\n- https://www.googleapis.com/auth/userinfo.email"]
    pub fn service_account_scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account_scopes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of this instance."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet` after provisioning.\nThe name of the subnet that this instance is in.\nFormat: projects/{project_id}/regions/{region}/subnetworks/{subnetwork_id}"]
    pub fn subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nThe Compute Engine tags to add to instance."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nInstance update time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_config` after provisioning.\n"]
    pub fn accelerator_config(&self) -> ListRef<NotebooksInstanceAcceleratorConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(&self) -> ListRef<NotebooksInstanceContainerImageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_image", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> ListRef<NotebooksInstanceReservationAffinityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_affinity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<NotebooksInstanceShieldedInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NotebooksInstanceTimeoutsElRef {
        NotebooksInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vm_image` after provisioning.\n"]
    pub fn vm_image(&self) -> ListRef<NotebooksInstanceVmImageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vm_image", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NotebooksInstanceAcceleratorConfigEl {
    core_count: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl NotebooksInstanceAcceleratorConfigEl {}
impl ToListMappable for NotebooksInstanceAcceleratorConfigEl {
    type O = BlockAssignable<NotebooksInstanceAcceleratorConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksInstanceAcceleratorConfigEl {
    #[doc = "Count of cores of this accelerator."]
    pub core_count: PrimField<f64>,
    #[doc = "Type of this accelerator. Possible values: [\"ACCELERATOR_TYPE_UNSPECIFIED\", \"NVIDIA_TESLA_K80\", \"NVIDIA_TESLA_P100\", \"NVIDIA_TESLA_V100\", \"NVIDIA_TESLA_P4\", \"NVIDIA_TESLA_T4\", \"NVIDIA_TESLA_T4_VWS\", \"NVIDIA_TESLA_P100_VWS\", \"NVIDIA_TESLA_P4_VWS\", \"NVIDIA_TESLA_A100\", \"TPU_V2\", \"TPU_V3\"]"]
    pub type_: PrimField<String>,
}
impl BuildNotebooksInstanceAcceleratorConfigEl {
    pub fn build(self) -> NotebooksInstanceAcceleratorConfigEl {
        NotebooksInstanceAcceleratorConfigEl {
            core_count: self.core_count,
            type_: self.type_,
        }
    }
}
pub struct NotebooksInstanceAcceleratorConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksInstanceAcceleratorConfigElRef {
    fn new(shared: StackShared, base: String) -> NotebooksInstanceAcceleratorConfigElRef {
        NotebooksInstanceAcceleratorConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksInstanceAcceleratorConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `core_count` after provisioning.\nCount of cores of this accelerator."]
    pub fn core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_count", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of this accelerator. Possible values: [\"ACCELERATOR_TYPE_UNSPECIFIED\", \"NVIDIA_TESLA_K80\", \"NVIDIA_TESLA_P100\", \"NVIDIA_TESLA_V100\", \"NVIDIA_TESLA_P4\", \"NVIDIA_TESLA_T4\", \"NVIDIA_TESLA_T4_VWS\", \"NVIDIA_TESLA_P100_VWS\", \"NVIDIA_TESLA_P4_VWS\", \"NVIDIA_TESLA_A100\", \"TPU_V2\", \"TPU_V3\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct NotebooksInstanceContainerImageEl {
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl NotebooksInstanceContainerImageEl {
    #[doc = "Set the field `tag`.\nThe tag of the container image. If not specified, this defaults to the latest tag."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for NotebooksInstanceContainerImageEl {
    type O = BlockAssignable<NotebooksInstanceContainerImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksInstanceContainerImageEl {
    #[doc = "The path to the container image repository.\nFor example: gcr.io/{project_id}/{imageName}"]
    pub repository: PrimField<String>,
}
impl BuildNotebooksInstanceContainerImageEl {
    pub fn build(self) -> NotebooksInstanceContainerImageEl {
        NotebooksInstanceContainerImageEl {
            repository: self.repository,
            tag: core::default::Default::default(),
        }
    }
}
pub struct NotebooksInstanceContainerImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksInstanceContainerImageElRef {
    fn new(shared: StackShared, base: String) -> NotebooksInstanceContainerImageElRef {
        NotebooksInstanceContainerImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksInstanceContainerImageElRef {
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
pub struct NotebooksInstanceReservationAffinityEl {
    consume_reservation_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl NotebooksInstanceReservationAffinityEl {
    #[doc = "Set the field `key`.\nCorresponds to the label key of reservation resource."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\nCorresponds to the label values of reservation resource."]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for NotebooksInstanceReservationAffinityEl {
    type O = BlockAssignable<NotebooksInstanceReservationAffinityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksInstanceReservationAffinityEl {
    #[doc = "The type of Compute Reservation. Possible values: [\"NO_RESERVATION\", \"ANY_RESERVATION\", \"SPECIFIC_RESERVATION\"]"]
    pub consume_reservation_type: PrimField<String>,
}
impl BuildNotebooksInstanceReservationAffinityEl {
    pub fn build(self) -> NotebooksInstanceReservationAffinityEl {
        NotebooksInstanceReservationAffinityEl {
            consume_reservation_type: self.consume_reservation_type,
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct NotebooksInstanceReservationAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksInstanceReservationAffinityElRef {
    fn new(shared: StackShared, base: String) -> NotebooksInstanceReservationAffinityElRef {
        NotebooksInstanceReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksInstanceReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consume_reservation_type` after provisioning.\nThe type of Compute Reservation. Possible values: [\"NO_RESERVATION\", \"ANY_RESERVATION\", \"SPECIFIC_RESERVATION\"]"]
    pub fn consume_reservation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consume_reservation_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nCorresponds to the label key of reservation resource."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nCorresponds to the label values of reservation resource."]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct NotebooksInstanceShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_vtpm: Option<PrimField<bool>>,
}
impl NotebooksInstanceShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\nDefines whether the instance has integrity monitoring enabled. Enables monitoring and attestation of the\nboot integrity of the instance. The attestation is performed against the integrity policy baseline.\nThis baseline is initially derived from the implicitly trusted boot image when the instance is created.\nEnabled by default."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_secure_boot`.\nDefines whether the instance has Secure Boot enabled. Secure Boot helps ensure that the system only runs\nauthentic software by verifying the digital signature of all boot components, and halting the boot process\nif signature verification fails.\nDisabled by default."]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_vtpm`.\nDefines whether the instance has the vTPM enabled.\nEnabled by default."]
    pub fn set_enable_vtpm(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_vtpm = Some(v.into());
        self
    }
}
impl ToListMappable for NotebooksInstanceShieldedInstanceConfigEl {
    type O = BlockAssignable<NotebooksInstanceShieldedInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksInstanceShieldedInstanceConfigEl {}
impl BuildNotebooksInstanceShieldedInstanceConfigEl {
    pub fn build(self) -> NotebooksInstanceShieldedInstanceConfigEl {
        NotebooksInstanceShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}
pub struct NotebooksInstanceShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksInstanceShieldedInstanceConfigElRef {
    fn new(shared: StackShared, base: String) -> NotebooksInstanceShieldedInstanceConfigElRef {
        NotebooksInstanceShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksInstanceShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nDefines whether the instance has integrity monitoring enabled. Enables monitoring and attestation of the\nboot integrity of the instance. The attestation is performed against the integrity policy baseline.\nThis baseline is initially derived from the implicitly trusted boot image when the instance is created.\nEnabled by default."]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_integrity_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\nDefines whether the instance has Secure Boot enabled. Secure Boot helps ensure that the system only runs\nauthentic software by verifying the digital signature of all boot components, and halting the boot process\nif signature verification fails.\nDisabled by default."]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_vtpm` after provisioning.\nDefines whether the instance has the vTPM enabled.\nEnabled by default."]
    pub fn enable_vtpm(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}
#[derive(Serialize)]
pub struct NotebooksInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NotebooksInstanceTimeoutsEl {
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
impl ToListMappable for NotebooksInstanceTimeoutsEl {
    type O = BlockAssignable<NotebooksInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksInstanceTimeoutsEl {}
impl BuildNotebooksInstanceTimeoutsEl {
    pub fn build(self) -> NotebooksInstanceTimeoutsEl {
        NotebooksInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NotebooksInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NotebooksInstanceTimeoutsElRef {
        NotebooksInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksInstanceTimeoutsElRef {
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
pub struct NotebooksInstanceVmImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_name: Option<PrimField<String>>,
    project: PrimField<String>,
}
impl NotebooksInstanceVmImageEl {
    #[doc = "Set the field `image_family`.\nUse this VM image family to find the image; the newest image in this family will be used."]
    pub fn set_image_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_family = Some(v.into());
        self
    }
    #[doc = "Set the field `image_name`.\nUse VM image name to find the image."]
    pub fn set_image_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_name = Some(v.into());
        self
    }
}
impl ToListMappable for NotebooksInstanceVmImageEl {
    type O = BlockAssignable<NotebooksInstanceVmImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksInstanceVmImageEl {
    #[doc = "The name of the Google Cloud project that this VM image belongs to.\nFormat: projects/{project_id}"]
    pub project: PrimField<String>,
}
impl BuildNotebooksInstanceVmImageEl {
    pub fn build(self) -> NotebooksInstanceVmImageEl {
        NotebooksInstanceVmImageEl {
            image_family: core::default::Default::default(),
            image_name: core::default::Default::default(),
            project: self.project,
        }
    }
}
pub struct NotebooksInstanceVmImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksInstanceVmImageElRef {
    fn new(shared: StackShared, base: String) -> NotebooksInstanceVmImageElRef {
        NotebooksInstanceVmImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksInstanceVmImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_family` after provisioning.\nUse this VM image family to find the image; the newest image in this family will be used."]
    pub fn image_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_family", self.base))
    }
    #[doc = "Get a reference to the value of field `image_name` after provisioning.\nUse VM image name to find the image."]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe name of the Google Cloud project that this VM image belongs to.\nFormat: projects/{project_id}"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}
#[derive(Serialize, Default)]
struct NotebooksInstanceDynamic {
    accelerator_config: Option<DynamicBlock<NotebooksInstanceAcceleratorConfigEl>>,
    container_image: Option<DynamicBlock<NotebooksInstanceContainerImageEl>>,
    reservation_affinity: Option<DynamicBlock<NotebooksInstanceReservationAffinityEl>>,
    shielded_instance_config: Option<DynamicBlock<NotebooksInstanceShieldedInstanceConfigEl>>,
    vm_image: Option<DynamicBlock<NotebooksInstanceVmImageEl>>,
}
