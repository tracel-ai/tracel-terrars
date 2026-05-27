use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeImageData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    licenses: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_disk: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_locations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_os_features: Option<Vec<ComputeImageGuestOsFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_encryption_key: Option<Vec<ComputeImageImageEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<ComputeImageParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_disk: Option<Vec<ComputeImageRawDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_initial_state: Option<Vec<ComputeImageShieldedInstanceInitialStateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_disk_encryption_key: Option<Vec<ComputeImageSourceDiskEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_image_encryption_key: Option<Vec<ComputeImageSourceImageEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot_encryption_key: Option<Vec<ComputeImageSourceSnapshotEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeImageTimeoutsEl>,
    dynamic: ComputeImageDynamic,
}
struct ComputeImage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeImageData>,
}
#[derive(Clone)]
pub struct ComputeImage(Rc<ComputeImage_>);
impl ComputeImage {
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
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\nSize of the image when restored onto a persistent disk (in GB)."]
    pub fn set_disk_size_gb(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `family`.\nThe name of the image family to which this image belongs. You can\ncreate disks by specifying an image family instead of a specific\nimage name. The image family always returns its latest image that is\nnot deprecated. The name of the image family must comply with\nRFC1035."]
    pub fn set_family(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().family = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels to apply to this Image.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `licenses`.\nAny applicable license URI."]
    pub fn set_licenses(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().licenses = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `source_disk`.\nThe source disk to create this image based on.\nYou must provide either this property or the\nrawDisk.source property but not both to create an image."]
    pub fn set_source_disk(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_disk = Some(v.into());
        self
    }
    #[doc = "Set the field `source_image`.\nURL of the source image used to create this image. In order to create an image, you must provide the full or partial\nURL of one of the following:\n\n* The selfLink URL\n* This property\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn set_source_image(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_image = Some(v.into());
        self
    }
    #[doc = "Set the field `source_snapshot`.\nURL of the source snapshot used to create this image.\n\nIn order to create an image, you must provide the full or partial URL of one of the following:\n\n* The selfLink URL\n* This property\n* The sourceImage URL\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn set_source_snapshot(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_snapshot = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_locations`.\nCloud Storage bucket storage location of the image\n(regional or multi-regional).\nReference link: https://cloud.google.com/compute/docs/reference/rest/v1/images"]
    pub fn set_storage_locations(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().storage_locations = Some(v.into());
        self
    }
    #[doc = "Set the field `guest_os_features`.\n"]
    pub fn set_guest_os_features(
        self,
        v: impl Into<BlockAssignable<ComputeImageGuestOsFeaturesEl>>,
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
    #[doc = "Set the field `image_encryption_key`.\n"]
    pub fn set_image_encryption_key(
        self,
        v: impl Into<BlockAssignable<ComputeImageImageEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_encryption_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl Into<BlockAssignable<ComputeImageParamsEl>>) -> Self {
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
    #[doc = "Set the field `raw_disk`.\n"]
    pub fn set_raw_disk(self, v: impl Into<BlockAssignable<ComputeImageRawDiskEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().raw_disk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.raw_disk = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `shielded_instance_initial_state`.\n"]
    pub fn set_shielded_instance_initial_state(
        self,
        v: impl Into<BlockAssignable<ComputeImageShieldedInstanceInitialStateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shielded_instance_initial_state = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .shielded_instance_initial_state = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_disk_encryption_key`.\n"]
    pub fn set_source_disk_encryption_key(
        self,
        v: impl Into<BlockAssignable<ComputeImageSourceDiskEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_disk_encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_disk_encryption_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_image_encryption_key`.\n"]
    pub fn set_source_image_encryption_key(
        self,
        v: impl Into<BlockAssignable<ComputeImageSourceImageEncryptionKeyEl>>,
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
        v: impl Into<BlockAssignable<ComputeImageSourceSnapshotEncryptionKeyEl>>,
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
    pub fn set_timeouts(self, v: impl Into<ComputeImageTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `archive_size_bytes` after provisioning.\nSize of the image tar.gz archive stored in Google Cloud Storage (in\nbytes)."]
    pub fn archive_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.archive_size_bytes", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the image when restored onto a persistent disk (in GB)."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `family` after provisioning.\nThe name of the image family to which this image belongs. You can\ncreate disks by specifying an image family instead of a specific\nimage name. The image family always returns its latest image that is\nnot deprecated. The name of the image family must comply with\nRFC1035."]
    pub fn family(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.family", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource. Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this Image.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `licenses` after provisioning.\nAny applicable license URI."]
    pub fn licenses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.licenses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_disk` after provisioning.\nThe source disk to create this image based on.\nYou must provide either this property or the\nrawDisk.source property but not both to create an image."]
    pub fn source_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_image` after provisioning.\nURL of the source image used to create this image. In order to create an image, you must provide the full or partial\nURL of one of the following:\n\n* The selfLink URL\n* This property\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn source_image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_image", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot` after provisioning.\nURL of the source snapshot used to create this image.\n\nIn order to create an image, you must provide the full or partial URL of one of the following:\n\n* The selfLink URL\n* This property\n* The sourceImage URL\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn source_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_locations` after provisioning.\nCloud Storage bucket storage location of the image\n(regional or multi-regional).\nReference link: https://cloud.google.com/compute/docs/reference/rest/v1/images"]
    pub fn storage_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_encryption_key` after provisioning.\n"]
    pub fn image_encryption_key(&self) -> ListRef<ComputeImageImageEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeImageParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `raw_disk` after provisioning.\n"]
    pub fn raw_disk(&self) -> ListRef<ComputeImageRawDiskElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.raw_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_initial_state` after provisioning.\n"]
    pub fn shielded_instance_initial_state(
        &self,
    ) -> ListRef<ComputeImageShieldedInstanceInitialStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_initial_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_disk_encryption_key` after provisioning.\n"]
    pub fn source_disk_encryption_key(&self) -> ListRef<ComputeImageSourceDiskEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_disk_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_image_encryption_key` after provisioning.\n"]
    pub fn source_image_encryption_key(
        &self,
    ) -> ListRef<ComputeImageSourceImageEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_image_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(
        &self,
    ) -> ListRef<ComputeImageSourceSnapshotEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_snapshot_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeImageTimeoutsElRef {
        ComputeImageTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeImage {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeImage {}
impl ToListMappable for ComputeImage {
    type O = ListRef<ComputeImageRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeImage_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_image".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeImage {
    pub tf_id: String,
    #[doc = "Name of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeImage {
    pub fn build(self, stack: &mut Stack) -> ComputeImage {
        let out = ComputeImage(Rc::new(ComputeImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeImageData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disk_size_gb: core::default::Default::default(),
                family: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                licenses: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                source_disk: core::default::Default::default(),
                source_image: core::default::Default::default(),
                source_snapshot: core::default::Default::default(),
                storage_locations: core::default::Default::default(),
                guest_os_features: core::default::Default::default(),
                image_encryption_key: core::default::Default::default(),
                params: core::default::Default::default(),
                raw_disk: core::default::Default::default(),
                shielded_instance_initial_state: core::default::Default::default(),
                source_disk_encryption_key: core::default::Default::default(),
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
pub struct ComputeImageRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeImageRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_size_bytes` after provisioning.\nSize of the image tar.gz archive stored in Google Cloud Storage (in\nbytes)."]
    pub fn archive_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.archive_size_bytes", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the image when restored onto a persistent disk (in GB)."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_size_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `family` after provisioning.\nThe name of the image family to which this image belongs. You can\ncreate disks by specifying an image family instead of a specific\nimage name. The image family always returns its latest image that is\nnot deprecated. The name of the image family must comply with\nRFC1035."]
    pub fn family(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.family", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource. Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this Image.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `licenses` after provisioning.\nAny applicable license URI."]
    pub fn licenses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.licenses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_disk` after provisioning.\nThe source disk to create this image based on.\nYou must provide either this property or the\nrawDisk.source property but not both to create an image."]
    pub fn source_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_image` after provisioning.\nURL of the source image used to create this image. In order to create an image, you must provide the full or partial\nURL of one of the following:\n\n* The selfLink URL\n* This property\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn source_image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_image", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot` after provisioning.\nURL of the source snapshot used to create this image.\n\nIn order to create an image, you must provide the full or partial URL of one of the following:\n\n* The selfLink URL\n* This property\n* The sourceImage URL\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn source_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_locations` after provisioning.\nCloud Storage bucket storage location of the image\n(regional or multi-regional).\nReference link: https://cloud.google.com/compute/docs/reference/rest/v1/images"]
    pub fn storage_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_encryption_key` after provisioning.\n"]
    pub fn image_encryption_key(&self) -> ListRef<ComputeImageImageEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeImageParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `raw_disk` after provisioning.\n"]
    pub fn raw_disk(&self) -> ListRef<ComputeImageRawDiskElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.raw_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_initial_state` after provisioning.\n"]
    pub fn shielded_instance_initial_state(
        &self,
    ) -> ListRef<ComputeImageShieldedInstanceInitialStateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_initial_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_disk_encryption_key` after provisioning.\n"]
    pub fn source_disk_encryption_key(&self) -> ListRef<ComputeImageSourceDiskEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_disk_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_image_encryption_key` after provisioning.\n"]
    pub fn source_image_encryption_key(
        &self,
    ) -> ListRef<ComputeImageSourceImageEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_image_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(
        &self,
    ) -> ListRef<ComputeImageSourceSnapshotEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_snapshot_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeImageTimeoutsElRef {
        ComputeImageTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeImageGuestOsFeaturesEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ComputeImageGuestOsFeaturesEl {}
impl ToListMappable for ComputeImageGuestOsFeaturesEl {
    type O = BlockAssignable<ComputeImageGuestOsFeaturesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageGuestOsFeaturesEl {
    #[doc = "The type of supported feature. Read [Enabling guest operating system features](https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features) to see a list of available options. Possible values: [\"MULTI_IP_SUBNET\", \"SECURE_BOOT\", \"SEV_CAPABLE\", \"UEFI_COMPATIBLE\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"GVNIC\", \"IDPF\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"TDX_CAPABLE\", \"SEV_LIVE_MIGRATABLE_V2\", \"SNP_SVSM_CAPABLE\"]"]
    pub type_: PrimField<String>,
}
impl BuildComputeImageGuestOsFeaturesEl {
    pub fn build(self) -> ComputeImageGuestOsFeaturesEl {
        ComputeImageGuestOsFeaturesEl { type_: self.type_ }
    }
}
pub struct ComputeImageGuestOsFeaturesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageGuestOsFeaturesElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageGuestOsFeaturesElRef {
        ComputeImageGuestOsFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageGuestOsFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of supported feature. Read [Enabling guest operating system features](https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features) to see a list of available options. Possible values: [\"MULTI_IP_SUBNET\", \"SECURE_BOOT\", \"SEV_CAPABLE\", \"UEFI_COMPATIBLE\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"GVNIC\", \"IDPF\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"TDX_CAPABLE\", \"SEV_LIVE_MIGRATABLE_V2\", \"SNP_SVSM_CAPABLE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeImageImageEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl ComputeImageImageEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\nThe self link of the encryption key that is stored in Google Cloud\nKMS."]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `raw_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
    #[doc = "Set the field `rsa_encrypted_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn set_rsa_encrypted_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rsa_encrypted_key = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeImageImageEncryptionKeyEl {
    type O = BlockAssignable<ComputeImageImageEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageImageEncryptionKeyEl {}
impl BuildComputeImageImageEncryptionKeyEl {
    pub fn build(self) -> ComputeImageImageEncryptionKeyEl {
        ComputeImageImageEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct ComputeImageImageEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageImageEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageImageEncryptionKeyElRef {
        ComputeImageImageEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageImageEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key that is stored in Google Cloud\nKMS."]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
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
    #[doc = "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn rsa_encrypted_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rsa_encrypted_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeImageParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl ComputeImageParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the image. Tag keys and values have the\nsame definition as resource manager tags. Keys and values can be either in numeric format,\nsuch as tagKeys/{tag_key_id} and tagValues/{tag_value_id} or in namespaced format such as\n{org_id|projectId}/{tag_key_short_name} and {tag_value_short_name}. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeImageParamsEl {
    type O = BlockAssignable<ComputeImageParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageParamsEl {}
impl BuildComputeImageParamsEl {
    pub fn build(self) -> ComputeImageParamsEl {
        ComputeImageParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct ComputeImageParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageParamsElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageParamsElRef {
        ComputeImageParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the image. Tag keys and values have the\nsame definition as resource manager tags. Keys and values can be either in numeric format,\nsuch as tagKeys/{tag_key_id} and tagValues/{tag_value_id} or in namespaced format such as\n{org_id|projectId}/{tag_key_short_name} and {tag_value_short_name}. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeImageRawDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha1: Option<PrimField<String>>,
    source: PrimField<String>,
}
impl ComputeImageRawDiskEl {
    #[doc = "Set the field `container_type`.\nThe format used to encode and transmit the block device, which\nshould be TAR. This is just a container and transmission format\nand not a runtime format. Provided by the client when the disk\nimage is created. Default value: \"TAR\" Possible values: [\"TAR\"]"]
    pub fn set_container_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_type = Some(v.into());
        self
    }
    #[doc = "Set the field `sha1`.\nAn optional SHA1 checksum of the disk image before unpackaging.\nThis is provided by the client when the disk image is created."]
    pub fn set_sha1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha1 = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeImageRawDiskEl {
    type O = BlockAssignable<ComputeImageRawDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageRawDiskEl {
    #[doc = "The full Google Cloud Storage URL where disk storage is stored\nYou must provide either this property or the sourceDisk property\nbut not both."]
    pub source: PrimField<String>,
}
impl BuildComputeImageRawDiskEl {
    pub fn build(self) -> ComputeImageRawDiskEl {
        ComputeImageRawDiskEl {
            container_type: core::default::Default::default(),
            sha1: core::default::Default::default(),
            source: self.source,
        }
    }
}
pub struct ComputeImageRawDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageRawDiskElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageRawDiskElRef {
        ComputeImageRawDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageRawDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_type` after provisioning.\nThe format used to encode and transmit the block device, which\nshould be TAR. This is just a container and transmission format\nand not a runtime format. Provided by the client when the disk\nimage is created. Default value: \"TAR\" Possible values: [\"TAR\"]"]
    pub fn container_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sha1` after provisioning.\nAn optional SHA1 checksum of the disk image before unpackaging.\nThis is provided by the client when the disk image is created."]
    pub fn sha1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha1", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\nThe full Google Cloud Storage URL where disk storage is stored\nYou must provide either this property or the sourceDisk property\nbut not both."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeImageShieldedInstanceInitialStateElDbsEl {
    content: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_type: Option<PrimField<String>>,
}
impl ComputeImageShieldedInstanceInitialStateElDbsEl {
    #[doc = "Set the field `file_type`.\nThe file type of source file."]
    pub fn set_file_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_type = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeImageShieldedInstanceInitialStateElDbsEl {
    type O = BlockAssignable<ComputeImageShieldedInstanceInitialStateElDbsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageShieldedInstanceInitialStateElDbsEl {
    #[doc = "The raw content in the secure keys file.\n\nA base64-encoded string."]
    pub content: PrimField<String>,
}
impl BuildComputeImageShieldedInstanceInitialStateElDbsEl {
    pub fn build(self) -> ComputeImageShieldedInstanceInitialStateElDbsEl {
        ComputeImageShieldedInstanceInitialStateElDbsEl {
            content: self.content,
            file_type: core::default::Default::default(),
        }
    }
}
pub struct ComputeImageShieldedInstanceInitialStateElDbsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageShieldedInstanceInitialStateElDbsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeImageShieldedInstanceInitialStateElDbsElRef {
        ComputeImageShieldedInstanceInitialStateElDbsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageShieldedInstanceInitialStateElDbsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe raw content in the secure keys file.\n\nA base64-encoded string."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
    #[doc = "Get a reference to the value of field `file_type` after provisioning.\nThe file type of source file."]
    pub fn file_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeImageShieldedInstanceInitialStateElDbxsEl {
    content: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_type: Option<PrimField<String>>,
}
impl ComputeImageShieldedInstanceInitialStateElDbxsEl {
    #[doc = "Set the field `file_type`.\nThe file type of source file."]
    pub fn set_file_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_type = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeImageShieldedInstanceInitialStateElDbxsEl {
    type O = BlockAssignable<ComputeImageShieldedInstanceInitialStateElDbxsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageShieldedInstanceInitialStateElDbxsEl {
    #[doc = "The raw content in the secure keys file.\n\nA base64-encoded string."]
    pub content: PrimField<String>,
}
impl BuildComputeImageShieldedInstanceInitialStateElDbxsEl {
    pub fn build(self) -> ComputeImageShieldedInstanceInitialStateElDbxsEl {
        ComputeImageShieldedInstanceInitialStateElDbxsEl {
            content: self.content,
            file_type: core::default::Default::default(),
        }
    }
}
pub struct ComputeImageShieldedInstanceInitialStateElDbxsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageShieldedInstanceInitialStateElDbxsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeImageShieldedInstanceInitialStateElDbxsElRef {
        ComputeImageShieldedInstanceInitialStateElDbxsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageShieldedInstanceInitialStateElDbxsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe raw content in the secure keys file.\n\nA base64-encoded string."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
    #[doc = "Get a reference to the value of field `file_type` after provisioning.\nThe file type of source file."]
    pub fn file_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeImageShieldedInstanceInitialStateElKeksEl {
    content: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_type: Option<PrimField<String>>,
}
impl ComputeImageShieldedInstanceInitialStateElKeksEl {
    #[doc = "Set the field `file_type`.\nThe file type of source file."]
    pub fn set_file_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_type = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeImageShieldedInstanceInitialStateElKeksEl {
    type O = BlockAssignable<ComputeImageShieldedInstanceInitialStateElKeksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageShieldedInstanceInitialStateElKeksEl {
    #[doc = "The raw content in the secure keys file.\n\nA base64-encoded string."]
    pub content: PrimField<String>,
}
impl BuildComputeImageShieldedInstanceInitialStateElKeksEl {
    pub fn build(self) -> ComputeImageShieldedInstanceInitialStateElKeksEl {
        ComputeImageShieldedInstanceInitialStateElKeksEl {
            content: self.content,
            file_type: core::default::Default::default(),
        }
    }
}
pub struct ComputeImageShieldedInstanceInitialStateElKeksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageShieldedInstanceInitialStateElKeksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeImageShieldedInstanceInitialStateElKeksElRef {
        ComputeImageShieldedInstanceInitialStateElKeksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageShieldedInstanceInitialStateElKeksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe raw content in the secure keys file.\n\nA base64-encoded string."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
    #[doc = "Get a reference to the value of field `file_type` after provisioning.\nThe file type of source file."]
    pub fn file_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeImageShieldedInstanceInitialStateElPkEl {
    content: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_type: Option<PrimField<String>>,
}
impl ComputeImageShieldedInstanceInitialStateElPkEl {
    #[doc = "Set the field `file_type`.\nThe file type of source file."]
    pub fn set_file_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_type = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeImageShieldedInstanceInitialStateElPkEl {
    type O = BlockAssignable<ComputeImageShieldedInstanceInitialStateElPkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageShieldedInstanceInitialStateElPkEl {
    #[doc = "The raw content in the secure keys file.\n\nA base64-encoded string."]
    pub content: PrimField<String>,
}
impl BuildComputeImageShieldedInstanceInitialStateElPkEl {
    pub fn build(self) -> ComputeImageShieldedInstanceInitialStateElPkEl {
        ComputeImageShieldedInstanceInitialStateElPkEl {
            content: self.content,
            file_type: core::default::Default::default(),
        }
    }
}
pub struct ComputeImageShieldedInstanceInitialStateElPkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageShieldedInstanceInitialStateElPkElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageShieldedInstanceInitialStateElPkElRef {
        ComputeImageShieldedInstanceInitialStateElPkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageShieldedInstanceInitialStateElPkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe raw content in the secure keys file.\n\nA base64-encoded string."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
    #[doc = "Get a reference to the value of field `file_type` after provisioning.\nThe file type of source file."]
    pub fn file_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeImageShieldedInstanceInitialStateElDynamic {
    dbs: Option<DynamicBlock<ComputeImageShieldedInstanceInitialStateElDbsEl>>,
    dbxs: Option<DynamicBlock<ComputeImageShieldedInstanceInitialStateElDbxsEl>>,
    keks: Option<DynamicBlock<ComputeImageShieldedInstanceInitialStateElKeksEl>>,
    pk: Option<DynamicBlock<ComputeImageShieldedInstanceInitialStateElPkEl>>,
}
#[derive(Serialize)]
pub struct ComputeImageShieldedInstanceInitialStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dbs: Option<Vec<ComputeImageShieldedInstanceInitialStateElDbsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dbxs: Option<Vec<ComputeImageShieldedInstanceInitialStateElDbxsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keks: Option<Vec<ComputeImageShieldedInstanceInitialStateElKeksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pk: Option<Vec<ComputeImageShieldedInstanceInitialStateElPkEl>>,
    dynamic: ComputeImageShieldedInstanceInitialStateElDynamic,
}
impl ComputeImageShieldedInstanceInitialStateEl {
    #[doc = "Set the field `dbs`.\n"]
    pub fn set_dbs(
        mut self,
        v: impl Into<BlockAssignable<ComputeImageShieldedInstanceInitialStateElDbsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dbs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dbs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dbxs`.\n"]
    pub fn set_dbxs(
        mut self,
        v: impl Into<BlockAssignable<ComputeImageShieldedInstanceInitialStateElDbxsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dbxs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dbxs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `keks`.\n"]
    pub fn set_keks(
        mut self,
        v: impl Into<BlockAssignable<ComputeImageShieldedInstanceInitialStateElKeksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.keks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.keks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pk`.\n"]
    pub fn set_pk(
        mut self,
        v: impl Into<BlockAssignable<ComputeImageShieldedInstanceInitialStateElPkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pk = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeImageShieldedInstanceInitialStateEl {
    type O = BlockAssignable<ComputeImageShieldedInstanceInitialStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageShieldedInstanceInitialStateEl {}
impl BuildComputeImageShieldedInstanceInitialStateEl {
    pub fn build(self) -> ComputeImageShieldedInstanceInitialStateEl {
        ComputeImageShieldedInstanceInitialStateEl {
            dbs: core::default::Default::default(),
            dbxs: core::default::Default::default(),
            keks: core::default::Default::default(),
            pk: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeImageShieldedInstanceInitialStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageShieldedInstanceInitialStateElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageShieldedInstanceInitialStateElRef {
        ComputeImageShieldedInstanceInitialStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageShieldedInstanceInitialStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dbs` after provisioning.\n"]
    pub fn dbs(&self) -> ListRef<ComputeImageShieldedInstanceInitialStateElDbsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dbs", self.base))
    }
    #[doc = "Get a reference to the value of field `dbxs` after provisioning.\n"]
    pub fn dbxs(&self) -> ListRef<ComputeImageShieldedInstanceInitialStateElDbxsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dbxs", self.base))
    }
    #[doc = "Get a reference to the value of field `keks` after provisioning.\n"]
    pub fn keks(&self) -> ListRef<ComputeImageShieldedInstanceInitialStateElKeksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.keks", self.base))
    }
    #[doc = "Get a reference to the value of field `pk` after provisioning.\n"]
    pub fn pk(&self) -> ListRef<ComputeImageShieldedInstanceInitialStateElPkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pk", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeImageSourceDiskEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl ComputeImageSourceDiskEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\nThe self link of the encryption key used to decrypt this resource. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
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
impl ToListMappable for ComputeImageSourceDiskEncryptionKeyEl {
    type O = BlockAssignable<ComputeImageSourceDiskEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageSourceDiskEncryptionKeyEl {}
impl BuildComputeImageSourceDiskEncryptionKeyEl {
    pub fn build(self) -> ComputeImageSourceDiskEncryptionKeyEl {
        ComputeImageSourceDiskEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct ComputeImageSourceDiskEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageSourceDiskEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageSourceDiskEncryptionKeyElRef {
        ComputeImageSourceDiskEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageSourceDiskEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key used to decrypt this resource. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
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
}
#[derive(Serialize)]
pub struct ComputeImageSourceImageEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl ComputeImageSourceImageEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\nThe self link of the encryption key used to decrypt this resource. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
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
impl ToListMappable for ComputeImageSourceImageEncryptionKeyEl {
    type O = BlockAssignable<ComputeImageSourceImageEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageSourceImageEncryptionKeyEl {}
impl BuildComputeImageSourceImageEncryptionKeyEl {
    pub fn build(self) -> ComputeImageSourceImageEncryptionKeyEl {
        ComputeImageSourceImageEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct ComputeImageSourceImageEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageSourceImageEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageSourceImageEncryptionKeyElRef {
        ComputeImageSourceImageEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageSourceImageEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key used to decrypt this resource. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
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
}
#[derive(Serialize)]
pub struct ComputeImageSourceSnapshotEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl ComputeImageSourceSnapshotEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\nThe self link of the encryption key used to decrypt this resource. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
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
impl ToListMappable for ComputeImageSourceSnapshotEncryptionKeyEl {
    type O = BlockAssignable<ComputeImageSourceSnapshotEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageSourceSnapshotEncryptionKeyEl {}
impl BuildComputeImageSourceSnapshotEncryptionKeyEl {
    pub fn build(self) -> ComputeImageSourceSnapshotEncryptionKeyEl {
        ComputeImageSourceSnapshotEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct ComputeImageSourceSnapshotEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageSourceSnapshotEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageSourceSnapshotEncryptionKeyElRef {
        ComputeImageSourceSnapshotEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageSourceSnapshotEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key used to decrypt this resource. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
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
}
#[derive(Serialize)]
pub struct ComputeImageTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeImageTimeoutsEl {
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
impl ToListMappable for ComputeImageTimeoutsEl {
    type O = BlockAssignable<ComputeImageTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeImageTimeoutsEl {}
impl BuildComputeImageTimeoutsEl {
    pub fn build(self) -> ComputeImageTimeoutsEl {
        ComputeImageTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeImageTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeImageTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageTimeoutsElRef {
        ComputeImageTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeImageTimeoutsElRef {
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
struct ComputeImageDynamic {
    guest_os_features: Option<DynamicBlock<ComputeImageGuestOsFeaturesEl>>,
    image_encryption_key: Option<DynamicBlock<ComputeImageImageEncryptionKeyEl>>,
    params: Option<DynamicBlock<ComputeImageParamsEl>>,
    raw_disk: Option<DynamicBlock<ComputeImageRawDiskEl>>,
    shielded_instance_initial_state:
        Option<DynamicBlock<ComputeImageShieldedInstanceInitialStateEl>>,
    source_disk_encryption_key: Option<DynamicBlock<ComputeImageSourceDiskEncryptionKeyEl>>,
    source_image_encryption_key: Option<DynamicBlock<ComputeImageSourceImageEncryptionKeyEl>>,
    source_snapshot_encryption_key: Option<DynamicBlock<ComputeImageSourceSnapshotEncryptionKeyEl>>,
}
