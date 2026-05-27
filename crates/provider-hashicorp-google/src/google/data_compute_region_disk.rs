use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeRegionDiskData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataComputeRegionDisk_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeRegionDiskData>,
}
#[derive(Clone)]
pub struct DataComputeRegionDisk(Rc<DataComputeRegionDisk_>);
impl DataComputeRegionDisk {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nA reference to the region where the disk resides."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\nThe access mode of the disk.\nFor example:\n  * READ_WRITE_SINGLE: The default AccessMode, means the disk can be attached to single instance in RW mode.\n  * READ_WRITE_MANY: The AccessMode means the disk can be attached to multiple instances in RW mode.\n  * READ_ONLY_SINGLE: The AccessMode means the disk can be attached to multiple instances in RO mode.\nThe AccessMode is only valid for Hyperdisk disk types."]
    pub fn access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `async_primary_disk` after provisioning.\nA nested object resource."]
    pub fn async_primary_disk(&self) -> ListRef<DataComputeRegionDiskAsyncPrimaryDiskElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.async_primary_disk", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `disk_encryption_key` after provisioning.\nEncrypts the disk using a customer-supplied encryption key.\n\nAfter you encrypt a disk with a customer-supplied key, you must\nprovide the same key if you use the disk later (e.g. to create a disk\nsnapshot or an image, or to attach the disk to a virtual machine).\n\nCustomer-supplied encryption keys do not protect access to metadata of\nthe disk.\n\nIf you do not provide an encryption key when creating the disk, then\nthe disk will be encrypted using an automatically generated key and\nyou do not need to provide a key to use the disk later."]
    pub fn disk_encryption_key(&self) -> ListRef<DataComputeRegionDiskDiskEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_encryption_key", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `guest_os_features` after provisioning.\nA list of features to enable on the guest operating system.\nApplicable only for bootable disks."]
    pub fn guest_os_features(&self) -> SetRef<DataComputeRegionDiskGuestOsFeaturesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.guest_os_features", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second\nthat the disk can handle. Values must be between 10,000 and 120,000.\nFor more details, see the Extreme persistent disk [documentation](https://cloud.google.com/compute/docs/disks/extreme-persistent-disk)."]
    pub fn provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk. This sets the number of throughput\nmb per second that the disk can handle. Values must be greater than or equal to 1."]
    pub fn provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nA reference to the region where the disk resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_zones` after provisioning.\nURLs of the zones where the disk should be replicated to."]
    pub fn replica_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_zones", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\nSize of the persistent disk, specified in GB. You can specify this\nfield when creating a persistent disk using the sourceImage or\nsourceSnapshot parameter, or specify it alone to create an empty\npersistent disk.\n\nIf you specify this field along with sourceImage or sourceSnapshot,\nthe value of sizeGb must not be less than the size of the sourceImage\nor the size of the snapshot."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot` after provisioning.\nThe source snapshot used to create this disk. You can provide this as\na partial or full URL to the resource. For example, the following are\nvalid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot'\n* 'projects/project/global/snapshots/snapshot'\n* 'global/snapshots/snapshot'\n* 'snapshot'"]
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
    #[doc = "Get a reference to the value of field `source_image_encryption_key` after provisioning.\nThe customer-supplied encryption key of the source image. Required if\nthe source image is protected by a customer-supplied encryption key."]
    pub fn source_image_encryption_key(
        &self,
    ) -> ListRef<DataComputeRegionDiskSourceImageEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_image_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_image_id` after provisioning.\nThe ID value of the image used to create this disk. This value\nidentifies the exact image that was used to create this persistent\ndisk. For example, if you created the persistent disk from an image\nthat was later deleted and recreated under the same name, the source\nimage ID would identify the exact version of the image that was used."]
    pub fn source_image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_image_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\nThe customer-supplied encryption key of the source snapshot. Required\nif the source snapshot is protected by a customer-supplied encryption\nkey."]
    pub fn source_snapshot_encryption_key(
        &self,
    ) -> ListRef<DataComputeRegionDiskSourceSnapshotEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_snapshot_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot_id` after provisioning.\nThe unique ID of the snapshot used to create this disk. This value\nidentifies the exact snapshot that was used to create this persistent\ndisk. For example, if you created the persistent disk from a snapshot\nthat was later deleted and recreated under the same name, the source\nsnapshot ID would identify the exact version of the snapshot that was\nused."]
    pub fn source_snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_snapshot_id", self.extract_ref()),
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
}
impl Referable for DataComputeRegionDisk {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeRegionDisk {}
impl ToListMappable for DataComputeRegionDisk {
    type O = ListRef<DataComputeRegionDiskRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeRegionDisk_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_region_disk".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeRegionDisk {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildDataComputeRegionDisk {
    pub fn build(self, stack: &mut Stack) -> DataComputeRegionDisk {
        let out = DataComputeRegionDisk(Rc::new(DataComputeRegionDisk_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeRegionDiskData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeRegionDiskRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionDiskRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeRegionDiskRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\nThe access mode of the disk.\nFor example:\n  * READ_WRITE_SINGLE: The default AccessMode, means the disk can be attached to single instance in RW mode.\n  * READ_WRITE_MANY: The AccessMode means the disk can be attached to multiple instances in RW mode.\n  * READ_ONLY_SINGLE: The AccessMode means the disk can be attached to multiple instances in RO mode.\nThe AccessMode is only valid for Hyperdisk disk types."]
    pub fn access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `async_primary_disk` after provisioning.\nA nested object resource."]
    pub fn async_primary_disk(&self) -> ListRef<DataComputeRegionDiskAsyncPrimaryDiskElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.async_primary_disk", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `disk_encryption_key` after provisioning.\nEncrypts the disk using a customer-supplied encryption key.\n\nAfter you encrypt a disk with a customer-supplied key, you must\nprovide the same key if you use the disk later (e.g. to create a disk\nsnapshot or an image, or to attach the disk to a virtual machine).\n\nCustomer-supplied encryption keys do not protect access to metadata of\nthe disk.\n\nIf you do not provide an encryption key when creating the disk, then\nthe disk will be encrypted using an automatically generated key and\nyou do not need to provide a key to use the disk later."]
    pub fn disk_encryption_key(&self) -> ListRef<DataComputeRegionDiskDiskEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_encryption_key", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `guest_os_features` after provisioning.\nA list of features to enable on the guest operating system.\nApplicable only for bootable disks."]
    pub fn guest_os_features(&self) -> SetRef<DataComputeRegionDiskGuestOsFeaturesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.guest_os_features", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second\nthat the disk can handle. Values must be between 10,000 and 120,000.\nFor more details, see the Extreme persistent disk [documentation](https://cloud.google.com/compute/docs/disks/extreme-persistent-disk)."]
    pub fn provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk. This sets the number of throughput\nmb per second that the disk can handle. Values must be greater than or equal to 1."]
    pub fn provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nA reference to the region where the disk resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_zones` after provisioning.\nURLs of the zones where the disk should be replicated to."]
    pub fn replica_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_zones", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\nSize of the persistent disk, specified in GB. You can specify this\nfield when creating a persistent disk using the sourceImage or\nsourceSnapshot parameter, or specify it alone to create an empty\npersistent disk.\n\nIf you specify this field along with sourceImage or sourceSnapshot,\nthe value of sizeGb must not be less than the size of the sourceImage\nor the size of the snapshot."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot` after provisioning.\nThe source snapshot used to create this disk. You can provide this as\na partial or full URL to the resource. For example, the following are\nvalid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot'\n* 'projects/project/global/snapshots/snapshot'\n* 'global/snapshots/snapshot'\n* 'snapshot'"]
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
    #[doc = "Get a reference to the value of field `source_image_encryption_key` after provisioning.\nThe customer-supplied encryption key of the source image. Required if\nthe source image is protected by a customer-supplied encryption key."]
    pub fn source_image_encryption_key(
        &self,
    ) -> ListRef<DataComputeRegionDiskSourceImageEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_image_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_image_id` after provisioning.\nThe ID value of the image used to create this disk. This value\nidentifies the exact image that was used to create this persistent\ndisk. For example, if you created the persistent disk from an image\nthat was later deleted and recreated under the same name, the source\nimage ID would identify the exact version of the image that was used."]
    pub fn source_image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_image_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\nThe customer-supplied encryption key of the source snapshot. Required\nif the source snapshot is protected by a customer-supplied encryption\nkey."]
    pub fn source_snapshot_encryption_key(
        &self,
    ) -> ListRef<DataComputeRegionDiskSourceSnapshotEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_snapshot_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot_id` after provisioning.\nThe unique ID of the snapshot used to create this disk. This value\nidentifies the exact snapshot that was used to create this persistent\ndisk. For example, if you created the persistent disk from a snapshot\nthat was later deleted and recreated under the same name, the source\nsnapshot ID would identify the exact version of the snapshot that was\nused."]
    pub fn source_snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_snapshot_id", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataComputeRegionDiskAsyncPrimaryDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<PrimField<String>>,
}
impl DataComputeRegionDiskAsyncPrimaryDiskEl {
    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionDiskAsyncPrimaryDiskEl {
    type O = BlockAssignable<DataComputeRegionDiskAsyncPrimaryDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionDiskAsyncPrimaryDiskEl {}
impl BuildDataComputeRegionDiskAsyncPrimaryDiskEl {
    pub fn build(self) -> DataComputeRegionDiskAsyncPrimaryDiskEl {
        DataComputeRegionDiskAsyncPrimaryDiskEl {
            disk: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionDiskAsyncPrimaryDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionDiskAsyncPrimaryDiskElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionDiskAsyncPrimaryDiskElRef {
        DataComputeRegionDiskAsyncPrimaryDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionDiskAsyncPrimaryDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionDiskDiskEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256: Option<PrimField<String>>,
}
impl DataComputeRegionDiskDiskEncryptionKeyEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `raw_key`.\n"]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
    #[doc = "Set the field `rsa_encrypted_key`.\n"]
    pub fn set_rsa_encrypted_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rsa_encrypted_key = Some(v.into());
        self
    }
    #[doc = "Set the field `sha256`.\n"]
    pub fn set_sha256(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha256 = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionDiskDiskEncryptionKeyEl {
    type O = BlockAssignable<DataComputeRegionDiskDiskEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionDiskDiskEncryptionKeyEl {}
impl BuildDataComputeRegionDiskDiskEncryptionKeyEl {
    pub fn build(self) -> DataComputeRegionDiskDiskEncryptionKeyEl {
        DataComputeRegionDiskDiskEncryptionKeyEl {
            kms_key_name: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
            sha256: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionDiskDiskEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionDiskDiskEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionDiskDiskEncryptionKeyElRef {
        DataComputeRegionDiskDiskEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionDiskDiskEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\n"]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }
    #[doc = "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\n"]
    pub fn rsa_encrypted_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rsa_encrypted_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sha256` after provisioning.\n"]
    pub fn sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionDiskGuestOsFeaturesEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataComputeRegionDiskGuestOsFeaturesEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionDiskGuestOsFeaturesEl {
    type O = BlockAssignable<DataComputeRegionDiskGuestOsFeaturesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionDiskGuestOsFeaturesEl {}
impl BuildDataComputeRegionDiskGuestOsFeaturesEl {
    pub fn build(self) -> DataComputeRegionDiskGuestOsFeaturesEl {
        DataComputeRegionDiskGuestOsFeaturesEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionDiskGuestOsFeaturesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionDiskGuestOsFeaturesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionDiskGuestOsFeaturesElRef {
        DataComputeRegionDiskGuestOsFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionDiskGuestOsFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionDiskSourceImageEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256: Option<PrimField<String>>,
}
impl DataComputeRegionDiskSourceImageEncryptionKeyEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\n"]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `raw_key`.\n"]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
    #[doc = "Set the field `rsa_encrypted_key`.\n"]
    pub fn set_rsa_encrypted_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rsa_encrypted_key = Some(v.into());
        self
    }
    #[doc = "Set the field `sha256`.\n"]
    pub fn set_sha256(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha256 = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionDiskSourceImageEncryptionKeyEl {
    type O = BlockAssignable<DataComputeRegionDiskSourceImageEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionDiskSourceImageEncryptionKeyEl {}
impl BuildDataComputeRegionDiskSourceImageEncryptionKeyEl {
    pub fn build(self) -> DataComputeRegionDiskSourceImageEncryptionKeyEl {
        DataComputeRegionDiskSourceImageEncryptionKeyEl {
            kms_key_name: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
            sha256: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionDiskSourceImageEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionDiskSourceImageEncryptionKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionDiskSourceImageEncryptionKeyElRef {
        DataComputeRegionDiskSourceImageEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionDiskSourceImageEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\n"]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\n"]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }
    #[doc = "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\n"]
    pub fn rsa_encrypted_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rsa_encrypted_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sha256` after provisioning.\n"]
    pub fn sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionDiskSourceSnapshotEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256: Option<PrimField<String>>,
}
impl DataComputeRegionDiskSourceSnapshotEncryptionKeyEl {
    #[doc = "Set the field `raw_key`.\n"]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
    #[doc = "Set the field `sha256`.\n"]
    pub fn set_sha256(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha256 = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionDiskSourceSnapshotEncryptionKeyEl {
    type O = BlockAssignable<DataComputeRegionDiskSourceSnapshotEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionDiskSourceSnapshotEncryptionKeyEl {}
impl BuildDataComputeRegionDiskSourceSnapshotEncryptionKeyEl {
    pub fn build(self) -> DataComputeRegionDiskSourceSnapshotEncryptionKeyEl {
        DataComputeRegionDiskSourceSnapshotEncryptionKeyEl {
            raw_key: core::default::Default::default(),
            sha256: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionDiskSourceSnapshotEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionDiskSourceSnapshotEncryptionKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionDiskSourceSnapshotEncryptionKeyElRef {
        DataComputeRegionDiskSourceSnapshotEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionDiskSourceSnapshotEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\n"]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }
    #[doc = "Get a reference to the value of field `sha256` after provisioning.\n"]
    pub fn sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }
}
