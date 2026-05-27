use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetappBackupVaultData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_vault_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_retention_policy: Option<Vec<NetappBackupVaultBackupRetentionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetappBackupVaultTimeoutsEl>,
    dynamic: NetappBackupVaultDynamic,
}
struct NetappBackupVault_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetappBackupVaultData>,
}
#[derive(Clone)]
pub struct NetappBackupVault(Rc<NetappBackupVault_>);
impl NetappBackupVault {
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
    #[doc = "Set the field `backup_region`.\nRegion in which backup is stored."]
    pub fn set_backup_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().backup_region = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_vault_type`.\nType of the backup vault to be created. Default is IN_REGION. Possible values: [\"BACKUP_VAULT_TYPE_UNSPECIFIED\", \"IN_REGION\", \"CROSS_REGION\"]"]
    pub fn set_backup_vault_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().backup_vault_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
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
    #[doc = "Set the field `kms_config`.\nSpecifies the Key Management System (KMS) configuration to be used for\nbackup encryption. Format:\n'projects/{{project}}/locations/{{location}}/kmsConfigs/{{kms_config}}'"]
    pub fn set_kms_config(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_config = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_retention_policy`.\n"]
    pub fn set_backup_retention_policy(
        self,
        v: impl Into<BlockAssignable<NetappBackupVaultBackupRetentionPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backup_retention_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backup_retention_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetappBackupVaultTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `backup_region` after provisioning.\nRegion in which backup is stored."]
    pub fn backup_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault_type` after provisioning.\nType of the backup vault to be created. Default is IN_REGION. Possible values: [\"BACKUP_VAULT_TYPE_UNSPECIFIED\", \"IN_REGION\", \"CROSS_REGION\"]"]
    pub fn backup_vault_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backups_crypto_key_version` after provisioning.\nThe crypto key version used to encrypt the backup vault.\nFormat:\n'projects/{{project}}/locations/{{location}}/keyRings/{{key_ring}}/cryptoKeys/{{crypto_key}}/cryptoKeyVersions/{{crypto_key_version}}'"]
    pub fn backups_crypto_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backups_crypto_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time of the backup vault. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `destination_backup_vault` after provisioning.\nName of the Backup vault created in backup region."]
    pub fn destination_backup_vault(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_backup_vault", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_state` after provisioning.\nEncryption state of customer-managed encryption keys (CMEK) backups."]
    pub fn encryption_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_config` after provisioning.\nSpecifies the Key Management System (KMS) configuration to be used for\nbackup encryption. Format:\n'projects/{{project}}/locations/{{location}}/kmsConfigs/{{kms_config}}'"]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation (region) of the backup vault."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the backup vault. Needs to be unique per location."]
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
    #[doc = "Get a reference to the value of field `source_backup_vault` after provisioning.\nName of the Backup vault created in source region."]
    pub fn source_backup_vault(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_backup_vault", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_region` after provisioning.\nRegion in which the backup vault is created."]
    pub fn source_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the Backup Vault."]
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
    #[doc = "Get a reference to the value of field `backup_retention_policy` after provisioning.\n"]
    pub fn backup_retention_policy(&self) -> ListRef<NetappBackupVaultBackupRetentionPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_retention_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappBackupVaultTimeoutsElRef {
        NetappBackupVaultTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetappBackupVault {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetappBackupVault {}
impl ToListMappable for NetappBackupVault {
    type O = ListRef<NetappBackupVaultRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetappBackupVault_ {
    fn extract_resource_type(&self) -> String {
        "google_netapp_backup_vault".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetappBackupVault {
    pub tf_id: String,
    #[doc = "Location (region) of the backup vault."]
    pub location: PrimField<String>,
    #[doc = "The resource name of the backup vault. Needs to be unique per location."]
    pub name: PrimField<String>,
}
impl BuildNetappBackupVault {
    pub fn build(self, stack: &mut Stack) -> NetappBackupVault {
        let out = NetappBackupVault(Rc::new(NetappBackupVault_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetappBackupVaultData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backup_region: core::default::Default::default(),
                backup_vault_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_config: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                backup_retention_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetappBackupVaultRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappBackupVaultRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetappBackupVaultRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_region` after provisioning.\nRegion in which backup is stored."]
    pub fn backup_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault_type` after provisioning.\nType of the backup vault to be created. Default is IN_REGION. Possible values: [\"BACKUP_VAULT_TYPE_UNSPECIFIED\", \"IN_REGION\", \"CROSS_REGION\"]"]
    pub fn backup_vault_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backups_crypto_key_version` after provisioning.\nThe crypto key version used to encrypt the backup vault.\nFormat:\n'projects/{{project}}/locations/{{location}}/keyRings/{{key_ring}}/cryptoKeys/{{crypto_key}}/cryptoKeyVersions/{{crypto_key_version}}'"]
    pub fn backups_crypto_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backups_crypto_key_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time of the backup vault. A timestamp in RFC3339 UTC \"Zulu\" format. Examples: \"2023-06-22T09:13:01.617Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `destination_backup_vault` after provisioning.\nName of the Backup vault created in backup region."]
    pub fn destination_backup_vault(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_backup_vault", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_state` after provisioning.\nEncryption state of customer-managed encryption keys (CMEK) backups."]
    pub fn encryption_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_config` after provisioning.\nSpecifies the Key Management System (KMS) configuration to be used for\nbackup encryption. Format:\n'projects/{{project}}/locations/{{location}}/kmsConfigs/{{kms_config}}'"]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation (region) of the backup vault."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the backup vault. Needs to be unique per location."]
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
    #[doc = "Get a reference to the value of field `source_backup_vault` after provisioning.\nName of the Backup vault created in source region."]
    pub fn source_backup_vault(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_backup_vault", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_region` after provisioning.\nRegion in which the backup vault is created."]
    pub fn source_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the Backup Vault."]
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
    #[doc = "Get a reference to the value of field `backup_retention_policy` after provisioning.\n"]
    pub fn backup_retention_policy(&self) -> ListRef<NetappBackupVaultBackupRetentionPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backup_retention_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappBackupVaultTimeoutsElRef {
        NetappBackupVaultTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetappBackupVaultBackupRetentionPolicyEl {
    backup_minimum_enforced_retention_days: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_backup_immutable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_backup_immutable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monthly_backup_immutable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_backup_immutable: Option<PrimField<bool>>,
}
impl NetappBackupVaultBackupRetentionPolicyEl {
    #[doc = "Set the field `daily_backup_immutable`.\nIndicates if the daily backups are immutable. At least one of daily_backup_immutable, weekly_backup_immutable, monthly_backup_immutable and manual_backup_immutable must be true."]
    pub fn set_daily_backup_immutable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.daily_backup_immutable = Some(v.into());
        self
    }
    #[doc = "Set the field `manual_backup_immutable`.\nIndicates if the manual backups are immutable. At least one of daily_backup_immutable, weekly_backup_immutable, monthly_backup_immutable and manual_backup_immutable must be true."]
    pub fn set_manual_backup_immutable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.manual_backup_immutable = Some(v.into());
        self
    }
    #[doc = "Set the field `monthly_backup_immutable`.\nIndicates if the monthly backups are immutable. At least one of daily_backup_immutable, weekly_backup_immutable, monthly_backup_immutable and manual_backup_immutable must be true."]
    pub fn set_monthly_backup_immutable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.monthly_backup_immutable = Some(v.into());
        self
    }
    #[doc = "Set the field `weekly_backup_immutable`.\nIndicates if the weekly backups are immutable. At least one of daily_backup_immutable, weekly_backup_immutable, monthly_backup_immutable and manual_backup_immutable must be true."]
    pub fn set_weekly_backup_immutable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.weekly_backup_immutable = Some(v.into());
        self
    }
}
impl ToListMappable for NetappBackupVaultBackupRetentionPolicyEl {
    type O = BlockAssignable<NetappBackupVaultBackupRetentionPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappBackupVaultBackupRetentionPolicyEl {
    #[doc = "Minimum retention duration in days for backups in the backup vault."]
    pub backup_minimum_enforced_retention_days: PrimField<f64>,
}
impl BuildNetappBackupVaultBackupRetentionPolicyEl {
    pub fn build(self) -> NetappBackupVaultBackupRetentionPolicyEl {
        NetappBackupVaultBackupRetentionPolicyEl {
            backup_minimum_enforced_retention_days: self.backup_minimum_enforced_retention_days,
            daily_backup_immutable: core::default::Default::default(),
            manual_backup_immutable: core::default::Default::default(),
            monthly_backup_immutable: core::default::Default::default(),
            weekly_backup_immutable: core::default::Default::default(),
        }
    }
}
pub struct NetappBackupVaultBackupRetentionPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappBackupVaultBackupRetentionPolicyElRef {
    fn new(shared: StackShared, base: String) -> NetappBackupVaultBackupRetentionPolicyElRef {
        NetappBackupVaultBackupRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappBackupVaultBackupRetentionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_minimum_enforced_retention_days` after provisioning.\nMinimum retention duration in days for backups in the backup vault."]
    pub fn backup_minimum_enforced_retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_minimum_enforced_retention_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `daily_backup_immutable` after provisioning.\nIndicates if the daily backups are immutable. At least one of daily_backup_immutable, weekly_backup_immutable, monthly_backup_immutable and manual_backup_immutable must be true."]
    pub fn daily_backup_immutable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.daily_backup_immutable", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `manual_backup_immutable` after provisioning.\nIndicates if the manual backups are immutable. At least one of daily_backup_immutable, weekly_backup_immutable, monthly_backup_immutable and manual_backup_immutable must be true."]
    pub fn manual_backup_immutable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manual_backup_immutable", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_backup_immutable` after provisioning.\nIndicates if the monthly backups are immutable. At least one of daily_backup_immutable, weekly_backup_immutable, monthly_backup_immutable and manual_backup_immutable must be true."]
    pub fn monthly_backup_immutable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_backup_immutable", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_backup_immutable` after provisioning.\nIndicates if the weekly backups are immutable. At least one of daily_backup_immutable, weekly_backup_immutable, monthly_backup_immutable and manual_backup_immutable must be true."]
    pub fn weekly_backup_immutable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.weekly_backup_immutable", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetappBackupVaultTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetappBackupVaultTimeoutsEl {
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
impl ToListMappable for NetappBackupVaultTimeoutsEl {
    type O = BlockAssignable<NetappBackupVaultTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappBackupVaultTimeoutsEl {}
impl BuildNetappBackupVaultTimeoutsEl {
    pub fn build(self) -> NetappBackupVaultTimeoutsEl {
        NetappBackupVaultTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetappBackupVaultTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappBackupVaultTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetappBackupVaultTimeoutsElRef {
        NetappBackupVaultTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappBackupVaultTimeoutsElRef {
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
struct NetappBackupVaultDynamic {
    backup_retention_policy: Option<DynamicBlock<NetappBackupVaultBackupRetentionPolicyEl>>,
}
