use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetappVolumeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    capacity_gib: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large_capacity: Option<PrimField<bool>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiple_endpoints: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    protocols: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_actions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_style: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smb_settings: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_directory: Option<PrimField<bool>>,
    storage_pool: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput_mibps: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unix_permissions: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_config: Option<Vec<NetappVolumeBackupConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_devices: Option<Vec<NetappVolumeBlockDevicesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_parameters: Option<Vec<NetappVolumeCacheParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_policy: Option<Vec<NetappVolumeExportPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hybrid_replication_parameters: Option<Vec<NetappVolumeHybridReplicationParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large_capacity_config: Option<Vec<NetappVolumeLargeCapacityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_parameters: Option<Vec<NetappVolumeRestoreParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_policy: Option<Vec<NetappVolumeSnapshotPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiering_policy: Option<Vec<NetappVolumeTieringPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetappVolumeTimeoutsEl>,
    dynamic: NetappVolumeDynamic,
}
struct NetappVolume_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetappVolumeData>,
}
#[derive(Clone)]
pub struct NetappVolume(Rc<NetappVolume_>);
impl NetappVolume {
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
    #[doc = "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/netapp_volume.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kerberos_enabled`.\nFlag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p)."]
    pub fn set_kerberos_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().kerberos_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `large_capacity`.\nOptional. Flag indicating if the volume will be a large capacity volume or a regular volume."]
    pub fn set_large_capacity(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().large_capacity = Some(v.into());
        self
    }
    #[doc = "Set the field `multiple_endpoints`.\nOptional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints.\nOnly the volume with largeCapacity will be allowed to have multiple endpoints."]
    pub fn set_multiple_endpoints(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multiple_endpoints = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `restricted_actions`.\nList of actions that are restricted on this volume. Possible values: [\"DELETE\"]"]
    pub fn set_restricted_actions(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().restricted_actions = Some(v.into());
        self
    }
    #[doc = "Set the field `security_style`.\nSecurity Style of the Volume. Use UNIX to use UNIX or NFSV4 ACLs for file permissions.\nUse NTFS to use NTFS ACLs for file permissions. Can only be set for volumes which use SMB together with NFS as protocol. Possible values: [\"NTFS\", \"UNIX\"]"]
    pub fn set_security_style(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_style = Some(v.into());
        self
    }
    #[doc = "Set the field `share_name`.\nShare name (SMB) or export path (NFS) of the volume. Needs to be unique per location."]
    pub fn set_share_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().share_name = Some(v.into());
        self
    }
    #[doc = "Set the field `smb_settings`.\nSettings for volumes with SMB access. Possible values: [\"ENCRYPT_DATA\", \"BROWSABLE\", \"CHANGE_NOTIFY\", \"NON_BROWSABLE\", \"OPLOCKS\", \"SHOW_SNAPSHOT\", \"SHOW_PREVIOUS_VERSIONS\", \"ACCESS_BASED_ENUMERATION\", \"CONTINUOUSLY_AVAILABLE\"]"]
    pub fn set_smb_settings(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().smb_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_directory`.\nIf enabled, a NFS volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. Will enable \"Previous Versions\" support for SMB."]
    pub fn set_snapshot_directory(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().snapshot_directory = Some(v.into());
        self
    }
    #[doc = "Set the field `throughput_mibps`.\nOptional. Custom Performance Total Throughput of the pool (in MiB/s)."]
    pub fn set_throughput_mibps(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throughput_mibps = Some(v.into());
        self
    }
    #[doc = "Set the field `unix_permissions`.\nUnix permission the mount point will be created with. Default is 0770. Applicable for UNIX security style volumes only."]
    pub fn set_unix_permissions(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().unix_permissions = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_config`.\n"]
    pub fn set_backup_config(
        self,
        v: impl Into<BlockAssignable<NetappVolumeBackupConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backup_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backup_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `block_devices`.\n"]
    pub fn set_block_devices(
        self,
        v: impl Into<BlockAssignable<NetappVolumeBlockDevicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().block_devices = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.block_devices = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cache_parameters`.\n"]
    pub fn set_cache_parameters(
        self,
        v: impl Into<BlockAssignable<NetappVolumeCacheParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `export_policy`.\n"]
    pub fn set_export_policy(
        self,
        v: impl Into<BlockAssignable<NetappVolumeExportPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().export_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.export_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hybrid_replication_parameters`.\n"]
    pub fn set_hybrid_replication_parameters(
        self,
        v: impl Into<BlockAssignable<NetappVolumeHybridReplicationParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hybrid_replication_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .hybrid_replication_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `large_capacity_config`.\n"]
    pub fn set_large_capacity_config(
        self,
        v: impl Into<BlockAssignable<NetappVolumeLargeCapacityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().large_capacity_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.large_capacity_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `restore_parameters`.\n"]
    pub fn set_restore_parameters(
        self,
        v: impl Into<BlockAssignable<NetappVolumeRestoreParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `snapshot_policy`.\n"]
    pub fn set_snapshot_policy(
        self,
        v: impl Into<BlockAssignable<NetappVolumeSnapshotPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snapshot_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snapshot_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tiering_policy`.\n"]
    pub fn set_tiering_policy(
        self,
        v: impl Into<BlockAssignable<NetappVolumeTieringPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tiering_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tiering_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetappVolumeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `active_directory` after provisioning.\nReports the resource name of the Active Directory policy being used. Inherited from storage pool."]
    pub fn active_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_gib` after provisioning.\nCapacity of the volume (in GiB)."]
    pub fn capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cold_tier_size_gib` after provisioning.\nOutput only. Size of the volume cold tier data in GiB."]
    pub fn cold_tier_size_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cold_tier_size_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time of the volume. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/netapp_volume.html.markdown for specifics"]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\nReports the data-at-rest encryption type of the volume. Inherited from storage pool."]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `has_replication` after provisioning.\nIndicates whether the volume is part of a volume replication relationship."]
    pub fn has_replication(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.has_replication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hot_tier_size_used_gib` after provisioning.\nTotal hot tier data rounded down to the nearest GiB used by the volume. This field is only used for flex Service Level"]
    pub fn hot_tier_size_used_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hot_tier_size_used_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kerberos_enabled` after provisioning.\nFlag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p)."]
    pub fn kerberos_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_config` after provisioning.\nReports the CMEK policy resurce name being used for volume encryption. Inherited from storage pool."]
    pub fn kms_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `large_capacity` after provisioning.\nOptional. Flag indicating if the volume will be a large capacity volume or a regular volume."]
    pub fn large_capacity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.large_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ldap_enabled` after provisioning.\nFlag indicating if the volume is NFS LDAP enabled or not. Inherited from storage pool."]
    pub fn ldap_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ldap_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nName of the pool location. Usually a region name, expect for some STANDARD service level pools which require a zone name."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\nReports mount instructions for this volume."]
    pub fn mount_options(&self) -> ListRef<NetappVolumeMountOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mount_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multiple_endpoints` after provisioning.\nOptional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints.\nOnly the volume with largeCapacity will be allowed to have multiple endpoints."]
    pub fn multiple_endpoints(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multiple_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the volume. Needs to be unique per location."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nVPC network name with format: 'projects/{{project}}/global/networks/{{network}}'. Inherited from storage pool."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocols` after provisioning.\nThe protocol of the volume. Allowed combinations are '['NFSV3']', '['NFSV4']', '['SMB']', '['NFSV3', 'NFSV4']', '['SMB', 'NFSV3']' and '['SMB', 'NFSV4']'. Possible values: [\"NFSV3\", \"NFSV4\", \"SMB\", \"ISCSI\"]"]
    pub fn protocols(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protocols", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psa_range` after provisioning.\nName of the Private Service Access allocated range. Inherited from storage pool."]
    pub fn psa_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psa_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_zone` after provisioning.\nSpecifies the replica zone for regional volume."]
    pub fn replica_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restricted_actions` after provisioning.\nList of actions that are restricted on this volume. Possible values: [\"DELETE\"]"]
    pub fn restricted_actions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restricted_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_style` after provisioning.\nSecurity Style of the Volume. Use UNIX to use UNIX or NFSV4 ACLs for file permissions.\nUse NTFS to use NTFS ACLs for file permissions. Can only be set for volumes which use SMB together with NFS as protocol. Possible values: [\"NTFS\", \"UNIX\"]"]
    pub fn security_style(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_style", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_level` after provisioning.\nService level of the volume. Inherited from storage pool. Supported values are : PREMIUM, EXTREME, STANDARD, FLEX."]
    pub fn service_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `share_name` after provisioning.\nShare name (SMB) or export path (NFS) of the volume. Needs to be unique per location."]
    pub fn share_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.share_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `smb_settings` after provisioning.\nSettings for volumes with SMB access. Possible values: [\"ENCRYPT_DATA\", \"BROWSABLE\", \"CHANGE_NOTIFY\", \"NON_BROWSABLE\", \"OPLOCKS\", \"SHOW_SNAPSHOT\", \"SHOW_PREVIOUS_VERSIONS\", \"ACCESS_BASED_ENUMERATION\", \"CONTINUOUSLY_AVAILABLE\"]"]
    pub fn smb_settings(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.smb_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_directory` after provisioning.\nIf enabled, a NFS volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. Will enable \"Previous Versions\" support for SMB."]
    pub fn snapshot_directory(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the volume."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nState details of the volume."]
    pub fn state_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_pool` after provisioning.\nName of the storage pool to create the volume in. Pool needs enough spare capacity to accommodate the volume."]
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
    #[doc = "Get a reference to the value of field `throughput_mibps` after provisioning.\nOptional. Custom Performance Total Throughput of the pool (in MiB/s)."]
    pub fn throughput_mibps(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throughput_mibps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unix_permissions` after provisioning.\nUnix permission the mount point will be created with. Default is 0770. Applicable for UNIX security style volumes only."]
    pub fn unix_permissions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unix_permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `used_gib` after provisioning.\nUsed capacity of the volume (in GiB). This is computed periodically and it does not represent the realtime usage."]
    pub fn used_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.used_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nSpecifies the active zone for regional volume."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_config` after provisioning.\n"]
    pub fn backup_config(&self) -> ListRef<NetappVolumeBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `block_devices` after provisioning.\n"]
    pub fn block_devices(&self) -> ListRef<NetappVolumeBlockDevicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.block_devices", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cache_parameters` after provisioning.\n"]
    pub fn cache_parameters(&self) -> ListRef<NetappVolumeCacheParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_policy` after provisioning.\n"]
    pub fn export_policy(&self) -> ListRef<NetappVolumeExportPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.export_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hybrid_replication_parameters` after provisioning.\n"]
    pub fn hybrid_replication_parameters(
        &self,
    ) -> ListRef<NetappVolumeHybridReplicationParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hybrid_replication_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `large_capacity_config` after provisioning.\n"]
    pub fn large_capacity_config(&self) -> ListRef<NetappVolumeLargeCapacityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.large_capacity_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_parameters` after provisioning.\n"]
    pub fn restore_parameters(&self) -> ListRef<NetappVolumeRestoreParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_policy` after provisioning.\n"]
    pub fn snapshot_policy(&self) -> ListRef<NetappVolumeSnapshotPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tiering_policy` after provisioning.\n"]
    pub fn tiering_policy(&self) -> ListRef<NetappVolumeTieringPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tiering_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappVolumeTimeoutsElRef {
        NetappVolumeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetappVolume {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetappVolume {}
impl ToListMappable for NetappVolume {
    type O = ListRef<NetappVolumeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetappVolume_ {
    fn extract_resource_type(&self) -> String {
        "google_netapp_volume".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetappVolume {
    pub tf_id: String,
    #[doc = "Capacity of the volume (in GiB)."]
    pub capacity_gib: PrimField<String>,
    #[doc = "Name of the pool location. Usually a region name, expect for some STANDARD service level pools which require a zone name."]
    pub location: PrimField<String>,
    #[doc = "The name of the volume. Needs to be unique per location."]
    pub name: PrimField<String>,
    #[doc = "The protocol of the volume. Allowed combinations are '['NFSV3']', '['NFSV4']', '['SMB']', '['NFSV3', 'NFSV4']', '['SMB', 'NFSV3']' and '['SMB', 'NFSV4']'. Possible values: [\"NFSV3\", \"NFSV4\", \"SMB\", \"ISCSI\"]"]
    pub protocols: ListField<PrimField<String>>,
    #[doc = "Name of the storage pool to create the volume in. Pool needs enough spare capacity to accommodate the volume."]
    pub storage_pool: PrimField<String>,
}
impl BuildNetappVolume {
    pub fn build(self, stack: &mut Stack) -> NetappVolume {
        let out = NetappVolume(Rc::new(NetappVolume_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetappVolumeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                capacity_gib: self.capacity_gib,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kerberos_enabled: core::default::Default::default(),
                labels: core::default::Default::default(),
                large_capacity: core::default::Default::default(),
                location: self.location,
                multiple_endpoints: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                protocols: self.protocols,
                restricted_actions: core::default::Default::default(),
                security_style: core::default::Default::default(),
                share_name: core::default::Default::default(),
                smb_settings: core::default::Default::default(),
                snapshot_directory: core::default::Default::default(),
                storage_pool: self.storage_pool,
                throughput_mibps: core::default::Default::default(),
                unix_permissions: core::default::Default::default(),
                backup_config: core::default::Default::default(),
                block_devices: core::default::Default::default(),
                cache_parameters: core::default::Default::default(),
                export_policy: core::default::Default::default(),
                hybrid_replication_parameters: core::default::Default::default(),
                large_capacity_config: core::default::Default::default(),
                restore_parameters: core::default::Default::default(),
                snapshot_policy: core::default::Default::default(),
                tiering_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetappVolumeRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetappVolumeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active_directory` after provisioning.\nReports the resource name of the Active Directory policy being used. Inherited from storage pool."]
    pub fn active_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_gib` after provisioning.\nCapacity of the volume (in GiB)."]
    pub fn capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cold_tier_size_gib` after provisioning.\nOutput only. Size of the volume cold tier data in GiB."]
    pub fn cold_tier_size_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cold_tier_size_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time of the volume. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/netapp_volume.html.markdown for specifics"]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\nReports the data-at-rest encryption type of the volume. Inherited from storage pool."]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `has_replication` after provisioning.\nIndicates whether the volume is part of a volume replication relationship."]
    pub fn has_replication(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.has_replication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hot_tier_size_used_gib` after provisioning.\nTotal hot tier data rounded down to the nearest GiB used by the volume. This field is only used for flex Service Level"]
    pub fn hot_tier_size_used_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hot_tier_size_used_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kerberos_enabled` after provisioning.\nFlag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p)."]
    pub fn kerberos_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_config` after provisioning.\nReports the CMEK policy resurce name being used for volume encryption. Inherited from storage pool."]
    pub fn kms_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `large_capacity` after provisioning.\nOptional. Flag indicating if the volume will be a large capacity volume or a regular volume."]
    pub fn large_capacity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.large_capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ldap_enabled` after provisioning.\nFlag indicating if the volume is NFS LDAP enabled or not. Inherited from storage pool."]
    pub fn ldap_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ldap_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nName of the pool location. Usually a region name, expect for some STANDARD service level pools which require a zone name."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\nReports mount instructions for this volume."]
    pub fn mount_options(&self) -> ListRef<NetappVolumeMountOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mount_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multiple_endpoints` after provisioning.\nOptional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints.\nOnly the volume with largeCapacity will be allowed to have multiple endpoints."]
    pub fn multiple_endpoints(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multiple_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the volume. Needs to be unique per location."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nVPC network name with format: 'projects/{{project}}/global/networks/{{network}}'. Inherited from storage pool."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocols` after provisioning.\nThe protocol of the volume. Allowed combinations are '['NFSV3']', '['NFSV4']', '['SMB']', '['NFSV3', 'NFSV4']', '['SMB', 'NFSV3']' and '['SMB', 'NFSV4']'. Possible values: [\"NFSV3\", \"NFSV4\", \"SMB\", \"ISCSI\"]"]
    pub fn protocols(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protocols", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psa_range` after provisioning.\nName of the Private Service Access allocated range. Inherited from storage pool."]
    pub fn psa_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psa_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_zone` after provisioning.\nSpecifies the replica zone for regional volume."]
    pub fn replica_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restricted_actions` after provisioning.\nList of actions that are restricted on this volume. Possible values: [\"DELETE\"]"]
    pub fn restricted_actions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restricted_actions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_style` after provisioning.\nSecurity Style of the Volume. Use UNIX to use UNIX or NFSV4 ACLs for file permissions.\nUse NTFS to use NTFS ACLs for file permissions. Can only be set for volumes which use SMB together with NFS as protocol. Possible values: [\"NTFS\", \"UNIX\"]"]
    pub fn security_style(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_style", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_level` after provisioning.\nService level of the volume. Inherited from storage pool. Supported values are : PREMIUM, EXTREME, STANDARD, FLEX."]
    pub fn service_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `share_name` after provisioning.\nShare name (SMB) or export path (NFS) of the volume. Needs to be unique per location."]
    pub fn share_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.share_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `smb_settings` after provisioning.\nSettings for volumes with SMB access. Possible values: [\"ENCRYPT_DATA\", \"BROWSABLE\", \"CHANGE_NOTIFY\", \"NON_BROWSABLE\", \"OPLOCKS\", \"SHOW_SNAPSHOT\", \"SHOW_PREVIOUS_VERSIONS\", \"ACCESS_BASED_ENUMERATION\", \"CONTINUOUSLY_AVAILABLE\"]"]
    pub fn smb_settings(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.smb_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_directory` after provisioning.\nIf enabled, a NFS volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. Will enable \"Previous Versions\" support for SMB."]
    pub fn snapshot_directory(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the volume."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nState details of the volume."]
    pub fn state_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_pool` after provisioning.\nName of the storage pool to create the volume in. Pool needs enough spare capacity to accommodate the volume."]
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
    #[doc = "Get a reference to the value of field `throughput_mibps` after provisioning.\nOptional. Custom Performance Total Throughput of the pool (in MiB/s)."]
    pub fn throughput_mibps(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throughput_mibps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `unix_permissions` after provisioning.\nUnix permission the mount point will be created with. Default is 0770. Applicable for UNIX security style volumes only."]
    pub fn unix_permissions(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unix_permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `used_gib` after provisioning.\nUsed capacity of the volume (in GiB). This is computed periodically and it does not represent the realtime usage."]
    pub fn used_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.used_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nSpecifies the active zone for regional volume."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_config` after provisioning.\n"]
    pub fn backup_config(&self) -> ListRef<NetappVolumeBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `block_devices` after provisioning.\n"]
    pub fn block_devices(&self) -> ListRef<NetappVolumeBlockDevicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.block_devices", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cache_parameters` after provisioning.\n"]
    pub fn cache_parameters(&self) -> ListRef<NetappVolumeCacheParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `export_policy` after provisioning.\n"]
    pub fn export_policy(&self) -> ListRef<NetappVolumeExportPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.export_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hybrid_replication_parameters` after provisioning.\n"]
    pub fn hybrid_replication_parameters(
        &self,
    ) -> ListRef<NetappVolumeHybridReplicationParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hybrid_replication_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `large_capacity_config` after provisioning.\n"]
    pub fn large_capacity_config(&self) -> ListRef<NetappVolumeLargeCapacityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.large_capacity_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_parameters` after provisioning.\n"]
    pub fn restore_parameters(&self) -> ListRef<NetappVolumeRestoreParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_policy` after provisioning.\n"]
    pub fn snapshot_policy(&self) -> ListRef<NetappVolumeSnapshotPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tiering_policy` after provisioning.\n"]
    pub fn tiering_policy(&self) -> ListRef<NetappVolumeTieringPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tiering_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappVolumeTimeoutsElRef {
        NetappVolumeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetappVolumeMountOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    export: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_full: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
}
impl NetappVolumeMountOptionsEl {
    #[doc = "Set the field `export`.\n"]
    pub fn set_export(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.export = Some(v.into());
        self
    }
    #[doc = "Set the field `export_full`.\n"]
    pub fn set_export_full(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.export_full = Some(v.into());
        self
    }
    #[doc = "Set the field `instructions`.\n"]
    pub fn set_instructions(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instructions = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeMountOptionsEl {
    type O = BlockAssignable<NetappVolumeMountOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeMountOptionsEl {}
impl BuildNetappVolumeMountOptionsEl {
    pub fn build(self) -> NetappVolumeMountOptionsEl {
        NetappVolumeMountOptionsEl {
            export: core::default::Default::default(),
            export_full: core::default::Default::default(),
            instructions: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            protocol: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeMountOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeMountOptionsElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeMountOptionsElRef {
        NetappVolumeMountOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeMountOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `export` after provisioning.\n"]
    pub fn export(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.export", self.base))
    }
    #[doc = "Get a reference to the value of field `export_full` after provisioning.\n"]
    pub fn export_full(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.export_full", self.base))
    }
    #[doc = "Get a reference to the value of field `instructions` after provisioning.\n"]
    pub fn instructions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instructions", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}
#[derive(Serialize)]
pub struct NetappVolumeBackupConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_policies: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_vault: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_backup_enabled: Option<PrimField<bool>>,
}
impl NetappVolumeBackupConfigEl {
    #[doc = "Set the field `backup_policies`.\nSpecify a single backup policy ID for scheduled backups. Format: 'projects/{{projectId}}/locations/{{location}}/backupPolicies/{{backupPolicyName}}'"]
    pub fn set_backup_policies(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.backup_policies = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_vault`.\nID of the backup vault to use. A backup vault is reqired to create manual or scheduled backups.\nFormat: 'projects/{{projectId}}/locations/{{location}}/backupVaults/{{backupVaultName}}'"]
    pub fn set_backup_vault(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_vault = Some(v.into());
        self
    }
    #[doc = "Set the field `scheduled_backup_enabled`.\nWhen set to true, scheduled backup is enabled on the volume. Omit if no backup_policy is specified."]
    pub fn set_scheduled_backup_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.scheduled_backup_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeBackupConfigEl {
    type O = BlockAssignable<NetappVolumeBackupConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeBackupConfigEl {}
impl BuildNetappVolumeBackupConfigEl {
    pub fn build(self) -> NetappVolumeBackupConfigEl {
        NetappVolumeBackupConfigEl {
            backup_policies: core::default::Default::default(),
            backup_vault: core::default::Default::default(),
            scheduled_backup_enabled: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeBackupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeBackupConfigElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeBackupConfigElRef {
        NetappVolumeBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeBackupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_policies` after provisioning.\nSpecify a single backup policy ID for scheduled backups. Format: 'projects/{{projectId}}/locations/{{location}}/backupPolicies/{{backupPolicyName}}'"]
    pub fn backup_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_policies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault` after provisioning.\nID of the backup vault to use. A backup vault is reqired to create manual or scheduled backups.\nFormat: 'projects/{{projectId}}/locations/{{location}}/backupVaults/{{backupVaultName}}'"]
    pub fn backup_vault(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_vault", self.base))
    }
    #[doc = "Get a reference to the value of field `scheduled_backup_enabled` after provisioning.\nWhen set to true, scheduled backup is enabled on the volume. Omit if no backup_policy is specified."]
    pub fn scheduled_backup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduled_backup_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetappVolumeBlockDevicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    os_type: PrimField<String>,
}
impl NetappVolumeBlockDevicesEl {
    #[doc = "Set the field `host_groups`.\nA list of host groups that identify hosts that can mount the block volume.\nFormat:\n'projects/{project_id}/locations/{location}/hostGroups/{host_group_id}'\nThis field can be updated after the block device is created."]
    pub fn set_host_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.host_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nUser-defined name for the block device, unique within the Volume. In case\nno user input is provided, name will be autogenerated in the backend.\nThe name must meet the following requirements:\n*   Be between 1 and 255 characters long.\n*   Contain only uppercase or lowercase letters (A-Z, a-z), numbers (0-9),\n    and the following special characters: \"-\", \"_\", \"}\", \"{\", \".\".\n*   Spaces are not allowed."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeBlockDevicesEl {
    type O = BlockAssignable<NetappVolumeBlockDevicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeBlockDevicesEl {
    #[doc = "The OS type of the volume.\nThis field can't be changed after the block device is created. Possible values: [\"LINUX\", \"WINDOWS\", \"ESXI\"]"]
    pub os_type: PrimField<String>,
}
impl BuildNetappVolumeBlockDevicesEl {
    pub fn build(self) -> NetappVolumeBlockDevicesEl {
        NetappVolumeBlockDevicesEl {
            host_groups: core::default::Default::default(),
            name: core::default::Default::default(),
            os_type: self.os_type,
        }
    }
}
pub struct NetappVolumeBlockDevicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeBlockDevicesElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeBlockDevicesElRef {
        NetappVolumeBlockDevicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeBlockDevicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_groups` after provisioning.\nA list of host groups that identify hosts that can mount the block volume.\nFormat:\n'projects/{project_id}/locations/{location}/hostGroups/{host_group_id}'\nThis field can be updated after the block device is created."]
    pub fn host_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.host_groups", self.base))
    }
    #[doc = "Get a reference to the value of field `identifier` after provisioning.\nDevice identifier of the Block volume. This represents lun_serial_number\nfor ISCSI volumes"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser-defined name for the block device, unique within the Volume. In case\nno user input is provided, name will be autogenerated in the backend.\nThe name must meet the following requirements:\n*   Be between 1 and 255 characters long.\n*   Contain only uppercase or lowercase letters (A-Z, a-z), numbers (0-9),\n    and the following special characters: \"-\", \"_\", \"}\", \"{\", \".\".\n*   Spaces are not allowed."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `os_type` after provisioning.\nThe OS type of the volume.\nThis field can't be changed after the block device is created. Possible values: [\"LINUX\", \"WINDOWS\", \"ESXI\"]"]
    pub fn os_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_type", self.base))
    }
    #[doc = "Get a reference to the value of field `size_gib` after provisioning.\nThe size of the block device in GiB.\nAny value provided in this field during Volume creation is IGNORED.\nThe block device's size is system-managed and will be set to match\nthe parent Volume's 'capacity_gib'."]
    pub fn size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gib", self.base))
    }
}
#[derive(Serialize)]
pub struct NetappVolumeCacheParametersElCacheConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cifs_change_notify_enabled: Option<PrimField<bool>>,
}
impl NetappVolumeCacheParametersElCacheConfigEl {
    #[doc = "Set the field `cifs_change_notify_enabled`.\nOptional. Flag indicating whether a CIFS change notification is enabled for the FlexCache volume."]
    pub fn set_cifs_change_notify_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cifs_change_notify_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeCacheParametersElCacheConfigEl {
    type O = BlockAssignable<NetappVolumeCacheParametersElCacheConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeCacheParametersElCacheConfigEl {}
impl BuildNetappVolumeCacheParametersElCacheConfigEl {
    pub fn build(self) -> NetappVolumeCacheParametersElCacheConfigEl {
        NetappVolumeCacheParametersElCacheConfigEl {
            cifs_change_notify_enabled: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeCacheParametersElCacheConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeCacheParametersElCacheConfigElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeCacheParametersElCacheConfigElRef {
        NetappVolumeCacheParametersElCacheConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeCacheParametersElCacheConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cifs_change_notify_enabled` after provisioning.\nOptional. Flag indicating whether a CIFS change notification is enabled for the FlexCache volume."]
    pub fn cifs_change_notify_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cifs_change_notify_enabled", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetappVolumeCacheParametersElDynamic {
    cache_config: Option<DynamicBlock<NetappVolumeCacheParametersElCacheConfigEl>>,
}
#[derive(Serialize)]
pub struct NetappVolumeCacheParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_global_file_lock: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_cluster_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_ip_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_svm_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_volume_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peering_command_expiry_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_config: Option<Vec<NetappVolumeCacheParametersElCacheConfigEl>>,
    dynamic: NetappVolumeCacheParametersElDynamic,
}
impl NetappVolumeCacheParametersEl {
    #[doc = "Set the field `enable_global_file_lock`.\nOptional. Field indicating whether cache volume as global file lock enabled."]
    pub fn set_enable_global_file_lock(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_global_file_lock = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_cluster_name`.\nRequired. Name of the origin volume's ONTAP cluster."]
    pub fn set_peer_cluster_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_cluster_name = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_ip_addresses`.\nRequired. List of IC LIF addresses of the origin volume's ONTAP cluster."]
    pub fn set_peer_ip_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.peer_ip_addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_svm_name`.\nRequired. Name of the origin volume's SVM."]
    pub fn set_peer_svm_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_svm_name = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_volume_name`.\nRequired. Name of the origin volume for the cache volume."]
    pub fn set_peer_volume_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_volume_name = Some(v.into());
        self
    }
    #[doc = "Set the field `peering_command_expiry_time`.\nOptional. Expiration time for the peering command to be executed on user's ONTAP. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn set_peering_command_expiry_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peering_command_expiry_time = Some(v.into());
        self
    }
    #[doc = "Set the field `cache_config`.\n"]
    pub fn set_cache_config(
        mut self,
        v: impl Into<BlockAssignable<NetappVolumeCacheParametersElCacheConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetappVolumeCacheParametersEl {
    type O = BlockAssignable<NetappVolumeCacheParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeCacheParametersEl {}
impl BuildNetappVolumeCacheParametersEl {
    pub fn build(self) -> NetappVolumeCacheParametersEl {
        NetappVolumeCacheParametersEl {
            enable_global_file_lock: core::default::Default::default(),
            peer_cluster_name: core::default::Default::default(),
            peer_ip_addresses: core::default::Default::default(),
            peer_svm_name: core::default::Default::default(),
            peer_volume_name: core::default::Default::default(),
            peering_command_expiry_time: core::default::Default::default(),
            cache_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetappVolumeCacheParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeCacheParametersElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeCacheParametersElRef {
        NetappVolumeCacheParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeCacheParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cache_state` after provisioning.\nState of the cache volume indicating the peering status."]
    pub fn cache_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_state", self.base))
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\nCopy-paste-able commands to be used on user's ONTAP to accept peering requests."]
    pub fn command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_global_file_lock` after provisioning.\nOptional. Field indicating whether cache volume as global file lock enabled."]
    pub fn enable_global_file_lock(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_global_file_lock", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `passphrase` after provisioning.\nTemporary passphrase generated to accept cluster peering command."]
    pub fn passphrase(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }
    #[doc = "Get a reference to the value of field `peer_cluster_name` after provisioning.\nRequired. Name of the origin volume's ONTAP cluster."]
    pub fn peer_cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_cluster_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peer_ip_addresses` after provisioning.\nRequired. List of IC LIF addresses of the origin volume's ONTAP cluster."]
    pub fn peer_ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.peer_ip_addresses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peer_svm_name` after provisioning.\nRequired. Name of the origin volume's SVM."]
    pub fn peer_svm_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_svm_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peer_volume_name` after provisioning.\nRequired. Name of the origin volume for the cache volume."]
    pub fn peer_volume_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_volume_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peering_command_expiry_time` after provisioning.\nOptional. Expiration time for the peering command to be executed on user's ONTAP. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn peering_command_expiry_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peering_command_expiry_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state_details` after provisioning.\nDetailed description of the current cache state."]
    pub fn state_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_details", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_config` after provisioning.\n"]
    pub fn cache_config(&self) -> ListRef<NetappVolumeCacheParametersElCacheConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cache_config", self.base))
    }
}
#[derive(Serialize)]
pub struct NetappVolumeExportPolicyElRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_clients: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anon_uid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_root_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos5_read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos5_read_write: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos5i_read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos5i_read_write: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos5p_read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos5p_read_write: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfsv3: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfsv4: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    squash_mode: Option<PrimField<String>>,
}
impl NetappVolumeExportPolicyElRulesEl {
    #[doc = "Set the field `access_type`.\nDefines the access type for clients matching the 'allowedClients' specification. Possible values: [\"READ_ONLY\", \"READ_WRITE\", \"READ_NONE\"]"]
    pub fn set_access_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_type = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_clients`.\nDefines the client ingress specification (allowed clients) as a comma separated list with IPv4 CIDRs or IPv4 host addresses."]
    pub fn set_allowed_clients(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allowed_clients = Some(v.into());
        self
    }
    #[doc = "Set the field `anon_uid`.\nAn integer representing the anonymous user ID. Range is 0 to 4294967295. Required when 'squash_mode' is 'ALL_SQUASH'."]
    pub fn set_anon_uid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.anon_uid = Some(v.into());
        self
    }
    #[doc = "Set the field `has_root_access`.\nIf enabled, the root user (UID = 0) of the specified clients doesn't get mapped to nobody (UID = 65534). This is also known as no_root_squash.\nUse either squash_mode or has_root_access, but never both at the same time. These parameters are mutually exclusive."]
    pub fn set_has_root_access(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.has_root_access = Some(v.into());
        self
    }
    #[doc = "Set the field `kerberos5_read_only`.\nIf enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode."]
    pub fn set_kerberos5_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.kerberos5_read_only = Some(v.into());
        self
    }
    #[doc = "Set the field `kerberos5_read_write`.\nIf enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode. The 'kerberos5ReadOnly' value is ignored if this is enabled."]
    pub fn set_kerberos5_read_write(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.kerberos5_read_write = Some(v.into());
        self
    }
    #[doc = "Set the field `kerberos5i_read_only`.\nIf enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode."]
    pub fn set_kerberos5i_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.kerberos5i_read_only = Some(v.into());
        self
    }
    #[doc = "Set the field `kerberos5i_read_write`.\nIf enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode. The 'kerberos5iReadOnly' value is ignored if this is enabled."]
    pub fn set_kerberos5i_read_write(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.kerberos5i_read_write = Some(v.into());
        self
    }
    #[doc = "Set the field `kerberos5p_read_only`.\nIf enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode."]
    pub fn set_kerberos5p_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.kerberos5p_read_only = Some(v.into());
        self
    }
    #[doc = "Set the field `kerberos5p_read_write`.\nIf enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode. The 'kerberos5pReadOnly' value is ignored if this is enabled."]
    pub fn set_kerberos5p_read_write(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.kerberos5p_read_write = Some(v.into());
        self
    }
    #[doc = "Set the field `nfsv3`.\nEnable to apply the export rule to NFSV3 clients."]
    pub fn set_nfsv3(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.nfsv3 = Some(v.into());
        self
    }
    #[doc = "Set the field `nfsv4`.\nEnable to apply the export rule to NFSV4.1 clients."]
    pub fn set_nfsv4(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.nfsv4 = Some(v.into());
        self
    }
    #[doc = "Set the field `squash_mode`.\nSquashMode defines how remote user privileges are restricted when accessing an NFS export. It controls how the user identities (like root) are mapped to anonymous users to limit access and enforce security.\nUse either squash_mode or has_root_access, but never both at the same time. These parameters are mutually exclusive. Possible values: [\"SQUASH_MODE_UNSPECIFIED\", \"NO_ROOT_SQUASH\", \"ROOT_SQUASH\", \"ALL_SQUASH\"]"]
    pub fn set_squash_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.squash_mode = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeExportPolicyElRulesEl {
    type O = BlockAssignable<NetappVolumeExportPolicyElRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeExportPolicyElRulesEl {}
impl BuildNetappVolumeExportPolicyElRulesEl {
    pub fn build(self) -> NetappVolumeExportPolicyElRulesEl {
        NetappVolumeExportPolicyElRulesEl {
            access_type: core::default::Default::default(),
            allowed_clients: core::default::Default::default(),
            anon_uid: core::default::Default::default(),
            has_root_access: core::default::Default::default(),
            kerberos5_read_only: core::default::Default::default(),
            kerberos5_read_write: core::default::Default::default(),
            kerberos5i_read_only: core::default::Default::default(),
            kerberos5i_read_write: core::default::Default::default(),
            kerberos5p_read_only: core::default::Default::default(),
            kerberos5p_read_write: core::default::Default::default(),
            nfsv3: core::default::Default::default(),
            nfsv4: core::default::Default::default(),
            squash_mode: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeExportPolicyElRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeExportPolicyElRulesElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeExportPolicyElRulesElRef {
        NetappVolumeExportPolicyElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeExportPolicyElRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_type` after provisioning.\nDefines the access type for clients matching the 'allowedClients' specification. Possible values: [\"READ_ONLY\", \"READ_WRITE\", \"READ_NONE\"]"]
    pub fn access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_type", self.base))
    }
    #[doc = "Get a reference to the value of field `allowed_clients` after provisioning.\nDefines the client ingress specification (allowed clients) as a comma separated list with IPv4 CIDRs or IPv4 host addresses."]
    pub fn allowed_clients(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allowed_clients", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `anon_uid` after provisioning.\nAn integer representing the anonymous user ID. Range is 0 to 4294967295. Required when 'squash_mode' is 'ALL_SQUASH'."]
    pub fn anon_uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.anon_uid", self.base))
    }
    #[doc = "Get a reference to the value of field `has_root_access` after provisioning.\nIf enabled, the root user (UID = 0) of the specified clients doesn't get mapped to nobody (UID = 65534). This is also known as no_root_squash.\nUse either squash_mode or has_root_access, but never both at the same time. These parameters are mutually exclusive."]
    pub fn has_root_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.has_root_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kerberos5_read_only` after provisioning.\nIf enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode."]
    pub fn kerberos5_read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos5_read_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kerberos5_read_write` after provisioning.\nIf enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode. The 'kerberos5ReadOnly' value is ignored if this is enabled."]
    pub fn kerberos5_read_write(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos5_read_write", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kerberos5i_read_only` after provisioning.\nIf enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode."]
    pub fn kerberos5i_read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos5i_read_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kerberos5i_read_write` after provisioning.\nIf enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode. The 'kerberos5iReadOnly' value is ignored if this is enabled."]
    pub fn kerberos5i_read_write(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos5i_read_write", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kerberos5p_read_only` after provisioning.\nIf enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode."]
    pub fn kerberos5p_read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos5p_read_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kerberos5p_read_write` after provisioning.\nIf enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode. The 'kerberos5pReadOnly' value is ignored if this is enabled."]
    pub fn kerberos5p_read_write(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos5p_read_write", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nfsv3` after provisioning.\nEnable to apply the export rule to NFSV3 clients."]
    pub fn nfsv3(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.nfsv3", self.base))
    }
    #[doc = "Get a reference to the value of field `nfsv4` after provisioning.\nEnable to apply the export rule to NFSV4.1 clients."]
    pub fn nfsv4(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.nfsv4", self.base))
    }
    #[doc = "Get a reference to the value of field `squash_mode` after provisioning.\nSquashMode defines how remote user privileges are restricted when accessing an NFS export. It controls how the user identities (like root) are mapped to anonymous users to limit access and enforce security.\nUse either squash_mode or has_root_access, but never both at the same time. These parameters are mutually exclusive. Possible values: [\"SQUASH_MODE_UNSPECIFIED\", \"NO_ROOT_SQUASH\", \"ROOT_SQUASH\", \"ALL_SQUASH\"]"]
    pub fn squash_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_mode", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetappVolumeExportPolicyElDynamic {
    rules: Option<DynamicBlock<NetappVolumeExportPolicyElRulesEl>>,
}
#[derive(Serialize)]
pub struct NetappVolumeExportPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<NetappVolumeExportPolicyElRulesEl>>,
    dynamic: NetappVolumeExportPolicyElDynamic,
}
impl NetappVolumeExportPolicyEl {
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl Into<BlockAssignable<NetappVolumeExportPolicyElRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetappVolumeExportPolicyEl {
    type O = BlockAssignable<NetappVolumeExportPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeExportPolicyEl {}
impl BuildNetappVolumeExportPolicyEl {
    pub fn build(self) -> NetappVolumeExportPolicyEl {
        NetappVolumeExportPolicyEl {
            rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetappVolumeExportPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeExportPolicyElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeExportPolicyElRef {
        NetappVolumeExportPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeExportPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<NetappVolumeExportPolicyElRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}
#[derive(Serialize)]
pub struct NetappVolumeHybridReplicationParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hybrid_replication_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large_volume_constituent_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_cluster_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_ip_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_svm_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_volume_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_schedule: Option<PrimField<String>>,
}
impl NetappVolumeHybridReplicationParametersEl {
    #[doc = "Set the field `cluster_location`.\nOptional. Name of source cluster location associated with the replication. This is a free-form field\nfor display purposes only."]
    pub fn set_cluster_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_location = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nOptional. Description of the replication."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `hybrid_replication_type`.\nOptional. Type of the hybrid replication. Use 'MIGRATION' to create a volume migration\nand 'ONPREM_REPLICATION' to create an external replication.\nOther values are read-only. 'REVERSE_ONPREM_REPLICATION' is used to represent an external\nreplication which got reversed. Default is 'MIGRATION'. Possible values: [\"MIGRATION\", \"CONTINUOUS_REPLICATION\", \"ONPREM_REPLICATION\", \"REVERSE_ONPREM_REPLICATION\"]"]
    pub fn set_hybrid_replication_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hybrid_replication_type = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Labels to be added to the replication as the key value pairs.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `large_volume_constituent_count`.\nOptional. If the source is a FlexGroup volume, this field needs to match the number of constituents in the FlexGroup."]
    pub fn set_large_volume_constituent_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.large_volume_constituent_count = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_cluster_name`.\nRequired. Name of the ONTAP source cluster to be peered with NetApp Volumes."]
    pub fn set_peer_cluster_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_cluster_name = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_ip_addresses`.\nRequired. List of all intercluster LIF IP addresses of the ONTAP source cluster."]
    pub fn set_peer_ip_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.peer_ip_addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_svm_name`.\nRequired. Name of the ONTAP source vserver SVM to be peered with NetApp Volumes."]
    pub fn set_peer_svm_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_svm_name = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_volume_name`.\nRequired. Name of the ONTAP source volume to be replicated to NetApp Volumes destination volume."]
    pub fn set_peer_volume_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_volume_name = Some(v.into());
        self
    }
    #[doc = "Set the field `replication`.\nRequired. Desired name for the replication of this volume."]
    pub fn set_replication(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replication = Some(v.into());
        self
    }
    #[doc = "Set the field `replication_schedule`.\nOptional. Replication Schedule for the replication created. Possible values: [\"EVERY_10_MINUTES\", \"HOURLY\", \"DAILY\"]"]
    pub fn set_replication_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replication_schedule = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeHybridReplicationParametersEl {
    type O = BlockAssignable<NetappVolumeHybridReplicationParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeHybridReplicationParametersEl {}
impl BuildNetappVolumeHybridReplicationParametersEl {
    pub fn build(self) -> NetappVolumeHybridReplicationParametersEl {
        NetappVolumeHybridReplicationParametersEl {
            cluster_location: core::default::Default::default(),
            description: core::default::Default::default(),
            hybrid_replication_type: core::default::Default::default(),
            labels: core::default::Default::default(),
            large_volume_constituent_count: core::default::Default::default(),
            peer_cluster_name: core::default::Default::default(),
            peer_ip_addresses: core::default::Default::default(),
            peer_svm_name: core::default::Default::default(),
            peer_volume_name: core::default::Default::default(),
            replication: core::default::Default::default(),
            replication_schedule: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeHybridReplicationParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeHybridReplicationParametersElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeHybridReplicationParametersElRef {
        NetappVolumeHybridReplicationParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeHybridReplicationParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_location` after provisioning.\nOptional. Name of source cluster location associated with the replication. This is a free-form field\nfor display purposes only."]
    pub fn cluster_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the replication."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `hybrid_replication_type` after provisioning.\nOptional. Type of the hybrid replication. Use 'MIGRATION' to create a volume migration\nand 'ONPREM_REPLICATION' to create an external replication.\nOther values are read-only. 'REVERSE_ONPREM_REPLICATION' is used to represent an external\nreplication which got reversed. Default is 'MIGRATION'. Possible values: [\"MIGRATION\", \"CONTINUOUS_REPLICATION\", \"ONPREM_REPLICATION\", \"REVERSE_ONPREM_REPLICATION\"]"]
    pub fn hybrid_replication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hybrid_replication_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to be added to the replication as the key value pairs.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `large_volume_constituent_count` after provisioning.\nOptional. If the source is a FlexGroup volume, this field needs to match the number of constituents in the FlexGroup."]
    pub fn large_volume_constituent_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.large_volume_constituent_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peer_cluster_name` after provisioning.\nRequired. Name of the ONTAP source cluster to be peered with NetApp Volumes."]
    pub fn peer_cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_cluster_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peer_ip_addresses` after provisioning.\nRequired. List of all intercluster LIF IP addresses of the ONTAP source cluster."]
    pub fn peer_ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.peer_ip_addresses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peer_svm_name` after provisioning.\nRequired. Name of the ONTAP source vserver SVM to be peered with NetApp Volumes."]
    pub fn peer_svm_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_svm_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peer_volume_name` after provisioning.\nRequired. Name of the ONTAP source volume to be replicated to NetApp Volumes destination volume."]
    pub fn peer_volume_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_volume_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `replication` after provisioning.\nRequired. Desired name for the replication of this volume."]
    pub fn replication(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replication", self.base))
    }
    #[doc = "Get a reference to the value of field `replication_schedule` after provisioning.\nOptional. Replication Schedule for the replication created. Possible values: [\"EVERY_10_MINUTES\", \"HOURLY\", \"DAILY\"]"]
    pub fn replication_schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_schedule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetappVolumeLargeCapacityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    constituent_count: Option<PrimField<f64>>,
}
impl NetappVolumeLargeCapacityConfigEl {
    #[doc = "Set the field `constituent_count`.\nThe number of internal constituents (e.g., FlexVols) for this large volume.\nThe minimum number of constituents is 2."]
    pub fn set_constituent_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.constituent_count = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeLargeCapacityConfigEl {
    type O = BlockAssignable<NetappVolumeLargeCapacityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeLargeCapacityConfigEl {}
impl BuildNetappVolumeLargeCapacityConfigEl {
    pub fn build(self) -> NetappVolumeLargeCapacityConfigEl {
        NetappVolumeLargeCapacityConfigEl {
            constituent_count: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeLargeCapacityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeLargeCapacityConfigElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeLargeCapacityConfigElRef {
        NetappVolumeLargeCapacityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeLargeCapacityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `constituent_count` after provisioning.\nThe number of internal constituents (e.g., FlexVols) for this large volume.\nThe minimum number of constituents is 2."]
    pub fn constituent_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.constituent_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetappVolumeRestoreParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_backup: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot: Option<PrimField<String>>,
}
impl NetappVolumeRestoreParametersEl {
    #[doc = "Set the field `source_backup`.\nFull name of the backup to use for creating this volume.\n'source_snapshot' and 'source_backup' cannot be used simultaneously.\nFormat: 'projects/{{project}}/locations/{{location}}/backupVaults/{{backupVaultId}}/backups/{{backup}}'."]
    pub fn set_source_backup(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_backup = Some(v.into());
        self
    }
    #[doc = "Set the field `source_snapshot`.\nFull name of the snapshot to use for creating this volume.\n'source_snapshot' and 'source_backup' cannot be used simultaneously.\nFormat: 'projects/{{project}}/locations/{{location}}/volumes/{{volume}}/snapshots/{{snapshot}}'."]
    pub fn set_source_snapshot(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_snapshot = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeRestoreParametersEl {
    type O = BlockAssignable<NetappVolumeRestoreParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeRestoreParametersEl {}
impl BuildNetappVolumeRestoreParametersEl {
    pub fn build(self) -> NetappVolumeRestoreParametersEl {
        NetappVolumeRestoreParametersEl {
            source_backup: core::default::Default::default(),
            source_snapshot: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeRestoreParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeRestoreParametersElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeRestoreParametersElRef {
        NetappVolumeRestoreParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeRestoreParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_backup` after provisioning.\nFull name of the backup to use for creating this volume.\n'source_snapshot' and 'source_backup' cannot be used simultaneously.\nFormat: 'projects/{{project}}/locations/{{location}}/backupVaults/{{backupVaultId}}/backups/{{backup}}'."]
    pub fn source_backup(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_backup", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_snapshot` after provisioning.\nFull name of the snapshot to use for creating this volume.\n'source_snapshot' and 'source_backup' cannot be used simultaneously.\nFormat: 'projects/{{project}}/locations/{{location}}/volumes/{{volume}}/snapshots/{{snapshot}}'."]
    pub fn source_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_snapshot", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetappVolumeSnapshotPolicyElDailyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hour: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minute: Option<PrimField<f64>>,
    snapshots_to_keep: PrimField<f64>,
}
impl NetappVolumeSnapshotPolicyElDailyScheduleEl {
    #[doc = "Set the field `hour`.\nSet the hour to create the snapshot (0-23), defaults to midnight (0)."]
    pub fn set_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hour = Some(v.into());
        self
    }
    #[doc = "Set the field `minute`.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn set_minute(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minute = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeSnapshotPolicyElDailyScheduleEl {
    type O = BlockAssignable<NetappVolumeSnapshotPolicyElDailyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeSnapshotPolicyElDailyScheduleEl {
    #[doc = "The maximum number of snapshots to keep for the daily schedule."]
    pub snapshots_to_keep: PrimField<f64>,
}
impl BuildNetappVolumeSnapshotPolicyElDailyScheduleEl {
    pub fn build(self) -> NetappVolumeSnapshotPolicyElDailyScheduleEl {
        NetappVolumeSnapshotPolicyElDailyScheduleEl {
            hour: core::default::Default::default(),
            minute: core::default::Default::default(),
            snapshots_to_keep: self.snapshots_to_keep,
        }
    }
}
pub struct NetappVolumeSnapshotPolicyElDailyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeSnapshotPolicyElDailyScheduleElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeSnapshotPolicyElDailyScheduleElRef {
        NetappVolumeSnapshotPolicyElDailyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeSnapshotPolicyElDailyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hour` after provisioning.\nSet the hour to create the snapshot (0-23), defaults to midnight (0)."]
    pub fn hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour", self.base))
    }
    #[doc = "Get a reference to the value of field `minute` after provisioning.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minute", self.base))
    }
    #[doc = "Get a reference to the value of field `snapshots_to_keep` after provisioning.\nThe maximum number of snapshots to keep for the daily schedule."]
    pub fn snapshots_to_keep(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshots_to_keep", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetappVolumeSnapshotPolicyElHourlyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    minute: Option<PrimField<f64>>,
    snapshots_to_keep: PrimField<f64>,
}
impl NetappVolumeSnapshotPolicyElHourlyScheduleEl {
    #[doc = "Set the field `minute`.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn set_minute(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minute = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeSnapshotPolicyElHourlyScheduleEl {
    type O = BlockAssignable<NetappVolumeSnapshotPolicyElHourlyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeSnapshotPolicyElHourlyScheduleEl {
    #[doc = "The maximum number of snapshots to keep for the hourly schedule."]
    pub snapshots_to_keep: PrimField<f64>,
}
impl BuildNetappVolumeSnapshotPolicyElHourlyScheduleEl {
    pub fn build(self) -> NetappVolumeSnapshotPolicyElHourlyScheduleEl {
        NetappVolumeSnapshotPolicyElHourlyScheduleEl {
            minute: core::default::Default::default(),
            snapshots_to_keep: self.snapshots_to_keep,
        }
    }
}
pub struct NetappVolumeSnapshotPolicyElHourlyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeSnapshotPolicyElHourlyScheduleElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeSnapshotPolicyElHourlyScheduleElRef {
        NetappVolumeSnapshotPolicyElHourlyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeSnapshotPolicyElHourlyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `minute` after provisioning.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minute", self.base))
    }
    #[doc = "Get a reference to the value of field `snapshots_to_keep` after provisioning.\nThe maximum number of snapshots to keep for the hourly schedule."]
    pub fn snapshots_to_keep(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshots_to_keep", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetappVolumeSnapshotPolicyElMonthlyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days_of_month: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hour: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minute: Option<PrimField<f64>>,
    snapshots_to_keep: PrimField<f64>,
}
impl NetappVolumeSnapshotPolicyElMonthlyScheduleEl {
    #[doc = "Set the field `days_of_month`.\nSet the day or days of the month to make a snapshot (1-31). Accepts a comma separated number of days. Defaults to '1'."]
    pub fn set_days_of_month(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.days_of_month = Some(v.into());
        self
    }
    #[doc = "Set the field `hour`.\nSet the hour to create the snapshot (0-23), defaults to midnight (0)."]
    pub fn set_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hour = Some(v.into());
        self
    }
    #[doc = "Set the field `minute`.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn set_minute(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minute = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeSnapshotPolicyElMonthlyScheduleEl {
    type O = BlockAssignable<NetappVolumeSnapshotPolicyElMonthlyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeSnapshotPolicyElMonthlyScheduleEl {
    #[doc = "The maximum number of snapshots to keep for the monthly schedule"]
    pub snapshots_to_keep: PrimField<f64>,
}
impl BuildNetappVolumeSnapshotPolicyElMonthlyScheduleEl {
    pub fn build(self) -> NetappVolumeSnapshotPolicyElMonthlyScheduleEl {
        NetappVolumeSnapshotPolicyElMonthlyScheduleEl {
            days_of_month: core::default::Default::default(),
            hour: core::default::Default::default(),
            minute: core::default::Default::default(),
            snapshots_to_keep: self.snapshots_to_keep,
        }
    }
}
pub struct NetappVolumeSnapshotPolicyElMonthlyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeSnapshotPolicyElMonthlyScheduleElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeSnapshotPolicyElMonthlyScheduleElRef {
        NetappVolumeSnapshotPolicyElMonthlyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeSnapshotPolicyElMonthlyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `days_of_month` after provisioning.\nSet the day or days of the month to make a snapshot (1-31). Accepts a comma separated number of days. Defaults to '1'."]
    pub fn days_of_month(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.days_of_month", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hour` after provisioning.\nSet the hour to create the snapshot (0-23), defaults to midnight (0)."]
    pub fn hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour", self.base))
    }
    #[doc = "Get a reference to the value of field `minute` after provisioning.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minute", self.base))
    }
    #[doc = "Get a reference to the value of field `snapshots_to_keep` after provisioning.\nThe maximum number of snapshots to keep for the monthly schedule"]
    pub fn snapshots_to_keep(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshots_to_keep", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetappVolumeSnapshotPolicyElWeeklyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hour: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minute: Option<PrimField<f64>>,
    snapshots_to_keep: PrimField<f64>,
}
impl NetappVolumeSnapshotPolicyElWeeklyScheduleEl {
    #[doc = "Set the field `day`.\nSet the day or days of the week to make a snapshot. Accepts a comma separated days of the week. Defaults to 'Sunday'."]
    pub fn set_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `hour`.\nSet the hour to create the snapshot (0-23), defaults to midnight (0)."]
    pub fn set_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hour = Some(v.into());
        self
    }
    #[doc = "Set the field `minute`.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn set_minute(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minute = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeSnapshotPolicyElWeeklyScheduleEl {
    type O = BlockAssignable<NetappVolumeSnapshotPolicyElWeeklyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeSnapshotPolicyElWeeklyScheduleEl {
    #[doc = "The maximum number of snapshots to keep for the weekly schedule."]
    pub snapshots_to_keep: PrimField<f64>,
}
impl BuildNetappVolumeSnapshotPolicyElWeeklyScheduleEl {
    pub fn build(self) -> NetappVolumeSnapshotPolicyElWeeklyScheduleEl {
        NetappVolumeSnapshotPolicyElWeeklyScheduleEl {
            day: core::default::Default::default(),
            hour: core::default::Default::default(),
            minute: core::default::Default::default(),
            snapshots_to_keep: self.snapshots_to_keep,
        }
    }
}
pub struct NetappVolumeSnapshotPolicyElWeeklyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeSnapshotPolicyElWeeklyScheduleElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeSnapshotPolicyElWeeklyScheduleElRef {
        NetappVolumeSnapshotPolicyElWeeklyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeSnapshotPolicyElWeeklyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nSet the day or days of the week to make a snapshot. Accepts a comma separated days of the week. Defaults to 'Sunday'."]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `hour` after provisioning.\nSet the hour to create the snapshot (0-23), defaults to midnight (0)."]
    pub fn hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour", self.base))
    }
    #[doc = "Get a reference to the value of field `minute` after provisioning.\nSet the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0)."]
    pub fn minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minute", self.base))
    }
    #[doc = "Get a reference to the value of field `snapshots_to_keep` after provisioning.\nThe maximum number of snapshots to keep for the weekly schedule."]
    pub fn snapshots_to_keep(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshots_to_keep", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetappVolumeSnapshotPolicyElDynamic {
    daily_schedule: Option<DynamicBlock<NetappVolumeSnapshotPolicyElDailyScheduleEl>>,
    hourly_schedule: Option<DynamicBlock<NetappVolumeSnapshotPolicyElHourlyScheduleEl>>,
    monthly_schedule: Option<DynamicBlock<NetappVolumeSnapshotPolicyElMonthlyScheduleEl>>,
    weekly_schedule: Option<DynamicBlock<NetappVolumeSnapshotPolicyElWeeklyScheduleEl>>,
}
#[derive(Serialize)]
pub struct NetappVolumeSnapshotPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_schedule: Option<Vec<NetappVolumeSnapshotPolicyElDailyScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hourly_schedule: Option<Vec<NetappVolumeSnapshotPolicyElHourlyScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monthly_schedule: Option<Vec<NetappVolumeSnapshotPolicyElMonthlyScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_schedule: Option<Vec<NetappVolumeSnapshotPolicyElWeeklyScheduleEl>>,
    dynamic: NetappVolumeSnapshotPolicyElDynamic,
}
impl NetappVolumeSnapshotPolicyEl {
    #[doc = "Set the field `enabled`.\nEnables automated snapshot creation according to defined schedule. Default is false.\nTo disable automatic snapshot creation you have to remove the whole snapshot_policy block."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `daily_schedule`.\n"]
    pub fn set_daily_schedule(
        mut self,
        v: impl Into<BlockAssignable<NetappVolumeSnapshotPolicyElDailyScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.daily_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.daily_schedule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hourly_schedule`.\n"]
    pub fn set_hourly_schedule(
        mut self,
        v: impl Into<BlockAssignable<NetappVolumeSnapshotPolicyElHourlyScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hourly_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hourly_schedule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `monthly_schedule`.\n"]
    pub fn set_monthly_schedule(
        mut self,
        v: impl Into<BlockAssignable<NetappVolumeSnapshotPolicyElMonthlyScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.monthly_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.monthly_schedule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `weekly_schedule`.\n"]
    pub fn set_weekly_schedule(
        mut self,
        v: impl Into<BlockAssignable<NetappVolumeSnapshotPolicyElWeeklyScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weekly_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_schedule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetappVolumeSnapshotPolicyEl {
    type O = BlockAssignable<NetappVolumeSnapshotPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeSnapshotPolicyEl {}
impl BuildNetappVolumeSnapshotPolicyEl {
    pub fn build(self) -> NetappVolumeSnapshotPolicyEl {
        NetappVolumeSnapshotPolicyEl {
            enabled: core::default::Default::default(),
            daily_schedule: core::default::Default::default(),
            hourly_schedule: core::default::Default::default(),
            monthly_schedule: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetappVolumeSnapshotPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeSnapshotPolicyElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeSnapshotPolicyElRef {
        NetappVolumeSnapshotPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeSnapshotPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnables automated snapshot creation according to defined schedule. Default is false.\nTo disable automatic snapshot creation you have to remove the whole snapshot_policy block."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `daily_schedule` after provisioning.\n"]
    pub fn daily_schedule(&self) -> ListRef<NetappVolumeSnapshotPolicyElDailyScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.daily_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hourly_schedule` after provisioning.\n"]
    pub fn hourly_schedule(&self) -> ListRef<NetappVolumeSnapshotPolicyElHourlyScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hourly_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_schedule` after provisioning.\n"]
    pub fn monthly_schedule(&self) -> ListRef<NetappVolumeSnapshotPolicyElMonthlyScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monthly_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(&self) -> ListRef<NetappVolumeSnapshotPolicyElWeeklyScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_schedule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetappVolumeTieringPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cooling_threshold_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hot_tier_bypass_mode_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier_action: Option<PrimField<String>>,
}
impl NetappVolumeTieringPolicyEl {
    #[doc = "Set the field `cooling_threshold_days`.\nOptional. Time in days to mark the volume's data block as cold and make it eligible for tiering, can be range from 2-183.\nDefault is 31."]
    pub fn set_cooling_threshold_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cooling_threshold_days = Some(v.into());
        self
    }
    #[doc = "Set the field `hot_tier_bypass_mode_enabled`.\nOptional. Flag indicating that the hot tier bypass mode is enabled. Default is false.\nOnly applicable to Flex service level."]
    pub fn set_hot_tier_bypass_mode_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.hot_tier_bypass_mode_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `tier_action`.\nOptional. Flag indicating if the volume has tiering policy enable/pause. Default is PAUSED. Default value: \"PAUSED\" Possible values: [\"ENABLED\", \"PAUSED\"]"]
    pub fn set_tier_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tier_action = Some(v.into());
        self
    }
}
impl ToListMappable for NetappVolumeTieringPolicyEl {
    type O = BlockAssignable<NetappVolumeTieringPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeTieringPolicyEl {}
impl BuildNetappVolumeTieringPolicyEl {
    pub fn build(self) -> NetappVolumeTieringPolicyEl {
        NetappVolumeTieringPolicyEl {
            cooling_threshold_days: core::default::Default::default(),
            hot_tier_bypass_mode_enabled: core::default::Default::default(),
            tier_action: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeTieringPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeTieringPolicyElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeTieringPolicyElRef {
        NetappVolumeTieringPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeTieringPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cooling_threshold_days` after provisioning.\nOptional. Time in days to mark the volume's data block as cold and make it eligible for tiering, can be range from 2-183.\nDefault is 31."]
    pub fn cooling_threshold_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cooling_threshold_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hot_tier_bypass_mode_enabled` after provisioning.\nOptional. Flag indicating that the hot tier bypass mode is enabled. Default is false.\nOnly applicable to Flex service level."]
    pub fn hot_tier_bypass_mode_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hot_tier_bypass_mode_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tier_action` after provisioning.\nOptional. Flag indicating if the volume has tiering policy enable/pause. Default is PAUSED. Default value: \"PAUSED\" Possible values: [\"ENABLED\", \"PAUSED\"]"]
    pub fn tier_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier_action", self.base))
    }
}
#[derive(Serialize)]
pub struct NetappVolumeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetappVolumeTimeoutsEl {
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
impl ToListMappable for NetappVolumeTimeoutsEl {
    type O = BlockAssignable<NetappVolumeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappVolumeTimeoutsEl {}
impl BuildNetappVolumeTimeoutsEl {
    pub fn build(self) -> NetappVolumeTimeoutsEl {
        NetappVolumeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetappVolumeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappVolumeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetappVolumeTimeoutsElRef {
        NetappVolumeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappVolumeTimeoutsElRef {
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
struct NetappVolumeDynamic {
    backup_config: Option<DynamicBlock<NetappVolumeBackupConfigEl>>,
    block_devices: Option<DynamicBlock<NetappVolumeBlockDevicesEl>>,
    cache_parameters: Option<DynamicBlock<NetappVolumeCacheParametersEl>>,
    export_policy: Option<DynamicBlock<NetappVolumeExportPolicyEl>>,
    hybrid_replication_parameters: Option<DynamicBlock<NetappVolumeHybridReplicationParametersEl>>,
    large_capacity_config: Option<DynamicBlock<NetappVolumeLargeCapacityConfigEl>>,
    restore_parameters: Option<DynamicBlock<NetappVolumeRestoreParametersEl>>,
    snapshot_policy: Option<DynamicBlock<NetappVolumeSnapshotPolicyEl>>,
    tiering_policy: Option<DynamicBlock<NetappVolumeTieringPolicyEl>>,
}
