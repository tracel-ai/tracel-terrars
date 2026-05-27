use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeDiskData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    architecture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_snapshot_before_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_snapshot_before_destroy_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_confidential_compute: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    licenses: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    physical_block_size_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_disk: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_instant_snapshot: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_storage_object: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_pool: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    async_primary_disk: Option<Vec<ComputeDiskAsyncPrimaryDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption_key: Option<Vec<ComputeDiskDiskEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_os_features: Option<Vec<ComputeDiskGuestOsFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<ComputeDiskParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_image_encryption_key: Option<Vec<ComputeDiskSourceImageEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot_encryption_key: Option<Vec<ComputeDiskSourceSnapshotEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeDiskTimeoutsEl>,
    dynamic: ComputeDiskDynamic,
}
struct ComputeDisk_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeDiskData>,
}
#[derive(Clone)]
pub struct ComputeDisk(Rc<ComputeDisk_>);
impl ComputeDisk {
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
    #[doc = "Set the field `access_mode`.\nThe access mode of the disk.\nFor example:\n  * READ_WRITE_SINGLE: The default AccessMode, means the disk can be attached to single instance in RW mode.\n  * READ_WRITE_MANY: The AccessMode means the disk can be attached to multiple instances in RW mode.\n  * READ_ONLY_SINGLE: The AccessMode means the disk can be attached to multiple instances in RO mode.\nThe AccessMode is only valid for Hyperdisk disk types."]
    pub fn set_access_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `architecture`.\nThe architecture of the disk. Values include 'X86_64', 'ARM64'."]
    pub fn set_architecture(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().architecture = Some(v.into());
        self
    }
    #[doc = "Set the field `create_snapshot_before_destroy`.\nIf set to true, a snapshot of the disk will be created before it is destroyed.\nIf your disk is encrypted with customer managed encryption keys these will be reused for the snapshot creation.\nThe name of the snapshot by default will be '{{disk-name}}-YYYYMMDD-HHmm'"]
    pub fn set_create_snapshot_before_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_snapshot_before_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `create_snapshot_before_destroy_prefix`.\nThis will set a custom name prefix for the snapshot that's created when the disk is deleted."]
    pub fn set_create_snapshot_before_destroy_prefix(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .create_snapshot_before_destroy_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_confidential_compute`.\nWhether this disk is using confidential compute mode.\nNote: Only supported on hyperdisk skus, disk_encryption_key is required when setting to true"]
    pub fn set_enable_confidential_compute(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_confidential_compute = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `image`.\nThe image from which to initialize this disk. This can be\none of: the image's 'self_link', 'projects/{project}/global/images/{image}',\n'projects/{project}/global/images/family/{family}', 'global/images/{image}',\n'global/images/family/{family}', 'family/{family}', '{project}/{family}',\n'{project}/{image}', '{family}', or '{image}'. If referred by family, the\nimages names must include the family name. If they don't, use the\n[google_compute_image data source](/docs/providers/google/d/compute_image.html).\nFor instance, the image 'centos-6-v20180104' includes its family name 'centos-6'.\nThese images can be referred by family name here."]
    pub fn set_image(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels to apply to this disk.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `licenses`.\nAny applicable license URI."]
    pub fn set_licenses(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().licenses = Some(v.into());
        self
    }
    #[doc = "Set the field `physical_block_size_bytes`.\nPhysical block size of the persistent disk, in bytes. If not present\nin a request, a default value is used. Currently supported sizes\nare 4096 and 16384, other sizes may be added in the future.\nIf an unsupported value is requested, the error message will list\nthe supported values for the caller's project."]
    pub fn set_physical_block_size_bytes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().physical_block_size_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_iops`.\nIndicates how many IOPS must be provisioned for the disk.\nNote: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk\nallows for an update of IOPS every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it"]
    pub fn set_provisioned_iops(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().provisioned_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_throughput`.\nIndicates how much Throughput must be provisioned for the disk.\nNote: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk\nallows for an update of Throughput every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it"]
    pub fn set_provisioned_throughput(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().provisioned_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `size`.\nSize of the persistent disk, specified in GB. You can specify this\nfield when creating a persistent disk using the 'image' or\n'snapshot' parameter, or specify it alone to create an empty\npersistent disk.\n\nIf you specify this field along with 'image' or 'snapshot',\nthe value must not be less than the size of the image\nor the size of the snapshot.\n\n~>**NOTE** If you change the size, Terraform updates the disk size\nif upsizing is detected but recreates the disk if downsizing is requested.\nYou can add 'lifecycle.prevent_destroy' in the config to prevent destroying\nand recreating."]
    pub fn set_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().size = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot`.\nThe source snapshot used to create this disk. You can provide this as\na partial or full URL to the resource. If the snapshot is in another\nproject than this disk, you must supply a full URL. For example, the\nfollowing are valid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot'\n* 'projects/project/global/snapshots/snapshot'\n* 'global/snapshots/snapshot'\n* 'snapshot'"]
    pub fn set_snapshot(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().snapshot = Some(v.into());
        self
    }
    #[doc = "Set the field `source_disk`.\nThe source disk used to create this disk. You can provide this as a partial or full URL to the resource.\nFor example, the following are valid values:\n\n* https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/disks/{disk}\n* https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/disks/{disk}\n* projects/{project}/zones/{zone}/disks/{disk}\n* projects/{project}/regions/{region}/disks/{disk}\n* zones/{zone}/disks/{disk}\n* regions/{region}/disks/{disk}"]
    pub fn set_source_disk(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_disk = Some(v.into());
        self
    }
    #[doc = "Set the field `source_instant_snapshot`.\nThe source instant snapshot used to create this disk. You can provide this as a partial or full URL to the resource.\nFor example, the following are valid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/project/zones/zone/instantSnapshots/instantSnapshot'\n* 'projects/project/zones/zone/instantSnapshots/instantSnapshot'\n* 'zones/zone/instantSnapshots/instantSnapshot'"]
    pub fn set_source_instant_snapshot(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_instant_snapshot = Some(v.into());
        self
    }
    #[doc = "Set the field `source_storage_object`.\nThe full Google Cloud Storage URI where the disk image is stored.\nThis file must be a gzip-compressed tarball whose name ends in .tar.gz or virtual machine disk whose name ends in vmdk.\nValid URIs may start with gs:// or https://storage.googleapis.com/.\nThis flag is not optimized for creating multiple disks from a source storage object.\nTo create many disks from a source storage object, use gcloud compute images import instead."]
    pub fn set_source_storage_object(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_storage_object = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_pool`.\nThe URL or the name of the storage pool in which the new disk is created.\nFor example:\n* https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/storagePools/{storagePool}\n* /projects/{project}/zones/{zone}/storagePools/{storagePool}\n* /zones/{zone}/storagePools/{storagePool}\n* /{storagePool}"]
    pub fn set_storage_pool(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nURL of the disk type resource describing which disk type to use to\ncreate the disk. Provide this when creating the disk."]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nA reference to the zone where the disk resides."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Set the field `async_primary_disk`.\n"]
    pub fn set_async_primary_disk(
        self,
        v: impl Into<BlockAssignable<ComputeDiskAsyncPrimaryDiskEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().async_primary_disk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.async_primary_disk = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disk_encryption_key`.\n"]
    pub fn set_disk_encryption_key(
        self,
        v: impl Into<BlockAssignable<ComputeDiskDiskEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_encryption_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `guest_os_features`.\n"]
    pub fn set_guest_os_features(
        self,
        v: impl Into<BlockAssignable<ComputeDiskGuestOsFeaturesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().guest_os_features = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.guest_os_features = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl Into<BlockAssignable<ComputeDiskParamsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_image_encryption_key`.\n"]
    pub fn set_source_image_encryption_key(
        self,
        v: impl Into<BlockAssignable<ComputeDiskSourceImageEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_image_encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_image_encryption_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_snapshot_encryption_key`.\n"]
    pub fn set_source_snapshot_encryption_key(
        self,
        v: impl Into<BlockAssignable<ComputeDiskSourceSnapshotEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_snapshot_encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .source_snapshot_encryption_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeDiskTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\nThe access mode of the disk.\nFor example:\n  * READ_WRITE_SINGLE: The default AccessMode, means the disk can be attached to single instance in RW mode.\n  * READ_WRITE_MANY: The AccessMode means the disk can be attached to multiple instances in RW mode.\n  * READ_ONLY_SINGLE: The AccessMode means the disk can be attached to multiple instances in RO mode.\nThe AccessMode is only valid for Hyperdisk disk types."]
    pub fn access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `architecture` after provisioning.\nThe architecture of the disk. Values include 'X86_64', 'ARM64'."]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.architecture", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_snapshot_before_destroy` after provisioning.\nIf set to true, a snapshot of the disk will be created before it is destroyed.\nIf your disk is encrypted with customer managed encryption keys these will be reused for the snapshot creation.\nThe name of the snapshot by default will be '{{disk-name}}-YYYYMMDD-HHmm'"]
    pub fn create_snapshot_before_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_snapshot_before_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_snapshot_before_destroy_prefix` after provisioning.\nThis will set a custom name prefix for the snapshot that's created when the disk is deleted."]
    pub fn create_snapshot_before_destroy_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.create_snapshot_before_destroy_prefix",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn disk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nWhether this disk is using confidential compute mode.\nNote: Only supported on hyperdisk skus, disk_encryption_key is required when setting to true"]
    pub fn enable_confidential_compute(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_confidential_compute", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\nThe image from which to initialize this disk. This can be\none of: the image's 'self_link', 'projects/{project}/global/images/{image}',\n'projects/{project}/global/images/family/{family}', 'global/images/{image}',\n'global/images/family/{family}', 'family/{family}', '{project}/{family}',\n'{project}/{image}', '{family}', or '{image}'. If referred by family, the\nimages names must include the family name. If they don't, use the\n[google_compute_image data source](/docs/providers/google/d/compute_image.html).\nFor instance, the image 'centos-6-v20180104' includes its family name 'centos-6'.\nThese images can be referred by family name here."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.  Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this disk.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_attach_timestamp` after provisioning.\nLast attach timestamp in RFC3339 text format."]
    pub fn last_attach_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_attach_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_detach_timestamp` after provisioning.\nLast detach timestamp in RFC3339 text format."]
    pub fn last_detach_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_detach_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `licenses` after provisioning.\nAny applicable license URI."]
    pub fn licenses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.licenses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `physical_block_size_bytes` after provisioning.\nPhysical block size of the persistent disk, in bytes. If not present\nin a request, a default value is used. Currently supported sizes\nare 4096 and 16384, other sizes may be added in the future.\nIf an unsupported value is requested, the error message will list\nthe supported values for the caller's project."]
    pub fn physical_block_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.physical_block_size_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_iops` after provisioning.\nIndicates how many IOPS must be provisioned for the disk.\nNote: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk\nallows for an update of IOPS every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it"]
    pub fn provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\nIndicates how much Throughput must be provisioned for the disk.\nNote: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk\nallows for an update of Throughput every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it"]
    pub fn provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\nSize of the persistent disk, specified in GB. You can specify this\nfield when creating a persistent disk using the 'image' or\n'snapshot' parameter, or specify it alone to create an empty\npersistent disk.\n\nIf you specify this field along with 'image' or 'snapshot',\nthe value must not be less than the size of the image\nor the size of the snapshot.\n\n~>**NOTE** If you change the size, Terraform updates the disk size\nif upsizing is detected but recreates the disk if downsizing is requested.\nYou can add 'lifecycle.prevent_destroy' in the config to prevent destroying\nand recreating."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot` after provisioning.\nThe source snapshot used to create this disk. You can provide this as\na partial or full URL to the resource. If the snapshot is in another\nproject than this disk, you must supply a full URL. For example, the\nfollowing are valid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot'\n* 'projects/project/global/snapshots/snapshot'\n* 'global/snapshots/snapshot'\n* 'snapshot'"]
    pub fn snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_disk` after provisioning.\nThe source disk used to create this disk. You can provide this as a partial or full URL to the resource.\nFor example, the following are valid values:\n\n* https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/disks/{disk}\n* https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/disks/{disk}\n* projects/{project}/zones/{zone}/disks/{disk}\n* projects/{project}/regions/{region}/disks/{disk}\n* zones/{zone}/disks/{disk}\n* regions/{region}/disks/{disk}"]
    pub fn source_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_disk_id` after provisioning.\nThe ID value of the disk used to create this image. This value may\nbe used to determine whether the image was taken from the current\nor a previous instance of a given disk name."]
    pub fn source_disk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_disk_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_image_id` after provisioning.\nThe ID value of the image used to create this disk. This value\nidentifies the exact image that was used to create this persistent\ndisk. For example, if you created the persistent disk from an image\nthat was later deleted and recreated under the same name, the source\nimage ID would identify the exact version of the image that was used."]
    pub fn source_image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_image_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_instant_snapshot` after provisioning.\nThe source instant snapshot used to create this disk. You can provide this as a partial or full URL to the resource.\nFor example, the following are valid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/project/zones/zone/instantSnapshots/instantSnapshot'\n* 'projects/project/zones/zone/instantSnapshots/instantSnapshot'\n* 'zones/zone/instantSnapshots/instantSnapshot'"]
    pub fn source_instant_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instant_snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_instant_snapshot_id` after provisioning.\nThe unique ID of the instant snapshot used to create this disk. This value identifies\nthe exact instant snapshot that was used to create this persistent disk.\nFor example, if you created the persistent disk from an instant snapshot that was later\ndeleted and recreated under the same name, the source instant snapshot ID would identify\nthe exact version of the instant snapshot that was used."]
    pub fn source_instant_snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instant_snapshot_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot_id` after provisioning.\nThe unique ID of the snapshot used to create this disk. This value\nidentifies the exact snapshot that was used to create this persistent\ndisk. For example, if you created the persistent disk from a snapshot\nthat was later deleted and recreated under the same name, the source\nsnapshot ID would identify the exact version of the snapshot that was\nused."]
    pub fn source_snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_snapshot_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_storage_object` after provisioning.\nThe full Google Cloud Storage URI where the disk image is stored.\nThis file must be a gzip-compressed tarball whose name ends in .tar.gz or virtual machine disk whose name ends in vmdk.\nValid URIs may start with gs:// or https://storage.googleapis.com/.\nThis flag is not optimized for creating multiple disks from a source storage object.\nTo create many disks from a source storage object, use gcloud compute images import instead."]
    pub fn source_storage_object(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_storage_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_pool` after provisioning.\nThe URL or the name of the storage pool in which the new disk is created.\nFor example:\n* https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/storagePools/{storagePool}\n* /projects/{project}/zones/{zone}/storagePools/{storagePool}\n* /zones/{zone}/storagePools/{storagePool}\n* /{storagePool}"]
    pub fn storage_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nURL of the disk type resource describing which disk type to use to\ncreate the disk. Provide this when creating the disk."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `users` after provisioning.\nLinks to the users of the disk (attached instances) in form:\nproject/zones/zone/instances/instance"]
    pub fn users(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.users", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nA reference to the zone where the disk resides."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `async_primary_disk` after provisioning.\n"]
    pub fn async_primary_disk(&self) -> ListRef<ComputeDiskAsyncPrimaryDiskElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.async_primary_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_encryption_key` after provisioning.\n"]
    pub fn disk_encryption_key(&self) -> ListRef<ComputeDiskDiskEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeDiskParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_image_encryption_key` after provisioning.\n"]
    pub fn source_image_encryption_key(&self) -> ListRef<ComputeDiskSourceImageEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_image_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(
        &self,
    ) -> ListRef<ComputeDiskSourceSnapshotEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_snapshot_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeDiskTimeoutsElRef {
        ComputeDiskTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeDisk {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeDisk {}
impl ToListMappable for ComputeDisk {
    type O = ListRef<ComputeDiskRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeDisk_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_disk".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeDisk {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeDisk {
    pub fn build(self, stack: &mut Stack) -> ComputeDisk {
        let out = ComputeDisk(Rc::new(ComputeDisk_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeDiskData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_mode: core::default::Default::default(),
                architecture: core::default::Default::default(),
                create_snapshot_before_destroy: core::default::Default::default(),
                create_snapshot_before_destroy_prefix: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_confidential_compute: core::default::Default::default(),
                id: core::default::Default::default(),
                image: core::default::Default::default(),
                labels: core::default::Default::default(),
                licenses: core::default::Default::default(),
                name: self.name,
                physical_block_size_bytes: core::default::Default::default(),
                project: core::default::Default::default(),
                provisioned_iops: core::default::Default::default(),
                provisioned_throughput: core::default::Default::default(),
                size: core::default::Default::default(),
                snapshot: core::default::Default::default(),
                source_disk: core::default::Default::default(),
                source_instant_snapshot: core::default::Default::default(),
                source_storage_object: core::default::Default::default(),
                storage_pool: core::default::Default::default(),
                type_: core::default::Default::default(),
                zone: core::default::Default::default(),
                async_primary_disk: core::default::Default::default(),
                disk_encryption_key: core::default::Default::default(),
                guest_os_features: core::default::Default::default(),
                params: core::default::Default::default(),
                source_image_encryption_key: core::default::Default::default(),
                source_snapshot_encryption_key: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeDiskRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeDiskRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeDiskRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\nThe access mode of the disk.\nFor example:\n  * READ_WRITE_SINGLE: The default AccessMode, means the disk can be attached to single instance in RW mode.\n  * READ_WRITE_MANY: The AccessMode means the disk can be attached to multiple instances in RW mode.\n  * READ_ONLY_SINGLE: The AccessMode means the disk can be attached to multiple instances in RO mode.\nThe AccessMode is only valid for Hyperdisk disk types."]
    pub fn access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `architecture` after provisioning.\nThe architecture of the disk. Values include 'X86_64', 'ARM64'."]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.architecture", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_snapshot_before_destroy` after provisioning.\nIf set to true, a snapshot of the disk will be created before it is destroyed.\nIf your disk is encrypted with customer managed encryption keys these will be reused for the snapshot creation.\nThe name of the snapshot by default will be '{{disk-name}}-YYYYMMDD-HHmm'"]
    pub fn create_snapshot_before_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_snapshot_before_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_snapshot_before_destroy_prefix` after provisioning.\nThis will set a custom name prefix for the snapshot that's created when the disk is deleted."]
    pub fn create_snapshot_before_destroy_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.create_snapshot_before_destroy_prefix",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn disk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nWhether this disk is using confidential compute mode.\nNote: Only supported on hyperdisk skus, disk_encryption_key is required when setting to true"]
    pub fn enable_confidential_compute(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_confidential_compute", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\nThe image from which to initialize this disk. This can be\none of: the image's 'self_link', 'projects/{project}/global/images/{image}',\n'projects/{project}/global/images/family/{family}', 'global/images/{image}',\n'global/images/family/{family}', 'family/{family}', '{project}/{family}',\n'{project}/{image}', '{family}', or '{image}'. If referred by family, the\nimages names must include the family name. If they don't, use the\n[google_compute_image data source](/docs/providers/google/d/compute_image.html).\nFor instance, the image 'centos-6-v20180104' includes its family name 'centos-6'.\nThese images can be referred by family name here."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.  Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this disk.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_attach_timestamp` after provisioning.\nLast attach timestamp in RFC3339 text format."]
    pub fn last_attach_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_attach_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_detach_timestamp` after provisioning.\nLast detach timestamp in RFC3339 text format."]
    pub fn last_detach_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_detach_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `licenses` after provisioning.\nAny applicable license URI."]
    pub fn licenses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.licenses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `physical_block_size_bytes` after provisioning.\nPhysical block size of the persistent disk, in bytes. If not present\nin a request, a default value is used. Currently supported sizes\nare 4096 and 16384, other sizes may be added in the future.\nIf an unsupported value is requested, the error message will list\nthe supported values for the caller's project."]
    pub fn physical_block_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.physical_block_size_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_iops` after provisioning.\nIndicates how many IOPS must be provisioned for the disk.\nNote: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk\nallows for an update of IOPS every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it"]
    pub fn provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\nIndicates how much Throughput must be provisioned for the disk.\nNote: Updating currently is only supported by hyperdisk skus without the need to delete and recreate the disk, hyperdisk\nallows for an update of Throughput every 4 hours. To update your hyperdisk more frequently, you'll need to manually delete and recreate it"]
    pub fn provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\nSize of the persistent disk, specified in GB. You can specify this\nfield when creating a persistent disk using the 'image' or\n'snapshot' parameter, or specify it alone to create an empty\npersistent disk.\n\nIf you specify this field along with 'image' or 'snapshot',\nthe value must not be less than the size of the image\nor the size of the snapshot.\n\n~>**NOTE** If you change the size, Terraform updates the disk size\nif upsizing is detected but recreates the disk if downsizing is requested.\nYou can add 'lifecycle.prevent_destroy' in the config to prevent destroying\nand recreating."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot` after provisioning.\nThe source snapshot used to create this disk. You can provide this as\na partial or full URL to the resource. If the snapshot is in another\nproject than this disk, you must supply a full URL. For example, the\nfollowing are valid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot'\n* 'projects/project/global/snapshots/snapshot'\n* 'global/snapshots/snapshot'\n* 'snapshot'"]
    pub fn snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_disk` after provisioning.\nThe source disk used to create this disk. You can provide this as a partial or full URL to the resource.\nFor example, the following are valid values:\n\n* https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/disks/{disk}\n* https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/disks/{disk}\n* projects/{project}/zones/{zone}/disks/{disk}\n* projects/{project}/regions/{region}/disks/{disk}\n* zones/{zone}/disks/{disk}\n* regions/{region}/disks/{disk}"]
    pub fn source_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_disk_id` after provisioning.\nThe ID value of the disk used to create this image. This value may\nbe used to determine whether the image was taken from the current\nor a previous instance of a given disk name."]
    pub fn source_disk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_disk_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_image_id` after provisioning.\nThe ID value of the image used to create this disk. This value\nidentifies the exact image that was used to create this persistent\ndisk. For example, if you created the persistent disk from an image\nthat was later deleted and recreated under the same name, the source\nimage ID would identify the exact version of the image that was used."]
    pub fn source_image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_image_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_instant_snapshot` after provisioning.\nThe source instant snapshot used to create this disk. You can provide this as a partial or full URL to the resource.\nFor example, the following are valid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/project/zones/zone/instantSnapshots/instantSnapshot'\n* 'projects/project/zones/zone/instantSnapshots/instantSnapshot'\n* 'zones/zone/instantSnapshots/instantSnapshot'"]
    pub fn source_instant_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instant_snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_instant_snapshot_id` after provisioning.\nThe unique ID of the instant snapshot used to create this disk. This value identifies\nthe exact instant snapshot that was used to create this persistent disk.\nFor example, if you created the persistent disk from an instant snapshot that was later\ndeleted and recreated under the same name, the source instant snapshot ID would identify\nthe exact version of the instant snapshot that was used."]
    pub fn source_instant_snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instant_snapshot_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot_id` after provisioning.\nThe unique ID of the snapshot used to create this disk. This value\nidentifies the exact snapshot that was used to create this persistent\ndisk. For example, if you created the persistent disk from a snapshot\nthat was later deleted and recreated under the same name, the source\nsnapshot ID would identify the exact version of the snapshot that was\nused."]
    pub fn source_snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_snapshot_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_storage_object` after provisioning.\nThe full Google Cloud Storage URI where the disk image is stored.\nThis file must be a gzip-compressed tarball whose name ends in .tar.gz or virtual machine disk whose name ends in vmdk.\nValid URIs may start with gs:// or https://storage.googleapis.com/.\nThis flag is not optimized for creating multiple disks from a source storage object.\nTo create many disks from a source storage object, use gcloud compute images import instead."]
    pub fn source_storage_object(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_storage_object", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_pool` after provisioning.\nThe URL or the name of the storage pool in which the new disk is created.\nFor example:\n* https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/storagePools/{storagePool}\n* /projects/{project}/zones/{zone}/storagePools/{storagePool}\n* /zones/{zone}/storagePools/{storagePool}\n* /{storagePool}"]
    pub fn storage_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nURL of the disk type resource describing which disk type to use to\ncreate the disk. Provide this when creating the disk."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `users` after provisioning.\nLinks to the users of the disk (attached instances) in form:\nproject/zones/zone/instances/instance"]
    pub fn users(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.users", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nA reference to the zone where the disk resides."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `async_primary_disk` after provisioning.\n"]
    pub fn async_primary_disk(&self) -> ListRef<ComputeDiskAsyncPrimaryDiskElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.async_primary_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_encryption_key` after provisioning.\n"]
    pub fn disk_encryption_key(&self) -> ListRef<ComputeDiskDiskEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeDiskParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_image_encryption_key` after provisioning.\n"]
    pub fn source_image_encryption_key(&self) -> ListRef<ComputeDiskSourceImageEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_image_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(
        &self,
    ) -> ListRef<ComputeDiskSourceSnapshotEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_snapshot_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeDiskTimeoutsElRef {
        ComputeDiskTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeDiskAsyncPrimaryDiskEl {
    disk: PrimField<String>,
}
impl ComputeDiskAsyncPrimaryDiskEl {}
impl ToListMappable for ComputeDiskAsyncPrimaryDiskEl {
    type O = BlockAssignable<ComputeDiskAsyncPrimaryDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeDiskAsyncPrimaryDiskEl {
    #[doc = "Primary disk for asynchronous disk replication."]
    pub disk: PrimField<String>,
}
impl BuildComputeDiskAsyncPrimaryDiskEl {
    pub fn build(self) -> ComputeDiskAsyncPrimaryDiskEl {
        ComputeDiskAsyncPrimaryDiskEl { disk: self.disk }
    }
}
pub struct ComputeDiskAsyncPrimaryDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeDiskAsyncPrimaryDiskElRef {
    fn new(shared: StackShared, base: String) -> ComputeDiskAsyncPrimaryDiskElRef {
        ComputeDiskAsyncPrimaryDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeDiskAsyncPrimaryDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk` after provisioning.\nPrimary disk for asynchronous disk replication."]
    pub fn disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeDiskDiskEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl ComputeDiskDiskEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\nThe self link of the encryption key used to encrypt the disk. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\nThe service account used for the encryption request for the given KMS key.\nIf absent, the Compute Engine Service Agent service account is used."]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `raw_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
    #[doc = "Set the field `rsa_encrypted_key`.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit\ncustomer-supplied encryption key to either encrypt or decrypt\nthis resource. You can provide either the rawKey or the rsaEncryptedKey."]
    pub fn set_rsa_encrypted_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rsa_encrypted_key = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeDiskDiskEncryptionKeyEl {
    type O = BlockAssignable<ComputeDiskDiskEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeDiskDiskEncryptionKeyEl {}
impl BuildComputeDiskDiskEncryptionKeyEl {
    pub fn build(self) -> ComputeDiskDiskEncryptionKeyEl {
        ComputeDiskDiskEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct ComputeDiskDiskEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeDiskDiskEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeDiskDiskEncryptionKeyElRef {
        ComputeDiskDiskEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeDiskDiskEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key used to encrypt the disk. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account used for the encryption request for the given KMS key.\nIf absent, the Compute Engine Service Agent service account is used."]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }
    #[doc = "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit\ncustomer-supplied encryption key to either encrypt or decrypt\nthis resource. You can provide either the rawKey or the rsaEncryptedKey."]
    pub fn rsa_encrypted_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rsa_encrypted_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sha256` after provisioning.\nThe RFC 4648 base64 encoded SHA-256 hash of the customer-supplied\nencryption key that protects this resource."]
    pub fn sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeDiskGuestOsFeaturesEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ComputeDiskGuestOsFeaturesEl {}
impl ToListMappable for ComputeDiskGuestOsFeaturesEl {
    type O = BlockAssignable<ComputeDiskGuestOsFeaturesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeDiskGuestOsFeaturesEl {
    #[doc = "The type of supported feature. Read [Enabling guest operating system features](https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features) to see a list of available options."]
    pub type_: PrimField<String>,
}
impl BuildComputeDiskGuestOsFeaturesEl {
    pub fn build(self) -> ComputeDiskGuestOsFeaturesEl {
        ComputeDiskGuestOsFeaturesEl { type_: self.type_ }
    }
}
pub struct ComputeDiskGuestOsFeaturesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeDiskGuestOsFeaturesElRef {
    fn new(shared: StackShared, base: String) -> ComputeDiskGuestOsFeaturesElRef {
        ComputeDiskGuestOsFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeDiskGuestOsFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of supported feature. Read [Enabling guest operating system features](https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features) to see a list of available options."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeDiskParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl ComputeDiskParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the disk. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeDiskParamsEl {
    type O = BlockAssignable<ComputeDiskParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeDiskParamsEl {}
impl BuildComputeDiskParamsEl {
    pub fn build(self) -> ComputeDiskParamsEl {
        ComputeDiskParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct ComputeDiskParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeDiskParamsElRef {
    fn new(shared: StackShared, base: String) -> ComputeDiskParamsElRef {
        ComputeDiskParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeDiskParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the disk. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeDiskSourceImageEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
}
impl ComputeDiskSourceImageEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\nThe self link of the encryption key used to encrypt the disk. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\nThe service account used for the encryption request for the given KMS key.\nIf absent, the Compute Engine Service Agent service account is used."]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `raw_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeDiskSourceImageEncryptionKeyEl {
    type O = BlockAssignable<ComputeDiskSourceImageEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeDiskSourceImageEncryptionKeyEl {}
impl BuildComputeDiskSourceImageEncryptionKeyEl {
    pub fn build(self) -> ComputeDiskSourceImageEncryptionKeyEl {
        ComputeDiskSourceImageEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
        }
    }
}
pub struct ComputeDiskSourceImageEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeDiskSourceImageEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeDiskSourceImageEncryptionKeyElRef {
        ComputeDiskSourceImageEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeDiskSourceImageEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key used to encrypt the disk. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account used for the encryption request for the given KMS key.\nIf absent, the Compute Engine Service Agent service account is used."]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }
    #[doc = "Get a reference to the value of field `sha256` after provisioning.\nThe RFC 4648 base64 encoded SHA-256 hash of the customer-supplied\nencryption key that protects this resource."]
    pub fn sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeDiskSourceSnapshotEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
}
impl ComputeDiskSourceSnapshotEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\nThe self link of the encryption key used to encrypt the disk. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\nThe service account used for the encryption request for the given KMS key.\nIf absent, the Compute Engine Service Agent service account is used."]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `raw_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeDiskSourceSnapshotEncryptionKeyEl {
    type O = BlockAssignable<ComputeDiskSourceSnapshotEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeDiskSourceSnapshotEncryptionKeyEl {}
impl BuildComputeDiskSourceSnapshotEncryptionKeyEl {
    pub fn build(self) -> ComputeDiskSourceSnapshotEncryptionKeyEl {
        ComputeDiskSourceSnapshotEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
        }
    }
}
pub struct ComputeDiskSourceSnapshotEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeDiskSourceSnapshotEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeDiskSourceSnapshotEncryptionKeyElRef {
        ComputeDiskSourceSnapshotEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeDiskSourceSnapshotEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key used to encrypt the disk. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account used for the encryption request for the given KMS key.\nIf absent, the Compute Engine Service Agent service account is used."]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }
    #[doc = "Get a reference to the value of field `sha256` after provisioning.\nThe RFC 4648 base64 encoded SHA-256 hash of the customer-supplied\nencryption key that protects this resource."]
    pub fn sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeDiskTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeDiskTimeoutsEl {
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
impl ToListMappable for ComputeDiskTimeoutsEl {
    type O = BlockAssignable<ComputeDiskTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeDiskTimeoutsEl {}
impl BuildComputeDiskTimeoutsEl {
    pub fn build(self) -> ComputeDiskTimeoutsEl {
        ComputeDiskTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeDiskTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeDiskTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeDiskTimeoutsElRef {
        ComputeDiskTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeDiskTimeoutsElRef {
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
struct ComputeDiskDynamic {
    async_primary_disk: Option<DynamicBlock<ComputeDiskAsyncPrimaryDiskEl>>,
    disk_encryption_key: Option<DynamicBlock<ComputeDiskDiskEncryptionKeyEl>>,
    guest_os_features: Option<DynamicBlock<ComputeDiskGuestOsFeaturesEl>>,
    params: Option<DynamicBlock<ComputeDiskParamsEl>>,
    source_image_encryption_key: Option<DynamicBlock<ComputeDiskSourceImageEncryptionKeyEl>>,
    source_snapshot_encryption_key: Option<DynamicBlock<ComputeDiskSourceSnapshotEncryptionKeyEl>>,
}
