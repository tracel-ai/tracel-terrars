use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBackupDrBackupVaultData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    backup_vault_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataBackupDrBackupVault_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBackupDrBackupVaultData>,
}
#[derive(Clone)]
pub struct DataBackupDrBackupVault(Rc<DataBackupDrBackupVault_>);
impl DataBackupDrBackupVault {
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
    #[doc = "Get a reference to the value of field `access_restriction` after provisioning.\nAccess restriction for the backup vault. Default value is 'WITHIN_ORGANIZATION' if not provided during creation. Default value: \"WITHIN_ORGANIZATION\" Possible values: [\"ACCESS_RESTRICTION_UNSPECIFIED\", \"WITHIN_PROJECT\", \"WITHIN_ORGANIZATION\", \"UNRESTRICTED\", \"WITHIN_ORG_BUT_UNRESTRICTED_FOR_BA\"]"]
    pub fn access_restriction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_restriction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_missing` after provisioning.\nAllow idempotent deletion of backup vault. The request will still succeed in case the backup vault does not exist."]
    pub fn allow_missing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_missing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. See https://google.aip.dev/128#annotations\nStores small amounts of arbitrary data. \n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_count` after provisioning.\nOutput only. The number of backups in this backup vault."]
    pub fn backup_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_minimum_enforced_retention_duration` after provisioning.\nRequired. The default and minimum enforced retention for each backup within the backup vault. The enforced retention for each backup can be extended."]
    pub fn backup_minimum_enforced_retention_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.backup_minimum_enforced_retention_duration",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `backup_retention_inheritance` after provisioning.\nHow a backup's enforced retention end time is inherited. Default value is 'INHERIT_VAULT_RETENTION' if not provided during creation. Possible values: [\"BACKUP_RETENTION_INHERITANCE_UNSPECIFIED\", \"INHERIT_VAULT_RETENTION\", \"MATCH_BACKUP_EXPIRE_TIME\"]"]
    pub fn backup_retention_inheritance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_retention_inheritance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\nRequired. ID of the requesting object."]
    pub fn backup_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the instance was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletable` after provisioning.\nOutput only. Set to true when there are no backups nested under this resource."]
    pub fn deletable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. The description of the BackupVault instance (2048 characters or less)."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\nOptional. Time after which the BackupVault resource is locked."]
    pub fn effective_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\nEncryption configuration for the backup vault."]
    pub fn encryption_config(&self) -> ListRef<DataBackupDrBackupVaultEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. Server specified ETag for the backup vault resource to prevent simultaneous updates from overwiting each other."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance containing no backups, but still containing empty datasources.\n   * deletion of a backup vault instance that is being referenced by an active backup plan."]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_update` after provisioning.\nIf set, allow update to extend the minimum enforced retention for backup vault. This overrides\n the restriction against conflicting retention periods. This conflict may occur when the\n expiration schedule defined by the associated backup plan is shorter than the minimum\n retention set by the backup vault."]
    pub fn force_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_update", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_backup_plan_references` after provisioning.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance that is being referenced by an active backup plan."]
    pub fn ignore_backup_plan_references(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_backup_plan_references", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_inactive_datasources` after provisioning.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance containing no backups, but still containing empty datasources."]
    pub fn ignore_inactive_datasources(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_inactive_datasources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Resource labels to represent user provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe GCP location for the backup vault."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. Identifier. The resource name."]
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
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nOutput only. Service account used by the BackupVault Service for this BackupVault.  The user should grant this account permissions in their workload project to enable the service to run backups and restores there."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The BackupVault resource instance state. \n Possible values:\n STATE_UNSPECIFIED\n CREATING\n ACTIVE\n DELETING\n ERROR"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_stored_bytes` after provisioning.\nOutput only. Total size of the storage used by all backup resources."]
    pub fn total_stored_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_stored_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Output only Immutable after resource creation until resource deletion."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the instance was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
impl Referable for DataBackupDrBackupVault {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBackupDrBackupVault {}
impl ToListMappable for DataBackupDrBackupVault {
    type O = ListRef<DataBackupDrBackupVaultRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBackupDrBackupVault_ {
    fn extract_datasource_type(&self) -> String {
        "google_backup_dr_backup_vault".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBackupDrBackupVault {
    pub tf_id: String,
    #[doc = "Required. ID of the requesting object."]
    pub backup_vault_id: PrimField<String>,
    #[doc = "The GCP location for the backup vault."]
    pub location: PrimField<String>,
}
impl BuildDataBackupDrBackupVault {
    pub fn build(self, stack: &mut Stack) -> DataBackupDrBackupVault {
        let out = DataBackupDrBackupVault(Rc::new(DataBackupDrBackupVault_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBackupDrBackupVaultData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                backup_vault_id: self.backup_vault_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBackupDrBackupVaultRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupVaultRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBackupDrBackupVaultRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `access_restriction` after provisioning.\nAccess restriction for the backup vault. Default value is 'WITHIN_ORGANIZATION' if not provided during creation. Default value: \"WITHIN_ORGANIZATION\" Possible values: [\"ACCESS_RESTRICTION_UNSPECIFIED\", \"WITHIN_PROJECT\", \"WITHIN_ORGANIZATION\", \"UNRESTRICTED\", \"WITHIN_ORG_BUT_UNRESTRICTED_FOR_BA\"]"]
    pub fn access_restriction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_restriction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_missing` after provisioning.\nAllow idempotent deletion of backup vault. The request will still succeed in case the backup vault does not exist."]
    pub fn allow_missing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_missing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. See https://google.aip.dev/128#annotations\nStores small amounts of arbitrary data. \n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_count` after provisioning.\nOutput only. The number of backups in this backup vault."]
    pub fn backup_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_minimum_enforced_retention_duration` after provisioning.\nRequired. The default and minimum enforced retention for each backup within the backup vault. The enforced retention for each backup can be extended."]
    pub fn backup_minimum_enforced_retention_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.backup_minimum_enforced_retention_duration",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `backup_retention_inheritance` after provisioning.\nHow a backup's enforced retention end time is inherited. Default value is 'INHERIT_VAULT_RETENTION' if not provided during creation. Possible values: [\"BACKUP_RETENTION_INHERITANCE_UNSPECIFIED\", \"INHERIT_VAULT_RETENTION\", \"MATCH_BACKUP_EXPIRE_TIME\"]"]
    pub fn backup_retention_inheritance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_retention_inheritance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\nRequired. ID of the requesting object."]
    pub fn backup_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the instance was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletable` after provisioning.\nOutput only. Set to true when there are no backups nested under this resource."]
    pub fn deletable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. The description of the BackupVault instance (2048 characters or less)."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\nOptional. Time after which the BackupVault resource is locked."]
    pub fn effective_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\nEncryption configuration for the backup vault."]
    pub fn encryption_config(&self) -> ListRef<DataBackupDrBackupVaultEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nOptional. Server specified ETag for the backup vault resource to prevent simultaneous updates from overwiting each other."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance containing no backups, but still containing empty datasources.\n   * deletion of a backup vault instance that is being referenced by an active backup plan."]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_update` after provisioning.\nIf set, allow update to extend the minimum enforced retention for backup vault. This overrides\n the restriction against conflicting retention periods. This conflict may occur when the\n expiration schedule defined by the associated backup plan is shorter than the minimum\n retention set by the backup vault."]
    pub fn force_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_update", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_backup_plan_references` after provisioning.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance that is being referenced by an active backup plan."]
    pub fn ignore_backup_plan_references(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_backup_plan_references", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_inactive_datasources` after provisioning.\nIf set, the following restrictions against deletion of the backup vault instance can be overridden:\n   * deletion of a backup vault instance containing no backups, but still containing empty datasources."]
    pub fn ignore_inactive_datasources(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_inactive_datasources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Resource labels to represent user provided metadata. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe GCP location for the backup vault."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. Identifier. The resource name."]
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
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nOutput only. Service account used by the BackupVault Service for this BackupVault.  The user should grant this account permissions in their workload project to enable the service to run backups and restores there."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The BackupVault resource instance state. \n Possible values:\n STATE_UNSPECIFIED\n CREATING\n ACTIVE\n DELETING\n ERROR"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_stored_bytes` after provisioning.\nOutput only. Total size of the storage used by all backup resources."]
    pub fn total_stored_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_stored_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Output only Immutable after resource creation until resource deletion."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the instance was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBackupDrBackupVaultEncryptionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}
impl DataBackupDrBackupVaultEncryptionConfigEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupDrBackupVaultEncryptionConfigEl {
    type O = BlockAssignable<DataBackupDrBackupVaultEncryptionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupDrBackupVaultEncryptionConfigEl {}
impl BuildDataBackupDrBackupVaultEncryptionConfigEl {
    pub fn build(self) -> DataBackupDrBackupVaultEncryptionConfigEl {
        DataBackupDrBackupVaultEncryptionConfigEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}
pub struct DataBackupDrBackupVaultEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupDrBackupVaultEncryptionConfigElRef {
    fn new(shared: StackShared, base: String) -> DataBackupDrBackupVaultEncryptionConfigElRef {
        DataBackupDrBackupVaultEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupDrBackupVaultEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
