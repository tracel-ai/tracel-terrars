use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GkeBackupRestorePlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    backup_plan: PrimField<String>,
    cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_config: Option<Vec<GkeBackupRestorePlanRestoreConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeBackupRestorePlanTimeoutsEl>,
    dynamic: GkeBackupRestorePlanDynamic,
}
struct GkeBackupRestorePlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeBackupRestorePlanData>,
}
#[derive(Clone)]
pub struct GkeBackupRestorePlan(Rc<GkeBackupRestorePlan_>);
impl GkeBackupRestorePlan {
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
    #[doc = "Set the field `description`.\nUser specified descriptive string for this RestorePlan."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nDescription: A set of custom labels supplied by the user.\nA list of key->value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `restore_config`.\n"]
    pub fn set_restore_config(
        self,
        v: impl Into<BlockAssignable<GkeBackupRestorePlanRestoreConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GkeBackupRestorePlanTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `backup_plan` after provisioning.\nA reference to the BackupPlan from which Backups may be used\nas the source for Restores created via this RestorePlan."]
    pub fn backup_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_plan", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe name of the target cluster to which you want to Restore via this RestorePlan."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser specified descriptive string for this RestorePlan."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nDescription: A set of custom labels supplied by the user.\nA list of key->value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe region of the Restore Plan."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full name of the BackupPlan Resource."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe State of the RestorePlan."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\nDetailed description of why RestorePlan is in its current state."]
    pub fn state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nServer generated, unique identifier of UUID format."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `restore_config` after provisioning.\n"]
    pub fn restore_config(&self) -> ListRef<GkeBackupRestorePlanRestoreConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeBackupRestorePlanTimeoutsElRef {
        GkeBackupRestorePlanTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for GkeBackupRestorePlan {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GkeBackupRestorePlan {}
impl ToListMappable for GkeBackupRestorePlan {
    type O = ListRef<GkeBackupRestorePlanRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GkeBackupRestorePlan_ {
    fn extract_resource_type(&self) -> String {
        "google_gke_backup_restore_plan".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGkeBackupRestorePlan {
    pub tf_id: String,
    #[doc = "A reference to the BackupPlan from which Backups may be used\nas the source for Restores created via this RestorePlan."]
    pub backup_plan: PrimField<String>,
    #[doc = "The name of the target cluster to which you want to Restore via this RestorePlan."]
    pub cluster: PrimField<String>,
    #[doc = "The region of the Restore Plan."]
    pub location: PrimField<String>,
    #[doc = "The full name of the BackupPlan Resource."]
    pub name: PrimField<String>,
}
impl BuildGkeBackupRestorePlan {
    pub fn build(self, stack: &mut Stack) -> GkeBackupRestorePlan {
        let out = GkeBackupRestorePlan(Rc::new(GkeBackupRestorePlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeBackupRestorePlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backup_plan: self.backup_plan,
                cluster: self.cluster,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                restore_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GkeBackupRestorePlanRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GkeBackupRestorePlanRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backup_plan` after provisioning.\nA reference to the BackupPlan from which Backups may be used\nas the source for Restores created via this RestorePlan."]
    pub fn backup_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_plan", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe name of the target cluster to which you want to Restore via this RestorePlan."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser specified descriptive string for this RestorePlan."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nDescription: A set of custom labels supplied by the user.\nA list of key->value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe region of the Restore Plan."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full name of the BackupPlan Resource."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe State of the RestorePlan."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\nDetailed description of why RestorePlan is in its current state."]
    pub fn state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nServer generated, unique identifier of UUID format."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `restore_config` after provisioning.\n"]
    pub fn restore_config(&self) -> ListRef<GkeBackupRestorePlanRestoreConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeBackupRestorePlanTimeoutsElRef {
        GkeBackupRestorePlanTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_kind: Option<PrimField<String>>,
}
impl GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl {
    #[doc = "Set the field `resource_group`.\nAPI Group string of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn set_resource_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_group = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_kind`.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn set_resource_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_kind = Some(v.into());
        self
    }
}
impl ToListMappable
    for GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl
{
    type O = BlockAssignable<
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl
{}
impl BuildGkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl {
    pub fn build(
        self,
    ) -> GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl {
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl {
            resource_group: core::default::Default::default(),
            resource_kind: core::default::Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsElRef
    {
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_group` after provisioning.\nAPI Group string of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn resource_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_kind` after provisioning.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn resource_kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_kind", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_kind: Option<PrimField<String>>,
}
impl GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl {
    #[doc = "Set the field `resource_group`.\nAPI Group string of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn set_resource_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_group = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_kind`.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn set_resource_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_kind = Some(v.into());
        self
    }
}
impl ToListMappable
    for GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl
{
    type O = BlockAssignable<
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl
{}
impl BuildGkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl {
    pub fn build(
        self,
    ) -> GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl {
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl {
            resource_group: core::default::Default::default(),
            resource_kind: core::default::Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsElRef
    {
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_group` after provisioning.\nAPI Group string of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn resource_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_kind` after provisioning.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn resource_kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_kind", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElDynamic {
    excluded_group_kinds: Option<
        DynamicBlock<
            GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl,
        >,
    >,
    selected_group_kinds: Option<
        DynamicBlock<
            GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_group_kinds: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_group_kinds: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_group_kinds: Option<
        Vec<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_group_kinds: Option<
        Vec<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl>,
    >,
    dynamic: GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElDynamic,
}
impl GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {
    #[doc = "Set the field `all_group_kinds`.\nIf True, all valid cluster-scoped resources will be restored.\nMutually exclusive to any other field in 'clusterResourceRestoreScope'."]
    pub fn set_all_group_kinds(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_group_kinds = Some(v.into());
        self
    }
    #[doc = "Set the field `no_group_kinds`.\nIf True, no cluster-scoped resources will be restored.\nMutually exclusive to any other field in 'clusterResourceRestoreScope'."]
    pub fn set_no_group_kinds(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_group_kinds = Some(v.into());
        self
    }
    #[doc = "Set the field `excluded_group_kinds`.\n"]
    pub fn set_excluded_group_kinds(
        mut self,
        v : impl Into < BlockAssignable < GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.excluded_group_kinds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.excluded_group_kinds = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `selected_group_kinds`.\n"]
    pub fn set_selected_group_kinds(
        mut self,
        v : impl Into < BlockAssignable < GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.selected_group_kinds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_group_kinds = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {
    type O = BlockAssignable<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {}
impl BuildGkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {
            all_group_kinds: core::default::Default::default(),
            no_group_kinds: core::default::Default::default(),
            excluded_group_kinds: core::default::Default::default(),
            selected_group_kinds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElRef {
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_group_kinds` after provisioning.\nIf True, all valid cluster-scoped resources will be restored.\nMutually exclusive to any other field in 'clusterResourceRestoreScope'."]
    pub fn all_group_kinds(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_group_kinds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `no_group_kinds` after provisioning.\nIf True, no cluster-scoped resources will be restored.\nMutually exclusive to any other field in 'clusterResourceRestoreScope'."]
    pub fn no_group_kinds(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_group_kinds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_group_kinds` after provisioning.\n"]
    pub fn excluded_group_kinds(
        &self,
    ) -> ListRef<
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_group_kinds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `selected_group_kinds` after provisioning.\n"]
    pub fn selected_group_kinds(
        &self,
    ) -> ListRef<
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.selected_group_kinds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl {
    namespaces: ListField<PrimField<String>>,
}
impl GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl {}
impl ToListMappable for GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl {
    type O = BlockAssignable<GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl {
    #[doc = "A list of Kubernetes Namespaces."]
    pub namespaces: ListField<PrimField<String>>,
}
impl BuildGkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl {
        GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl {
            namespaces: self.namespaces,
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElExcludedNamespacesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRestoreConfigElExcludedNamespacesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElExcludedNamespacesElRef {
        GkeBackupRestorePlanRestoreConfigElExcludedNamespacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElExcludedNamespacesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\nA list of Kubernetes Namespaces."]
    pub fn namespaces(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.namespaces", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_kind: Option<PrimField<String>>,
}
impl GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringEl {
    #[doc = "Set the field `resource_group`.\nAPI Group of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn set_resource_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_group = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_kind`.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn set_resource_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_kind = Some(v.into());
        self
    }
}
impl ToListMappable
    for GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringEl
{
    type O = BlockAssignable<
        GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringEl
{}
impl BuildGkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringEl {
    pub fn build(
        self,
    ) -> GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringEl {
        GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringEl {
            resource_group: core::default::Default::default(),
            resource_kind: core::default::Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringElRef
    {
        GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_group` after provisioning.\nAPI Group of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn resource_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_kind` after provisioning.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn resource_kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_kind", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_kind: Option<PrimField<String>>,
}
impl GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingEl {
    #[doc = "Set the field `resource_group`.\nAPI Group of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn set_resource_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_group = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_kind`.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn set_resource_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_kind = Some(v.into());
        self
    }
}
impl ToListMappable
    for GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingEl
{
    type O = BlockAssignable<
        GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingEl
{}
impl BuildGkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingEl {
    pub fn build(
        self,
    ) -> GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingEl {
        GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingEl {
            resource_group: core::default::Default::default(),
            resource_kind: core::default::Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingElRef
    {
        GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_group` after provisioning.\nAPI Group of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn resource_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_kind` after provisioning.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn resource_kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_kind", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElDynamic {
    requiring: Option<
        DynamicBlock<
            GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringEl,
        >,
    >,
    satisfying: Option<
        DynamicBlock<
            GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    requiring: Option<
        Vec<GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    satisfying: Option<
        Vec<GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingEl>,
    >,
    dynamic: GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElDynamic,
}
impl GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesEl {
    #[doc = "Set the field `requiring`.\n"]
    pub fn set_requiring(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.requiring = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.requiring = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `satisfying`.\n"]
    pub fn set_satisfying(
        mut self,
        v : impl Into < BlockAssignable < GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.satisfying = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.satisfying = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesEl {
    type O =
        BlockAssignable<GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesEl {}
impl BuildGkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesEl {
        GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesEl {
            requiring: core::default::Default::default(),
            satisfying: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRef {
        GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `requiring` after provisioning.\n"]
    pub fn requiring(
        &self,
    ) -> ListRef<
        GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRequiringElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.requiring", self.base))
    }
    #[doc = "Get a reference to the value of field `satisfying` after provisioning.\n"]
    pub fn satisfying(
        &self,
    ) -> ListRef<
        GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElSatisfyingElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.satisfying", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeBackupRestorePlanRestoreConfigElRestoreOrderElDynamic {
    group_kind_dependencies: Option<
        DynamicBlock<GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesEl>,
    >,
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElRestoreOrderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_kind_dependencies:
        Option<Vec<GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesEl>>,
    dynamic: GkeBackupRestorePlanRestoreConfigElRestoreOrderElDynamic,
}
impl GkeBackupRestorePlanRestoreConfigElRestoreOrderEl {
    #[doc = "Set the field `group_kind_dependencies`.\n"]
    pub fn set_group_kind_dependencies(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.group_kind_dependencies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.group_kind_dependencies = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeBackupRestorePlanRestoreConfigElRestoreOrderEl {
    type O = BlockAssignable<GkeBackupRestorePlanRestoreConfigElRestoreOrderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElRestoreOrderEl {}
impl BuildGkeBackupRestorePlanRestoreConfigElRestoreOrderEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElRestoreOrderEl {
        GkeBackupRestorePlanRestoreConfigElRestoreOrderEl {
            group_kind_dependencies: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElRestoreOrderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRestoreConfigElRestoreOrderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElRestoreOrderElRef {
        GkeBackupRestorePlanRestoreConfigElRestoreOrderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElRestoreOrderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `group_kind_dependencies` after provisioning.\n"]
    pub fn group_kind_dependencies(
        &self,
    ) -> ListRef<GkeBackupRestorePlanRestoreConfigElRestoreOrderElGroupKindDependenciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_kind_dependencies", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {
    name: PrimField<String>,
    namespace: PrimField<String>,
}
impl GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {}
impl ToListMappable for GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {
    type O =
        BlockAssignable<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {
    #[doc = "The name of a Kubernetes Resource."]
    pub name: PrimField<String>,
    #[doc = "The namespace of a Kubernetes Resource."]
    pub namespace: PrimField<String>,
}
impl BuildGkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {
    pub fn build(
        self,
    ) -> GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {
        GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {
            name: self.name,
            namespace: self.namespace,
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesElRef {
        GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of a Kubernetes Resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe namespace of a Kubernetes Resource."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElDynamic {
    namespaced_names: Option<
        DynamicBlock<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl>,
    >,
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaced_names:
        Option<Vec<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl>>,
    dynamic: GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElDynamic,
}
impl GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {
    #[doc = "Set the field `namespaced_names`.\n"]
    pub fn set_namespaced_names(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.namespaced_names = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.namespaced_names = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {
    type O = BlockAssignable<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {}
impl BuildGkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {
        GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {
            namespaced_names: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElRef {
        GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `namespaced_names` after provisioning.\n"]
    pub fn namespaced_names(
        &self,
    ) -> ListRef<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.namespaced_names", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl {
    namespaces: ListField<PrimField<String>>,
}
impl GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl {}
impl ToListMappable for GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl {
    type O = BlockAssignable<GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl {
    #[doc = "A list of Kubernetes Namespaces."]
    pub namespaces: ListField<PrimField<String>>,
}
impl BuildGkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl {
        GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl {
            namespaces: self.namespaces,
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElSelectedNamespacesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRestoreConfigElSelectedNamespacesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElSelectedNamespacesElRef {
        GkeBackupRestorePlanRestoreConfigElSelectedNamespacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElSelectedNamespacesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\nA list of Kubernetes Namespaces."]
    pub fn namespaces(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.namespaces", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_path: Option<PrimField<String>>,
    op: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
    #[doc = "Set the field `from_path`.\nA string containing a JSON Pointer value that references the\nlocation in the target document to move the value from."]
    pub fn set_from_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.from_path = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\nA string containing a JSON-Pointer value that references a\nlocation within the target document where the operation is performed."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nA string that specifies the desired value in string format\nto use for transformation."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
    type O =
        BlockAssignable<GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
    #[doc = "Specifies the operation to perform. Possible values: [\"REMOVE\", \"MOVE\", \"COPY\", \"ADD\", \"TEST\", \"REPLACE\"]"]
    pub op: PrimField<String>,
}
impl BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
            from_path: core::default::Default::default(),
            op: self.op,
            path: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsElRef {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `from_path` after provisioning.\nA string containing a JSON Pointer value that references the\nlocation in the target document to move the value from."]
    pub fn from_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_path", self.base))
    }
    #[doc = "Get a reference to the value of field `op` after provisioning.\nSpecifies the operation to perform. Possible values: [\"REMOVE\", \"MOVE\", \"COPY\", \"ADD\", \"TEST\", \"REPLACE\"]"]
    pub fn op(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.op", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nA string containing a JSON-Pointer value that references a\nlocation within the target document where the operation is performed."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA string that specifies the desired value in string format\nto use for transformation."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_kind: Option<PrimField<String>>,
}
impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl {
    #[doc = "Set the field `resource_group`.\nAPI Group string of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn set_resource_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_group = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_kind`.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn set_resource_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_kind = Some(v.into());
        self
    }
}
impl ToListMappable
    for GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl
{
    type O = BlockAssignable<
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl
{}
impl BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl {
    pub fn build(
        self,
    ) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl {
            resource_group: core::default::Default::default(),
            resource_kind: core::default::Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsElRef
    {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_group` after provisioning.\nAPI Group string of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn resource_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_kind` after provisioning.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn resource_kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_kind", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElDynamic {
    group_kinds: Option<
        DynamicBlock<
            GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaces: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_kinds: Option<
        Vec<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl>,
    >,
    dynamic: GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElDynamic,
}
impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {
    #[doc = "Set the field `json_path`.\nThis is a JSONPath expression that matches specific fields of\ncandidate resources and it operates as a filtering parameter\n(resources that are not matched with this expression will not\nbe candidates for transformation)."]
    pub fn set_json_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.json_path = Some(v.into());
        self
    }
    #[doc = "Set the field `namespaces`.\n(Filtering parameter) Any resource subject to transformation must\nbe contained within one of the listed Kubernetes Namespace in the\nBackup. If this field is not provided, no namespace filtering will\nbe performed (all resources in all Namespaces, including all\ncluster-scoped resources, will be candidates for transformation).\nTo mix cluster-scoped and namespaced resources in the same rule,\nuse an empty string (\"\") as one of the target namespaces."]
    pub fn set_namespaces(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.namespaces = Some(v.into());
        self
    }
    #[doc = "Set the field `group_kinds`.\n"]
    pub fn set_group_kinds(
        mut self,
        v : impl Into < BlockAssignable < GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.group_kinds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.group_kinds = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {
    type O =
        BlockAssignable<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {}
impl BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {
            json_path: core::default::Default::default(),
            namespaces: core::default::Default::default(),
            group_kinds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElRef {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `json_path` after provisioning.\nThis is a JSONPath expression that matches specific fields of\ncandidate resources and it operates as a filtering parameter\n(resources that are not matched with this expression will not\nbe candidates for transformation)."]
    pub fn json_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.json_path", self.base))
    }
    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\n(Filtering parameter) Any resource subject to transformation must\nbe contained within one of the listed Kubernetes Namespace in the\nBackup. If this field is not provided, no namespace filtering will\nbe performed (all resources in all Namespaces, including all\ncluster-scoped resources, will be candidates for transformation).\nTo mix cluster-scoped and namespaced resources in the same rule,\nuse an empty string (\"\") as one of the target namespaces."]
    pub fn namespaces(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.namespaces", self.base))
    }
    #[doc = "Get a reference to the value of field `group_kinds` after provisioning.\n"]
    pub fn group_kinds(
        &self,
    ) -> ListRef<
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.group_kinds", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElDynamic {
    field_actions: Option<
        DynamicBlock<GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl>,
    >,
    resource_filter: Option<
        DynamicBlock<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl>,
    >,
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_actions:
        Option<Vec<GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_filter:
        Option<Vec<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl>>,
    dynamic: GkeBackupRestorePlanRestoreConfigElTransformationRulesElDynamic,
}
impl GkeBackupRestorePlanRestoreConfigElTransformationRulesEl {
    #[doc = "Set the field `description`.\nThe description is a user specified string description\nof the transformation rule."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `field_actions`.\n"]
    pub fn set_field_actions(
        mut self,
        v: impl Into<
            BlockAssignable<GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field_actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field_actions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_filter`.\n"]
    pub fn set_resource_filter(
        mut self,
        v: impl Into<
            BlockAssignable<
                GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_filter = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeBackupRestorePlanRestoreConfigElTransformationRulesEl {
    type O = BlockAssignable<GkeBackupRestorePlanRestoreConfigElTransformationRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesEl {}
impl BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesEl {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesEl {
            description: core::default::Default::default(),
            field_actions: core::default::Default::default(),
            resource_filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRestoreConfigElTransformationRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElRef {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description is a user specified string description\nof the transformation rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `field_actions` after provisioning.\n"]
    pub fn field_actions(
        &self,
    ) -> ListRef<GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.field_actions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_filter` after provisioning.\n"]
    pub fn resource_filter(
        &self,
    ) -> ListRef<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_filter", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsEl {
    policy: PrimField<String>,
    volume_type: PrimField<String>,
}
impl GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsEl {}
impl ToListMappable for GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsEl {
    type O = BlockAssignable<GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsEl {
    #[doc = "Specifies the mechanism to be used to restore this volume data.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#VolumeDataRestorePolicy\nfor more information on each policy option. Possible values: [\"RESTORE_VOLUME_DATA_FROM_BACKUP\", \"REUSE_VOLUME_HANDLE_FROM_BACKUP\", \"NO_VOLUME_DATA_RESTORATION\"]"]
    pub policy: PrimField<String>,
    #[doc = "The volume type, as determined by the PVC's\nbound PV, to apply the policy to. Possible values: [\"GCE_PERSISTENT_DISK\"]"]
    pub volume_type: PrimField<String>,
}
impl BuildGkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsEl {
        GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsEl {
            policy: self.policy,
            volume_type: self.volume_type,
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsElRef {
        GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\nSpecifies the mechanism to be used to restore this volume data.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#VolumeDataRestorePolicy\nfor more information on each policy option. Possible values: [\"RESTORE_VOLUME_DATA_FROM_BACKUP\", \"REUSE_VOLUME_HANDLE_FROM_BACKUP\", \"NO_VOLUME_DATA_RESTORATION\"]"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\nThe volume type, as determined by the PVC's\nbound PV, to apply the policy to. Possible values: [\"GCE_PERSISTENT_DISK\"]"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct GkeBackupRestorePlanRestoreConfigElDynamic {
    cluster_resource_restore_scope:
        Option<DynamicBlock<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl>>,
    excluded_namespaces:
        Option<DynamicBlock<GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl>>,
    restore_order: Option<DynamicBlock<GkeBackupRestorePlanRestoreConfigElRestoreOrderEl>>,
    selected_applications:
        Option<DynamicBlock<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl>>,
    selected_namespaces:
        Option<DynamicBlock<GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl>>,
    transformation_rules:
        Option<DynamicBlock<GkeBackupRestorePlanRestoreConfigElTransformationRulesEl>>,
    volume_data_restore_policy_bindings:
        Option<DynamicBlock<GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsEl>>,
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_namespaces: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_resource_conflict_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaced_resource_restore_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_namespaces: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_data_restore_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_resource_restore_scope:
        Option<Vec<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_namespaces: Option<Vec<GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_order: Option<Vec<GkeBackupRestorePlanRestoreConfigElRestoreOrderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_applications: Option<Vec<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_namespaces: Option<Vec<GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transformation_rules: Option<Vec<GkeBackupRestorePlanRestoreConfigElTransformationRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_data_restore_policy_bindings:
        Option<Vec<GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsEl>>,
    dynamic: GkeBackupRestorePlanRestoreConfigElDynamic,
}
impl GkeBackupRestorePlanRestoreConfigEl {
    #[doc = "Set the field `all_namespaces`.\nIf True, restore all namespaced resources in the Backup.\nSetting this field to False will result in an error."]
    pub fn set_all_namespaces(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_namespaces = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_resource_conflict_policy`.\nDefines the behavior for handling the situation where cluster-scoped resources\nbeing restored already exist in the target cluster.\nThis MUST be set to a value other than 'CLUSTER_RESOURCE_CONFLICT_POLICY_UNSPECIFIED'\nif 'clusterResourceRestoreScope' is anyting other than 'noGroupKinds'.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#clusterresourceconflictpolicy\nfor more information on each policy option. Possible values: [\"USE_EXISTING_VERSION\", \"USE_BACKUP_VERSION\"]"]
    pub fn set_cluster_resource_conflict_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_resource_conflict_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `namespaced_resource_restore_mode`.\nDefines the behavior for handling the situation where sets of namespaced resources\nbeing restored already exist in the target cluster.\nThis MUST be set to a value other than 'NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED'\nif the 'namespacedResourceRestoreScope' is anything other than 'noNamespaces'.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#namespacedresourcerestoremode\nfor more information on each mode. Possible values: [\"DELETE_AND_RESTORE\", \"FAIL_ON_CONFLICT\", \"MERGE_SKIP_ON_CONFLICT\", \"MERGE_REPLACE_VOLUME_ON_CONFLICT\", \"MERGE_REPLACE_ON_CONFLICT\"]"]
    pub fn set_namespaced_resource_restore_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespaced_resource_restore_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `no_namespaces`.\nDo not restore any namespaced resources if set to \"True\".\nSpecifying this field to \"False\" is not allowed."]
    pub fn set_no_namespaces(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_namespaces = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_data_restore_policy`.\nSpecifies the mechanism to be used to restore volume data.\nThis should be set to a value other than 'NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED'\nif the 'namespacedResourceRestoreScope' is anything other than 'noNamespaces'.\nIf not specified, it will be treated as 'NO_VOLUME_DATA_RESTORATION'.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#VolumeDataRestorePolicy\nfor more information on each policy option. Possible values: [\"RESTORE_VOLUME_DATA_FROM_BACKUP\", \"REUSE_VOLUME_HANDLE_FROM_BACKUP\", \"NO_VOLUME_DATA_RESTORATION\"]"]
    pub fn set_volume_data_restore_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_data_restore_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_resource_restore_scope`.\n"]
    pub fn set_cluster_resource_restore_scope(
        mut self,
        v: impl Into<BlockAssignable<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cluster_resource_restore_scope = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cluster_resource_restore_scope = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `excluded_namespaces`.\n"]
    pub fn set_excluded_namespaces(
        mut self,
        v: impl Into<BlockAssignable<GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.excluded_namespaces = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.excluded_namespaces = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `restore_order`.\n"]
    pub fn set_restore_order(
        mut self,
        v: impl Into<BlockAssignable<GkeBackupRestorePlanRestoreConfigElRestoreOrderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.restore_order = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.restore_order = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `selected_applications`.\n"]
    pub fn set_selected_applications(
        mut self,
        v: impl Into<BlockAssignable<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.selected_applications = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_applications = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `selected_namespaces`.\n"]
    pub fn set_selected_namespaces(
        mut self,
        v: impl Into<BlockAssignable<GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.selected_namespaces = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_namespaces = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `transformation_rules`.\n"]
    pub fn set_transformation_rules(
        mut self,
        v: impl Into<BlockAssignable<GkeBackupRestorePlanRestoreConfigElTransformationRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transformation_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `volume_data_restore_policy_bindings`.\n"]
    pub fn set_volume_data_restore_policy_bindings(
        mut self,
        v: impl Into<
            BlockAssignable<GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.volume_data_restore_policy_bindings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_data_restore_policy_bindings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeBackupRestorePlanRestoreConfigEl {
    type O = BlockAssignable<GkeBackupRestorePlanRestoreConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanRestoreConfigEl {}
impl BuildGkeBackupRestorePlanRestoreConfigEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigEl {
        GkeBackupRestorePlanRestoreConfigEl {
            all_namespaces: core::default::Default::default(),
            cluster_resource_conflict_policy: core::default::Default::default(),
            namespaced_resource_restore_mode: core::default::Default::default(),
            no_namespaces: core::default::Default::default(),
            volume_data_restore_policy: core::default::Default::default(),
            cluster_resource_restore_scope: core::default::Default::default(),
            excluded_namespaces: core::default::Default::default(),
            restore_order: core::default::Default::default(),
            selected_applications: core::default::Default::default(),
            selected_namespaces: core::default::Default::default(),
            transformation_rules: core::default::Default::default(),
            volume_data_restore_policy_bindings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanRestoreConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanRestoreConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeBackupRestorePlanRestoreConfigElRef {
        GkeBackupRestorePlanRestoreConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanRestoreConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_namespaces` after provisioning.\nIf True, restore all namespaced resources in the Backup.\nSetting this field to False will result in an error."]
    pub fn all_namespaces(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_namespaces", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_resource_conflict_policy` after provisioning.\nDefines the behavior for handling the situation where cluster-scoped resources\nbeing restored already exist in the target cluster.\nThis MUST be set to a value other than 'CLUSTER_RESOURCE_CONFLICT_POLICY_UNSPECIFIED'\nif 'clusterResourceRestoreScope' is anyting other than 'noGroupKinds'.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#clusterresourceconflictpolicy\nfor more information on each policy option. Possible values: [\"USE_EXISTING_VERSION\", \"USE_BACKUP_VERSION\"]"]
    pub fn cluster_resource_conflict_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_resource_conflict_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `namespaced_resource_restore_mode` after provisioning.\nDefines the behavior for handling the situation where sets of namespaced resources\nbeing restored already exist in the target cluster.\nThis MUST be set to a value other than 'NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED'\nif the 'namespacedResourceRestoreScope' is anything other than 'noNamespaces'.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#namespacedresourcerestoremode\nfor more information on each mode. Possible values: [\"DELETE_AND_RESTORE\", \"FAIL_ON_CONFLICT\", \"MERGE_SKIP_ON_CONFLICT\", \"MERGE_REPLACE_VOLUME_ON_CONFLICT\", \"MERGE_REPLACE_ON_CONFLICT\"]"]
    pub fn namespaced_resource_restore_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespaced_resource_restore_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `no_namespaces` after provisioning.\nDo not restore any namespaced resources if set to \"True\".\nSpecifying this field to \"False\" is not allowed."]
    pub fn no_namespaces(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_namespaces", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_data_restore_policy` after provisioning.\nSpecifies the mechanism to be used to restore volume data.\nThis should be set to a value other than 'NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED'\nif the 'namespacedResourceRestoreScope' is anything other than 'noNamespaces'.\nIf not specified, it will be treated as 'NO_VOLUME_DATA_RESTORATION'.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#VolumeDataRestorePolicy\nfor more information on each policy option. Possible values: [\"RESTORE_VOLUME_DATA_FROM_BACKUP\", \"REUSE_VOLUME_HANDLE_FROM_BACKUP\", \"NO_VOLUME_DATA_RESTORATION\"]"]
    pub fn volume_data_restore_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_data_restore_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_resource_restore_scope` after provisioning.\n"]
    pub fn cluster_resource_restore_scope(
        &self,
    ) -> ListRef<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_resource_restore_scope", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_namespaces` after provisioning.\n"]
    pub fn excluded_namespaces(
        &self,
    ) -> ListRef<GkeBackupRestorePlanRestoreConfigElExcludedNamespacesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_namespaces", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `restore_order` after provisioning.\n"]
    pub fn restore_order(&self) -> ListRef<GkeBackupRestorePlanRestoreConfigElRestoreOrderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_order", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `selected_applications` after provisioning.\n"]
    pub fn selected_applications(
        &self,
    ) -> ListRef<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.selected_applications", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `selected_namespaces` after provisioning.\n"]
    pub fn selected_namespaces(
        &self,
    ) -> ListRef<GkeBackupRestorePlanRestoreConfigElSelectedNamespacesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.selected_namespaces", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transformation_rules` after provisioning.\n"]
    pub fn transformation_rules(
        &self,
    ) -> ListRef<GkeBackupRestorePlanRestoreConfigElTransformationRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transformation_rules", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_data_restore_policy_bindings` after provisioning.\n"]
    pub fn volume_data_restore_policy_bindings(
        &self,
    ) -> ListRef<GkeBackupRestorePlanRestoreConfigElVolumeDataRestorePolicyBindingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.volume_data_restore_policy_bindings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GkeBackupRestorePlanTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl GkeBackupRestorePlanTimeoutsEl {
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
impl ToListMappable for GkeBackupRestorePlanTimeoutsEl {
    type O = BlockAssignable<GkeBackupRestorePlanTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeBackupRestorePlanTimeoutsEl {}
impl BuildGkeBackupRestorePlanTimeoutsEl {
    pub fn build(self) -> GkeBackupRestorePlanTimeoutsEl {
        GkeBackupRestorePlanTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct GkeBackupRestorePlanTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeBackupRestorePlanTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeBackupRestorePlanTimeoutsElRef {
        GkeBackupRestorePlanTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeBackupRestorePlanTimeoutsElRef {
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
struct GkeBackupRestorePlanDynamic {
    restore_config: Option<DynamicBlock<GkeBackupRestorePlanRestoreConfigEl>>,
}
