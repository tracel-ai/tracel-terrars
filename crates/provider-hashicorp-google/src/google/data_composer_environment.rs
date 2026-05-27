use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComposerEnvironmentData {
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
struct DataComposerEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComposerEnvironmentData>,
}
#[derive(Clone)]
pub struct DataComposerEnvironment(Rc<DataComposerEnvironment_>);
impl DataComposerEnvironment {
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
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe location or Compute Engine region for the environment."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\nConfiguration parameters for this environment."]
    pub fn config(&self) -> ListRef<DataComposerEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: [a-z]([-a-z0-9]*[a-z0-9])?. Label values must be between 0 and 63 characters long and must conform to the regular expression ([a-z]([-a-z0-9]*[a-z0-9])?)?. No more than 64 labels can be associated with a given environment. Both keys and values must be <= 128 bytes in size.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the environment."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe location or Compute Engine region for the environment."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\nConfiguration options for storage used by Composer environment."]
    pub fn storage_config(&self) -> ListRef<DataComposerEnvironmentStorageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
}
impl Referable for DataComposerEnvironment {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComposerEnvironment {}
impl ToListMappable for DataComposerEnvironment {
    type O = ListRef<DataComposerEnvironmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComposerEnvironment_ {
    fn extract_datasource_type(&self) -> String {
        "google_composer_environment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComposerEnvironment {
    pub tf_id: String,
    #[doc = "Name of the environment."]
    pub name: PrimField<String>,
}
impl BuildDataComposerEnvironment {
    pub fn build(self, stack: &mut Stack) -> DataComposerEnvironment {
        let out = DataComposerEnvironment(Rc::new(DataComposerEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComposerEnvironmentData {
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
pub struct DataComposerEnvironmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComposerEnvironmentRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\nConfiguration parameters for this environment."]
    pub fn config(&self) -> ListRef<DataComposerEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: [a-z]([-a-z0-9]*[a-z0-9])?. Label values must be between 0 and 63 characters long and must conform to the regular expression ([a-z]([-a-z0-9]*[a-z0-9])?)?. No more than 64 labels can be associated with a given environment. Both keys and values must be <= 128 bytes in size.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the environment."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe location or Compute Engine region for the environment."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\nConfiguration options for storage used by Composer environment."]
    pub fn storage_config(&self) -> ListRef<DataComposerEnvironmentStorageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_mode: Option<PrimField<String>>,
}
impl DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl {
    #[doc = "Set the field `retention_days`.\n"]
    pub fn set_retention_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retention_days = Some(v.into());
        self
    }
    #[doc = "Set the field `retention_mode`.\n"]
    pub fn set_retention_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retention_mode = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl
{
    type O = BlockAssignable<
        DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl
{}
impl BuildDataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl {
    pub fn build(
        self,
    ) -> DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl {
        DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl {
            retention_days: core::default::Default::default(),
            retention_mode: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigElRef
    {
        DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `retention_days` after provisioning.\n"]
    pub fn retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retention_mode` after provisioning.\n"]
    pub fn retention_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_mode: Option<PrimField<String>>,
}
impl DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {
    #[doc = "Set the field `storage_mode`.\n"]
    pub fn set_storage_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_mode = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl
{
    type O = BlockAssignable<
        DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {}
impl BuildDataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {
    pub fn build(
        self,
    ) -> DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {
        DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {
            storage_mode: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigElRef {
        DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `storage_mode` after provisioning.\n"]
    pub fn storage_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElDataRetentionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    airflow_metadata_retention_config: Option<
        ListField<
            DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_logs_retention_config: Option<
        ListField<DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl>,
    >,
}
impl DataComposerEnvironmentConfigElDataRetentionConfigEl {
    #[doc = "Set the field `airflow_metadata_retention_config`.\n"]
    pub fn set_airflow_metadata_retention_config(
        mut self,
        v : impl Into < ListField < DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl > >,
    ) -> Self {
        self.airflow_metadata_retention_config = Some(v.into());
        self
    }
    #[doc = "Set the field `task_logs_retention_config`.\n"]
    pub fn set_task_logs_retention_config(
        mut self,
        v: impl Into<
            ListField<
                DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl,
            >,
        >,
    ) -> Self {
        self.task_logs_retention_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElDataRetentionConfigEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElDataRetentionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElDataRetentionConfigEl {}
impl BuildDataComposerEnvironmentConfigElDataRetentionConfigEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElDataRetentionConfigEl {
        DataComposerEnvironmentConfigElDataRetentionConfigEl {
            airflow_metadata_retention_config: core::default::Default::default(),
            task_logs_retention_config: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElDataRetentionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElDataRetentionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElDataRetentionConfigElRef {
        DataComposerEnvironmentConfigElDataRetentionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElDataRetentionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `airflow_metadata_retention_config` after provisioning.\n"]
    pub fn airflow_metadata_retention_config(
        &self,
    ) -> ListRef<
        DataComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.airflow_metadata_retention_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `task_logs_retention_config` after provisioning.\n"]
    pub fn task_logs_retention_config(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.task_logs_retention_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElDatabaseConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl DataComposerEnvironmentConfigElDatabaseConfigEl {
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElDatabaseConfigEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElDatabaseConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElDatabaseConfigEl {}
impl BuildDataComposerEnvironmentConfigElDatabaseConfigEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElDatabaseConfigEl {
        DataComposerEnvironmentConfigElDatabaseConfigEl {
            machine_type: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElDatabaseConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElDatabaseConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElDatabaseConfigElRef {
        DataComposerEnvironmentConfigElDatabaseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElDatabaseConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElEncryptionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}
impl DataComposerEnvironmentConfigElEncryptionConfigEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElEncryptionConfigEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElEncryptionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElEncryptionConfigEl {}
impl BuildDataComposerEnvironmentConfigElEncryptionConfigEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElEncryptionConfigEl {
        DataComposerEnvironmentConfigElEncryptionConfigEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElEncryptionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElEncryptionConfigElRef {
        DataComposerEnvironmentConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElMaintenanceWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurrence: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataComposerEnvironmentConfigElMaintenanceWindowEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `recurrence`.\n"]
    pub fn set_recurrence(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.recurrence = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElMaintenanceWindowEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElMaintenanceWindowEl {}
impl BuildDataComposerEnvironmentConfigElMaintenanceWindowEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElMaintenanceWindowEl {
        DataComposerEnvironmentConfigElMaintenanceWindowEl {
            end_time: core::default::Default::default(),
            recurrence: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElMaintenanceWindowElRef {
        DataComposerEnvironmentConfigElMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
}
impl DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl
{
    type O = BlockAssignable<
        DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {}
impl BuildDataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    pub fn build(
        self,
    ) -> DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
        DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
            cidr_block: core::default::Default::default(),
            display_name: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
        DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_blocks: Option<
        SetField<DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    #[doc = "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(
        mut self,
        v: impl Into<
            SetField<DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl>,
        >,
    ) -> Self {
        self.cidr_blocks = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {}
impl BuildDataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
        DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
            cidr_blocks: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
        DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr_blocks` after provisioning.\n"]
    pub fn cidr_blocks(
        &self,
    ) -> SetRef<DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef>
    {
        SetRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_secondary_range_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services_secondary_range_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_ip_aliases: Option<PrimField<bool>>,
}
impl DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
    #[doc = "Set the field `cluster_ipv4_cidr_block`.\n"]
    pub fn set_cluster_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_secondary_range_name`.\n"]
    pub fn set_cluster_secondary_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_secondary_range_name = Some(v.into());
        self
    }
    #[doc = "Set the field `services_ipv4_cidr_block`.\n"]
    pub fn set_services_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.services_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `services_secondary_range_name`.\n"]
    pub fn set_services_secondary_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.services_secondary_range_name = Some(v.into());
        self
    }
    #[doc = "Set the field `use_ip_aliases`.\n"]
    pub fn set_use_ip_aliases(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_ip_aliases = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {}
impl BuildDataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
        DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
            cluster_ipv4_cidr_block: core::default::Default::default(),
            cluster_secondary_range_name: core::default::Default::default(),
            services_ipv4_cidr_block: core::default::Default::default(),
            services_secondary_range_name: core::default::Default::default(),
            use_ip_aliases: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
        DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_ipv4_cidr_block` after provisioning.\n"]
    pub fn cluster_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_secondary_range_name` after provisioning.\n"]
    pub fn cluster_secondary_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_secondary_range_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `services_ipv4_cidr_block` after provisioning.\n"]
    pub fn services_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.services_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `services_secondary_range_name` after provisioning.\n"]
    pub fn services_secondary_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.services_secondary_range_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_ip_aliases` after provisioning.\n"]
    pub fn use_ip_aliases(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_ip_aliases", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    composer_internal_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    composer_network_attachment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ip_masq_agent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_allocation_policy:
        Option<ListField<DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_scopes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl DataComposerEnvironmentConfigElNodeConfigEl {
    #[doc = "Set the field `composer_internal_ipv4_cidr_block`.\n"]
    pub fn set_composer_internal_ipv4_cidr_block(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.composer_internal_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `composer_network_attachment`.\n"]
    pub fn set_composer_network_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.composer_network_attachment = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_ip_masq_agent`.\n"]
    pub fn set_enable_ip_masq_agent(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ip_masq_agent = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_allocation_policy`.\n"]
    pub fn set_ip_allocation_policy(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl>>,
    ) -> Self {
        self.ip_allocation_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_scopes`.\n"]
    pub fn set_oauth_scopes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.oauth_scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElNodeConfigEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElNodeConfigEl {}
impl BuildDataComposerEnvironmentConfigElNodeConfigEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElNodeConfigEl {
        DataComposerEnvironmentConfigElNodeConfigEl {
            composer_internal_ipv4_cidr_block: core::default::Default::default(),
            composer_network_attachment: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            enable_ip_masq_agent: core::default::Default::default(),
            ip_allocation_policy: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            network: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            service_account: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComposerEnvironmentConfigElNodeConfigElRef {
        DataComposerEnvironmentConfigElNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `composer_internal_ipv4_cidr_block` after provisioning.\n"]
    pub fn composer_internal_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.composer_internal_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `composer_network_attachment` after provisioning.\n"]
    pub fn composer_network_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.composer_network_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_ip_masq_agent` after provisioning.\n"]
    pub fn enable_ip_masq_agent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ip_masq_agent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_allocation_policy` after provisioning.\n"]
    pub fn ip_allocation_policy(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_allocation_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth_scopes` after provisioning.\n"]
    pub fn oauth_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_composer_connection_subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_composer_network_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_endpoint: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_privately_used_public_ips: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_server_ipv4_cidr_block: Option<PrimField<String>>,
}
impl DataComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
    #[doc = "Set the field `cloud_composer_connection_subnetwork`.\n"]
    pub fn set_cloud_composer_connection_subnetwork(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.cloud_composer_connection_subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_composer_network_ipv4_cidr_block`.\n"]
    pub fn set_cloud_composer_network_ipv4_cidr_block(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.cloud_composer_network_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_sql_ipv4_cidr_block`.\n"]
    pub fn set_cloud_sql_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_sql_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_type = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_private_endpoint`.\n"]
    pub fn set_enable_private_endpoint(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_privately_used_public_ips`.\n"]
    pub fn set_enable_privately_used_public_ips(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_privately_used_public_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `master_ipv4_cidr_block`.\n"]
    pub fn set_master_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.master_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `web_server_ipv4_cidr_block`.\n"]
    pub fn set_web_server_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_server_ipv4_cidr_block = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElPrivateEnvironmentConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElPrivateEnvironmentConfigEl {}
impl BuildDataComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
        DataComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
            cloud_composer_connection_subnetwork: core::default::Default::default(),
            cloud_composer_network_ipv4_cidr_block: core::default::Default::default(),
            cloud_sql_ipv4_cidr_block: core::default::Default::default(),
            connection_type: core::default::Default::default(),
            enable_private_endpoint: core::default::Default::default(),
            enable_privately_used_public_ips: core::default::Default::default(),
            master_ipv4_cidr_block: core::default::Default::default(),
            web_server_ipv4_cidr_block: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
        DataComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_composer_connection_subnetwork` after provisioning.\n"]
    pub fn cloud_composer_connection_subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_composer_connection_subnetwork", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_composer_network_ipv4_cidr_block` after provisioning.\n"]
    pub fn cloud_composer_network_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_composer_network_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_sql_ipv4_cidr_block` after provisioning.\n"]
    pub fn cloud_sql_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_sql_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_private_endpoint` after provisioning.\n"]
    pub fn enable_private_endpoint(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_privately_used_public_ips` after provisioning.\n"]
    pub fn enable_privately_used_public_ips(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_privately_used_public_ips", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `master_ipv4_cidr_block` after provisioning.\n"]
    pub fn master_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `web_server_ipv4_cidr_block` after provisioning.\n"]
    pub fn web_server_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_server_ipv4_cidr_block", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_creation_schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
}
impl DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_creation_schedule`.\n"]
    pub fn set_snapshot_creation_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot_creation_schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_location`.\n"]
    pub fn set_snapshot_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot_location = Some(v.into());
        self
    }
    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    type O =
        BlockAssignable<DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {}
impl BuildDataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    pub fn build(
        self,
    ) -> DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
        DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
            enabled: core::default::Default::default(),
            snapshot_creation_schedule: core::default::Default::default(),
            snapshot_location: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
        DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `snapshot_creation_schedule` after provisioning.\n"]
    pub fn snapshot_creation_schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_creation_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_location` after provisioning.\n"]
    pub fn snapshot_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElRecoveryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_snapshots_config: Option<
        ListField<DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl>,
    >,
}
impl DataComposerEnvironmentConfigElRecoveryConfigEl {
    #[doc = "Set the field `scheduled_snapshots_config`.\n"]
    pub fn set_scheduled_snapshots_config(
        mut self,
        v: impl Into<
            ListField<DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl>,
        >,
    ) -> Self {
        self.scheduled_snapshots_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElRecoveryConfigEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElRecoveryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElRecoveryConfigEl {}
impl BuildDataComposerEnvironmentConfigElRecoveryConfigEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElRecoveryConfigEl {
        DataComposerEnvironmentConfigElRecoveryConfigEl {
            scheduled_snapshots_config: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElRecoveryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElRecoveryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElRecoveryConfigElRef {
        DataComposerEnvironmentConfigElRecoveryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElRecoveryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scheduled_snapshots_config` after provisioning.\n"]
    pub fn scheduled_snapshots_config(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduled_snapshots_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl
{
    type O = BlockAssignable<
        DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {}
impl BuildDataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {
    pub fn build(
        self,
    ) -> DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {
        DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationElRef {
        DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElSoftwareConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    airflow_config_overrides: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_data_lineage_integration: Option<
        ListField<DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    env_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pypi_packages: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduler_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_server_plugins_mode: Option<PrimField<String>>,
}
impl DataComposerEnvironmentConfigElSoftwareConfigEl {
    #[doc = "Set the field `airflow_config_overrides`.\n"]
    pub fn set_airflow_config_overrides(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.airflow_config_overrides = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_data_lineage_integration`.\n"]
    pub fn set_cloud_data_lineage_integration(
        mut self,
        v: impl Into<
            ListField<DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl>,
        >,
    ) -> Self {
        self.cloud_data_lineage_integration = Some(v.into());
        self
    }
    #[doc = "Set the field `env_variables`.\n"]
    pub fn set_env_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.env_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `image_version`.\n"]
    pub fn set_image_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_version = Some(v.into());
        self
    }
    #[doc = "Set the field `pypi_packages`.\n"]
    pub fn set_pypi_packages(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.pypi_packages = Some(v.into());
        self
    }
    #[doc = "Set the field `python_version`.\n"]
    pub fn set_python_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_version = Some(v.into());
        self
    }
    #[doc = "Set the field `scheduler_count`.\n"]
    pub fn set_scheduler_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scheduler_count = Some(v.into());
        self
    }
    #[doc = "Set the field `web_server_plugins_mode`.\n"]
    pub fn set_web_server_plugins_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_server_plugins_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElSoftwareConfigEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElSoftwareConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElSoftwareConfigEl {}
impl BuildDataComposerEnvironmentConfigElSoftwareConfigEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElSoftwareConfigEl {
        DataComposerEnvironmentConfigElSoftwareConfigEl {
            airflow_config_overrides: core::default::Default::default(),
            cloud_data_lineage_integration: core::default::Default::default(),
            env_variables: core::default::Default::default(),
            image_version: core::default::Default::default(),
            pypi_packages: core::default::Default::default(),
            python_version: core::default::Default::default(),
            scheduler_count: core::default::Default::default(),
            web_server_plugins_mode: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElSoftwareConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElSoftwareConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElSoftwareConfigElRef {
        DataComposerEnvironmentConfigElSoftwareConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElSoftwareConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `airflow_config_overrides` after provisioning.\n"]
    pub fn airflow_config_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.airflow_config_overrides", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_data_lineage_integration` after provisioning.\n"]
    pub fn cloud_data_lineage_integration(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_data_lineage_integration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `env_variables` after provisioning.\n"]
    pub fn env_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.env_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_version` after provisioning.\n"]
    pub fn image_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pypi_packages` after provisioning.\n"]
    pub fn pypi_packages(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.pypi_packages", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `python_version` after provisioning.\n"]
    pub fn python_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.python_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scheduler_count` after provisioning.\n"]
    pub fn scheduler_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduler_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `web_server_plugins_mode` after provisioning.\n"]
    pub fn web_server_plugins_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_server_plugins_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElWebServerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
}
impl DataComposerEnvironmentConfigElWebServerConfigEl {
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElWebServerConfigEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElWebServerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElWebServerConfigEl {}
impl BuildDataComposerEnvironmentConfigElWebServerConfigEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElWebServerConfigEl {
        DataComposerEnvironmentConfigElWebServerConfigEl {
            machine_type: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElWebServerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElWebServerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElWebServerConfigElRef {
        DataComposerEnvironmentConfigElWebServerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElWebServerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl
{
    type O = BlockAssignable<
        DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {}
impl BuildDataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    pub fn build(
        self,
    ) -> DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
        DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
            description: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
        DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_ip_range: Option<
        SetField<DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl>,
    >,
}
impl DataComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
    #[doc = "Set the field `allowed_ip_range`.\n"]
    pub fn set_allowed_ip_range(
        mut self,
        v: impl Into<
            SetField<
                DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl,
            >,
        >,
    ) -> Self {
        self.allowed_ip_range = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElWebServerNetworkAccessControlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElWebServerNetworkAccessControlEl {}
impl BuildDataComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
        DataComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
            allowed_ip_range: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
        DataComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_ip_range` after provisioning.\n"]
    pub fn allowed_ip_range(
        &self,
    ) -> SetRef<DataComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef>
    {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_ip_range", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_gb: Option<PrimField<f64>>,
}
impl DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_gb`.\n"]
    pub fn set_memory_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_gb`.\n"]
    pub fn set_storage_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_gb = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {}
impl BuildDataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {
        DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {
            count: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorElRef {
        DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\n"]
    pub fn memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\n"]
    pub fn storage_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_gb: Option<PrimField<f64>>,
}
impl DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_gb`.\n"]
    pub fn set_memory_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_gb`.\n"]
    pub fn set_storage_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_gb = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {}
impl BuildDataComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
        DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
            count: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
        DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\n"]
    pub fn memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\n"]
    pub fn storage_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_gb: Option<PrimField<f64>>,
}
impl DataComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_gb`.\n"]
    pub fn set_memory_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_gb = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElWorkloadsConfigElTriggererEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {}
impl BuildDataComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
        DataComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
            count: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
        DataComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\n"]
    pub fn memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_gb: Option<PrimField<f64>>,
}
impl DataComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_gb`.\n"]
    pub fn set_memory_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_gb`.\n"]
    pub fn set_storage_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_gb = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElWorkloadsConfigElWebServerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {}
impl BuildDataComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
        DataComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
        DataComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\n"]
    pub fn memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\n"]
    pub fn storage_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_gb: Option<PrimField<f64>>,
}
impl DataComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `max_count`.\n"]
    pub fn set_max_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_count = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_gb`.\n"]
    pub fn set_memory_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `min_count`.\n"]
    pub fn set_min_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_count = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_gb`.\n"]
    pub fn set_storage_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_gb = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElWorkloadsConfigElWorkerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {}
impl BuildDataComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
        DataComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
            cpu: core::default::Default::default(),
            max_count: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            min_count: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
        DataComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `max_count` after provisioning.\n"]
    pub fn max_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_count", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\n"]
    pub fn memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `min_count` after provisioning.\n"]
    pub fn min_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_count", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\n"]
    pub fn storage_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigElWorkloadsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dag_processor:
        Option<ListField<DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduler: Option<ListField<DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    triggerer: Option<ListField<DataComposerEnvironmentConfigElWorkloadsConfigElTriggererEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_server: Option<ListField<DataComposerEnvironmentConfigElWorkloadsConfigElWebServerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker: Option<ListField<DataComposerEnvironmentConfigElWorkloadsConfigElWorkerEl>>,
}
impl DataComposerEnvironmentConfigElWorkloadsConfigEl {
    #[doc = "Set the field `dag_processor`.\n"]
    pub fn set_dag_processor(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl>>,
    ) -> Self {
        self.dag_processor = Some(v.into());
        self
    }
    #[doc = "Set the field `scheduler`.\n"]
    pub fn set_scheduler(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl>>,
    ) -> Self {
        self.scheduler = Some(v.into());
        self
    }
    #[doc = "Set the field `triggerer`.\n"]
    pub fn set_triggerer(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElWorkloadsConfigElTriggererEl>>,
    ) -> Self {
        self.triggerer = Some(v.into());
        self
    }
    #[doc = "Set the field `web_server`.\n"]
    pub fn set_web_server(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElWorkloadsConfigElWebServerEl>>,
    ) -> Self {
        self.web_server = Some(v.into());
        self
    }
    #[doc = "Set the field `worker`.\n"]
    pub fn set_worker(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElWorkloadsConfigElWorkerEl>>,
    ) -> Self {
        self.worker = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigElWorkloadsConfigEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigElWorkloadsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigElWorkloadsConfigEl {}
impl BuildDataComposerEnvironmentConfigElWorkloadsConfigEl {
    pub fn build(self) -> DataComposerEnvironmentConfigElWorkloadsConfigEl {
        DataComposerEnvironmentConfigElWorkloadsConfigEl {
            dag_processor: core::default::Default::default(),
            scheduler: core::default::Default::default(),
            triggerer: core::default::Default::default(),
            web_server: core::default::Default::default(),
            worker: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElWorkloadsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElWorkloadsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComposerEnvironmentConfigElWorkloadsConfigElRef {
        DataComposerEnvironmentConfigElWorkloadsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElWorkloadsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dag_processor` after provisioning.\n"]
    pub fn dag_processor(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElWorkloadsConfigElDagProcessorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dag_processor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scheduler` after provisioning.\n"]
    pub fn scheduler(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduler", self.base))
    }
    #[doc = "Get a reference to the value of field `triggerer` after provisioning.\n"]
    pub fn triggerer(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef> {
        ListRef::new(self.shared().clone(), format!("{}.triggerer", self.base))
    }
    #[doc = "Get a reference to the value of field `web_server` after provisioning.\n"]
    pub fn web_server(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.web_server", self.base))
    }
    #[doc = "Get a reference to the value of field `worker` after provisioning.\n"]
    pub fn worker(&self) -> ListRef<DataComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    airflow_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dag_gcs_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_retention_config: Option<ListField<DataComposerEnvironmentConfigElDataRetentionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_config: Option<ListField<DataComposerEnvironmentConfigElDatabaseConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_builds_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_environment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config: Option<ListField<DataComposerEnvironmentConfigElEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window: Option<ListField<DataComposerEnvironmentConfigElMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_authorized_networks_config:
        Option<ListField<DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<ListField<DataComposerEnvironmentConfigElNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_environment_config:
        Option<ListField<DataComposerEnvironmentConfigElPrivateEnvironmentConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recovery_config: Option<ListField<DataComposerEnvironmentConfigElRecoveryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resilience_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    software_config: Option<ListField<DataComposerEnvironmentConfigElSoftwareConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_server_config: Option<ListField<DataComposerEnvironmentConfigElWebServerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_server_network_access_control:
        Option<ListField<DataComposerEnvironmentConfigElWebServerNetworkAccessControlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workloads_config: Option<ListField<DataComposerEnvironmentConfigElWorkloadsConfigEl>>,
}
impl DataComposerEnvironmentConfigEl {
    #[doc = "Set the field `airflow_uri`.\n"]
    pub fn set_airflow_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.airflow_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `dag_gcs_prefix`.\n"]
    pub fn set_dag_gcs_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dag_gcs_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `data_retention_config`.\n"]
    pub fn set_data_retention_config(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElDataRetentionConfigEl>>,
    ) -> Self {
        self.data_retention_config = Some(v.into());
        self
    }
    #[doc = "Set the field `database_config`.\n"]
    pub fn set_database_config(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElDatabaseConfigEl>>,
    ) -> Self {
        self.database_config = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_private_builds_only`.\n"]
    pub fn set_enable_private_builds_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_builds_only = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_private_environment`.\n"]
    pub fn set_enable_private_environment(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_environment = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElEncryptionConfigEl>>,
    ) -> Self {
        self.encryption_config = Some(v.into());
        self
    }
    #[doc = "Set the field `environment_size`.\n"]
    pub fn set_environment_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_size = Some(v.into());
        self
    }
    #[doc = "Set the field `gke_cluster`.\n"]
    pub fn set_gke_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gke_cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElMaintenanceWindowEl>>,
    ) -> Self {
        self.maintenance_window = Some(v.into());
        self
    }
    #[doc = "Set the field `master_authorized_networks_config`.\n"]
    pub fn set_master_authorized_networks_config(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl>>,
    ) -> Self {
        self.master_authorized_networks_config = Some(v.into());
        self
    }
    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElNodeConfigEl>>,
    ) -> Self {
        self.node_config = Some(v.into());
        self
    }
    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `private_environment_config`.\n"]
    pub fn set_private_environment_config(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElPrivateEnvironmentConfigEl>>,
    ) -> Self {
        self.private_environment_config = Some(v.into());
        self
    }
    #[doc = "Set the field `recovery_config`.\n"]
    pub fn set_recovery_config(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElRecoveryConfigEl>>,
    ) -> Self {
        self.recovery_config = Some(v.into());
        self
    }
    #[doc = "Set the field `resilience_mode`.\n"]
    pub fn set_resilience_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resilience_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `software_config`.\n"]
    pub fn set_software_config(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElSoftwareConfigEl>>,
    ) -> Self {
        self.software_config = Some(v.into());
        self
    }
    #[doc = "Set the field `web_server_config`.\n"]
    pub fn set_web_server_config(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElWebServerConfigEl>>,
    ) -> Self {
        self.web_server_config = Some(v.into());
        self
    }
    #[doc = "Set the field `web_server_network_access_control`.\n"]
    pub fn set_web_server_network_access_control(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElWebServerNetworkAccessControlEl>>,
    ) -> Self {
        self.web_server_network_access_control = Some(v.into());
        self
    }
    #[doc = "Set the field `workloads_config`.\n"]
    pub fn set_workloads_config(
        mut self,
        v: impl Into<ListField<DataComposerEnvironmentConfigElWorkloadsConfigEl>>,
    ) -> Self {
        self.workloads_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentConfigEl {
    type O = BlockAssignable<DataComposerEnvironmentConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentConfigEl {}
impl BuildDataComposerEnvironmentConfigEl {
    pub fn build(self) -> DataComposerEnvironmentConfigEl {
        DataComposerEnvironmentConfigEl {
            airflow_uri: core::default::Default::default(),
            dag_gcs_prefix: core::default::Default::default(),
            data_retention_config: core::default::Default::default(),
            database_config: core::default::Default::default(),
            enable_private_builds_only: core::default::Default::default(),
            enable_private_environment: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            environment_size: core::default::Default::default(),
            gke_cluster: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            master_authorized_networks_config: core::default::Default::default(),
            node_config: core::default::Default::default(),
            node_count: core::default::Default::default(),
            private_environment_config: core::default::Default::default(),
            recovery_config: core::default::Default::default(),
            resilience_mode: core::default::Default::default(),
            software_config: core::default::Default::default(),
            web_server_config: core::default::Default::default(),
            web_server_network_access_control: core::default::Default::default(),
            workloads_config: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComposerEnvironmentConfigElRef {
        DataComposerEnvironmentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `airflow_uri` after provisioning.\n"]
    pub fn airflow_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.airflow_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `dag_gcs_prefix` after provisioning.\n"]
    pub fn dag_gcs_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dag_gcs_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_retention_config` after provisioning.\n"]
    pub fn data_retention_config(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElDataRetentionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_retention_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database_config` after provisioning.\n"]
    pub fn database_config(&self) -> ListRef<DataComposerEnvironmentConfigElDatabaseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_private_builds_only` after provisioning.\n"]
    pub fn enable_private_builds_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_builds_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_private_environment` after provisioning.\n"]
    pub fn enable_private_environment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_environment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment_size` after provisioning.\n"]
    pub fn environment_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gke_cluster` after provisioning.\n"]
    pub fn gke_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gke_cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\n"]
    pub fn master_authorized_networks_config(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.master_authorized_networks_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<DataComposerEnvironmentConfigElNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
    #[doc = "Get a reference to the value of field `private_environment_config` after provisioning.\n"]
    pub fn private_environment_config(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElPrivateEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_environment_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `recovery_config` after provisioning.\n"]
    pub fn recovery_config(&self) -> ListRef<DataComposerEnvironmentConfigElRecoveryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recovery_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resilience_mode` after provisioning.\n"]
    pub fn resilience_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resilience_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> ListRef<DataComposerEnvironmentConfigElSoftwareConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.software_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `web_server_config` after provisioning.\n"]
    pub fn web_server_config(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElWebServerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.web_server_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `web_server_network_access_control` after provisioning.\n"]
    pub fn web_server_network_access_control(
        &self,
    ) -> ListRef<DataComposerEnvironmentConfigElWebServerNetworkAccessControlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.web_server_network_access_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workloads_config` after provisioning.\n"]
    pub fn workloads_config(&self) -> ListRef<DataComposerEnvironmentConfigElWorkloadsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workloads_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComposerEnvironmentStorageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
}
impl DataComposerEnvironmentStorageConfigEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerEnvironmentStorageConfigEl {
    type O = BlockAssignable<DataComposerEnvironmentStorageConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerEnvironmentStorageConfigEl {}
impl BuildDataComposerEnvironmentStorageConfigEl {
    pub fn build(self) -> DataComposerEnvironmentStorageConfigEl {
        DataComposerEnvironmentStorageConfigEl {
            bucket: core::default::Default::default(),
        }
    }
}
pub struct DataComposerEnvironmentStorageConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerEnvironmentStorageConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComposerEnvironmentStorageConfigElRef {
        DataComposerEnvironmentStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerEnvironmentStorageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}
