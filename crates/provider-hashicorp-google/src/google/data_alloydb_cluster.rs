use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataAlloydbClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataAlloydbCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAlloydbClusterData>,
}
#[derive(Clone)]
pub struct DataAlloydbCluster(Rc<DataAlloydbCluster_>);
impl DataAlloydbCluster {
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
    #[doc = "Set the field `location`.\nThe location where the alloydb cluster should reside."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automated_backup_policy` after provisioning.\nThe automated backup policy for this cluster. AutomatedBackupPolicy is disabled by default."]
    pub fn automated_backup_policy(&self) -> ListRef<DataAlloydbClusterAutomatedBackupPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automated_backup_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_source` after provisioning.\nCluster created from backup."]
    pub fn backup_source(&self) -> ListRef<DataAlloydbClusterBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backupdr_backup_source` after provisioning.\nCluster created from a BackupDR backup."]
    pub fn backupdr_backup_source(&self) -> ListRef<DataAlloydbClusterBackupdrBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backupdr_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID of the alloydb cluster."]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\nThe type of cluster. If not set, defaults to PRIMARY. Default value: \"PRIMARY\" Possible values: [\"PRIMARY\", \"SECONDARY\"]"]
    pub fn cluster_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `continuous_backup_config` after provisioning.\nThe continuous backup config for this cluster.\n\nIf no policy is provided then the default policy will be used. The default policy takes one backup a day and retains backups for 14 days."]
    pub fn continuous_backup_config(
        &self,
    ) -> ListRef<DataAlloydbClusterContinuousBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.continuous_backup_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `continuous_backup_info` after provisioning.\nContinuousBackupInfo describes the continuous backup properties of a cluster."]
    pub fn continuous_backup_info(&self) -> ListRef<DataAlloydbClusterContinuousBackupInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.continuous_backup_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_version` after provisioning.\nThe database engine major version. This is an optional field and it's populated at the Cluster creation time.\nNote: Changing this field to a higer version results in upgrading the AlloyDB cluster which is an irreversible change."]
    pub fn database_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataplex_config` after provisioning.\nConfiguration for Dataplex integration. This is an optional field. If not set, Dataplex integration will be enabled by default."]
    pub fn dataplex_config(&self) -> ListRef<DataAlloydbClusterDataplexConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataplex_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/alloydb_cluster.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the cluster.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the cluster will fail.\nWhen the field is set to false, deleting the cluster is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Cluster."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\nEncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key)."]
    pub fn encryption_config(&self) -> ListRef<DataAlloydbClusterEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_info` after provisioning.\nEncryptionInfo describes the encryption information of a cluster or a backup."]
    pub fn encryption_info(&self) -> ListRef<DataAlloydbClusterEncryptionInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nFor Resource freshness validation (https://google.aip.dev/154)"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `initial_user` after provisioning.\nInitial user to setup during cluster creation. If unset for new Clusters, a postgres role with null password is created. You will need to create additional users or set the password in order to log in."]
    pub fn initial_user(&self) -> ListRef<DataAlloydbClusterInitialUserElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.initial_user", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb cluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the alloydb cluster should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_update_policy` after provisioning.\nMaintenanceUpdatePolicy defines the policy for system updates."]
    pub fn maintenance_update_policy(
        &self,
    ) -> ListRef<DataAlloydbClusterMaintenanceUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_update_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `migration_source` after provisioning.\nCluster created via DMS migration."]
    pub fn migration_source(&self) -> ListRef<DataAlloydbClusterMigrationSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.migration_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\nMetadata related to network configuration."]
    pub fn network_config(&self) -> ListRef<DataAlloydbClusterNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\nConfiguration for Private Service Connect (PSC) for the cluster."]
    pub fn psc_config(&self) -> ListRef<DataAlloydbClusterPscConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Reconciling (https://google.aip.dev/128#reconciliation).\nSet to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them.\nThis can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_backup_source` after provisioning.\nThe source when restoring from a backup. Conflicts with 'restore_continuous_backup_source', 'restore_backupdr_backup_source' and 'restore_backupdr_pitr_source', they can't be set together."]
    pub fn restore_backup_source(&self) -> ListRef<DataAlloydbClusterRestoreBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_backupdr_backup_source` after provisioning.\nThe source when restoring from a backup. Conflicts with 'restore_continuous_backup_source',  'restore_backup_source' and 'restore_backupdr_pitr_source', they can't be set together."]
    pub fn restore_backupdr_backup_source(
        &self,
    ) -> ListRef<DataAlloydbClusterRestoreBackupdrBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_backupdr_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_backupdr_pitr_source` after provisioning.\nThe BackupDR source used for point in time recovery. Conflicts with 'restore_backupdr_backup_source', 'restore_continuous_backup_source' and 'restore_backupdr_backup_source', they can't be set togeter."]
    pub fn restore_backupdr_pitr_source(
        &self,
    ) -> ListRef<DataAlloydbClusterRestoreBackupdrPitrSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_backupdr_pitr_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_continuous_backup_source` after provisioning.\nThe source when restoring via point in time recovery (PITR). Conflicts with 'restore_backup_source', 'restore_backupdr_backup_source' and 'restore_backupdr_pitr_source', they can't be set together."]
    pub fn restore_continuous_backup_source(
        &self,
    ) -> ListRef<DataAlloydbClusterRestoreContinuousBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_continuous_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_config` after provisioning.\nConfiguration of the secondary cluster for Cross Region Replication. This should be set if and only if the cluster is of type SECONDARY."]
    pub fn secondary_config(&self) -> ListRef<DataAlloydbClusterSecondaryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_await_major_version_upgrade` after provisioning.\nSet to true to skip awaiting on the major version upgrade of the cluster.\nPossible values: true, false\nDefault value: \"true\""]
    pub fn skip_await_major_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_await_major_version_upgrade", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current serving state of the cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscription_type` after provisioning.\nThe subscrition type of cluster. Possible values: [\"TRIAL\", \"STANDARD\"]"]
    pub fn subscription_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trial_metadata` after provisioning.\nContains information and all metadata related to TRIAL clusters."]
    pub fn trial_metadata(&self) -> ListRef<DataAlloydbClusterTrialMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trial_metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
}
impl Referable for DataAlloydbCluster {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataAlloydbCluster {}
impl ToListMappable for DataAlloydbCluster {
    type O = ListRef<DataAlloydbClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataAlloydbCluster_ {
    fn extract_datasource_type(&self) -> String {
        "google_alloydb_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataAlloydbCluster {
    pub tf_id: String,
    #[doc = "The ID of the alloydb cluster."]
    pub cluster_id: PrimField<String>,
}
impl BuildDataAlloydbCluster {
    pub fn build(self, stack: &mut Stack) -> DataAlloydbCluster {
        let out = DataAlloydbCluster(Rc::new(DataAlloydbCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAlloydbClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataAlloydbClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataAlloydbClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automated_backup_policy` after provisioning.\nThe automated backup policy for this cluster. AutomatedBackupPolicy is disabled by default."]
    pub fn automated_backup_policy(&self) -> ListRef<DataAlloydbClusterAutomatedBackupPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automated_backup_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_source` after provisioning.\nCluster created from backup."]
    pub fn backup_source(&self) -> ListRef<DataAlloydbClusterBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backupdr_backup_source` after provisioning.\nCluster created from a BackupDR backup."]
    pub fn backupdr_backup_source(&self) -> ListRef<DataAlloydbClusterBackupdrBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backupdr_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID of the alloydb cluster."]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\nThe type of cluster. If not set, defaults to PRIMARY. Default value: \"PRIMARY\" Possible values: [\"PRIMARY\", \"SECONDARY\"]"]
    pub fn cluster_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `continuous_backup_config` after provisioning.\nThe continuous backup config for this cluster.\n\nIf no policy is provided then the default policy will be used. The default policy takes one backup a day and retains backups for 14 days."]
    pub fn continuous_backup_config(
        &self,
    ) -> ListRef<DataAlloydbClusterContinuousBackupConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.continuous_backup_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `continuous_backup_info` after provisioning.\nContinuousBackupInfo describes the continuous backup properties of a cluster."]
    pub fn continuous_backup_info(&self) -> ListRef<DataAlloydbClusterContinuousBackupInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.continuous_backup_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_version` after provisioning.\nThe database engine major version. This is an optional field and it's populated at the Cluster creation time.\nNote: Changing this field to a higer version results in upgrading the AlloyDB cluster which is an irreversible change."]
    pub fn database_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataplex_config` after provisioning.\nConfiguration for Dataplex integration. This is an optional field. If not set, Dataplex integration will be enabled by default."]
    pub fn dataplex_config(&self) -> ListRef<DataAlloydbClusterDataplexConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataplex_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/alloydb_cluster.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the cluster.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the cluster will fail.\nWhen the field is set to false, deleting the cluster is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Cluster."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\nEncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key)."]
    pub fn encryption_config(&self) -> ListRef<DataAlloydbClusterEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_info` after provisioning.\nEncryptionInfo describes the encryption information of a cluster or a backup."]
    pub fn encryption_info(&self) -> ListRef<DataAlloydbClusterEncryptionInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nFor Resource freshness validation (https://google.aip.dev/154)"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `initial_user` after provisioning.\nInitial user to setup during cluster creation. If unset for new Clusters, a postgres role with null password is created. You will need to create additional users or set the password in order to log in."]
    pub fn initial_user(&self) -> ListRef<DataAlloydbClusterInitialUserElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.initial_user", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb cluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the alloydb cluster should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_update_policy` after provisioning.\nMaintenanceUpdatePolicy defines the policy for system updates."]
    pub fn maintenance_update_policy(
        &self,
    ) -> ListRef<DataAlloydbClusterMaintenanceUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_update_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `migration_source` after provisioning.\nCluster created via DMS migration."]
    pub fn migration_source(&self) -> ListRef<DataAlloydbClusterMigrationSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.migration_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\nMetadata related to network configuration."]
    pub fn network_config(&self) -> ListRef<DataAlloydbClusterNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\nConfiguration for Private Service Connect (PSC) for the cluster."]
    pub fn psc_config(&self) -> ListRef<DataAlloydbClusterPscConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Reconciling (https://google.aip.dev/128#reconciliation).\nSet to true if the current state of Cluster does not match the user's intended state, and the service is actively updating the resource to reconcile them.\nThis can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_backup_source` after provisioning.\nThe source when restoring from a backup. Conflicts with 'restore_continuous_backup_source', 'restore_backupdr_backup_source' and 'restore_backupdr_pitr_source', they can't be set together."]
    pub fn restore_backup_source(&self) -> ListRef<DataAlloydbClusterRestoreBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_backupdr_backup_source` after provisioning.\nThe source when restoring from a backup. Conflicts with 'restore_continuous_backup_source',  'restore_backup_source' and 'restore_backupdr_pitr_source', they can't be set together."]
    pub fn restore_backupdr_backup_source(
        &self,
    ) -> ListRef<DataAlloydbClusterRestoreBackupdrBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_backupdr_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_backupdr_pitr_source` after provisioning.\nThe BackupDR source used for point in time recovery. Conflicts with 'restore_backupdr_backup_source', 'restore_continuous_backup_source' and 'restore_backupdr_backup_source', they can't be set togeter."]
    pub fn restore_backupdr_pitr_source(
        &self,
    ) -> ListRef<DataAlloydbClusterRestoreBackupdrPitrSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_backupdr_pitr_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_continuous_backup_source` after provisioning.\nThe source when restoring via point in time recovery (PITR). Conflicts with 'restore_backup_source', 'restore_backupdr_backup_source' and 'restore_backupdr_pitr_source', they can't be set together."]
    pub fn restore_continuous_backup_source(
        &self,
    ) -> ListRef<DataAlloydbClusterRestoreContinuousBackupSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_continuous_backup_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_config` after provisioning.\nConfiguration of the secondary cluster for Cross Region Replication. This should be set if and only if the cluster is of type SECONDARY."]
    pub fn secondary_config(&self) -> ListRef<DataAlloydbClusterSecondaryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `skip_await_major_version_upgrade` after provisioning.\nSet to true to skip awaiting on the major version upgrade of the cluster.\nPossible values: true, false\nDefault value: \"true\""]
    pub fn skip_await_major_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_await_major_version_upgrade", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current serving state of the cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscription_type` after provisioning.\nThe subscrition type of cluster. Possible values: [\"TRIAL\", \"STANDARD\"]"]
    pub fn subscription_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trial_metadata` after provisioning.\nContains information and all metadata related to TRIAL clusters."]
    pub fn trial_metadata(&self) -> ListRef<DataAlloydbClusterTrialMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trial_metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}
impl DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {
    type O = BlockAssignable<DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {}
impl BuildDataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {
    pub fn build(self) -> DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {
        DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigElRef {
        DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
}
impl DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {
    type O = BlockAssignable<DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {}
impl BuildDataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {
    pub fn build(self) -> DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {
        DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl {
            count: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionElRef {
        DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_period: Option<PrimField<String>>,
}
impl DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {
    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retention_period = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {
    type O = BlockAssignable<DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {}
impl BuildDataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {
    pub fn build(self) -> DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {
        DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl {
            retention_period: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionElRef {
        DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_period", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {
    type O = BlockAssignable<DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {}
impl BuildDataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {
    pub fn build(self) -> DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {
        DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesElRef {
        DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days_of_week: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_times:
        Option<ListField<DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl>>,
}
impl DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {
    #[doc = "Set the field `days_of_week`.\n"]
    pub fn set_days_of_week(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.days_of_week = Some(v.into());
        self
    }
    #[doc = "Set the field `start_times`.\n"]
    pub fn set_start_times(
        mut self,
        v: impl Into<ListField<DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesEl>>,
    ) -> Self {
        self.start_times = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {
    type O = BlockAssignable<DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {}
impl BuildDataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {
    pub fn build(self) -> DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {
        DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl {
            days_of_week: core::default::Default::default(),
            start_times: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElRef {
        DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `days_of_week` after provisioning.\n"]
    pub fn days_of_week(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }
    #[doc = "Get a reference to the value of field `start_times` after provisioning.\n"]
    pub fn start_times(
        &self,
    ) -> ListRef<DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElStartTimesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_times", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterAutomatedBackupPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config:
        Option<ListField<DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity_based_retention:
        Option<ListField<DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_based_retention:
        Option<ListField<DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_schedule: Option<ListField<DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl>>,
}
impl DataAlloydbClusterAutomatedBackupPolicyEl {
    #[doc = "Set the field `backup_window`.\n"]
    pub fn set_backup_window(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_window = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl Into<ListField<DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigEl>>,
    ) -> Self {
        self.encryption_config = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `quantity_based_retention`.\n"]
    pub fn set_quantity_based_retention(
        mut self,
        v: impl Into<ListField<DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionEl>>,
    ) -> Self {
        self.quantity_based_retention = Some(v.into());
        self
    }
    #[doc = "Set the field `time_based_retention`.\n"]
    pub fn set_time_based_retention(
        mut self,
        v: impl Into<ListField<DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionEl>>,
    ) -> Self {
        self.time_based_retention = Some(v.into());
        self
    }
    #[doc = "Set the field `weekly_schedule`.\n"]
    pub fn set_weekly_schedule(
        mut self,
        v: impl Into<ListField<DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleEl>>,
    ) -> Self {
        self.weekly_schedule = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterAutomatedBackupPolicyEl {
    type O = BlockAssignable<DataAlloydbClusterAutomatedBackupPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterAutomatedBackupPolicyEl {}
impl BuildDataAlloydbClusterAutomatedBackupPolicyEl {
    pub fn build(self) -> DataAlloydbClusterAutomatedBackupPolicyEl {
        DataAlloydbClusterAutomatedBackupPolicyEl {
            backup_window: core::default::Default::default(),
            enabled: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: core::default::Default::default(),
            quantity_based_retention: core::default::Default::default(),
            time_based_retention: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterAutomatedBackupPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterAutomatedBackupPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterAutomatedBackupPolicyElRef {
        DataAlloydbClusterAutomatedBackupPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterAutomatedBackupPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_window` after provisioning.\n"]
    pub fn backup_window(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_window", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> ListRef<DataAlloydbClusterAutomatedBackupPolicyElEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `quantity_based_retention` after provisioning.\n"]
    pub fn quantity_based_retention(
        &self,
    ) -> ListRef<DataAlloydbClusterAutomatedBackupPolicyElQuantityBasedRetentionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quantity_based_retention", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_based_retention` after provisioning.\n"]
    pub fn time_based_retention(
        &self,
    ) -> ListRef<DataAlloydbClusterAutomatedBackupPolicyElTimeBasedRetentionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.time_based_retention", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(
        &self,
    ) -> ListRef<DataAlloydbClusterAutomatedBackupPolicyElWeeklyScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_schedule", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterBackupSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_name: Option<PrimField<String>>,
}
impl DataAlloydbClusterBackupSourceEl {
    #[doc = "Set the field `backup_name`.\n"]
    pub fn set_backup_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterBackupSourceEl {
    type O = BlockAssignable<DataAlloydbClusterBackupSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterBackupSourceEl {}
impl BuildDataAlloydbClusterBackupSourceEl {
    pub fn build(self) -> DataAlloydbClusterBackupSourceEl {
        DataAlloydbClusterBackupSourceEl {
            backup_name: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterBackupSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterBackupSourceElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterBackupSourceElRef {
        DataAlloydbClusterBackupSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterBackupSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_name` after provisioning.\n"]
    pub fn backup_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterBackupdrBackupSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup: Option<PrimField<String>>,
}
impl DataAlloydbClusterBackupdrBackupSourceEl {
    #[doc = "Set the field `backup`.\n"]
    pub fn set_backup(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterBackupdrBackupSourceEl {
    type O = BlockAssignable<DataAlloydbClusterBackupdrBackupSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterBackupdrBackupSourceEl {}
impl BuildDataAlloydbClusterBackupdrBackupSourceEl {
    pub fn build(self) -> DataAlloydbClusterBackupdrBackupSourceEl {
        DataAlloydbClusterBackupdrBackupSourceEl {
            backup: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterBackupdrBackupSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterBackupdrBackupSourceElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterBackupdrBackupSourceElRef {
        DataAlloydbClusterBackupdrBackupSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterBackupdrBackupSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup` after provisioning.\n"]
    pub fn backup(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterContinuousBackupConfigElEncryptionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}
impl DataAlloydbClusterContinuousBackupConfigElEncryptionConfigEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterContinuousBackupConfigElEncryptionConfigEl {
    type O = BlockAssignable<DataAlloydbClusterContinuousBackupConfigElEncryptionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterContinuousBackupConfigElEncryptionConfigEl {}
impl BuildDataAlloydbClusterContinuousBackupConfigElEncryptionConfigEl {
    pub fn build(self) -> DataAlloydbClusterContinuousBackupConfigElEncryptionConfigEl {
        DataAlloydbClusterContinuousBackupConfigElEncryptionConfigEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterContinuousBackupConfigElEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterContinuousBackupConfigElEncryptionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbClusterContinuousBackupConfigElEncryptionConfigElRef {
        DataAlloydbClusterContinuousBackupConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterContinuousBackupConfigElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterContinuousBackupConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config:
        Option<ListField<DataAlloydbClusterContinuousBackupConfigElEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recovery_window_days: Option<PrimField<f64>>,
}
impl DataAlloydbClusterContinuousBackupConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl Into<ListField<DataAlloydbClusterContinuousBackupConfigElEncryptionConfigEl>>,
    ) -> Self {
        self.encryption_config = Some(v.into());
        self
    }
    #[doc = "Set the field `recovery_window_days`.\n"]
    pub fn set_recovery_window_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.recovery_window_days = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterContinuousBackupConfigEl {
    type O = BlockAssignable<DataAlloydbClusterContinuousBackupConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterContinuousBackupConfigEl {}
impl BuildDataAlloydbClusterContinuousBackupConfigEl {
    pub fn build(self) -> DataAlloydbClusterContinuousBackupConfigEl {
        DataAlloydbClusterContinuousBackupConfigEl {
            enabled: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            recovery_window_days: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterContinuousBackupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterContinuousBackupConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterContinuousBackupConfigElRef {
        DataAlloydbClusterContinuousBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterContinuousBackupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> ListRef<DataAlloydbClusterContinuousBackupConfigElEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `recovery_window_days` after provisioning.\n"]
    pub fn recovery_window_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.recovery_window_days", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterContinuousBackupInfoElEncryptionInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_versions: Option<ListField<PrimField<String>>>,
}
impl DataAlloydbClusterContinuousBackupInfoElEncryptionInfoEl {
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_type = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_versions`.\n"]
    pub fn set_kms_key_versions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.kms_key_versions = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterContinuousBackupInfoElEncryptionInfoEl {
    type O = BlockAssignable<DataAlloydbClusterContinuousBackupInfoElEncryptionInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterContinuousBackupInfoElEncryptionInfoEl {}
impl BuildDataAlloydbClusterContinuousBackupInfoElEncryptionInfoEl {
    pub fn build(self) -> DataAlloydbClusterContinuousBackupInfoElEncryptionInfoEl {
        DataAlloydbClusterContinuousBackupInfoElEncryptionInfoEl {
            encryption_type: core::default::Default::default(),
            kms_key_versions: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterContinuousBackupInfoElEncryptionInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterContinuousBackupInfoElEncryptionInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbClusterContinuousBackupInfoElEncryptionInfoElRef {
        DataAlloydbClusterContinuousBackupInfoElEncryptionInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterContinuousBackupInfoElEncryptionInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_versions` after provisioning.\n"]
    pub fn kms_key_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kms_key_versions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterContinuousBackupInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    earliest_restorable_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_info: Option<ListField<DataAlloydbClusterContinuousBackupInfoElEncryptionInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<ListField<PrimField<String>>>,
}
impl DataAlloydbClusterContinuousBackupInfoEl {
    #[doc = "Set the field `earliest_restorable_time`.\n"]
    pub fn set_earliest_restorable_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.earliest_restorable_time = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled_time`.\n"]
    pub fn set_enabled_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enabled_time = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_info`.\n"]
    pub fn set_encryption_info(
        mut self,
        v: impl Into<ListField<DataAlloydbClusterContinuousBackupInfoElEncryptionInfoEl>>,
    ) -> Self {
        self.encryption_info = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.schedule = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterContinuousBackupInfoEl {
    type O = BlockAssignable<DataAlloydbClusterContinuousBackupInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterContinuousBackupInfoEl {}
impl BuildDataAlloydbClusterContinuousBackupInfoEl {
    pub fn build(self) -> DataAlloydbClusterContinuousBackupInfoEl {
        DataAlloydbClusterContinuousBackupInfoEl {
            earliest_restorable_time: core::default::Default::default(),
            enabled_time: core::default::Default::default(),
            encryption_info: core::default::Default::default(),
            schedule: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterContinuousBackupInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterContinuousBackupInfoElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterContinuousBackupInfoElRef {
        DataAlloydbClusterContinuousBackupInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterContinuousBackupInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `earliest_restorable_time` after provisioning.\n"]
    pub fn earliest_restorable_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.earliest_restorable_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled_time` after provisioning.\n"]
    pub fn enabled_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled_time", self.base))
    }
    #[doc = "Get a reference to the value of field `encryption_info` after provisioning.\n"]
    pub fn encryption_info(
        &self,
    ) -> ListRef<DataAlloydbClusterContinuousBackupInfoElEncryptionInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_info", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterDataplexConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataAlloydbClusterDataplexConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterDataplexConfigEl {
    type O = BlockAssignable<DataAlloydbClusterDataplexConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterDataplexConfigEl {}
impl BuildDataAlloydbClusterDataplexConfigEl {
    pub fn build(self) -> DataAlloydbClusterDataplexConfigEl {
        DataAlloydbClusterDataplexConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterDataplexConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterDataplexConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterDataplexConfigElRef {
        DataAlloydbClusterDataplexConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterDataplexConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterEncryptionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}
impl DataAlloydbClusterEncryptionConfigEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterEncryptionConfigEl {
    type O = BlockAssignable<DataAlloydbClusterEncryptionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterEncryptionConfigEl {}
impl BuildDataAlloydbClusterEncryptionConfigEl {
    pub fn build(self) -> DataAlloydbClusterEncryptionConfigEl {
        DataAlloydbClusterEncryptionConfigEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterEncryptionConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterEncryptionConfigElRef {
        DataAlloydbClusterEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterEncryptionInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_versions: Option<ListField<PrimField<String>>>,
}
impl DataAlloydbClusterEncryptionInfoEl {
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_type = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_versions`.\n"]
    pub fn set_kms_key_versions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.kms_key_versions = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterEncryptionInfoEl {
    type O = BlockAssignable<DataAlloydbClusterEncryptionInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterEncryptionInfoEl {}
impl BuildDataAlloydbClusterEncryptionInfoEl {
    pub fn build(self) -> DataAlloydbClusterEncryptionInfoEl {
        DataAlloydbClusterEncryptionInfoEl {
            encryption_type: core::default::Default::default(),
            kms_key_versions: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterEncryptionInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterEncryptionInfoElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterEncryptionInfoElRef {
        DataAlloydbClusterEncryptionInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterEncryptionInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_versions` after provisioning.\n"]
    pub fn kms_key_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kms_key_versions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterInitialUserEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_wo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_wo_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<PrimField<String>>,
}
impl DataAlloydbClusterInitialUserEl {
    #[doc = "Set the field `password`.\n"]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }
    #[doc = "Set the field `password_wo`.\n"]
    pub fn set_password_wo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_wo = Some(v.into());
        self
    }
    #[doc = "Set the field `password_wo_version`.\n"]
    pub fn set_password_wo_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_wo_version = Some(v.into());
        self
    }
    #[doc = "Set the field `user`.\n"]
    pub fn set_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterInitialUserEl {
    type O = BlockAssignable<DataAlloydbClusterInitialUserEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterInitialUserEl {}
impl BuildDataAlloydbClusterInitialUserEl {
    pub fn build(self) -> DataAlloydbClusterInitialUserEl {
        DataAlloydbClusterInitialUserEl {
            password: core::default::Default::default(),
            password_wo: core::default::Default::default(),
            password_wo_version: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterInitialUserElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterInitialUserElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterInitialUserElRef {
        DataAlloydbClusterInitialUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterInitialUserElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `password_wo` after provisioning.\n"]
    pub fn password_wo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_wo", self.base))
    }
    #[doc = "Get a reference to the value of field `password_wo_version` after provisioning.\n"]
    pub fn password_wo_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_wo_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeEl {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeEl {
    type O =
        BlockAssignable<DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeEl {}
impl BuildDataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeEl {
    pub fn build(
        self,
    ) -> DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeEl {
        DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeElRef {
        DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<
        ListField<DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeEl>,
    >,
}
impl DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsEl {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<
            ListField<DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeEl>,
        >,
    ) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsEl {
    type O = BlockAssignable<DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsEl {}
impl BuildDataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsEl {
    pub fn build(self) -> DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsEl {
        DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsEl {
            day: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElRef {
        DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElStartTimeElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterMaintenanceUpdatePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_windows:
        Option<ListField<DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsEl>>,
}
impl DataAlloydbClusterMaintenanceUpdatePolicyEl {
    #[doc = "Set the field `maintenance_windows`.\n"]
    pub fn set_maintenance_windows(
        mut self,
        v: impl Into<ListField<DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsEl>>,
    ) -> Self {
        self.maintenance_windows = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterMaintenanceUpdatePolicyEl {
    type O = BlockAssignable<DataAlloydbClusterMaintenanceUpdatePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterMaintenanceUpdatePolicyEl {}
impl BuildDataAlloydbClusterMaintenanceUpdatePolicyEl {
    pub fn build(self) -> DataAlloydbClusterMaintenanceUpdatePolicyEl {
        DataAlloydbClusterMaintenanceUpdatePolicyEl {
            maintenance_windows: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterMaintenanceUpdatePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterMaintenanceUpdatePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterMaintenanceUpdatePolicyElRef {
        DataAlloydbClusterMaintenanceUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterMaintenanceUpdatePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maintenance_windows` after provisioning.\n"]
    pub fn maintenance_windows(
        &self,
    ) -> ListRef<DataAlloydbClusterMaintenanceUpdatePolicyElMaintenanceWindowsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_windows", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterMigrationSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_port: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_type: Option<PrimField<String>>,
}
impl DataAlloydbClusterMigrationSourceEl {
    #[doc = "Set the field `host_port`.\n"]
    pub fn set_host_port(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_port = Some(v.into());
        self
    }
    #[doc = "Set the field `reference_id`.\n"]
    pub fn set_reference_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reference_id = Some(v.into());
        self
    }
    #[doc = "Set the field `source_type`.\n"]
    pub fn set_source_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterMigrationSourceEl {
    type O = BlockAssignable<DataAlloydbClusterMigrationSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterMigrationSourceEl {}
impl BuildDataAlloydbClusterMigrationSourceEl {
    pub fn build(self) -> DataAlloydbClusterMigrationSourceEl {
        DataAlloydbClusterMigrationSourceEl {
            host_port: core::default::Default::default(),
            reference_id: core::default::Default::default(),
            source_type: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterMigrationSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterMigrationSourceElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterMigrationSourceElRef {
        DataAlloydbClusterMigrationSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterMigrationSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_port` after provisioning.\n"]
    pub fn host_port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_port", self.base))
    }
    #[doc = "Get a reference to the value of field `reference_id` after provisioning.\n"]
    pub fn reference_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.base))
    }
    #[doc = "Get a reference to the value of field `source_type` after provisioning.\n"]
    pub fn source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocated_ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
}
impl DataAlloydbClusterNetworkConfigEl {
    #[doc = "Set the field `allocated_ip_range`.\n"]
    pub fn set_allocated_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocated_ip_range = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterNetworkConfigEl {
    type O = BlockAssignable<DataAlloydbClusterNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterNetworkConfigEl {}
impl BuildDataAlloydbClusterNetworkConfigEl {
    pub fn build(self) -> DataAlloydbClusterNetworkConfigEl {
        DataAlloydbClusterNetworkConfigEl {
            allocated_ip_range: core::default::Default::default(),
            network: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterNetworkConfigElRef {
        DataAlloydbClusterNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocated_ip_range` after provisioning.\n"]
    pub fn allocated_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_ip_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterPscConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_owned_project_number: Option<PrimField<f64>>,
}
impl DataAlloydbClusterPscConfigEl {
    #[doc = "Set the field `psc_enabled`.\n"]
    pub fn set_psc_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.psc_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `service_owned_project_number`.\n"]
    pub fn set_service_owned_project_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.service_owned_project_number = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterPscConfigEl {
    type O = BlockAssignable<DataAlloydbClusterPscConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterPscConfigEl {}
impl BuildDataAlloydbClusterPscConfigEl {
    pub fn build(self) -> DataAlloydbClusterPscConfigEl {
        DataAlloydbClusterPscConfigEl {
            psc_enabled: core::default::Default::default(),
            service_owned_project_number: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterPscConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterPscConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterPscConfigElRef {
        DataAlloydbClusterPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterPscConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `psc_enabled` after provisioning.\n"]
    pub fn psc_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.psc_enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `service_owned_project_number` after provisioning.\n"]
    pub fn service_owned_project_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_owned_project_number", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterRestoreBackupSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_name: Option<PrimField<String>>,
}
impl DataAlloydbClusterRestoreBackupSourceEl {
    #[doc = "Set the field `backup_name`.\n"]
    pub fn set_backup_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterRestoreBackupSourceEl {
    type O = BlockAssignable<DataAlloydbClusterRestoreBackupSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterRestoreBackupSourceEl {}
impl BuildDataAlloydbClusterRestoreBackupSourceEl {
    pub fn build(self) -> DataAlloydbClusterRestoreBackupSourceEl {
        DataAlloydbClusterRestoreBackupSourceEl {
            backup_name: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterRestoreBackupSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterRestoreBackupSourceElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterRestoreBackupSourceElRef {
        DataAlloydbClusterRestoreBackupSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterRestoreBackupSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_name` after provisioning.\n"]
    pub fn backup_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterRestoreBackupdrBackupSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup: Option<PrimField<String>>,
}
impl DataAlloydbClusterRestoreBackupdrBackupSourceEl {
    #[doc = "Set the field `backup`.\n"]
    pub fn set_backup(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterRestoreBackupdrBackupSourceEl {
    type O = BlockAssignable<DataAlloydbClusterRestoreBackupdrBackupSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterRestoreBackupdrBackupSourceEl {}
impl BuildDataAlloydbClusterRestoreBackupdrBackupSourceEl {
    pub fn build(self) -> DataAlloydbClusterRestoreBackupdrBackupSourceEl {
        DataAlloydbClusterRestoreBackupdrBackupSourceEl {
            backup: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterRestoreBackupdrBackupSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterRestoreBackupdrBackupSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbClusterRestoreBackupdrBackupSourceElRef {
        DataAlloydbClusterRestoreBackupdrBackupSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterRestoreBackupdrBackupSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup` after provisioning.\n"]
    pub fn backup(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterRestoreBackupdrPitrSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_in_time: Option<PrimField<String>>,
}
impl DataAlloydbClusterRestoreBackupdrPitrSourceEl {
    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_source = Some(v.into());
        self
    }
    #[doc = "Set the field `point_in_time`.\n"]
    pub fn set_point_in_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.point_in_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterRestoreBackupdrPitrSourceEl {
    type O = BlockAssignable<DataAlloydbClusterRestoreBackupdrPitrSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterRestoreBackupdrPitrSourceEl {}
impl BuildDataAlloydbClusterRestoreBackupdrPitrSourceEl {
    pub fn build(self) -> DataAlloydbClusterRestoreBackupdrPitrSourceEl {
        DataAlloydbClusterRestoreBackupdrPitrSourceEl {
            data_source: core::default::Default::default(),
            point_in_time: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterRestoreBackupdrPitrSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterRestoreBackupdrPitrSourceElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterRestoreBackupdrPitrSourceElRef {
        DataAlloydbClusterRestoreBackupdrPitrSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterRestoreBackupdrPitrSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }
    #[doc = "Get a reference to the value of field `point_in_time` after provisioning.\n"]
    pub fn point_in_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.point_in_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterRestoreContinuousBackupSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_in_time: Option<PrimField<String>>,
}
impl DataAlloydbClusterRestoreContinuousBackupSourceEl {
    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `point_in_time`.\n"]
    pub fn set_point_in_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.point_in_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterRestoreContinuousBackupSourceEl {
    type O = BlockAssignable<DataAlloydbClusterRestoreContinuousBackupSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterRestoreContinuousBackupSourceEl {}
impl BuildDataAlloydbClusterRestoreContinuousBackupSourceEl {
    pub fn build(self) -> DataAlloydbClusterRestoreContinuousBackupSourceEl {
        DataAlloydbClusterRestoreContinuousBackupSourceEl {
            cluster: core::default::Default::default(),
            point_in_time: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterRestoreContinuousBackupSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterRestoreContinuousBackupSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAlloydbClusterRestoreContinuousBackupSourceElRef {
        DataAlloydbClusterRestoreContinuousBackupSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterRestoreContinuousBackupSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `point_in_time` after provisioning.\n"]
    pub fn point_in_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.point_in_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterSecondaryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_cluster_name: Option<PrimField<String>>,
}
impl DataAlloydbClusterSecondaryConfigEl {
    #[doc = "Set the field `primary_cluster_name`.\n"]
    pub fn set_primary_cluster_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_cluster_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterSecondaryConfigEl {
    type O = BlockAssignable<DataAlloydbClusterSecondaryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterSecondaryConfigEl {}
impl BuildDataAlloydbClusterSecondaryConfigEl {
    pub fn build(self) -> DataAlloydbClusterSecondaryConfigEl {
        DataAlloydbClusterSecondaryConfigEl {
            primary_cluster_name: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterSecondaryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterSecondaryConfigElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterSecondaryConfigElRef {
        DataAlloydbClusterSecondaryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterSecondaryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `primary_cluster_name` after provisioning.\n"]
    pub fn primary_cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_cluster_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAlloydbClusterTrialMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grace_end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade_time: Option<PrimField<String>>,
}
impl DataAlloydbClusterTrialMetadataEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `grace_end_time`.\n"]
    pub fn set_grace_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.grace_end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `upgrade_time`.\n"]
    pub fn set_upgrade_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.upgrade_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataAlloydbClusterTrialMetadataEl {
    type O = BlockAssignable<DataAlloydbClusterTrialMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAlloydbClusterTrialMetadataEl {}
impl BuildDataAlloydbClusterTrialMetadataEl {
    pub fn build(self) -> DataAlloydbClusterTrialMetadataEl {
        DataAlloydbClusterTrialMetadataEl {
            end_time: core::default::Default::default(),
            grace_end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            upgrade_time: core::default::Default::default(),
        }
    }
}
pub struct DataAlloydbClusterTrialMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAlloydbClusterTrialMetadataElRef {
    fn new(shared: StackShared, base: String) -> DataAlloydbClusterTrialMetadataElRef {
        DataAlloydbClusterTrialMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAlloydbClusterTrialMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `grace_end_time` after provisioning.\n"]
    pub fn grace_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grace_end_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `upgrade_time` after provisioning.\n"]
    pub fn upgrade_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.upgrade_time", self.base))
    }
}
