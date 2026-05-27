use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComposerEnvironmentData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<ComposerEnvironmentConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_config: Option<Vec<ComposerEnvironmentStorageConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComposerEnvironmentTimeoutsEl>,
    dynamic: ComposerEnvironmentDynamic,
}
struct ComposerEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComposerEnvironmentData>,
}
#[derive(Clone)]
pub struct ComposerEnvironment(Rc<ComposerEnvironment_>);
impl ComposerEnvironment {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: [a-z]([-a-z0-9]*[a-z0-9])?. Label values must be between 0 and 63 characters long and must conform to the regular expression ([a-z]([-a-z0-9]*[a-z0-9])?)?. No more than 64 labels can be associated with a given environment. Both keys and values must be <= 128 bytes in size.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
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
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl Into<BlockAssignable<ComposerEnvironmentConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `storage_config`.\n"]
    pub fn set_storage_config(
        self,
        v: impl Into<BlockAssignable<ComposerEnvironmentStorageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComposerEnvironmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<ComposerEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> ListRef<ComposerEnvironmentStorageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComposerEnvironmentTimeoutsElRef {
        ComposerEnvironmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComposerEnvironment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComposerEnvironment {}
impl ToListMappable for ComposerEnvironment {
    type O = ListRef<ComposerEnvironmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComposerEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "google_composer_environment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComposerEnvironment {
    pub tf_id: String,
    #[doc = "Name of the environment."]
    pub name: PrimField<String>,
}
impl BuildComposerEnvironment {
    pub fn build(self, stack: &mut Stack) -> ComposerEnvironment {
        let out = ComposerEnvironment(Rc::new(ComposerEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComposerEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                config: core::default::Default::default(),
                storage_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComposerEnvironmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComposerEnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<ComposerEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> ListRef<ComposerEnvironmentStorageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComposerEnvironmentTimeoutsElRef {
        ComposerEnvironmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_mode: Option<PrimField<String>>,
}
impl ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl {
    #[doc = "Set the field `retention_days`.\nHow many days data should be retained for. This field is supported for Cloud Composer environments in composer 3 and newer."]
    pub fn set_retention_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retention_days = Some(v.into());
        self
    }
    #[doc = "Set the field `retention_mode`.\nWhether database retention is enabled or not. This field is supported for Cloud Composer environments in composer 3 and newer."]
    pub fn set_retention_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retention_mode = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl
{
    type O = BlockAssignable<
        ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl {
}
impl BuildComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl {
    pub fn build(
        self,
    ) -> ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl {
        ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl {
            retention_days: core::default::Default::default(),
            retention_mode: core::default::Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigElRef {
        ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `retention_days` after provisioning.\nHow many days data should be retained for. This field is supported for Cloud Composer environments in composer 3 and newer."]
    pub fn retention_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retention_mode` after provisioning.\nWhether database retention is enabled or not. This field is supported for Cloud Composer environments in composer 3 and newer."]
    pub fn retention_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_mode: Option<PrimField<String>>,
}
impl ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {
    #[doc = "Set the field `storage_mode`.\nWhether logs in cloud logging only is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.0.32-airflow-2.1.4 and newer but not in composer-3*"]
    pub fn set_storage_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_mode = Some(v.into());
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {
    type O =
        BlockAssignable<ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {}
impl BuildComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {
    pub fn build(
        self,
    ) -> ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {
        ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl {
            storage_mode: core::default::Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigElRef {
        ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `storage_mode` after provisioning.\nWhether logs in cloud logging only is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.0.32-airflow-2.1.4 and newer but not in composer-3*"]
    pub fn storage_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_mode", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElDataRetentionConfigElDynamic {
    airflow_metadata_retention_config: Option<
        DynamicBlock<
            ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl,
        >,
    >,
    task_logs_retention_config: Option<
        DynamicBlock<ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElDataRetentionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    airflow_metadata_retention_config: Option<
        Vec<ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_logs_retention_config:
        Option<Vec<ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl>>,
    dynamic: ComposerEnvironmentConfigElDataRetentionConfigElDynamic,
}
impl ComposerEnvironmentConfigElDataRetentionConfigEl {
    #[doc = "Set the field `airflow_metadata_retention_config`.\n"]
    pub fn set_airflow_metadata_retention_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.airflow_metadata_retention_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.airflow_metadata_retention_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `task_logs_retention_config`.\n"]
    pub fn set_task_logs_retention_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.task_logs_retention_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.task_logs_retention_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElDataRetentionConfigEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElDataRetentionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElDataRetentionConfigEl {}
impl BuildComposerEnvironmentConfigElDataRetentionConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElDataRetentionConfigEl {
        ComposerEnvironmentConfigElDataRetentionConfigEl {
            airflow_metadata_retention_config: core::default::Default::default(),
            task_logs_retention_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElDataRetentionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElDataRetentionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElDataRetentionConfigElRef {
        ComposerEnvironmentConfigElDataRetentionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElDataRetentionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `airflow_metadata_retention_config` after provisioning.\n"]
    pub fn airflow_metadata_retention_config(
        &self,
    ) -> ListRef<ComposerEnvironmentConfigElDataRetentionConfigElAirflowMetadataRetentionConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.airflow_metadata_retention_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `task_logs_retention_config` after provisioning.\n"]
    pub fn task_logs_retention_config(
        &self,
    ) -> ListRef<ComposerEnvironmentConfigElDataRetentionConfigElTaskLogsRetentionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.task_logs_retention_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElDatabaseConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl ComposerEnvironmentConfigElDatabaseConfigEl {
    #[doc = "Set the field `machine_type`.\nOptional. Cloud SQL machine type used by Airflow database. It has to be one of: db-n1-standard-2, db-n1-standard-4, db-n1-standard-8 or db-n1-standard-16. If not specified, db-n1-standard-2 will be used."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nOptional. Cloud SQL database preferred zone."]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElDatabaseConfigEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElDatabaseConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElDatabaseConfigEl {}
impl BuildComposerEnvironmentConfigElDatabaseConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElDatabaseConfigEl {
        ComposerEnvironmentConfigElDatabaseConfigEl {
            machine_type: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElDatabaseConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElDatabaseConfigElRef {
    fn new(shared: StackShared, base: String) -> ComposerEnvironmentConfigElDatabaseConfigElRef {
        ComposerEnvironmentConfigElDatabaseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElDatabaseConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nOptional. Cloud SQL machine type used by Airflow database. It has to be one of: db-n1-standard-2, db-n1-standard-4, db-n1-standard-8 or db-n1-standard-16. If not specified, db-n1-standard-2 will be used."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nOptional. Cloud SQL database preferred zone."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElEncryptionConfigEl {
    kms_key_name: PrimField<String>,
}
impl ComposerEnvironmentConfigElEncryptionConfigEl {}
impl ToListMappable for ComposerEnvironmentConfigElEncryptionConfigEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElEncryptionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElEncryptionConfigEl {
    #[doc = "Optional. Customer-managed Encryption Key available through Google's Key Management Service. Cannot be updated."]
    pub kms_key_name: PrimField<String>,
}
impl BuildComposerEnvironmentConfigElEncryptionConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElEncryptionConfigEl {
        ComposerEnvironmentConfigElEncryptionConfigEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct ComposerEnvironmentConfigElEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElEncryptionConfigElRef {
    fn new(shared: StackShared, base: String) -> ComposerEnvironmentConfigElEncryptionConfigElRef {
        ComposerEnvironmentConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nOptional. Customer-managed Encryption Key available through Google's Key Management Service. Cannot be updated."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElMaintenanceWindowEl {
    end_time: PrimField<String>,
    recurrence: PrimField<String>,
    start_time: PrimField<String>,
}
impl ComposerEnvironmentConfigElMaintenanceWindowEl {}
impl ToListMappable for ComposerEnvironmentConfigElMaintenanceWindowEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElMaintenanceWindowEl {
    #[doc = "Maintenance window end time. It is used only to calculate the duration of the maintenance window. The value for end-time must be in the future, relative to 'start_time'."]
    pub end_time: PrimField<String>,
    #[doc = "Maintenance window recurrence. Format is a subset of RFC-5545 (https://tools.ietf.org/html/rfc5545) 'RRULE'. The only allowed values for 'FREQ' field are 'FREQ=DAILY' and 'FREQ=WEEKLY;BYDAY=...'. Example values: 'FREQ=WEEKLY;BYDAY=TU,WE', 'FREQ=DAILY'."]
    pub recurrence: PrimField<String>,
    #[doc = "Start time of the first recurrence of the maintenance window."]
    pub start_time: PrimField<String>,
}
impl BuildComposerEnvironmentConfigElMaintenanceWindowEl {
    pub fn build(self) -> ComposerEnvironmentConfigElMaintenanceWindowEl {
        ComposerEnvironmentConfigElMaintenanceWindowEl {
            end_time: self.end_time,
            recurrence: self.recurrence,
            start_time: self.start_time,
        }
    }
}
pub struct ComposerEnvironmentConfigElMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElMaintenanceWindowElRef {
    fn new(shared: StackShared, base: String) -> ComposerEnvironmentConfigElMaintenanceWindowElRef {
        ComposerEnvironmentConfigElMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nMaintenance window end time. It is used only to calculate the duration of the maintenance window. The value for end-time must be in the future, relative to 'start_time'."]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\nMaintenance window recurrence. Format is a subset of RFC-5545 (https://tools.ietf.org/html/rfc5545) 'RRULE'. The only allowed values for 'FREQ' field are 'FREQ=DAILY' and 'FREQ=WEEKLY;BYDAY=...'. Example values: 'FREQ=WEEKLY;BYDAY=TU,WE', 'FREQ=DAILY'."]
    pub fn recurrence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nStart time of the first recurrence of the maintenance window."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    cidr_block: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
}
impl ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    #[doc = "Set the field `display_name`.\ndisplay_name is a field for users to identify CIDR blocks."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    type O =
        BlockAssignable<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    #[doc = "cidr_block must be specified in CIDR notation."]
    pub cidr_block: PrimField<String>,
}
impl BuildComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
    pub fn build(self) -> ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
        ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl {
            cidr_block: self.cidr_block,
            display_name: core::default::Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
        ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\ncidr_block must be specified in CIDR notation."]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\ndisplay_name is a field for users to identify CIDR blocks."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElDynamic {
    cidr_blocks: Option<
        DynamicBlock<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl>,
    >,
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_blocks:
        Option<Vec<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl>>,
    dynamic: ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElDynamic,
}
impl ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    #[doc = "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElCidrBlocksEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cidr_blocks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cidr_blocks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    #[doc = "Whether or not master authorized networks is enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
        ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl {
            enabled: self.enabled,
            cidr_blocks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
        ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not master authorized networks is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
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
impl ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
    #[doc = "Set the field `cluster_ipv4_cidr_block`.\nThe IP address range used to allocate IP addresses to pods in the cluster. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. /14) to have GKE choose a range with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use. Specify either cluster_secondary_range_name or cluster_ipv4_cidr_block but not both."]
    pub fn set_cluster_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_secondary_range_name`.\nThe name of the cluster's secondary range used to allocate IP addresses to pods. Specify either cluster_secondary_range_name or cluster_ipv4_cidr_block but not both. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true."]
    pub fn set_cluster_secondary_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_secondary_range_name = Some(v.into());
        self
    }
    #[doc = "Set the field `services_ipv4_cidr_block`.\nThe IP address range used to allocate IP addresses in this cluster. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. /14) to have GKE choose a range with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use. Specify either services_secondary_range_name or services_ipv4_cidr_block but not both."]
    pub fn set_services_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.services_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `services_secondary_range_name`.\nThe name of the services' secondary range used to allocate IP addresses to the cluster. Specify either services_secondary_range_name or services_ipv4_cidr_block but not both. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true."]
    pub fn set_services_secondary_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.services_secondary_range_name = Some(v.into());
        self
    }
    #[doc = "Set the field `use_ip_aliases`.\nWhether or not to enable Alias IPs in the GKE cluster. If true, a VPC-native cluster is created. Defaults to true if the ip_allocation_policy block is present in config. This field is only supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*. Environments in newer versions always use VPC-native GKE clusters."]
    pub fn set_use_ip_aliases(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_ip_aliases = Some(v.into());
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {}
impl BuildComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
    pub fn build(self) -> ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
        ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl {
            cluster_ipv4_cidr_block: core::default::Default::default(),
            cluster_secondary_range_name: core::default::Default::default(),
            services_ipv4_cidr_block: core::default::Default::default(),
            services_secondary_range_name: core::default::Default::default(),
            use_ip_aliases: core::default::Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
        ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_ipv4_cidr_block` after provisioning.\nThe IP address range used to allocate IP addresses to pods in the cluster. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. /14) to have GKE choose a range with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use. Specify either cluster_secondary_range_name or cluster_ipv4_cidr_block but not both."]
    pub fn cluster_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_secondary_range_name` after provisioning.\nThe name of the cluster's secondary range used to allocate IP addresses to pods. Specify either cluster_secondary_range_name or cluster_ipv4_cidr_block but not both. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true."]
    pub fn cluster_secondary_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_secondary_range_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `services_ipv4_cidr_block` after provisioning.\nThe IP address range used to allocate IP addresses in this cluster. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. /14) to have GKE choose a range with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use. Specify either services_secondary_range_name or services_ipv4_cidr_block but not both."]
    pub fn services_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.services_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `services_secondary_range_name` after provisioning.\nThe name of the services' secondary range used to allocate IP addresses to the cluster. Specify either services_secondary_range_name or services_ipv4_cidr_block but not both. For Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*, this field is applicable only when use_ip_aliases is true."]
    pub fn services_secondary_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.services_secondary_range_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_ip_aliases` after provisioning.\nWhether or not to enable Alias IPs in the GKE cluster. If true, a VPC-native cluster is created. Defaults to true if the ip_allocation_policy block is present in config. This field is only supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*. Environments in newer versions always use VPC-native GKE clusters."]
    pub fn use_ip_aliases(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_ip_aliases", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElNodeConfigElDynamic {
    ip_allocation_policy:
        Option<DynamicBlock<ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl>>,
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    composer_internal_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    composer_network_attachment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ip_masq_agent: Option<PrimField<bool>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_allocation_policy: Option<Vec<ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl>>,
    dynamic: ComposerEnvironmentConfigElNodeConfigElDynamic,
}
impl ComposerEnvironmentConfigElNodeConfigEl {
    #[doc = "Set the field `composer_internal_ipv4_cidr_block`.\nIPv4 cidr range that will be used by Composer internal components."]
    pub fn set_composer_internal_ipv4_cidr_block(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.composer_internal_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `composer_network_attachment`.\nPSC (Private Service Connect) Network entry point. Customers can pre-create the Network Attachment and point Cloud Composer environment to use. It is possible to share network attachment among many environments, provided enough IP addresses are available."]
    pub fn set_composer_network_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.composer_network_attachment = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\nThe disk size in GB used for node VMs. Minimum size is 20GB. If unspecified, defaults to 100GB. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_ip_masq_agent`.\nDeploys 'ip-masq-agent' daemon set in the GKE cluster and defines nonMasqueradeCIDRs equals to pod IP range so IP masquerading is used for all destination addresses, except between pods traffic. See: https://cloud.google.com/kubernetes-engine/docs/how-to/ip-masquerade-agent"]
    pub fn set_enable_ip_masq_agent(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ip_masq_agent = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: \"projects/{project}/zones/{zone}/machineTypes/{machineType}\". Must belong to the enclosing environment's project and region/zone. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: \"projects/{project}/zones/{zone}/machineTypes/{machineType}\". Must belong to the enclosing environment's project and region/zone. The network must belong to the environment's project. If unspecified, the \"default\" network ID in the environment's project is used. If a Custom Subnet Network is provided, subnetwork must also be provided."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_scopes`.\nThe set of Google API scopes to be made available on all node VMs. Cannot be updated. If empty, defaults to [\"https://www.googleapis.com/auth/cloud-platform\"]. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_oauth_scopes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.oauth_scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nThe Google Cloud Platform Service Account to be used by the node VMs. If a service account is not specified, the \"default\" Compute Engine service account is used. Cannot be updated. If given, note that the service account must have roles/composer.worker for any GCP resources created under the Cloud Composer Environment."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThe Compute Engine subnetwork to be used for machine communications, specified as a self-link, relative resource name (e.g. \"projects/{project}/regions/{region}/subnetworks/{subnetwork}\"), or by name. If subnetwork is provided, network must also be provided and the subnetwork must belong to the enclosing environment's project and region."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nThe list of instance tags applied to all node VMs. Tags are used to identify valid sources or targets for network firewalls. Each tag within the list must comply with RFC1035. Cannot be updated."]
    pub fn set_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe Compute Engine zone in which to deploy the VMs running the Apache Airflow software, specified as the zone name or relative resource name (e.g. \"projects/{project}/zones/{zone}\"). Must belong to the enclosing environment's project and region. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_allocation_policy`.\n"]
    pub fn set_ip_allocation_policy(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_allocation_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_allocation_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElNodeConfigEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElNodeConfigEl {}
impl BuildComposerEnvironmentConfigElNodeConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElNodeConfigEl {
        ComposerEnvironmentConfigElNodeConfigEl {
            composer_internal_ipv4_cidr_block: core::default::Default::default(),
            composer_network_attachment: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            enable_ip_masq_agent: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            network: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            service_account: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
            zone: core::default::Default::default(),
            ip_allocation_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> ComposerEnvironmentConfigElNodeConfigElRef {
        ComposerEnvironmentConfigElNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `composer_internal_ipv4_cidr_block` after provisioning.\nIPv4 cidr range that will be used by Composer internal components."]
    pub fn composer_internal_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.composer_internal_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `composer_network_attachment` after provisioning.\nPSC (Private Service Connect) Network entry point. Customers can pre-create the Network Attachment and point Cloud Composer environment to use. It is possible to share network attachment among many environments, provided enough IP addresses are available."]
    pub fn composer_network_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.composer_network_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nThe disk size in GB used for node VMs. Minimum size is 20GB. If unspecified, defaults to 100GB. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_ip_masq_agent` after provisioning.\nDeploys 'ip-masq-agent' daemon set in the GKE cluster and defines nonMasqueradeCIDRs equals to pod IP range so IP masquerading is used for all destination addresses, except between pods traffic. See: https://cloud.google.com/kubernetes-engine/docs/how-to/ip-masquerade-agent"]
    pub fn enable_ip_masq_agent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ip_masq_agent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: \"projects/{project}/zones/{zone}/machineTypes/{machineType}\". Must belong to the enclosing environment's project and region/zone. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe Compute Engine machine type used for cluster instances, specified as a name or relative resource name. For example: \"projects/{project}/zones/{zone}/machineTypes/{machineType}\". Must belong to the enclosing environment's project and region/zone. The network must belong to the environment's project. If unspecified, the \"default\" network ID in the environment's project is used. If a Custom Subnet Network is provided, subnetwork must also be provided."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth_scopes` after provisioning.\nThe set of Google API scopes to be made available on all node VMs. Cannot be updated. If empty, defaults to [\"https://www.googleapis.com/auth/cloud-platform\"]. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn oauth_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe Google Cloud Platform Service Account to be used by the node VMs. If a service account is not specified, the \"default\" Compute Engine service account is used. Cannot be updated. If given, note that the service account must have roles/composer.worker for any GCP resources created under the Cloud Composer Environment."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe Compute Engine subnetwork to be used for machine communications, specified as a self-link, relative resource name (e.g. \"projects/{project}/regions/{region}/subnetworks/{subnetwork}\"), or by name. If subnetwork is provided, network must also be provided and the subnetwork must belong to the enclosing environment's project and region."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nThe list of instance tags applied to all node VMs. Tags are used to identify valid sources or targets for network firewalls. Each tag within the list must comply with RFC1035. Cannot be updated."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe Compute Engine zone in which to deploy the VMs running the Apache Airflow software, specified as the zone name or relative resource name (e.g. \"projects/{project}/zones/{zone}\"). Must belong to the enclosing environment's project and region. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_allocation_policy` after provisioning.\n"]
    pub fn ip_allocation_policy(
        &self,
    ) -> ListRef<ComposerEnvironmentConfigElNodeConfigElIpAllocationPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_allocation_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
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
impl ComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
    #[doc = "Set the field `cloud_composer_connection_subnetwork`.\nWhen specified, the environment will use Private Service Connect instead of VPC peerings to connect to Cloud SQL in the Tenant Project, and the PSC endpoint in the Customer Project will use an IP address from this subnetwork. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn set_cloud_composer_connection_subnetwork(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.cloud_composer_connection_subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_composer_network_ipv4_cidr_block`.\nThe CIDR block from which IP range for Cloud Composer Network in tenant project will be reserved. Needs to be disjoint from private_cluster_config.master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn set_cloud_composer_network_ipv4_cidr_block(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.cloud_composer_network_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_sql_ipv4_cidr_block`.\nThe CIDR block from which IP range in tenant project will be reserved for Cloud SQL. Needs to be disjoint from web_server_ipv4_cidr_block."]
    pub fn set_cloud_sql_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_sql_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_type`.\nMode of internal communication within the Composer environment. Must be one of \"VPC_PEERING\" or \"PRIVATE_SERVICE_CONNECT\"."]
    pub fn set_connection_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_type = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_private_endpoint`.\nIf true, access to the public endpoint of the GKE cluster is denied. If this field is set to true, ip_allocation_policy.use_ip_aliases must be set to true for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_enable_private_endpoint(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_privately_used_public_ips`.\nWhen enabled, IPs from public (non-RFC1918) ranges can be used for ip_allocation_policy.cluster_ipv4_cidr_block and ip_allocation_policy.service_ipv4_cidr_block."]
    pub fn set_enable_privately_used_public_ips(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_privately_used_public_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `master_ipv4_cidr_block`.\nThe IP range in CIDR notation to use for the hosted master network. This range is used for assigning internal IP addresses to the cluster master or set of masters and to the internal load balancer virtual IP. This range must not overlap with any other ranges in use within the cluster's network. If left blank, the default value of '172.16.0.0/28' is used."]
    pub fn set_master_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.master_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `web_server_ipv4_cidr_block`.\nThe CIDR block from which IP range for web server will be reserved. Needs to be disjoint from master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_web_server_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_server_ipv4_cidr_block = Some(v.into());
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElPrivateEnvironmentConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElPrivateEnvironmentConfigEl {}
impl BuildComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
        ComposerEnvironmentConfigElPrivateEnvironmentConfigEl {
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
pub struct ComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
        ComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElPrivateEnvironmentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_composer_connection_subnetwork` after provisioning.\nWhen specified, the environment will use Private Service Connect instead of VPC peerings to connect to Cloud SQL in the Tenant Project, and the PSC endpoint in the Customer Project will use an IP address from this subnetwork. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn cloud_composer_connection_subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_composer_connection_subnetwork", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_composer_network_ipv4_cidr_block` after provisioning.\nThe CIDR block from which IP range for Cloud Composer Network in tenant project will be reserved. Needs to be disjoint from private_cluster_config.master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn cloud_composer_network_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_composer_network_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_sql_ipv4_cidr_block` after provisioning.\nThe CIDR block from which IP range in tenant project will be reserved for Cloud SQL. Needs to be disjoint from web_server_ipv4_cidr_block."]
    pub fn cloud_sql_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_sql_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\nMode of internal communication within the Composer environment. Must be one of \"VPC_PEERING\" or \"PRIVATE_SERVICE_CONNECT\"."]
    pub fn connection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_private_endpoint` after provisioning.\nIf true, access to the public endpoint of the GKE cluster is denied. If this field is set to true, ip_allocation_policy.use_ip_aliases must be set to true for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn enable_private_endpoint(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_privately_used_public_ips` after provisioning.\nWhen enabled, IPs from public (non-RFC1918) ranges can be used for ip_allocation_policy.cluster_ipv4_cidr_block and ip_allocation_policy.service_ipv4_cidr_block."]
    pub fn enable_privately_used_public_ips(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_privately_used_public_ips", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `master_ipv4_cidr_block` after provisioning.\nThe IP range in CIDR notation to use for the hosted master network. This range is used for assigning internal IP addresses to the cluster master or set of masters and to the internal load balancer virtual IP. This range must not overlap with any other ranges in use within the cluster's network. If left blank, the default value of '172.16.0.0/28' is used."]
    pub fn master_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `web_server_ipv4_cidr_block` after provisioning.\nThe CIDR block from which IP range for web server will be reserved. Needs to be disjoint from master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn web_server_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_server_ipv4_cidr_block", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_creation_schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
}
impl ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    #[doc = "Set the field `snapshot_creation_schedule`.\nSnapshot schedule, in the unix-cron format."]
    pub fn set_snapshot_creation_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot_creation_schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `snapshot_location`.\nthe URI of a bucket folder where to save the snapshot."]
    pub fn set_snapshot_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snapshot_location = Some(v.into());
        self
    }
    #[doc = "Set the field `time_zone`.\nA time zone for the schedule. This value is a time offset and does not take into account daylight saving time changes. Valid values are from UTC-12 to UTC+12. Examples: UTC, UTC-01, UTC+03."]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    #[doc = "When enabled, Cloud Composer periodically saves snapshots of your environment to a Cloud Storage bucket."]
    pub enabled: PrimField<bool>,
}
impl BuildComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
        ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl {
            enabled: self.enabled,
            snapshot_creation_schedule: core::default::Default::default(),
            snapshot_location: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
        ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhen enabled, Cloud Composer periodically saves snapshots of your environment to a Cloud Storage bucket."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `snapshot_creation_schedule` after provisioning.\nSnapshot schedule, in the unix-cron format."]
    pub fn snapshot_creation_schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_creation_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_location` after provisioning.\nthe URI of a bucket folder where to save the snapshot."]
    pub fn snapshot_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\nA time zone for the schedule. This value is a time offset and does not take into account daylight saving time changes. Valid values are from UTC-12 to UTC+12. Examples: UTC, UTC-01, UTC+03."]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElRecoveryConfigElDynamic {
    scheduled_snapshots_config:
        Option<DynamicBlock<ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl>>,
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElRecoveryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_snapshots_config:
        Option<Vec<ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl>>,
    dynamic: ComposerEnvironmentConfigElRecoveryConfigElDynamic,
}
impl ComposerEnvironmentConfigElRecoveryConfigEl {
    #[doc = "Set the field `scheduled_snapshots_config`.\n"]
    pub fn set_scheduled_snapshots_config(
        mut self,
        v: impl Into<
            BlockAssignable<ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scheduled_snapshots_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduled_snapshots_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElRecoveryConfigEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElRecoveryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElRecoveryConfigEl {}
impl BuildComposerEnvironmentConfigElRecoveryConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElRecoveryConfigEl {
        ComposerEnvironmentConfigElRecoveryConfigEl {
            scheduled_snapshots_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElRecoveryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElRecoveryConfigElRef {
    fn new(shared: StackShared, base: String) -> ComposerEnvironmentConfigElRecoveryConfigElRef {
        ComposerEnvironmentConfigElRecoveryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElRecoveryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scheduled_snapshots_config` after provisioning.\n"]
    pub fn scheduled_snapshots_config(
        &self,
    ) -> ListRef<ComposerEnvironmentConfigElRecoveryConfigElScheduledSnapshotsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduled_snapshots_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {
    enabled: PrimField<bool>,
}
impl ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {}
impl ToListMappable for ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {
    type O =
        BlockAssignable<ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {
    #[doc = "Whether or not Cloud Data Lineage integration is enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {
    pub fn build(self) -> ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {
        ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl {
            enabled: self.enabled,
        }
    }
}
pub struct ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationElRef {
        ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether or not Cloud Data Lineage integration is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElSoftwareConfigElDynamic {
    cloud_data_lineage_integration: Option<
        DynamicBlock<ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl>,
    >,
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElSoftwareConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    airflow_config_overrides: Option<RecField<PrimField<String>>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_data_lineage_integration:
        Option<Vec<ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl>>,
    dynamic: ComposerEnvironmentConfigElSoftwareConfigElDynamic,
}
impl ComposerEnvironmentConfigElSoftwareConfigEl {
    #[doc = "Set the field `airflow_config_overrides`.\nApache Airflow configuration properties to override. Property keys contain the section and property names, separated by a hyphen, for example \"core-dags_are_paused_at_creation\". Section names must not contain hyphens (\"-\"), opening square brackets (\"[\"), or closing square brackets (\"]\"). The property name must not be empty and cannot contain \"=\" or \";\". Section and property names cannot contain characters: \".\" Apache Airflow configuration property names must be written in snake_case. Property values can contain any character, and can be written in any lower/upper case format. Certain Apache Airflow configuration property values are blacklisted, and cannot be overridden."]
    pub fn set_airflow_config_overrides(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.airflow_config_overrides = Some(v.into());
        self
    }
    #[doc = "Set the field `env_variables`.\nAdditional environment variables to provide to the Apache Airflow scheduler, worker, and webserver processes. Environment variable names must match the regular expression [a-zA-Z_][a-zA-Z0-9_]*. They cannot specify Apache Airflow software configuration overrides (they cannot match the regular expression AIRFLOW__[A-Z0-9_]+__[A-Z0-9_]+), and they cannot match any of the following reserved names: AIRFLOW_HOME C_FORCE_ROOT CONTAINER_NAME DAGS_FOLDER GCP_PROJECT GCS_BUCKET GKE_CLUSTER_NAME SQL_DATABASE SQL_INSTANCE SQL_PASSWORD SQL_PROJECT SQL_REGION SQL_USER."]
    pub fn set_env_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.env_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `image_version`.\nThe version of the software running in the environment. This encapsulates both the version of Cloud Composer functionality and the version of Apache Airflow. It must match the regular expression composer-([0-9]+(\\.[0-9]+\\.[0-9]+(-preview\\.[0-9]+)?)?|latest)-airflow-([0-9]+(\\.[0-9]+(\\.[0-9]+)?)?). The Cloud Composer portion of the image version is a full semantic version, or an alias in the form of major version number or 'latest'. The Apache Airflow portion of the image version is a full semantic version that points to one of the supported Apache Airflow versions, or an alias in the form of only major or major.minor versions specified. See documentation for more details and version list."]
    pub fn set_image_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_version = Some(v.into());
        self
    }
    #[doc = "Set the field `pypi_packages`.\nCustom Python Package Index (PyPI) packages to be installed in the environment. Keys refer to the lowercase package name (e.g. \"numpy\"). Values are the lowercase extras and version specifier (e.g. \"==1.12.0\", \"[devel,gcp_api]\", \"[devel]>=1.8.2, <1.9.2\"). To specify a package without pinning it to a version specifier, use the empty string as the value."]
    pub fn set_pypi_packages(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.pypi_packages = Some(v.into());
        self
    }
    #[doc = "Set the field `python_version`.\nThe major version of Python used to run the Apache Airflow scheduler, worker, and webserver processes. Can be set to '2' or '3'. If not specified, the default is '2'. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*. Environments in newer versions always use Python major version 3."]
    pub fn set_python_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_version = Some(v.into());
        self
    }
    #[doc = "Set the field `scheduler_count`.\nThe number of schedulers for Airflow. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-2.*.*."]
    pub fn set_scheduler_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scheduler_count = Some(v.into());
        self
    }
    #[doc = "Set the field `web_server_plugins_mode`.\nShould be either 'ENABLED' or 'DISABLED'. Defaults to 'ENABLED'. Used in Composer 3."]
    pub fn set_web_server_plugins_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_server_plugins_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_data_lineage_integration`.\n"]
    pub fn set_cloud_data_lineage_integration(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_data_lineage_integration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_data_lineage_integration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElSoftwareConfigEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElSoftwareConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElSoftwareConfigEl {}
impl BuildComposerEnvironmentConfigElSoftwareConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElSoftwareConfigEl {
        ComposerEnvironmentConfigElSoftwareConfigEl {
            airflow_config_overrides: core::default::Default::default(),
            env_variables: core::default::Default::default(),
            image_version: core::default::Default::default(),
            pypi_packages: core::default::Default::default(),
            python_version: core::default::Default::default(),
            scheduler_count: core::default::Default::default(),
            web_server_plugins_mode: core::default::Default::default(),
            cloud_data_lineage_integration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElSoftwareConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElSoftwareConfigElRef {
    fn new(shared: StackShared, base: String) -> ComposerEnvironmentConfigElSoftwareConfigElRef {
        ComposerEnvironmentConfigElSoftwareConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElSoftwareConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `airflow_config_overrides` after provisioning.\nApache Airflow configuration properties to override. Property keys contain the section and property names, separated by a hyphen, for example \"core-dags_are_paused_at_creation\". Section names must not contain hyphens (\"-\"), opening square brackets (\"[\"), or closing square brackets (\"]\"). The property name must not be empty and cannot contain \"=\" or \";\". Section and property names cannot contain characters: \".\" Apache Airflow configuration property names must be written in snake_case. Property values can contain any character, and can be written in any lower/upper case format. Certain Apache Airflow configuration property values are blacklisted, and cannot be overridden."]
    pub fn airflow_config_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.airflow_config_overrides", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `env_variables` after provisioning.\nAdditional environment variables to provide to the Apache Airflow scheduler, worker, and webserver processes. Environment variable names must match the regular expression [a-zA-Z_][a-zA-Z0-9_]*. They cannot specify Apache Airflow software configuration overrides (they cannot match the regular expression AIRFLOW__[A-Z0-9_]+__[A-Z0-9_]+), and they cannot match any of the following reserved names: AIRFLOW_HOME C_FORCE_ROOT CONTAINER_NAME DAGS_FOLDER GCP_PROJECT GCS_BUCKET GKE_CLUSTER_NAME SQL_DATABASE SQL_INSTANCE SQL_PASSWORD SQL_PROJECT SQL_REGION SQL_USER."]
    pub fn env_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.env_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_version` after provisioning.\nThe version of the software running in the environment. This encapsulates both the version of Cloud Composer functionality and the version of Apache Airflow. It must match the regular expression composer-([0-9]+(\\.[0-9]+\\.[0-9]+(-preview\\.[0-9]+)?)?|latest)-airflow-([0-9]+(\\.[0-9]+(\\.[0-9]+)?)?). The Cloud Composer portion of the image version is a full semantic version, or an alias in the form of major version number or 'latest'. The Apache Airflow portion of the image version is a full semantic version that points to one of the supported Apache Airflow versions, or an alias in the form of only major or major.minor versions specified. See documentation for more details and version list."]
    pub fn image_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pypi_packages` after provisioning.\nCustom Python Package Index (PyPI) packages to be installed in the environment. Keys refer to the lowercase package name (e.g. \"numpy\"). Values are the lowercase extras and version specifier (e.g. \"==1.12.0\", \"[devel,gcp_api]\", \"[devel]>=1.8.2, <1.9.2\"). To specify a package without pinning it to a version specifier, use the empty string as the value."]
    pub fn pypi_packages(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.pypi_packages", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `python_version` after provisioning.\nThe major version of Python used to run the Apache Airflow scheduler, worker, and webserver processes. Can be set to '2' or '3'. If not specified, the default is '2'. Cannot be updated. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*. Environments in newer versions always use Python major version 3."]
    pub fn python_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.python_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scheduler_count` after provisioning.\nThe number of schedulers for Airflow. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-2.*.*."]
    pub fn scheduler_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduler_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `web_server_plugins_mode` after provisioning.\nShould be either 'ENABLED' or 'DISABLED'. Defaults to 'ENABLED'. Used in Composer 3."]
    pub fn web_server_plugins_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_server_plugins_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_data_lineage_integration` after provisioning.\n"]
    pub fn cloud_data_lineage_integration(
        &self,
    ) -> ListRef<ComposerEnvironmentConfigElSoftwareConfigElCloudDataLineageIntegrationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_data_lineage_integration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWebServerConfigEl {
    machine_type: PrimField<String>,
}
impl ComposerEnvironmentConfigElWebServerConfigEl {}
impl ToListMappable for ComposerEnvironmentConfigElWebServerConfigEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElWebServerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElWebServerConfigEl {
    #[doc = "Optional. Machine type on which Airflow web server is running. It has to be one of: composer-n1-webserver-2, composer-n1-webserver-4 or composer-n1-webserver-8. If not specified, composer-n1-webserver-2 will be used. Value custom is returned only in response, if Airflow web server parameters were manually changed to a non-standard values."]
    pub machine_type: PrimField<String>,
}
impl BuildComposerEnvironmentConfigElWebServerConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWebServerConfigEl {
        ComposerEnvironmentConfigElWebServerConfigEl {
            machine_type: self.machine_type,
        }
    }
}
pub struct ComposerEnvironmentConfigElWebServerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElWebServerConfigElRef {
    fn new(shared: StackShared, base: String) -> ComposerEnvironmentConfigElWebServerConfigElRef {
        ComposerEnvironmentConfigElWebServerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElWebServerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nOptional. Machine type on which Airflow web server is running. It has to be one of: composer-n1-webserver-2, composer-n1-webserver-4 or composer-n1-webserver-8. If not specified, composer-n1-webserver-2 will be used. Value custom is returned only in response, if Airflow web server parameters were manually changed to a non-standard values."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    value: PrimField<String>,
}
impl ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    #[doc = "Set the field `description`.\nA description of this ip range."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    type O =
        BlockAssignable<ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    #[doc = "IP address or range, defined using CIDR notation, of requests that this rule applies to. Examples: 192.168.1.1 or 192.168.0.0/16 or 2001:db8::/32 or 2001:0db8:0000:0042:0000:8a2e:0370:7334. IP range prefixes should be properly truncated. For example, 1.2.3.4/24 should be truncated to 1.2.3.0/24. Similarly, for IPv6, 2001:db8::1/32 should be truncated to 2001:db8::/32."]
    pub value: PrimField<String>,
}
impl BuildComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
    pub fn build(
        self,
    ) -> ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
        ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl {
            description: core::default::Default::default(),
            value: self.value,
        }
    }
}
pub struct ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
        ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of this ip range."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nIP address or range, defined using CIDR notation, of requests that this rule applies to. Examples: 192.168.1.1 or 192.168.0.0/16 or 2001:db8::/32 or 2001:0db8:0000:0042:0000:8a2e:0370:7334. IP range prefixes should be properly truncated. For example, 1.2.3.4/24 should be truncated to 1.2.3.0/24. Similarly, for IPv6, 2001:db8::1/32 should be truncated to 2001:db8::/32."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElWebServerNetworkAccessControlElDynamic {
    allowed_ip_range: Option<
        DynamicBlock<ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl>,
    >,
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_ip_range:
        Option<Vec<ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl>>,
    dynamic: ComposerEnvironmentConfigElWebServerNetworkAccessControlElDynamic,
}
impl ComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
    #[doc = "Set the field `allowed_ip_range`.\n"]
    pub fn set_allowed_ip_range(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComposerEnvironmentConfigElWebServerNetworkAccessControlElAllowedIpRangeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allowed_ip_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_ip_range = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElWebServerNetworkAccessControlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElWebServerNetworkAccessControlEl {}
impl BuildComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
        ComposerEnvironmentConfigElWebServerNetworkAccessControlEl {
            allowed_ip_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
        ComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElWebServerNetworkAccessControlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_gb: Option<PrimField<f64>>,
}
impl ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {
    #[doc = "Set the field `count`.\nNumber of DAG processors."]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu`.\nCPU request and limit for DAG processor."]
    pub fn set_cpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_gb`.\nMemory (GB) request and limit for DAG processor."]
    pub fn set_memory_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_gb`.\nStorage (GB) request and limit for DAG processor."]
    pub fn set_storage_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_gb = Some(v.into());
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {}
impl BuildComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {
        ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl {
            count: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorElRef {
        ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nNumber of DAG processors."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for DAG processor."]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for DAG processor."]
    pub fn memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\nStorage (GB) request and limit for DAG processor."]
    pub fn storage_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_gb: Option<PrimField<f64>>,
}
impl ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
    #[doc = "Set the field `count`.\nThe number of schedulers."]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu`.\nCPU request and limit for a single Airflow scheduler replica"]
    pub fn set_cpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_gb`.\nMemory (GB) request and limit for a single Airflow scheduler replica."]
    pub fn set_memory_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_gb`.\nStorage (GB) request and limit for a single Airflow scheduler replica."]
    pub fn set_storage_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_gb = Some(v.into());
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {}
impl BuildComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
        ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl {
            count: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
        ComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe number of schedulers."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for a single Airflow scheduler replica"]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for a single Airflow scheduler replica."]
    pub fn memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\nStorage (GB) request and limit for a single Airflow scheduler replica."]
    pub fn storage_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
    count: PrimField<f64>,
    cpu: PrimField<f64>,
    memory_gb: PrimField<f64>,
}
impl ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {}
impl ToListMappable for ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
    #[doc = "The number of triggerers."]
    pub count: PrimField<f64>,
    #[doc = "CPU request and limit for a single Airflow triggerer replica."]
    pub cpu: PrimField<f64>,
    #[doc = "Memory (GB) request and limit for a single Airflow triggerer replica."]
    pub memory_gb: PrimField<f64>,
}
impl BuildComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
        ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl {
            count: self.count,
            cpu: self.cpu,
            memory_gb: self.memory_gb,
        }
    }
}
pub struct ComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
        ComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe number of triggerers."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for a single Airflow triggerer replica."]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for a single Airflow triggerer replica."]
    pub fn memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_gb: Option<PrimField<f64>>,
}
impl ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
    #[doc = "Set the field `cpu`.\nCPU request and limit for Airflow web server."]
    pub fn set_cpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_gb`.\nMemory (GB) request and limit for Airflow web server."]
    pub fn set_memory_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_gb`.\nStorage (GB) request and limit for Airflow web server."]
    pub fn set_storage_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_gb = Some(v.into());
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {}
impl BuildComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
        ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl {
            cpu: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
        ComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for Airflow web server."]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for Airflow web server."]
    pub fn memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\nStorage (GB) request and limit for Airflow web server."]
    pub fn storage_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
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
impl ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
    #[doc = "Set the field `cpu`.\nCPU request and limit for a single Airflow worker replica."]
    pub fn set_cpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `max_count`.\nMaximum number of workers for autoscaling."]
    pub fn set_max_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_count = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_gb`.\nMemory (GB) request and limit for a single Airflow worker replica."]
    pub fn set_memory_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `min_count`.\nMinimum number of workers for autoscaling."]
    pub fn set_min_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_count = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_gb`.\nStorage (GB) request and limit for a single Airflow worker replica."]
    pub fn set_storage_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_gb = Some(v.into());
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {}
impl BuildComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
        ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl {
            cpu: core::default::Default::default(),
            max_count: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            min_count: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
        ComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\nCPU request and limit for a single Airflow worker replica."]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `max_count` after provisioning.\nMaximum number of workers for autoscaling."]
    pub fn max_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_count", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) request and limit for a single Airflow worker replica."]
    pub fn memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `min_count` after provisioning.\nMinimum number of workers for autoscaling."]
    pub fn min_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_count", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\nStorage (GB) request and limit for a single Airflow worker replica."]
    pub fn storage_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElWorkloadsConfigElDynamic {
    dag_processor: Option<DynamicBlock<ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl>>,
    scheduler: Option<DynamicBlock<ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl>>,
    triggerer: Option<DynamicBlock<ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl>>,
    web_server: Option<DynamicBlock<ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl>>,
    worker: Option<DynamicBlock<ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl>>,
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigElWorkloadsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dag_processor: Option<Vec<ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduler: Option<Vec<ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    triggerer: Option<Vec<ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_server: Option<Vec<ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker: Option<Vec<ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl>>,
    dynamic: ComposerEnvironmentConfigElWorkloadsConfigElDynamic,
}
impl ComposerEnvironmentConfigElWorkloadsConfigEl {
    #[doc = "Set the field `dag_processor`.\n"]
    pub fn set_dag_processor(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dag_processor = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dag_processor = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scheduler`.\n"]
    pub fn set_scheduler(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElSchedulerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scheduler = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduler = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `triggerer`.\n"]
    pub fn set_triggerer(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElTriggererEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.triggerer = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.triggerer = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `web_server`.\n"]
    pub fn set_web_server(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElWebServerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.web_server = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.web_server = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `worker`.\n"]
    pub fn set_worker(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigElWorkerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.worker = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.worker = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigElWorkloadsConfigEl {
    type O = BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigElWorkloadsConfigEl {}
impl BuildComposerEnvironmentConfigElWorkloadsConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigElWorkloadsConfigEl {
        ComposerEnvironmentConfigElWorkloadsConfigEl {
            dag_processor: core::default::Default::default(),
            scheduler: core::default::Default::default(),
            triggerer: core::default::Default::default(),
            web_server: core::default::Default::default(),
            worker: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElWorkloadsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElWorkloadsConfigElRef {
    fn new(shared: StackShared, base: String) -> ComposerEnvironmentConfigElWorkloadsConfigElRef {
        ComposerEnvironmentConfigElWorkloadsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElWorkloadsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dag_processor` after provisioning.\n"]
    pub fn dag_processor(
        &self,
    ) -> ListRef<ComposerEnvironmentConfigElWorkloadsConfigElDagProcessorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dag_processor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scheduler` after provisioning.\n"]
    pub fn scheduler(&self) -> ListRef<ComposerEnvironmentConfigElWorkloadsConfigElSchedulerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduler", self.base))
    }
    #[doc = "Get a reference to the value of field `triggerer` after provisioning.\n"]
    pub fn triggerer(&self) -> ListRef<ComposerEnvironmentConfigElWorkloadsConfigElTriggererElRef> {
        ListRef::new(self.shared().clone(), format!("{}.triggerer", self.base))
    }
    #[doc = "Get a reference to the value of field `web_server` after provisioning.\n"]
    pub fn web_server(
        &self,
    ) -> ListRef<ComposerEnvironmentConfigElWorkloadsConfigElWebServerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.web_server", self.base))
    }
    #[doc = "Get a reference to the value of field `worker` after provisioning.\n"]
    pub fn worker(&self) -> ListRef<ComposerEnvironmentConfigElWorkloadsConfigElWorkerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComposerEnvironmentConfigElDynamic {
    data_retention_config: Option<DynamicBlock<ComposerEnvironmentConfigElDataRetentionConfigEl>>,
    database_config: Option<DynamicBlock<ComposerEnvironmentConfigElDatabaseConfigEl>>,
    encryption_config: Option<DynamicBlock<ComposerEnvironmentConfigElEncryptionConfigEl>>,
    maintenance_window: Option<DynamicBlock<ComposerEnvironmentConfigElMaintenanceWindowEl>>,
    master_authorized_networks_config:
        Option<DynamicBlock<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl>>,
    node_config: Option<DynamicBlock<ComposerEnvironmentConfigElNodeConfigEl>>,
    private_environment_config:
        Option<DynamicBlock<ComposerEnvironmentConfigElPrivateEnvironmentConfigEl>>,
    recovery_config: Option<DynamicBlock<ComposerEnvironmentConfigElRecoveryConfigEl>>,
    software_config: Option<DynamicBlock<ComposerEnvironmentConfigElSoftwareConfigEl>>,
    web_server_config: Option<DynamicBlock<ComposerEnvironmentConfigElWebServerConfigEl>>,
    web_server_network_access_control:
        Option<DynamicBlock<ComposerEnvironmentConfigElWebServerNetworkAccessControlEl>>,
    workloads_config: Option<DynamicBlock<ComposerEnvironmentConfigElWorkloadsConfigEl>>,
}
#[derive(Serialize)]
pub struct ComposerEnvironmentConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_builds_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_environment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resilience_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_retention_config: Option<Vec<ComposerEnvironmentConfigElDataRetentionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_config: Option<Vec<ComposerEnvironmentConfigElDatabaseConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config: Option<Vec<ComposerEnvironmentConfigElEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window: Option<Vec<ComposerEnvironmentConfigElMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_authorized_networks_config:
        Option<Vec<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<Vec<ComposerEnvironmentConfigElNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_environment_config: Option<Vec<ComposerEnvironmentConfigElPrivateEnvironmentConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recovery_config: Option<Vec<ComposerEnvironmentConfigElRecoveryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    software_config: Option<Vec<ComposerEnvironmentConfigElSoftwareConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_server_config: Option<Vec<ComposerEnvironmentConfigElWebServerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_server_network_access_control:
        Option<Vec<ComposerEnvironmentConfigElWebServerNetworkAccessControlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workloads_config: Option<Vec<ComposerEnvironmentConfigElWorkloadsConfigEl>>,
    dynamic: ComposerEnvironmentConfigElDynamic,
}
impl ComposerEnvironmentConfigEl {
    #[doc = "Set the field `enable_private_builds_only`.\nOptional. If true, builds performed during operations that install Python packages have only private connectivity to Google services. If false, the builds also have access to the internet."]
    pub fn set_enable_private_builds_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_builds_only = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_private_environment`.\nOptional. If true, a private Composer environment will be created."]
    pub fn set_enable_private_environment(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_environment = Some(v.into());
        self
    }
    #[doc = "Set the field `environment_size`.\nThe size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn set_environment_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_size = Some(v.into());
        self
    }
    #[doc = "Set the field `node_count`.\nThe number of nodes in the Kubernetes Engine cluster that will be used to run this environment. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `resilience_mode`.\nWhether high resilience is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.1.15-airflow-*.*.* and newer."]
    pub fn set_resilience_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resilience_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `data_retention_config`.\n"]
    pub fn set_data_retention_config(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElDataRetentionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_retention_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_retention_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `database_config`.\n"]
    pub fn set_database_config(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElDatabaseConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.database_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.database_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElEncryptionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElMaintenanceWindowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.maintenance_window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_window = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `master_authorized_networks_config`.\n"]
    pub fn set_master_authorized_networks_config(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.master_authorized_networks_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.master_authorized_networks_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `private_environment_config`.\n"]
    pub fn set_private_environment_config(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElPrivateEnvironmentConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.private_environment_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.private_environment_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `recovery_config`.\n"]
    pub fn set_recovery_config(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElRecoveryConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.recovery_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.recovery_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `software_config`.\n"]
    pub fn set_software_config(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElSoftwareConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.software_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.software_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `web_server_config`.\n"]
    pub fn set_web_server_config(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElWebServerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.web_server_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.web_server_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `web_server_network_access_control`.\n"]
    pub fn set_web_server_network_access_control(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElWebServerNetworkAccessControlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.web_server_network_access_control = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.web_server_network_access_control = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `workloads_config`.\n"]
    pub fn set_workloads_config(
        mut self,
        v: impl Into<BlockAssignable<ComposerEnvironmentConfigElWorkloadsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.workloads_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.workloads_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComposerEnvironmentConfigEl {
    type O = BlockAssignable<ComposerEnvironmentConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentConfigEl {}
impl BuildComposerEnvironmentConfigEl {
    pub fn build(self) -> ComposerEnvironmentConfigEl {
        ComposerEnvironmentConfigEl {
            enable_private_builds_only: core::default::Default::default(),
            enable_private_environment: core::default::Default::default(),
            environment_size: core::default::Default::default(),
            node_count: core::default::Default::default(),
            resilience_mode: core::default::Default::default(),
            data_retention_config: core::default::Default::default(),
            database_config: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            master_authorized_networks_config: core::default::Default::default(),
            node_config: core::default::Default::default(),
            private_environment_config: core::default::Default::default(),
            recovery_config: core::default::Default::default(),
            software_config: core::default::Default::default(),
            web_server_config: core::default::Default::default(),
            web_server_network_access_control: core::default::Default::default(),
            workloads_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComposerEnvironmentConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentConfigElRef {
    fn new(shared: StackShared, base: String) -> ComposerEnvironmentConfigElRef {
        ComposerEnvironmentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `airflow_uri` after provisioning.\nThe URI of the Apache Airflow Web UI hosted within this environment."]
    pub fn airflow_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.airflow_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `dag_gcs_prefix` after provisioning.\nThe Cloud Storage prefix of the DAGs for this environment. Although Cloud Storage objects reside in a flat namespace, a hierarchical file tree can be simulated using '/'-delimited object name prefixes. DAG objects for this environment reside in a simulated directory with this prefix."]
    pub fn dag_gcs_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dag_gcs_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_private_builds_only` after provisioning.\nOptional. If true, builds performed during operations that install Python packages have only private connectivity to Google services. If false, the builds also have access to the internet."]
    pub fn enable_private_builds_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_builds_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_private_environment` after provisioning.\nOptional. If true, a private Composer environment will be created."]
    pub fn enable_private_environment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_environment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment_size` after provisioning.\nThe size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer."]
    pub fn environment_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gke_cluster` after provisioning.\nThe Kubernetes Engine cluster used to run this environment."]
    pub fn gke_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gke_cluster", self.base))
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes in the Kubernetes Engine cluster that will be used to run this environment. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
    #[doc = "Get a reference to the value of field `resilience_mode` after provisioning.\nWhether high resilience is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.1.15-airflow-*.*.* and newer."]
    pub fn resilience_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resilience_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_retention_config` after provisioning.\n"]
    pub fn data_retention_config(
        &self,
    ) -> ListRef<ComposerEnvironmentConfigElDataRetentionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_retention_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database_config` after provisioning.\n"]
    pub fn database_config(&self) -> ListRef<ComposerEnvironmentConfigElDatabaseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> ListRef<ComposerEnvironmentConfigElEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> ListRef<ComposerEnvironmentConfigElMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\n"]
    pub fn master_authorized_networks_config(
        &self,
    ) -> ListRef<ComposerEnvironmentConfigElMasterAuthorizedNetworksConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.master_authorized_networks_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<ComposerEnvironmentConfigElNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }
    #[doc = "Get a reference to the value of field `private_environment_config` after provisioning.\n"]
    pub fn private_environment_config(
        &self,
    ) -> ListRef<ComposerEnvironmentConfigElPrivateEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_environment_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `recovery_config` after provisioning.\n"]
    pub fn recovery_config(&self) -> ListRef<ComposerEnvironmentConfigElRecoveryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recovery_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> ListRef<ComposerEnvironmentConfigElSoftwareConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.software_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `web_server_config` after provisioning.\n"]
    pub fn web_server_config(&self) -> ListRef<ComposerEnvironmentConfigElWebServerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.web_server_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `web_server_network_access_control` after provisioning.\n"]
    pub fn web_server_network_access_control(
        &self,
    ) -> ListRef<ComposerEnvironmentConfigElWebServerNetworkAccessControlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.web_server_network_access_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workloads_config` after provisioning.\n"]
    pub fn workloads_config(&self) -> ListRef<ComposerEnvironmentConfigElWorkloadsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workloads_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentStorageConfigEl {
    bucket: PrimField<String>,
}
impl ComposerEnvironmentStorageConfigEl {}
impl ToListMappable for ComposerEnvironmentStorageConfigEl {
    type O = BlockAssignable<ComposerEnvironmentStorageConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentStorageConfigEl {
    #[doc = "Optional. Name of an existing Cloud Storage bucket to be used by the environment."]
    pub bucket: PrimField<String>,
}
impl BuildComposerEnvironmentStorageConfigEl {
    pub fn build(self) -> ComposerEnvironmentStorageConfigEl {
        ComposerEnvironmentStorageConfigEl {
            bucket: self.bucket,
        }
    }
}
pub struct ComposerEnvironmentStorageConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentStorageConfigElRef {
    fn new(shared: StackShared, base: String) -> ComposerEnvironmentStorageConfigElRef {
        ComposerEnvironmentStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentStorageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nOptional. Name of an existing Cloud Storage bucket to be used by the environment."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}
#[derive(Serialize)]
pub struct ComposerEnvironmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComposerEnvironmentTimeoutsEl {
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
impl ToListMappable for ComposerEnvironmentTimeoutsEl {
    type O = BlockAssignable<ComposerEnvironmentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComposerEnvironmentTimeoutsEl {}
impl BuildComposerEnvironmentTimeoutsEl {
    pub fn build(self) -> ComposerEnvironmentTimeoutsEl {
        ComposerEnvironmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComposerEnvironmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComposerEnvironmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComposerEnvironmentTimeoutsElRef {
        ComposerEnvironmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComposerEnvironmentTimeoutsElRef {
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
struct ComposerEnvironmentDynamic {
    config: Option<DynamicBlock<ComposerEnvironmentConfigEl>>,
    storage_config: Option<DynamicBlock<ComposerEnvironmentStorageConfigEl>>,
}
