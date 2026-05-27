use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BackupDrRestoreWorkloadData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    backup_id: PrimField<String>,
    backup_vault_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clear_overrides_field_mask: Option<PrimField<String>>,
    data_source_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_restored_instance: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_instance_restore_properties:
        Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_instance_target_environment:
        Option<Vec<BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_restore_properties: Option<Vec<BackupDrRestoreWorkloadDiskRestorePropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_target_environment: Option<Vec<BackupDrRestoreWorkloadDiskTargetEnvironmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_disk_target_environment:
        Option<Vec<BackupDrRestoreWorkloadRegionDiskTargetEnvironmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BackupDrRestoreWorkloadTimeoutsEl>,
    dynamic: BackupDrRestoreWorkloadDynamic,
}
struct BackupDrRestoreWorkload_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BackupDrRestoreWorkloadData>,
}
#[derive(Clone)]
pub struct BackupDrRestoreWorkload(Rc<BackupDrRestoreWorkload_>);
impl BackupDrRestoreWorkload {
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
    #[doc = "Set the field `clear_overrides_field_mask`.\nOptional. A field mask used to clear server-side default values during restore."]
    pub fn set_clear_overrides_field_mask(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().clear_overrides_field_mask = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_restored_instance`.\nOptional. If true (default), running terraform destroy will delete the live resource in GCP.\nIf false, only the restore record is removed from the state, leaving the resource active."]
    pub fn set_delete_restored_instance(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_restored_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe resource name of the backup instance."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `request_id`.\nOptional. An optional request ID to identify requests. Specify a unique request ID\nso that if you must retry your request, the server will know to ignore\nthe request if it has already been completed."]
    pub fn set_request_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().request_id = Some(v.into());
        self
    }
    #[doc = "Set the field `compute_instance_restore_properties`.\n"]
    pub fn set_compute_instance_restore_properties(
        self,
        v: impl Into<BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_instance_restore_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .compute_instance_restore_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `compute_instance_target_environment`.\n"]
    pub fn set_compute_instance_target_environment(
        self,
        v: impl Into<BlockAssignable<BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_instance_target_environment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .compute_instance_target_environment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disk_restore_properties`.\n"]
    pub fn set_disk_restore_properties(
        self,
        v: impl Into<BlockAssignable<BackupDrRestoreWorkloadDiskRestorePropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_restore_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_restore_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disk_target_environment`.\n"]
    pub fn set_disk_target_environment(
        self,
        v: impl Into<BlockAssignable<BackupDrRestoreWorkloadDiskTargetEnvironmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_target_environment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_target_environment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `region_disk_target_environment`.\n"]
    pub fn set_region_disk_target_environment(
        self,
        v: impl Into<BlockAssignable<BackupDrRestoreWorkloadRegionDiskTargetEnvironmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().region_disk_target_environment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .region_disk_target_environment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BackupDrRestoreWorkloadTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `backup_id` after provisioning.\nRequired. The ID of the backup to restore from."]
    pub fn backup_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\nRequired. The ID of the backup vault."]
    pub fn backup_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `clear_overrides_field_mask` after provisioning.\nOptional. A field mask used to clear server-side default values during restore."]
    pub fn clear_overrides_field_mask(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.clear_overrides_field_mask", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\nRequired. The ID of the data source."]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_restored_instance` after provisioning.\nOptional. If true (default), running terraform destroy will delete the live resource in GCP.\nIf false, only the restore record is removed from the state, leaving the resource active."]
    pub fn delete_restored_instance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_restored_instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nRequired. The location for the backup vault."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the backup instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_id` after provisioning.\nOptional. An optional request ID to identify requests. Specify a unique request ID\nso that if you must retry your request, the server will know to ignore\nthe request if it has already been completed."]
    pub fn request_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_resource` after provisioning.\nOutput only. Details of the target resource created/modified as part of restore."]
    pub fn target_resource(&self) -> ListRef<BackupDrRestoreWorkloadTargetResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_instance_restore_properties` after provisioning.\n"]
    pub fn compute_instance_restore_properties(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_restore_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_instance_target_environment` after provisioning.\n"]
    pub fn compute_instance_target_environment(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_target_environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_restore_properties` after provisioning.\n"]
    pub fn disk_restore_properties(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadDiskRestorePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_restore_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_target_environment` after provisioning.\n"]
    pub fn disk_target_environment(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadDiskTargetEnvironmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_target_environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region_disk_target_environment` after provisioning.\n"]
    pub fn region_disk_target_environment(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadRegionDiskTargetEnvironmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.region_disk_target_environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrRestoreWorkloadTimeoutsElRef {
        BackupDrRestoreWorkloadTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BackupDrRestoreWorkload {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BackupDrRestoreWorkload {}
impl ToListMappable for BackupDrRestoreWorkload {
    type O = ListRef<BackupDrRestoreWorkloadRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BackupDrRestoreWorkload_ {
    fn extract_resource_type(&self) -> String {
        "google_backup_dr_restore_workload".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBackupDrRestoreWorkload {
    pub tf_id: String,
    #[doc = "Required. The ID of the backup to restore from."]
    pub backup_id: PrimField<String>,
    #[doc = "Required. The ID of the backup vault."]
    pub backup_vault_id: PrimField<String>,
    #[doc = "Required. The ID of the data source."]
    pub data_source_id: PrimField<String>,
    #[doc = "Required. The location for the backup vault."]
    pub location: PrimField<String>,
}
impl BuildBackupDrRestoreWorkload {
    pub fn build(self, stack: &mut Stack) -> BackupDrRestoreWorkload {
        let out = BackupDrRestoreWorkload(Rc::new(BackupDrRestoreWorkload_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BackupDrRestoreWorkloadData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backup_id: self.backup_id,
                backup_vault_id: self.backup_vault_id,
                clear_overrides_field_mask: core::default::Default::default(),
                data_source_id: self.data_source_id,
                delete_restored_instance: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: core::default::Default::default(),
                request_id: core::default::Default::default(),
                compute_instance_restore_properties: core::default::Default::default(),
                compute_instance_target_environment: core::default::Default::default(),
                disk_restore_properties: core::default::Default::default(),
                disk_target_environment: core::default::Default::default(),
                region_disk_target_environment: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BackupDrRestoreWorkloadRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BackupDrRestoreWorkloadRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_id` after provisioning.\nRequired. The ID of the backup to restore from."]
    pub fn backup_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backup_vault_id` after provisioning.\nRequired. The ID of the backup vault."]
    pub fn backup_vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_vault_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `clear_overrides_field_mask` after provisioning.\nOptional. A field mask used to clear server-side default values during restore."]
    pub fn clear_overrides_field_mask(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.clear_overrides_field_mask", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\nRequired. The ID of the data source."]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_restored_instance` after provisioning.\nOptional. If true (default), running terraform destroy will delete the live resource in GCP.\nIf false, only the restore record is removed from the state, leaving the resource active."]
    pub fn delete_restored_instance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_restored_instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nRequired. The location for the backup vault."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the backup instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_id` after provisioning.\nOptional. An optional request ID to identify requests. Specify a unique request ID\nso that if you must retry your request, the server will know to ignore\nthe request if it has already been completed."]
    pub fn request_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_resource` after provisioning.\nOutput only. Details of the target resource created/modified as part of restore."]
    pub fn target_resource(&self) -> ListRef<BackupDrRestoreWorkloadTargetResourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_instance_restore_properties` after provisioning.\n"]
    pub fn compute_instance_restore_properties(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_restore_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_instance_target_environment` after provisioning.\n"]
    pub fn compute_instance_target_environment(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_target_environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_restore_properties` after provisioning.\n"]
    pub fn disk_restore_properties(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadDiskRestorePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_restore_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disk_target_environment` after provisioning.\n"]
    pub fn disk_target_environment(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadDiskTargetEnvironmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_target_environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region_disk_target_environment` after provisioning.\n"]
    pub fn region_disk_target_environment(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadRegionDiskTargetEnvironmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.region_disk_target_environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrRestoreWorkloadTimeoutsElRef {
        BackupDrRestoreWorkloadTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadTargetResourceElGcpResourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_resourcename: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadTargetResourceElGcpResourceEl {
    #[doc = "Set the field `gcp_resourcename`.\n"]
    pub fn set_gcp_resourcename(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_resourcename = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadTargetResourceElGcpResourceEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadTargetResourceElGcpResourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadTargetResourceElGcpResourceEl {}
impl BuildBackupDrRestoreWorkloadTargetResourceElGcpResourceEl {
    pub fn build(self) -> BackupDrRestoreWorkloadTargetResourceElGcpResourceEl {
        BackupDrRestoreWorkloadTargetResourceElGcpResourceEl {
            gcp_resourcename: core::default::Default::default(),
            location: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadTargetResourceElGcpResourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadTargetResourceElGcpResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadTargetResourceElGcpResourceElRef {
        BackupDrRestoreWorkloadTargetResourceElGcpResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadTargetResourceElGcpResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_resourcename` after provisioning.\n"]
    pub fn gcp_resourcename(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_resourcename", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadTargetResourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_resource: Option<ListField<BackupDrRestoreWorkloadTargetResourceElGcpResourceEl>>,
}
impl BackupDrRestoreWorkloadTargetResourceEl {
    #[doc = "Set the field `gcp_resource`.\n"]
    pub fn set_gcp_resource(
        mut self,
        v: impl Into<ListField<BackupDrRestoreWorkloadTargetResourceElGcpResourceEl>>,
    ) -> Self {
        self.gcp_resource = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadTargetResourceEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadTargetResourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadTargetResourceEl {}
impl BuildBackupDrRestoreWorkloadTargetResourceEl {
    pub fn build(self) -> BackupDrRestoreWorkloadTargetResourceEl {
        BackupDrRestoreWorkloadTargetResourceEl {
            gcp_resource: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadTargetResourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadTargetResourceElRef {
    fn new(shared: StackShared, base: String) -> BackupDrRestoreWorkloadTargetResourceElRef {
        BackupDrRestoreWorkloadTargetResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadTargetResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_resource` after provisioning.\n"]
    pub fn gcp_resource(&self) -> ListRef<BackupDrRestoreWorkloadTargetResourceElGcpResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcp_resource", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_nested_virtualization: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_uefi_networking: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible_core_count: Option<PrimField<f64>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesEl {
    #[doc = "Set the field `enable_nested_virtualization`.\nOptional. Whether to enable nested virtualization or not (default is false)."]
    pub fn set_enable_nested_virtualization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_uefi_networking`.\nOptional. Whether to enable UEFI networking for instance creation."]
    pub fn set_enable_uefi_networking(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_uefi_networking = Some(v.into());
        self
    }
    #[doc = "Set the field `threads_per_core`.\nOptional. The number of threads per physical core."]
    pub fn set_threads_per_core(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threads_per_core = Some(v.into());
        self
    }
    #[doc = "Set the field `visible_core_count`.\nOptional. The number of physical cores to expose to an instance."]
    pub fn set_visible_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.visible_core_count = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesEl
{}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesEl {
            enable_nested_virtualization: core::default::Default::default(),
            enable_uefi_networking: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
            visible_core_count: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nOptional. Whether to enable nested virtualization or not (default is false)."]
    pub fn enable_nested_virtualization(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_nested_virtualization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_uefi_networking` after provisioning.\nOptional. Whether to enable UEFI networking for instance creation."]
    pub fn enable_uefi_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_uefi_networking", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\nOptional. The number of threads per physical core."]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threads_per_core", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `visible_core_count` after provisioning.\nOptional. The number of physical cores to expose to an instance."]
    pub fn visible_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.visible_core_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consume_allocation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityEl {
    #[doc = "Set the field `consume_allocation_type`.\n Possible values: [\"TYPE_UNSPECIFIED\", \"NO_RESERVATION\", \"ANY_RESERVATION\", \"SPECIFIC_RESERVATION\"]"]
    pub fn set_consume_allocation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consume_allocation_type = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityEl {}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityEl {
            consume_allocation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consume_allocation_type` after provisioning.\n Possible values: [\"TYPE_UNSPECIFIED\", \"NO_RESERVATION\", \"ANY_RESERVATION\", \"SPECIFIC_RESERVATION\"]"]
    pub fn consume_allocation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consume_allocation_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_confidential_compute: Option<PrimField<bool>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigEl {
    #[doc = "Set the field `enable_confidential_compute`.\nOptional. Defines whether the instance should have confidential compute enabled."]
    pub fn set_enable_confidential_compute(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_confidential_compute = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigEl
{}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigEl {
            enable_confidential_compute: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigElRef
    {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nOptional. Defines whether the instance should have confidential compute enabled."]
    pub fn enable_confidential_compute(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_confidential_compute", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyEl {
    #[doc = "Set the field `kms_key_name`.\nOptional. The name of the encryption key that is stored in Google Cloud KMS."]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_service_account`.\nOptional. The service account being used for the encryption request."]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `raw_key`.\nOptional. Specifies a 256-bit customer-supplied encryption key."]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
    #[doc = "Set the field `rsa_encrypted_key`.\nOptional. RSA-wrapped 2048-bit customer-supplied encryption key."]
    pub fn set_rsa_encrypted_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rsa_encrypted_key = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyEl
{}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyEl {
            kms_key_name: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyElRef
    {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nOptional. The name of the encryption key that is stored in Google Cloud KMS."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\nOptional. The service account being used for the encryption request."]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `raw_key` after provisioning.\nOptional. Specifies a 256-bit customer-supplied encryption key."]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }
    #[doc = "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\nOptional. RSA-wrapped 2048-bit customer-supplied encryption key."]
    pub fn rsa_encrypted_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rsa_encrypted_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureEl {
    #[doc = "Set the field `type_`.\nOptional. The ID of a supported feature. Possible values: [\"FEATURE_TYPE_UNSPECIFIED\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"MULTI_IP_SUBNET\", \"UEFI_COMPATIBLE\", \"SECURE_BOOT\", \"GVNIC\", \"SEV_CAPABLE\", \"BARE_METAL_LINUX_COMPATIBLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"TDX_CAPABLE\", \"IDPF\", \"SEV_LIVE_MIGRATABLE_V2\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureEl {
}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional. The ID of a supported feature. Possible values: [\"FEATURE_TYPE_UNSPECIFIED\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"MULTI_IP_SUBNET\", \"UEFI_COMPATIBLE\", \"SECURE_BOOT\", \"GVNIC\", \"SEV_CAPABLE\", \"BARE_METAL_LINUX_COMPATIBLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"TDX_CAPABLE\", \"IDPF\", \"SEV_LIVE_MIGRATABLE_V2\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_zones: Option<ListField<PrimField<String>>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsEl {
    #[doc = "Set the field `disk_name`.\nOptional. Specifies the disk name."]
    pub fn set_disk_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_name = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_zones`.\nOptional. URL of the zone where the disk should be created."]
    pub fn set_replica_zones(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.replica_zones = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsEl
{}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsEl {
            disk_name: core::default::Default::default(),
            replica_zones: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_name` after provisioning.\nOptional. Specifies the disk name."]
    pub fn disk_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_name", self.base))
    }
    #[doc = "Get a reference to the value of field `replica_zones` after provisioning.\nOptional. URL of the zone where the disk should be created."]
    pub fn replica_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_zones", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDynamic {
    disk_encryption_key: Option<
        DynamicBlock<
            BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyEl,
        >,
    >,
    guest_os_feature: Option<
        DynamicBlock<
            BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureEl,
        >,
    >,
    initialize_params: Option<
        DynamicBlock<
            BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_interface: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saved_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption_key: Option<
        Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_os_feature: Option<
        Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    initialize_params: Option<
        Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsEl>,
    >,
    dynamic: BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDynamic,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksEl {
    #[doc = "Set the field `auto_delete`.\nOptional. Specifies whether the disk will be auto-deleted when the instance is deleted."]
    pub fn set_auto_delete(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_delete = Some(v.into());
        self
    }
    #[doc = "Set the field `boot`.\nOptional. Indicates that this is a boot disk."]
    pub fn set_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.boot = Some(v.into());
        self
    }
    #[doc = "Set the field `device_name`.\nOptional. This is used as an identifier for the disks."]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_interface`.\nOptional. Specifies the disk interface to use for attaching this disk. Possible values: [\"DISK_INTERFACE_UNSPECIFIED\", \"SCSI\", \"NVME\", \"NVDIMM\", \"ISCSI\"]"]
    pub fn set_disk_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_interface = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\nOptional. The size of the disk in GB."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\nOutput only. The URI of the disk type resource."]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `index`.\nOptional. A zero-based index to this disk, where 0 is reserved for the boot disk."]
    pub fn set_index(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.index = Some(v.into());
        self
    }
    #[doc = "Set the field `kind`.\nOptional. Type of the resource."]
    pub fn set_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kind = Some(v.into());
        self
    }
    #[doc = "Set the field `license`.\nOptional. Any valid publicly visible licenses."]
    pub fn set_license(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.license = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nOptional. The mode in which to attach this disk. Possible values: [\"DISK_MODE_UNSPECIFIED\", \"READ_WRITE\", \"READ_ONLY\", \"LOCKED\"]"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `saved_state`.\nOptional. Specifies the saved state of the disk. Possible values: [\"DISK_SAVED_STATE_UNSPECIFIED\", \"PRESERVED\"]"]
    pub fn set_saved_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.saved_state = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\nOptional. Specifies a valid partial or full URL to an existing Persistent Disk resource."]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nOptional. Specifies the type of the disk. Possible values: [\"DISK_TYPE_UNSPECIFIED\", \"SCRATCH\", \"PERSISTENT\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_encryption_key`.\n"]
    pub fn set_disk_encryption_key(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disk_encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_encryption_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `guest_os_feature`.\n"]
    pub fn set_guest_os_feature(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.guest_os_feature = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.guest_os_feature = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `initialize_params`.\n"]
    pub fn set_initialize_params(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.initialize_params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.initialize_params = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksEl {}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksEl {
    pub fn build(self) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksEl {
            auto_delete: core::default::Default::default(),
            boot: core::default::Default::default(),
            device_name: core::default::Default::default(),
            disk_interface: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            index: core::default::Default::default(),
            kind: core::default::Default::default(),
            license: core::default::Default::default(),
            mode: core::default::Default::default(),
            saved_state: core::default::Default::default(),
            source: core::default::Default::default(),
            type_: core::default::Default::default(),
            disk_encryption_key: core::default::Default::default(),
            guest_os_feature: core::default::Default::default(),
            initialize_params: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_delete` after provisioning.\nOptional. Specifies whether the disk will be auto-deleted when the instance is deleted."]
    pub fn auto_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }
    #[doc = "Get a reference to the value of field `boot` after provisioning.\nOptional. Indicates that this is a boot disk."]
    pub fn boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.boot", self.base))
    }
    #[doc = "Get a reference to the value of field `device_name` after provisioning.\nOptional. This is used as an identifier for the disks."]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_interface` after provisioning.\nOptional. Specifies the disk interface to use for attaching this disk. Possible values: [\"DISK_INTERFACE_UNSPECIFIED\", \"SCSI\", \"NVME\", \"NVDIMM\", \"ISCSI\"]"]
    pub fn disk_interface(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_interface", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nOptional. The size of the disk in GB."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nOutput only. The URI of the disk type resource."]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `index` after provisioning.\nOptional. A zero-based index to this disk, where 0 is reserved for the boot disk."]
    pub fn index(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.index", self.base))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nOptional. Type of the resource."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }
    #[doc = "Get a reference to the value of field `license` after provisioning.\nOptional. Any valid publicly visible licenses."]
    pub fn license(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.license", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nOptional. The mode in which to attach this disk. Possible values: [\"DISK_MODE_UNSPECIFIED\", \"READ_WRITE\", \"READ_ONLY\", \"LOCKED\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `saved_state` after provisioning.\nOptional. Specifies the saved state of the disk. Possible values: [\"DISK_SAVED_STATE_UNSPECIFIED\", \"PRESERVED\"]"]
    pub fn saved_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.saved_state", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\nOptional. Specifies a valid partial or full URL to an existing Persistent Disk resource."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional. Specifies the type of the disk. Possible values: [\"DISK_TYPE_UNSPECIFIED\", \"SCRATCH\", \"PERSISTENT\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_encryption_key` after provisioning.\n"]
    pub fn disk_encryption_key(
        &self,
    ) -> ListRef<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElDiskEncryptionKeyElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `guest_os_feature` after provisioning.\n"]
    pub fn guest_os_feature(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElGuestOsFeatureElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_os_feature", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `initialize_params` after provisioning.\n"]
    pub fn initialize_params(
        &self,
    ) -> ListRef<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElInitializeParamsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.initialize_params", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_display: Option<PrimField<bool>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceEl {
    #[doc = "Set the field `enable_display`.\nOptional. Enables display for the Compute Engine VM."]
    pub fn set_enable_display(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_display = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceEl {
    type O =
        BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceEl {}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceEl {
    pub fn build(self) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceEl {
            enable_display: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_display` after provisioning.\nOptional. Enables display for the Compute Engine VM."]
    pub fn enable_display(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_display", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_type: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsEl {
    #[doc = "Set the field `accelerator_count`.\nOptional. The number of the guest accelerator cards exposed to this instance."]
    pub fn set_accelerator_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.accelerator_count = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_type`.\nOptional. Full or partial URL of the accelerator type resource."]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsEl {}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsEl {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\nOptional. The number of the guest accelerator cards exposed to this instance."]
    pub fn accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\nOptional. Full or partial URL of the accelerator type resource."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyEl {
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
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyEl {
}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyEl {
            kms_key_name: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyElRef {
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
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsEl {
    #[doc = ""]
    pub key: PrimField<String>,
}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsEl {
    pub fn build(self) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsEl {
            key: self.key,
            value: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsEl {
    type O =
        BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsEl {}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElDynamic {
    items: Option<
        DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsEl>,
    >,
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsEl>>,
    dynamic: BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElDynamic,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.items = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.items = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataEl {}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataEl {
    pub fn build(self) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataEl {
            items: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElItemsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ipv6: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ipv6_prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_ptr_domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_public_ptr: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsEl {
    #[doc = "Set the field `external_ip`.\n"]
    pub fn set_external_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `external_ipv6`.\n"]
    pub fn set_external_ipv6(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_ipv6 = Some(v.into());
        self
    }
    #[doc = "Set the field `external_ipv6_prefix_length`.\n"]
    pub fn set_external_ipv6_prefix_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.external_ipv6_prefix_length = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nOptional. The name of this access configuration."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tier`.\n Possible values: [\"NETWORK_TIER_UNSPECIFIED\", \"PREMIUM\", \"STANDARD\"]"]
    pub fn set_network_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_tier = Some(v.into());
        self
    }
    #[doc = "Set the field `public_ptr_domain_name`.\n"]
    pub fn set_public_ptr_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_ptr_domain_name = Some(v.into());
        self
    }
    #[doc = "Set the field `set_public_ptr`.\n"]
    pub fn set_set_public_ptr(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.set_public_ptr = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nOptional. The type of configuration. Possible values: [\"ACCESS_TYPE_UNSPECIFIED\", \"ONE_TO_ONE_NAT\", \"DIRECT_IPV6\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsEl
{}
impl
    BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsEl
{
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsEl
    {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsEl { external_ip : core :: default :: Default :: default () , external_ipv6 : core :: default :: Default :: default () , external_ipv6_prefix_length : core :: default :: Default :: default () , name : core :: default :: Default :: default () , network_tier : core :: default :: Default :: default () , public_ptr_domain_name : core :: default :: Default :: default () , set_public_ptr : core :: default :: Default :: default () , type_ : core :: default :: Default :: default () , }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsElRef { fn new (shared : StackShared , base : String) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsElRef { BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsElRef { shared : shared , base : base . to_string () , } } }
impl
    BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_ip` after provisioning.\n"]
    pub fn external_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `external_ipv6` after provisioning.\n"]
    pub fn external_ipv6(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_ipv6", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `external_ipv6_prefix_length` after provisioning.\n"]
    pub fn external_ipv6_prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_ipv6_prefix_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOptional. The name of this access configuration."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\n Possible values: [\"NETWORK_TIER_UNSPECIFIED\", \"PREMIUM\", \"STANDARD\"]"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }
    #[doc = "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\n"]
    pub fn public_ptr_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ptr_domain_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `set_public_ptr` after provisioning.\n"]
    pub fn set_public_ptr(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.set_public_ptr", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional. The type of configuration. Possible values: [\"ACCESS_TYPE_UNSPECIFIED\", \"ONE_TO_ONE_NAT\", \"DIRECT_IPV6\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_cidr_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork_range_name: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesEl {
    #[doc = "Set the field `ip_cidr_range`.\n"]
    pub fn set_ip_cidr_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_cidr_range = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork_range_name`.\n"]
    pub fn set_subnetwork_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork_range_name = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesEl
{}
impl
    BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesEl
{
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesEl
    {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesEl { ip_cidr_range : core :: default :: Default :: default () , subnetwork_range_name : core :: default :: Default :: default () , }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesElRef { fn new (shared : StackShared , base : String) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesElRef { BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesElRef { shared : shared , base : base . to_string () , } } }
impl
    BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\n"]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork_range_name` after provisioning.\n"]
    pub fn subnetwork_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork_range_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ipv6: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ipv6_prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_ptr_domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_public_ptr: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl
    BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsEl
{
    #[doc = "Set the field `external_ip`.\n"]
    pub fn set_external_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `external_ipv6`.\n"]
    pub fn set_external_ipv6(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_ipv6 = Some(v.into());
        self
    }
    #[doc = "Set the field `external_ipv6_prefix_length`.\n"]
    pub fn set_external_ipv6_prefix_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.external_ipv6_prefix_length = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nOptional. The name of this access configuration."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tier`.\n Possible values: [\"NETWORK_TIER_UNSPECIFIED\", \"PREMIUM\", \"STANDARD\"]"]
    pub fn set_network_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_tier = Some(v.into());
        self
    }
    #[doc = "Set the field `public_ptr_domain_name`.\n"]
    pub fn set_public_ptr_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_ptr_domain_name = Some(v.into());
        self
    }
    #[doc = "Set the field `set_public_ptr`.\n"]
    pub fn set_set_public_ptr(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.set_public_ptr = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nOptional. The type of configuration. Possible values: [\"ACCESS_TYPE_UNSPECIFIED\", \"ONE_TO_ONE_NAT\", \"DIRECT_IPV6\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsEl { type O = BlockAssignable < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsEl
{}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsEl { pub fn build (self) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsEl { BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsEl { external_ip : core :: default :: Default :: default () , external_ipv6 : core :: default :: Default :: default () , external_ipv6_prefix_length : core :: default :: Default :: default () , name : core :: default :: Default :: default () , network_tier : core :: default :: Default :: default () , public_ptr_domain_name : core :: default :: Default :: default () , set_public_ptr : core :: default :: Default :: default () , type_ : core :: default :: Default :: default () , } } }
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsElRef { fn new (shared : StackShared , base : String) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsElRef { BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsElRef { shared : shared , base : base . to_string () , } } }
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `external_ip` after provisioning.\n"] pub fn external_ip (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.external_ip" , self . base)) } # [doc = "Get a reference to the value of field `external_ipv6` after provisioning.\n"] pub fn external_ipv6 (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.external_ipv6" , self . base)) } # [doc = "Get a reference to the value of field `external_ipv6_prefix_length` after provisioning.\n"] pub fn external_ipv6_prefix_length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.external_ipv6_prefix_length" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\nOptional. The name of this access configuration."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `network_tier` after provisioning.\n Possible values: [\"NETWORK_TIER_UNSPECIFIED\", \"PREMIUM\", \"STANDARD\"]"] pub fn network_tier (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.network_tier" , self . base)) } # [doc = "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\n"] pub fn public_ptr_domain_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.public_ptr_domain_name" , self . base)) } # [doc = "Get a reference to the value of field `set_public_ptr` after provisioning.\n"] pub fn set_public_ptr (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.set_public_ptr" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\nOptional. The type of configuration. Possible values: [\"ACCESS_TYPE_UNSPECIFIED\", \"ONE_TO_ONE_NAT\", \"DIRECT_IPV6\"]"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize, Default)]
struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElDynamic { access_configs : Option < DynamicBlock < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsEl >> , alias_ip_ranges : Option < DynamicBlock < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesEl >> , ipv6_access_configs : Option < DynamicBlock < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsEl >> , }
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesEl { # [serde (skip_serializing_if = "Option::is_none")] internal_ipv6_prefix_length : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] ip_address : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ipv6_access_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ipv6_address : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] network : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] network_attachment : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] nic_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] queue_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] stack_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subnetwork : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] access_configs : Option < Vec < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsEl > > , # [serde (skip_serializing_if = "Option::is_none")] alias_ip_ranges : Option < Vec < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesEl > > , # [serde (skip_serializing_if = "Option::is_none")] ipv6_access_configs : Option < Vec < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsEl > > , dynamic : BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElDynamic , }
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesEl {
    #[doc = "Set the field `internal_ipv6_prefix_length`.\nOptional. The prefix length of the primary internal IPv6 range."]
    pub fn set_internal_ipv6_prefix_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.internal_ipv6_prefix_length = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\nOptional. An IPv4 internal IP address to assign to the instance."]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_access_type`.\n Possible values: [\"UNSPECIFIED_IPV6_ACCESS_TYPE\", \"INTERNAL\", \"EXTERNAL\"]"]
    pub fn set_ipv6_access_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_access_type = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_address`.\nOptional. An IPv6 internal network address for this network interface."]
    pub fn set_ipv6_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nOptional. URL of the VPC network resource for this instance."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `network_attachment`.\n"]
    pub fn set_network_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_attachment = Some(v.into());
        self
    }
    #[doc = "Set the field `nic_type`.\n Possible values: [\"NIC_TYPE_UNSPECIFIED\", \"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn set_nic_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nic_type = Some(v.into());
        self
    }
    #[doc = "Set the field `queue_count`.\n"]
    pub fn set_queue_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.queue_count = Some(v.into());
        self
    }
    #[doc = "Set the field `stack_type`.\n Possible values: [\"STACK_TYPE_UNSPECIFIED\", \"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn set_stack_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stack_type = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nOptional. The URL of the Subnetwork resource for this instance."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `access_configs`.\n"]
    pub fn set_access_configs(
        mut self,
        v : impl Into < BlockAssignable < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `alias_ip_ranges`.\n"]
    pub fn set_alias_ip_ranges(
        mut self,
        v : impl Into < BlockAssignable < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.alias_ip_ranges = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.alias_ip_ranges = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ipv6_access_configs`.\n"]
    pub fn set_ipv6_access_configs(
        mut self,
        v : impl Into < BlockAssignable < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ipv6_access_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ipv6_access_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesEl {}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesEl {
            internal_ipv6_prefix_length: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            ipv6_access_type: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_attachment: core::default::Default::default(),
            nic_type: core::default::Default::default(),
            queue_count: core::default::Default::default(),
            stack_type: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            access_configs: core::default::Default::default(),
            alias_ip_ranges: core::default::Default::default(),
            ipv6_access_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `internal_ipv6_prefix_length` after provisioning.\nOptional. The prefix length of the primary internal IPv6 range."]
    pub fn internal_ipv6_prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.internal_ipv6_prefix_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nOptional. An IPv4 internal IP address to assign to the instance."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `ipv6_access_type` after provisioning.\n Possible values: [\"UNSPECIFIED_IPV6_ACCESS_TYPE\", \"INTERNAL\", \"EXTERNAL\"]"]
    pub fn ipv6_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_access_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_address` after provisioning.\nOptional. An IPv6 internal network address for this network interface."]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nOptional. URL of the VPC network resource for this instance."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `network_attachment` after provisioning.\n"]
    pub fn network_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nic_type` after provisioning.\n Possible values: [\"NIC_TYPE_UNSPECIFIED\", \"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn nic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nic_type", self.base))
    }
    #[doc = "Get a reference to the value of field `queue_count` after provisioning.\n"]
    pub fn queue_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_count", self.base))
    }
    #[doc = "Get a reference to the value of field `stack_type` after provisioning.\n Possible values: [\"STACK_TYPE_UNSPECIFIED\", \"IPV4_ONLY\", \"IPV4_IPV6\"]"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nOptional. The URL of the Subnetwork resource for this instance."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `access_configs` after provisioning.\n"]    pub fn access_configs (& self) -> ListRef < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAccessConfigsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `alias_ip_ranges` after provisioning.\n"]    pub fn alias_ip_ranges (& self) -> ListRef < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElAliasIpRangesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.alias_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_access_configs` after provisioning.\n"]    pub fn ipv6_access_configs (& self) -> ListRef < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElIpv6AccessConfigsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv6_access_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    total_egress_bandwidth_tier: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigEl {
    #[doc = "Set the field `total_egress_bandwidth_tier`.\n Possible values: [\"TIER_UNSPECIFIED\", \"DEFAULT\", \"TIER_1\"]"]
    pub fn set_total_egress_bandwidth_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_egress_bandwidth_tier = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigEl
{}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigElRef
    {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\n Possible values: [\"TIER_UNSPECIFIED\", \"DEFAULT\", \"TIER_1\"]"]
    pub fn total_egress_bandwidth_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_egress_bandwidth_tier", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsEl
{
    #[doc = ""]
    pub key: PrimField<String>,
}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsEl
    {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsEl {
            key: self.key,
            value: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsElRef
    {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElDynamic {
    resource_manager_tags: Option<
        DynamicBlock<
            BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<
        Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsEl>,
    >,
    dynamic: BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElDynamic,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(
        mut self,
        v : impl Into < BlockAssignable < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElResourceManagerTagsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_manager_tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_manager_tags = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsEl {}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsEl {
    pub fn build(self) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsEl {
            resource_manager_tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl
    BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutEl
{
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
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutEl { type O = BlockAssignable < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutEl
{}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutEl { pub fn build (self) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutEl { BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutEl { nanos : core :: default :: Default :: default () , seconds : core :: default :: Default :: default () , } } }
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutElRef { fn new (shared : StackShared , base : String) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutElRef { BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutElRef { shared : shared , base : base . to_string () , } } }
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `nanos` after provisioning.\n"] pub fn nanos (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nanos" , self . base)) } # [doc = "Get a reference to the value of field `seconds` after provisioning.\n"] pub fn seconds (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.seconds" , self . base)) } }
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationEl {
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
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationEl
{}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationElRef
    {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `operator`.\n Possible values: [\"OPERATOR_UNSPECIFIED\", \"IN\", \"NOT_IN\"]"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesEl
{}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesEl {
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesElRef
    {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n Possible values: [\"OPERATOR_UNSPECIFIED\", \"IN\", \"NOT_IN\"]"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElDynamic { local_ssd_recovery_timeout : Option < DynamicBlock < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutEl >> , max_run_duration : Option < DynamicBlock < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationEl >> , node_affinities : Option < DynamicBlock < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesEl >> , }
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingEl { # [serde (skip_serializing_if = "Option::is_none")] automatic_restart : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] instance_termination_action : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] min_node_cpus : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] on_host_maintenance : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] preemptible : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] provisioning_model : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] termination_time : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] local_ssd_recovery_timeout : Option < Vec < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutEl > > , # [serde (skip_serializing_if = "Option::is_none")] max_run_duration : Option < Vec < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationEl > > , # [serde (skip_serializing_if = "Option::is_none")] node_affinities : Option < Vec < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesEl > > , dynamic : BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElDynamic , }
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingEl {
    #[doc = "Set the field `automatic_restart`.\n"]
    pub fn set_automatic_restart(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.automatic_restart = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_termination_action`.\n Possible values: [\"INSTANCE_TERMINATION_ACTION_UNSPECIFIED\", \"DELETE\", \"STOP\"]"]
    pub fn set_instance_termination_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_termination_action = Some(v.into());
        self
    }
    #[doc = "Set the field `min_node_cpus`.\n"]
    pub fn set_min_node_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_cpus = Some(v.into());
        self
    }
    #[doc = "Set the field `on_host_maintenance`.\n Possible values: [\"ON_HOST_MAINTENANCE_UNSPECIFIED\", \"TERMINATE\", \"MIGRATE\"]"]
    pub fn set_on_host_maintenance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_host_maintenance = Some(v.into());
        self
    }
    #[doc = "Set the field `preemptible`.\n"]
    pub fn set_preemptible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preemptible = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioning_model`.\n Possible values: [\"PROVISIONING_MODEL_UNSPECIFIED\", \"STANDARD\", \"SPOT\"]"]
    pub fn set_provisioning_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provisioning_model = Some(v.into());
        self
    }
    #[doc = "Set the field `termination_time`.\n"]
    pub fn set_termination_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.termination_time = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_recovery_timeout`.\n"]
    pub fn set_local_ssd_recovery_timeout(
        mut self,
        v : impl Into < BlockAssignable < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.local_ssd_recovery_timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.local_ssd_recovery_timeout = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `max_run_duration`.\n"]
    pub fn set_max_run_duration(
        mut self,
        v : impl Into < BlockAssignable < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.max_run_duration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.max_run_duration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_affinities`.\n"]
    pub fn set_node_affinities(
        mut self,
        v : impl Into < BlockAssignable < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_affinities = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_affinities = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingEl {}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingEl {
    pub fn build(self) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingEl {
            automatic_restart: core::default::Default::default(),
            instance_termination_action: core::default::Default::default(),
            min_node_cpus: core::default::Default::default(),
            on_host_maintenance: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            provisioning_model: core::default::Default::default(),
            termination_time: core::default::Default::default(),
            local_ssd_recovery_timeout: core::default::Default::default(),
            max_run_duration: core::default::Default::default(),
            node_affinities: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `automatic_restart` after provisioning.\n"]
    pub fn automatic_restart(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_restart", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_termination_action` after provisioning.\n Possible values: [\"INSTANCE_TERMINATION_ACTION_UNSPECIFIED\", \"DELETE\", \"STOP\"]"]
    pub fn instance_termination_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_termination_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_node_cpus` after provisioning.\n"]
    pub fn min_node_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_cpus", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_host_maintenance` after provisioning.\n Possible values: [\"ON_HOST_MAINTENANCE_UNSPECIFIED\", \"TERMINATE\", \"MIGRATE\"]"]
    pub fn on_host_maintenance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_host_maintenance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preemptible` after provisioning.\n"]
    pub fn preemptible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }
    #[doc = "Get a reference to the value of field `provisioning_model` after provisioning.\n Possible values: [\"PROVISIONING_MODEL_UNSPECIFIED\", \"STANDARD\", \"SPOT\"]"]
    pub fn provisioning_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioning_model", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `termination_time` after provisioning.\n"]
    pub fn termination_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.termination_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_recovery_timeout` after provisioning.\n"]    pub fn local_ssd_recovery_timeout (& self) -> ListRef < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElLocalSsdRecoveryTimeoutElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.local_ssd_recovery_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_run_duration` after provisioning.\n"]
    pub fn max_run_duration(
        &self,
    ) -> ListRef<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElMaxRunDurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_run_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_affinities` after provisioning.\n"]
    pub fn node_affinities(
        &self,
    ) -> ListRef<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElNodeAffinitiesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_affinities", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsEl {
    type O =
        BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsEl {}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsEl {
            email: core::default::Default::default(),
            scopes: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_vtpm: Option<PrimField<bool>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\n"]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_secure_boot`.\n"]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_vtpm`.\n"]
    pub fn set_enable_vtpm(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_vtpm = Some(v.into());
        self
    }
}
impl ToListMappable
    for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigEl
{
    type O = BlockAssignable<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigEl {
}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigEl {
    pub fn build(
        self,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\n"]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_integrity_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\n"]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_vtpm` after provisioning.\n"]
    pub fn enable_vtpm(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<ListField<PrimField<String>>>,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsEl {}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsEl {
    pub fn build(self) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsEl {
            items: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize, Default)]
struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDynamic {
    advanced_machine_features: Option<
        DynamicBlock<
            BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesEl,
        >,
    >,
    allocation_affinity: Option<
        DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityEl>,
    >,
    confidential_instance_config: Option<
        DynamicBlock<
            BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigEl,
        >,
    >,
    disks: Option<DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksEl>>,
    display_device: Option<
        DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceEl>,
    >,
    guest_accelerators: Option<
        DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsEl>,
    >,
    instance_encryption_key: Option<
        DynamicBlock<
            BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyEl,
        >,
    >,
    labels: Option<DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsEl>>,
    metadata:
        Option<DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataEl>>,
    network_interfaces: Option<
        DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesEl>,
    >,
    network_performance_config: Option<
        DynamicBlock<
            BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigEl,
        >,
    >,
    params: Option<DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsEl>>,
    scheduling:
        Option<DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingEl>>,
    service_accounts: Option<
        DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsEl>,
    >,
    shielded_instance_config: Option<
        DynamicBlock<
            BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigEl,
        >,
    >,
    tags: Option<DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsEl>>,
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    can_ip_forward: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_revocation_action_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ipv6_google_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_policies: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_machine_features: Option<
        Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_affinity:
        Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_config: Option<
        Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    disks: Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_device:
        Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerators:
        Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_encryption_key: Option<
        Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interfaces:
        Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_performance_config: Option<
        Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduling: Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_accounts:
        Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config: Option<
        Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsEl>>,
    dynamic: BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDynamic,
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesEl {
    #[doc = "Set the field `can_ip_forward`.\nOptional. Allows this instance to send and receive packets with non-matching destination or source IPs."]
    pub fn set_can_ip_forward(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.can_ip_forward = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nOptional. Whether the resource should be protected against deletion."]
    pub fn set_deletion_protection(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nOptional. An optional description of this resource."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `hostname`.\nOptional. Specifies the hostname of the instance."]
    pub fn set_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname = Some(v.into());
        self
    }
    #[doc = "Set the field `key_revocation_action_type`.\nOptional. KeyRevocationActionType of the instance. Possible values: [\"KEY_REVOCATION_ACTION_TYPE_UNSPECIFIED\", \"NONE\", \"STOP\"]"]
    pub fn set_key_revocation_action_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_revocation_action_type = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nOptional. Full or partial URL of the machine type resource to use for this instance."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `min_cpu_platform`.\nOptional. Minimum CPU platform to use for this instance."]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }
    #[doc = "Set the field `private_ipv6_google_access`.\nOptional. The private IPv6 google access type for the VM. Possible values: [\"INSTANCE_PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED\", \"INHERIT_FROM_SUBNETWORK\", \"ENABLE_OUTBOUND_VM_ACCESS_TO_GOOGLE\", \"ENABLE_BIDIRECTIONAL_ACCESS_TO_GOOGLE\"]"]
    pub fn set_private_ipv6_google_access(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_ipv6_google_access = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_policies`.\nOptional. Resource policies applied to this instance."]
    pub fn set_resource_policies(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resource_policies = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_machine_features = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_machine_features = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `allocation_affinity`.\n"]
    pub fn set_allocation_affinity(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allocation_affinity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allocation_affinity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `confidential_instance_config`.\n"]
    pub fn set_confidential_instance_config(
        mut self,
        v : impl Into < BlockAssignable < BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.confidential_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.confidential_instance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disks`.\n"]
    pub fn set_disks(
        mut self,
        v: impl Into<BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `display_device`.\n"]
    pub fn set_display_device(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.display_device = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.display_device = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `guest_accelerators`.\n"]
    pub fn set_guest_accelerators(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.guest_accelerators = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.guest_accelerators = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `instance_encryption_key`.\n"]
    pub fn set_instance_encryption_key(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_encryption_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl Into<BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElLabelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.labels = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl Into<
            BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_interfaces`.\n"]
    pub fn set_network_interfaces(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_interfaces = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interfaces = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_performance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_performance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(
        mut self,
        v: impl Into<BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.params = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scheduling`.\n"]
    pub fn set_scheduling(
        mut self,
        v: impl Into<
            BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scheduling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduling = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_accounts`.\n"]
    pub fn set_service_accounts(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_accounts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_accounts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.shielded_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesEl {
    #[doc = "Required. Name of the compute instance."]
    pub name: PrimField<String>,
}
impl BuildBackupDrRestoreWorkloadComputeInstanceRestorePropertiesEl {
    pub fn build(self) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesEl {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesEl {
            can_ip_forward: core::default::Default::default(),
            deletion_protection: core::default::Default::default(),
            description: core::default::Default::default(),
            hostname: core::default::Default::default(),
            key_revocation_action_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            name: self.name,
            private_ipv6_google_access: core::default::Default::default(),
            resource_policies: core::default::Default::default(),
            advanced_machine_features: core::default::Default::default(),
            allocation_affinity: core::default::Default::default(),
            confidential_instance_config: core::default::Default::default(),
            disks: core::default::Default::default(),
            display_device: core::default::Default::default(),
            guest_accelerators: core::default::Default::default(),
            instance_encryption_key: core::default::Default::default(),
            labels: core::default::Default::default(),
            metadata: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
            network_performance_config: core::default::Default::default(),
            params: core::default::Default::default(),
            scheduling: core::default::Default::default(),
            service_accounts: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElRef {
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `can_ip_forward` after provisioning.\nOptional. Allows this instance to send and receive packets with non-matching destination or source IPs."]
    pub fn can_ip_forward(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.can_ip_forward", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nOptional. Whether the resource should be protected against deletion."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. An optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nOptional. Specifies the hostname of the instance."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `key_revocation_action_type` after provisioning.\nOptional. KeyRevocationActionType of the instance. Possible values: [\"KEY_REVOCATION_ACTION_TYPE_UNSPECIFIED\", \"NONE\", \"STOP\"]"]
    pub fn key_revocation_action_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_revocation_action_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nOptional. Full or partial URL of the machine type resource to use for this instance."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\nOptional. Minimum CPU platform to use for this instance."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cpu_platform", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Name of the compute instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nOptional. The private IPv6 google access type for the VM. Possible values: [\"INSTANCE_PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED\", \"INHERIT_FROM_SUBNETWORK\", \"ENABLE_OUTBOUND_VM_ACCESS_TO_GOOGLE\", \"ENABLE_BIDIRECTIONAL_ACCESS_TO_GOOGLE\"]"]
    pub fn private_ipv6_google_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ipv6_google_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\nOptional. Resource policies applied to this instance."]
    pub fn resource_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_policies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> ListRef<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAdvancedMachineFeaturesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_machine_features", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allocation_affinity` after provisioning.\n"]
    pub fn allocation_affinity(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElAllocationAffinityElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allocation_affinity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(
        &self,
    ) -> ListRef<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElConfidentialInstanceConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_instance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disks` after provisioning.\n"]
    pub fn disks(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disks", self.base))
    }
    #[doc = "Get a reference to the value of field `display_device` after provisioning.\n"]
    pub fn display_device(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElDisplayDeviceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.display_device", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `guest_accelerators` after provisioning.\n"]
    pub fn guest_accelerators(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElGuestAcceleratorsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_accelerators", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_encryption_key` after provisioning.\n"]
    pub fn instance_encryption_key(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElInstanceEncryptionKeyElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElMetadataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkInterfacesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interfaces", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(
        &self,
    ) -> ListRef<
        BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElNetworkPerformanceConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_performance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElParamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.params", self.base))
    }
    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElSchedulingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduling", self.base))
    }
    #[doc = "Get a reference to the value of field `service_accounts` after provisioning.\n"]
    pub fn service_accounts(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElServiceAccountsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_accounts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElShieldedInstanceConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesElTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentEl {
    project: PrimField<String>,
    zone: PrimField<String>,
}
impl BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentEl {}
impl ToListMappable for BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadComputeInstanceTargetEnvironmentEl {
    #[doc = "Required. Target project for the Compute Engine instance."]
    pub project: PrimField<String>,
    #[doc = "Required. The zone of the Compute Engine instance."]
    pub zone: PrimField<String>,
}
impl BuildBackupDrRestoreWorkloadComputeInstanceTargetEnvironmentEl {
    pub fn build(self) -> BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentEl {
        BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentEl {
            project: self.project,
            zone: self.zone,
        }
    }
}
pub struct BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentElRef {
        BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nRequired. Target project for the Compute Engine instance."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nRequired. The zone of the Compute Engine instance."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_encrypted_key: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyEl {
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
}
impl ToListMappable for BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyEl {}
impl BuildBackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyEl {
    pub fn build(self) -> BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyEl {
        BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyEl {
            kms_key_name: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyElRef {
        BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyElRef {
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
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureEl {
    #[doc = "Set the field `type_`.\n Possible values: [\"FEATURE_TYPE_UNSPECIFIED\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"MULTI_IP_SUBNET\", \"UEFI_COMPATIBLE\", \"SECURE_BOOT\", \"GVNIC\", \"SEV_CAPABLE\", \"BARE_METAL_LINUX_COMPATIBLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"TDX_CAPABLE\", \"IDPF\", \"SEV_LIVE_MIGRATABLE_V2\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureEl {}
impl BuildBackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureEl {
    pub fn build(self) -> BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureEl {
        BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureElRef {
        BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n Possible values: [\"FEATURE_TYPE_UNSPECIFIED\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"MULTI_IP_SUBNET\", \"UEFI_COMPATIBLE\", \"SECURE_BOOT\", \"GVNIC\", \"SEV_CAPABLE\", \"BARE_METAL_LINUX_COMPATIBLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"TDX_CAPABLE\", \"IDPF\", \"SEV_LIVE_MIGRATABLE_V2\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadDiskRestorePropertiesElLabelsEl {
    #[doc = ""]
    pub key: PrimField<String>,
}
impl BuildBackupDrRestoreWorkloadDiskRestorePropertiesElLabelsEl {
    pub fn build(self) -> BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsEl {
        BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsEl {
            key: self.key,
            value: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsElRef {
        BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsEl {
    #[doc = ""]
    pub key: PrimField<String>,
}
impl BuildBackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsEl {
    pub fn build(self) -> BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsEl {
        BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsEl {
            key: self.key,
            value: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsElRef {
        BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct BackupDrRestoreWorkloadDiskRestorePropertiesElDynamic {
    disk_encryption_key:
        Option<DynamicBlock<BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyEl>>,
    guest_os_feature:
        Option<DynamicBlock<BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureEl>>,
    labels: Option<DynamicBlock<BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsEl>>,
    resource_manager_tags:
        Option<DynamicBlock<BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsEl>>,
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadDiskRestorePropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    architecture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_confidential_compute: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    licenses: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    physical_block_size_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_policy: Option<ListField<PrimField<String>>>,
    size_gb: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_pool: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption_key:
        Option<Vec<BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_os_feature: Option<Vec<BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags:
        Option<Vec<BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsEl>>,
    dynamic: BackupDrRestoreWorkloadDiskRestorePropertiesElDynamic,
}
impl BackupDrRestoreWorkloadDiskRestorePropertiesEl {
    #[doc = "Set the field `access_mode`.\nOptional. The access mode of the disk. Possible values: [\"READ_WRITE_SINGLE\", \"READ_WRITE_MANY\", \"READ_ONLY_MANY\"]"]
    pub fn set_access_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `architecture`.\nOptional. The architecture of the source disk. Possible values: [\"ARCHITECTURE_UNSPECIFIED\", \"X86_64\", \"ARM64\"]"]
    pub fn set_architecture(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.architecture = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nOptional. An optional description of this resource."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_confidential_compute`.\nOptional. Indicates whether this disk is using confidential compute mode."]
    pub fn set_enable_confidential_compute(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_confidential_compute = Some(v.into());
        self
    }
    #[doc = "Set the field `licenses`.\nOptional. A list of publicly available licenses that are applicable to this backup."]
    pub fn set_licenses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.licenses = Some(v.into());
        self
    }
    #[doc = "Set the field `physical_block_size_bytes`.\nOptional. Physical block size of the persistent disk, in bytes."]
    pub fn set_physical_block_size_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.physical_block_size_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_iops`.\nOptional. Indicates how many IOPS to provision for the disk."]
    pub fn set_provisioned_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_throughput`.\nOptional. Indicates how much throughput to provision for the disk."]
    pub fn set_provisioned_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_policy`.\nOptional. Resource policies applied to this disk."]
    pub fn set_resource_policy(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resource_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_pool`.\nOptional. The storage pool in which the new disk is created."]
    pub fn set_storage_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_encryption_key`.\n"]
    pub fn set_disk_encryption_key(
        mut self,
        v: impl Into<BlockAssignable<BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disk_encryption_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_encryption_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `guest_os_feature`.\n"]
    pub fn set_guest_os_feature(
        mut self,
        v: impl Into<BlockAssignable<BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.guest_os_feature = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.guest_os_feature = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl Into<BlockAssignable<BackupDrRestoreWorkloadDiskRestorePropertiesElLabelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.labels = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl Into<
            BlockAssignable<BackupDrRestoreWorkloadDiskRestorePropertiesElResourceManagerTagsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_manager_tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_manager_tags = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BackupDrRestoreWorkloadDiskRestorePropertiesEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadDiskRestorePropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadDiskRestorePropertiesEl {
    #[doc = "Required. Name of the disk."]
    pub name: PrimField<String>,
    #[doc = "Required. The size of the disk in GB."]
    pub size_gb: PrimField<f64>,
    #[doc = "Required. URL of the disk type resource describing which disk type to use."]
    pub type_: PrimField<String>,
}
impl BuildBackupDrRestoreWorkloadDiskRestorePropertiesEl {
    pub fn build(self) -> BackupDrRestoreWorkloadDiskRestorePropertiesEl {
        BackupDrRestoreWorkloadDiskRestorePropertiesEl {
            access_mode: core::default::Default::default(),
            architecture: core::default::Default::default(),
            description: core::default::Default::default(),
            enable_confidential_compute: core::default::Default::default(),
            licenses: core::default::Default::default(),
            name: self.name,
            physical_block_size_bytes: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            resource_policy: core::default::Default::default(),
            size_gb: self.size_gb,
            storage_pool: core::default::Default::default(),
            type_: self.type_,
            disk_encryption_key: core::default::Default::default(),
            guest_os_feature: core::default::Default::default(),
            labels: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadDiskRestorePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadDiskRestorePropertiesElRef {
    fn new(shared: StackShared, base: String) -> BackupDrRestoreWorkloadDiskRestorePropertiesElRef {
        BackupDrRestoreWorkloadDiskRestorePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadDiskRestorePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\nOptional. The access mode of the disk. Possible values: [\"READ_WRITE_SINGLE\", \"READ_WRITE_MANY\", \"READ_ONLY_MANY\"]"]
    pub fn access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `architecture` after provisioning.\nOptional. The architecture of the source disk. Possible values: [\"ARCHITECTURE_UNSPECIFIED\", \"X86_64\", \"ARM64\"]"]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. An optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nOptional. Indicates whether this disk is using confidential compute mode."]
    pub fn enable_confidential_compute(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_confidential_compute", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `licenses` after provisioning.\nOptional. A list of publicly available licenses that are applicable to this backup."]
    pub fn licenses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.licenses", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Name of the disk."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `physical_block_size_bytes` after provisioning.\nOptional. Physical block size of the persistent disk, in bytes."]
    pub fn physical_block_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.physical_block_size_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_iops` after provisioning.\nOptional. Indicates how many IOPS to provision for the disk."]
    pub fn provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\nOptional. Indicates how much throughput to provision for the disk."]
    pub fn provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_policy` after provisioning.\nOptional. Resource policies applied to this disk."]
    pub fn resource_policy(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\nRequired. The size of the disk in GB."]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_pool` after provisioning.\nOptional. The storage pool in which the new disk is created."]
    pub fn storage_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_pool", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nRequired. URL of the disk type resource describing which disk type to use."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_encryption_key` after provisioning.\n"]
    pub fn disk_encryption_key(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadDiskRestorePropertiesElDiskEncryptionKeyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disk_encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `guest_os_feature` after provisioning.\n"]
    pub fn guest_os_feature(
        &self,
    ) -> ListRef<BackupDrRestoreWorkloadDiskRestorePropertiesElGuestOsFeatureElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_os_feature", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadDiskTargetEnvironmentEl {
    project: PrimField<String>,
    zone: PrimField<String>,
}
impl BackupDrRestoreWorkloadDiskTargetEnvironmentEl {}
impl ToListMappable for BackupDrRestoreWorkloadDiskTargetEnvironmentEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadDiskTargetEnvironmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadDiskTargetEnvironmentEl {
    #[doc = "Required. Target project for the disk."]
    pub project: PrimField<String>,
    #[doc = "Required. Target zone for the disk."]
    pub zone: PrimField<String>,
}
impl BuildBackupDrRestoreWorkloadDiskTargetEnvironmentEl {
    pub fn build(self) -> BackupDrRestoreWorkloadDiskTargetEnvironmentEl {
        BackupDrRestoreWorkloadDiskTargetEnvironmentEl {
            project: self.project,
            zone: self.zone,
        }
    }
}
pub struct BackupDrRestoreWorkloadDiskTargetEnvironmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadDiskTargetEnvironmentElRef {
    fn new(shared: StackShared, base: String) -> BackupDrRestoreWorkloadDiskTargetEnvironmentElRef {
        BackupDrRestoreWorkloadDiskTargetEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadDiskTargetEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nRequired. Target project for the disk."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nRequired. Target zone for the disk."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadRegionDiskTargetEnvironmentEl {
    project: PrimField<String>,
    region: PrimField<String>,
    replica_zones: ListField<PrimField<String>>,
}
impl BackupDrRestoreWorkloadRegionDiskTargetEnvironmentEl {}
impl ToListMappable for BackupDrRestoreWorkloadRegionDiskTargetEnvironmentEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadRegionDiskTargetEnvironmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadRegionDiskTargetEnvironmentEl {
    #[doc = "Required. Target project for the disk."]
    pub project: PrimField<String>,
    #[doc = "Required. Target region for the disk."]
    pub region: PrimField<String>,
    #[doc = "Required. Target URLs of the replica zones for the disk."]
    pub replica_zones: ListField<PrimField<String>>,
}
impl BuildBackupDrRestoreWorkloadRegionDiskTargetEnvironmentEl {
    pub fn build(self) -> BackupDrRestoreWorkloadRegionDiskTargetEnvironmentEl {
        BackupDrRestoreWorkloadRegionDiskTargetEnvironmentEl {
            project: self.project,
            region: self.region,
            replica_zones: self.replica_zones,
        }
    }
}
pub struct BackupDrRestoreWorkloadRegionDiskTargetEnvironmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadRegionDiskTargetEnvironmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupDrRestoreWorkloadRegionDiskTargetEnvironmentElRef {
        BackupDrRestoreWorkloadRegionDiskTargetEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadRegionDiskTargetEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nRequired. Target project for the disk."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRequired. Target region for the disk."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `replica_zones` after provisioning.\nRequired. Target URLs of the replica zones for the disk."]
    pub fn replica_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_zones", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BackupDrRestoreWorkloadTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl BackupDrRestoreWorkloadTimeoutsEl {
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
}
impl ToListMappable for BackupDrRestoreWorkloadTimeoutsEl {
    type O = BlockAssignable<BackupDrRestoreWorkloadTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBackupDrRestoreWorkloadTimeoutsEl {}
impl BuildBackupDrRestoreWorkloadTimeoutsEl {
    pub fn build(self) -> BackupDrRestoreWorkloadTimeoutsEl {
        BackupDrRestoreWorkloadTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct BackupDrRestoreWorkloadTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BackupDrRestoreWorkloadTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BackupDrRestoreWorkloadTimeoutsElRef {
        BackupDrRestoreWorkloadTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BackupDrRestoreWorkloadTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct BackupDrRestoreWorkloadDynamic {
    compute_instance_restore_properties:
        Option<DynamicBlock<BackupDrRestoreWorkloadComputeInstanceRestorePropertiesEl>>,
    compute_instance_target_environment:
        Option<DynamicBlock<BackupDrRestoreWorkloadComputeInstanceTargetEnvironmentEl>>,
    disk_restore_properties: Option<DynamicBlock<BackupDrRestoreWorkloadDiskRestorePropertiesEl>>,
    disk_target_environment: Option<DynamicBlock<BackupDrRestoreWorkloadDiskTargetEnvironmentEl>>,
    region_disk_target_environment:
        Option<DynamicBlock<BackupDrRestoreWorkloadRegionDiskTargetEnvironmentEl>>,
}
