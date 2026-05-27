use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeSnapshotData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    most_recent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataComputeSnapshot_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeSnapshotData>,
}
#[derive(Clone)]
pub struct DataComputeSnapshot(Rc<DataComputeSnapshot_>);
impl DataComputeSnapshot {
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
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `most_recent`.\n"]
    pub fn set_most_recent(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().most_recent = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `chain_name` after provisioning.\nCreates the new snapshot in the snapshot chain labeled with the\nspecified name. The chain name must be 1-63 characters long and\ncomply with RFC1035. This is an uncommon option only for advanced\nservice owners who needs to create separate snapshot chains, for\nexample, for chargeback tracking.  When you describe your snapshot\nresource, this field is visible only if it has a non-empty value."]
    pub fn chain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.chain_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the snapshot, specified in GB."]
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
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this Snapshot.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `licenses` after provisioning.\nA list of public visible licenses that apply to this snapshot. This\ncan be because the original image had licenses attached (such as a\nWindows image).  snapshotEncryptionKey nested object Encrypts the\nsnapshot using a customer-supplied encryption key."]
    pub fn licenses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.licenses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.most_recent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `snapshot_encryption_key` after provisioning.\nEncrypts the snapshot using a customer-supplied encryption key.\n\nAfter you encrypt a snapshot using a customer-supplied key, you must\nprovide the same key if you use the snapshot later. For example, you\nmust provide the encryption key when you create a disk from the\nencrypted snapshot in a future request.\n\nCustomer-supplied encryption keys do not protect access to metadata of\nthe snapshot.\n\nIf you do not provide an encryption key when creating the snapshot,\nthen the snapshot will be encrypted using an automatically generated\nkey and you do not need to provide a key to use the snapshot later."]
    pub fn snapshot_encryption_key(
        &self,
    ) -> ListRef<DataComputeSnapshotSnapshotEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn snapshot_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_type` after provisioning.\nIndicates the type of the snapshot. Possible values: [\"ARCHIVE\", \"STANDARD\"]"]
    pub fn snapshot_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_disk` after provisioning.\nA reference to the disk used to create this snapshot."]
    pub fn source_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_disk_encryption_key` after provisioning.\nThe customer-supplied encryption key of the source snapshot. Required\nif the source snapshot is protected by a customer-supplied encryption\nkey."]
    pub fn source_disk_encryption_key(
        &self,
    ) -> ListRef<DataComputeSnapshotSourceDiskEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_disk_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_instant_snapshot` after provisioning.\nA reference to the instant snapshot used to create this snapshot."]
    pub fn source_instant_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instant_snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_bytes` after provisioning.\nA size of the storage used by the snapshot. As snapshots share\nstorage, this number is expected to change with snapshot\ncreation/deletion."]
    pub fn storage_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_locations` after provisioning.\nCloud Storage bucket storage location of the snapshot (regional or multi-regional)."]
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
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nA reference to the zone where the disk is hosted."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeSnapshot {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeSnapshot {}
impl ToListMappable for DataComputeSnapshot {
    type O = ListRef<DataComputeSnapshotRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeSnapshot_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_snapshot".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeSnapshot {
    pub tf_id: String,
}
impl BuildDataComputeSnapshot {
    pub fn build(self, stack: &mut Stack) -> DataComputeSnapshot {
        let out = DataComputeSnapshot(Rc::new(DataComputeSnapshot_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeSnapshotData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                most_recent: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeSnapshotRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSnapshotRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeSnapshotRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `chain_name` after provisioning.\nCreates the new snapshot in the snapshot chain labeled with the\nspecified name. The chain name must be 1-63 characters long and\ncomply with RFC1035. This is an uncommon option only for advanced\nservice owners who needs to create separate snapshot chains, for\nexample, for chargeback tracking.  When you describe your snapshot\nresource, this field is visible only if it has a non-empty value."]
    pub fn chain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.chain_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the snapshot, specified in GB."]
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
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this Snapshot.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `licenses` after provisioning.\nA list of public visible licenses that apply to this snapshot. This\ncan be because the original image had licenses attached (such as a\nWindows image).  snapshotEncryptionKey nested object Encrypts the\nsnapshot using a customer-supplied encryption key."]
    pub fn licenses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.licenses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.most_recent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `snapshot_encryption_key` after provisioning.\nEncrypts the snapshot using a customer-supplied encryption key.\n\nAfter you encrypt a snapshot using a customer-supplied key, you must\nprovide the same key if you use the snapshot later. For example, you\nmust provide the encryption key when you create a disk from the\nencrypted snapshot in a future request.\n\nCustomer-supplied encryption keys do not protect access to metadata of\nthe snapshot.\n\nIf you do not provide an encryption key when creating the snapshot,\nthen the snapshot will be encrypted using an automatically generated\nkey and you do not need to provide a key to use the snapshot later."]
    pub fn snapshot_encryption_key(
        &self,
    ) -> ListRef<DataComputeSnapshotSnapshotEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn snapshot_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_type` after provisioning.\nIndicates the type of the snapshot. Possible values: [\"ARCHIVE\", \"STANDARD\"]"]
    pub fn snapshot_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_disk` after provisioning.\nA reference to the disk used to create this snapshot."]
    pub fn source_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_disk", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_disk_encryption_key` after provisioning.\nThe customer-supplied encryption key of the source snapshot. Required\nif the source snapshot is protected by a customer-supplied encryption\nkey."]
    pub fn source_disk_encryption_key(
        &self,
    ) -> ListRef<DataComputeSnapshotSourceDiskEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_disk_encryption_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_instant_snapshot` after provisioning.\nA reference to the instant snapshot used to create this snapshot."]
    pub fn source_instant_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instant_snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_bytes` after provisioning.\nA size of the storage used by the snapshot. As snapshots share\nstorage, this number is expected to change with snapshot\ncreation/deletion."]
    pub fn storage_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_locations` after provisioning.\nCloud Storage bucket storage location of the snapshot (regional or multi-regional)."]
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
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nA reference to the zone where the disk is hosted."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeSnapshotSnapshotEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256: Option<PrimField<String>>,
}
impl DataComputeSnapshotSnapshotEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\n"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
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
impl ToListMappable for DataComputeSnapshotSnapshotEncryptionKeyEl {
    type O = BlockAssignable<DataComputeSnapshotSnapshotEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSnapshotSnapshotEncryptionKeyEl {}
impl BuildDataComputeSnapshotSnapshotEncryptionKeyEl {
    pub fn build(self) -> DataComputeSnapshotSnapshotEncryptionKeyEl {
        DataComputeSnapshotSnapshotEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
            sha256: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSnapshotSnapshotEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSnapshotSnapshotEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> DataComputeSnapshotSnapshotEncryptionKeyElRef {
        DataComputeSnapshotSnapshotEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSnapshotSnapshotEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\n"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
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
pub struct DataComputeSnapshotSourceDiskEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl DataComputeSnapshotSourceDiskEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\n"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
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
}
impl ToListMappable for DataComputeSnapshotSourceDiskEncryptionKeyEl {
    type O = BlockAssignable<DataComputeSnapshotSourceDiskEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeSnapshotSourceDiskEncryptionKeyEl {}
impl BuildDataComputeSnapshotSourceDiskEncryptionKeyEl {
    pub fn build(self) -> DataComputeSnapshotSourceDiskEncryptionKeyEl {
        DataComputeSnapshotSourceDiskEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct DataComputeSnapshotSourceDiskEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeSnapshotSourceDiskEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> DataComputeSnapshotSourceDiskEncryptionKeyElRef {
        DataComputeSnapshotSourceDiskEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeSnapshotSourceDiskEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\n"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_self_link", self.base),
        )
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
}
