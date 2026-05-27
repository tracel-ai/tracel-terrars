use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SpannerBackupScheduleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    database: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    retention_duration: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config: Option<Vec<SpannerBackupScheduleEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_backup_spec: Option<Vec<SpannerBackupScheduleFullBackupSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    incremental_backup_spec: Option<Vec<SpannerBackupScheduleIncrementalBackupSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<SpannerBackupScheduleSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SpannerBackupScheduleTimeoutsEl>,
    dynamic: SpannerBackupScheduleDynamic,
}
struct SpannerBackupSchedule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SpannerBackupScheduleData>,
}
#[derive(Clone)]
pub struct SpannerBackupSchedule(Rc<SpannerBackupSchedule_>);
impl SpannerBackupSchedule {
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
    #[doc = "Set the field `name`.\nA unique identifier for the backup schedule, which cannot be changed after\nthe backup schedule is created. Values are of the form [a-z][-a-z0-9]*[a-z0-9]."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl Into<BlockAssignable<SpannerBackupScheduleEncryptionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `full_backup_spec`.\n"]
    pub fn set_full_backup_spec(
        self,
        v: impl Into<BlockAssignable<SpannerBackupScheduleFullBackupSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().full_backup_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.full_backup_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `incremental_backup_spec`.\n"]
    pub fn set_incremental_backup_spec(
        self,
        v: impl Into<BlockAssignable<SpannerBackupScheduleIncrementalBackupSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().incremental_backup_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.incremental_backup_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl Into<BlockAssignable<SpannerBackupScheduleSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SpannerBackupScheduleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe database to create the backup schedule on."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe instance to create the database on."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the backup schedule, which cannot be changed after\nthe backup schedule is created. Values are of the form [a-z][-a-z0-9]*[a-z0-9]."]
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
    #[doc = "Get a reference to the value of field `retention_duration` after provisioning.\nAt what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'.\nYou can set this to a value up to 366 days."]
    pub fn retention_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> ListRef<SpannerBackupScheduleEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `full_backup_spec` after provisioning.\n"]
    pub fn full_backup_spec(&self) -> ListRef<SpannerBackupScheduleFullBackupSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.full_backup_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `incremental_backup_spec` after provisioning.\n"]
    pub fn incremental_backup_spec(
        &self,
    ) -> ListRef<SpannerBackupScheduleIncrementalBackupSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.incremental_backup_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<SpannerBackupScheduleSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerBackupScheduleTimeoutsElRef {
        SpannerBackupScheduleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SpannerBackupSchedule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SpannerBackupSchedule {}
impl ToListMappable for SpannerBackupSchedule {
    type O = ListRef<SpannerBackupScheduleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SpannerBackupSchedule_ {
    fn extract_resource_type(&self) -> String {
        "google_spanner_backup_schedule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSpannerBackupSchedule {
    pub tf_id: String,
    #[doc = "The database to create the backup schedule on."]
    pub database: PrimField<String>,
    #[doc = "The instance to create the database on."]
    pub instance: PrimField<String>,
    #[doc = "At what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'.\nYou can set this to a value up to 366 days."]
    pub retention_duration: PrimField<String>,
}
impl BuildSpannerBackupSchedule {
    pub fn build(self, stack: &mut Stack) -> SpannerBackupSchedule {
        let out = SpannerBackupSchedule(Rc::new(SpannerBackupSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SpannerBackupScheduleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                database: self.database,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                retention_duration: self.retention_duration,
                encryption_config: core::default::Default::default(),
                full_backup_spec: core::default::Default::default(),
                incremental_backup_spec: core::default::Default::default(),
                spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SpannerBackupScheduleRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerBackupScheduleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SpannerBackupScheduleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe database to create the backup schedule on."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe instance to create the database on."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the backup schedule, which cannot be changed after\nthe backup schedule is created. Values are of the form [a-z][-a-z0-9]*[a-z0-9]."]
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
    #[doc = "Get a reference to the value of field `retention_duration` after provisioning.\nAt what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'.\nYou can set this to a value up to 366 days."]
    pub fn retention_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> ListRef<SpannerBackupScheduleEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `full_backup_spec` after provisioning.\n"]
    pub fn full_backup_spec(&self) -> ListRef<SpannerBackupScheduleFullBackupSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.full_backup_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `incremental_backup_spec` after provisioning.\n"]
    pub fn incremental_backup_spec(
        &self,
    ) -> ListRef<SpannerBackupScheduleIncrementalBackupSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.incremental_backup_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<SpannerBackupScheduleSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerBackupScheduleTimeoutsElRef {
        SpannerBackupScheduleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SpannerBackupScheduleEncryptionConfigEl {
    encryption_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_names: Option<ListField<PrimField<String>>>,
}
impl SpannerBackupScheduleEncryptionConfigEl {
    #[doc = "Set the field `kms_key_name`.\nThe resource name of the Cloud KMS key to use for encryption.\nFormat: 'projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}'"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_names`.\nFully qualified name of the KMS keys to use to encrypt this database. The keys must exist\nin the same locations as the Spanner Database."]
    pub fn set_kms_key_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.kms_key_names = Some(v.into());
        self
    }
}
impl ToListMappable for SpannerBackupScheduleEncryptionConfigEl {
    type O = BlockAssignable<SpannerBackupScheduleEncryptionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerBackupScheduleEncryptionConfigEl {
    #[doc = "The encryption type of backups created by the backup schedule.\nPossible values are USE_DATABASE_ENCRYPTION, GOOGLE_DEFAULT_ENCRYPTION, or CUSTOMER_MANAGED_ENCRYPTION.\nIf you use CUSTOMER_MANAGED_ENCRYPTION, you must specify a kmsKeyName or kmsKeyNames. Possible values: [\"USE_DATABASE_ENCRYPTION\", \"GOOGLE_DEFAULT_ENCRYPTION\", \"CUSTOMER_MANAGED_ENCRYPTION\"]"]
    pub encryption_type: PrimField<String>,
}
impl BuildSpannerBackupScheduleEncryptionConfigEl {
    pub fn build(self) -> SpannerBackupScheduleEncryptionConfigEl {
        SpannerBackupScheduleEncryptionConfigEl {
            encryption_type: self.encryption_type,
            kms_key_name: core::default::Default::default(),
            kms_key_names: core::default::Default::default(),
        }
    }
}
pub struct SpannerBackupScheduleEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerBackupScheduleEncryptionConfigElRef {
    fn new(shared: StackShared, base: String) -> SpannerBackupScheduleEncryptionConfigElRef {
        SpannerBackupScheduleEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerBackupScheduleEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\nThe encryption type of backups created by the backup schedule.\nPossible values are USE_DATABASE_ENCRYPTION, GOOGLE_DEFAULT_ENCRYPTION, or CUSTOMER_MANAGED_ENCRYPTION.\nIf you use CUSTOMER_MANAGED_ENCRYPTION, you must specify a kmsKeyName or kmsKeyNames. Possible values: [\"USE_DATABASE_ENCRYPTION\", \"GOOGLE_DEFAULT_ENCRYPTION\", \"CUSTOMER_MANAGED_ENCRYPTION\"]"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name of the Cloud KMS key to use for encryption.\nFormat: 'projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}'"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_names` after provisioning.\nFully qualified name of the KMS keys to use to encrypt this database. The keys must exist\nin the same locations as the Spanner Database."]
    pub fn kms_key_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kms_key_names", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SpannerBackupScheduleFullBackupSpecEl {}
impl SpannerBackupScheduleFullBackupSpecEl {}
impl ToListMappable for SpannerBackupScheduleFullBackupSpecEl {
    type O = BlockAssignable<SpannerBackupScheduleFullBackupSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerBackupScheduleFullBackupSpecEl {}
impl BuildSpannerBackupScheduleFullBackupSpecEl {
    pub fn build(self) -> SpannerBackupScheduleFullBackupSpecEl {
        SpannerBackupScheduleFullBackupSpecEl {}
    }
}
pub struct SpannerBackupScheduleFullBackupSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerBackupScheduleFullBackupSpecElRef {
    fn new(shared: StackShared, base: String) -> SpannerBackupScheduleFullBackupSpecElRef {
        SpannerBackupScheduleFullBackupSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerBackupScheduleFullBackupSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct SpannerBackupScheduleIncrementalBackupSpecEl {}
impl SpannerBackupScheduleIncrementalBackupSpecEl {}
impl ToListMappable for SpannerBackupScheduleIncrementalBackupSpecEl {
    type O = BlockAssignable<SpannerBackupScheduleIncrementalBackupSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerBackupScheduleIncrementalBackupSpecEl {}
impl BuildSpannerBackupScheduleIncrementalBackupSpecEl {
    pub fn build(self) -> SpannerBackupScheduleIncrementalBackupSpecEl {
        SpannerBackupScheduleIncrementalBackupSpecEl {}
    }
}
pub struct SpannerBackupScheduleIncrementalBackupSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerBackupScheduleIncrementalBackupSpecElRef {
    fn new(shared: StackShared, base: String) -> SpannerBackupScheduleIncrementalBackupSpecElRef {
        SpannerBackupScheduleIncrementalBackupSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerBackupScheduleIncrementalBackupSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct SpannerBackupScheduleSpecElCronSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl SpannerBackupScheduleSpecElCronSpecEl {
    #[doc = "Set the field `text`.\nTextual representation of the crontab. User can customize the\nbackup frequency and the backup version time using the cron\nexpression. The version time must be in UTC timzeone.\nThe backup will contain an externally consistent copy of the\ndatabase at the version time. Allowed frequencies are 12 hour, 1 day,\n1 week and 1 month. Examples of valid cron specifications:\n  0 2/12 * * * : every 12 hours at (2, 14) hours past midnight in UTC.\n  0 2,14 * * * : every 12 hours at (2,14) hours past midnight in UTC.\n  0 2 * * *    : once a day at 2 past midnight in UTC.\n  0 2 * * 0    : once a week every Sunday at 2 past midnight in UTC.\n  0 2 8 * *    : once a month on 8th day at 2 past midnight in UTC."]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for SpannerBackupScheduleSpecElCronSpecEl {
    type O = BlockAssignable<SpannerBackupScheduleSpecElCronSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerBackupScheduleSpecElCronSpecEl {}
impl BuildSpannerBackupScheduleSpecElCronSpecEl {
    pub fn build(self) -> SpannerBackupScheduleSpecElCronSpecEl {
        SpannerBackupScheduleSpecElCronSpecEl {
            text: core::default::Default::default(),
        }
    }
}
pub struct SpannerBackupScheduleSpecElCronSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerBackupScheduleSpecElCronSpecElRef {
    fn new(shared: StackShared, base: String) -> SpannerBackupScheduleSpecElCronSpecElRef {
        SpannerBackupScheduleSpecElCronSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerBackupScheduleSpecElCronSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\nTextual representation of the crontab. User can customize the\nbackup frequency and the backup version time using the cron\nexpression. The version time must be in UTC timzeone.\nThe backup will contain an externally consistent copy of the\ndatabase at the version time. Allowed frequencies are 12 hour, 1 day,\n1 week and 1 month. Examples of valid cron specifications:\n  0 2/12 * * * : every 12 hours at (2, 14) hours past midnight in UTC.\n  0 2,14 * * * : every 12 hours at (2,14) hours past midnight in UTC.\n  0 2 * * *    : once a day at 2 past midnight in UTC.\n  0 2 * * 0    : once a week every Sunday at 2 past midnight in UTC.\n  0 2 8 * *    : once a month on 8th day at 2 past midnight in UTC."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize, Default)]
struct SpannerBackupScheduleSpecElDynamic {
    cron_spec: Option<DynamicBlock<SpannerBackupScheduleSpecElCronSpecEl>>,
}
#[derive(Serialize)]
pub struct SpannerBackupScheduleSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cron_spec: Option<Vec<SpannerBackupScheduleSpecElCronSpecEl>>,
    dynamic: SpannerBackupScheduleSpecElDynamic,
}
impl SpannerBackupScheduleSpecEl {
    #[doc = "Set the field `cron_spec`.\n"]
    pub fn set_cron_spec(
        mut self,
        v: impl Into<BlockAssignable<SpannerBackupScheduleSpecElCronSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cron_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cron_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SpannerBackupScheduleSpecEl {
    type O = BlockAssignable<SpannerBackupScheduleSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerBackupScheduleSpecEl {}
impl BuildSpannerBackupScheduleSpecEl {
    pub fn build(self) -> SpannerBackupScheduleSpecEl {
        SpannerBackupScheduleSpecEl {
            cron_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SpannerBackupScheduleSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerBackupScheduleSpecElRef {
    fn new(shared: StackShared, base: String) -> SpannerBackupScheduleSpecElRef {
        SpannerBackupScheduleSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerBackupScheduleSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cron_spec` after provisioning.\n"]
    pub fn cron_spec(&self) -> ListRef<SpannerBackupScheduleSpecElCronSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cron_spec", self.base))
    }
}
#[derive(Serialize)]
pub struct SpannerBackupScheduleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SpannerBackupScheduleTimeoutsEl {
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
impl ToListMappable for SpannerBackupScheduleTimeoutsEl {
    type O = BlockAssignable<SpannerBackupScheduleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerBackupScheduleTimeoutsEl {}
impl BuildSpannerBackupScheduleTimeoutsEl {
    pub fn build(self) -> SpannerBackupScheduleTimeoutsEl {
        SpannerBackupScheduleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SpannerBackupScheduleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerBackupScheduleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SpannerBackupScheduleTimeoutsElRef {
        SpannerBackupScheduleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerBackupScheduleTimeoutsElRef {
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
struct SpannerBackupScheduleDynamic {
    encryption_config: Option<DynamicBlock<SpannerBackupScheduleEncryptionConfigEl>>,
    full_backup_spec: Option<DynamicBlock<SpannerBackupScheduleFullBackupSpecEl>>,
    incremental_backup_spec: Option<DynamicBlock<SpannerBackupScheduleIncrementalBackupSpecEl>>,
    spec: Option<DynamicBlock<SpannerBackupScheduleSpecEl>>,
}
