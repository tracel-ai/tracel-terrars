use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BackupPlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_backup_setting: Option<Vec<BackupPlanAdvancedBackupSettingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<BackupPlanRuleEl>>,
    dynamic: BackupPlanDynamic,
}

struct BackupPlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BackupPlanData>,
}

#[derive(Clone)]
pub struct BackupPlan(Rc<BackupPlan_>);

impl BackupPlan {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_backup_setting`.\n"]
    pub fn set_advanced_backup_setting(self, v: impl Into<BlockAssignable<BackupPlanAdvancedBackupSettingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_backup_setting = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_backup_setting = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl Into<BlockAssignable<BackupPlanRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl Referable for BackupPlan {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BackupPlan { }

impl ToListMappable for BackupPlan {
    type O = ListRef<BackupPlanRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BackupPlan_ {
    fn extract_resource_type(&self) -> String {
        "aws_backup_plan".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBackupPlan {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBackupPlan {
    pub fn build(self, stack: &mut Stack) -> BackupPlan {
        let out = BackupPlan(Rc::new(BackupPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BackupPlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                advanced_backup_setting: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BackupPlanRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupPlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl BackupPlanRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BackupPlanAdvancedBackupSettingEl {
    backup_options: RecField<PrimField<String>>,
    resource_type: PrimField<String>,
}

impl BackupPlanAdvancedBackupSettingEl { }

impl ToListMappable for BackupPlanAdvancedBackupSettingEl {
    type O = BlockAssignable<BackupPlanAdvancedBackupSettingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupPlanAdvancedBackupSettingEl {
    #[doc = ""]
    pub backup_options: RecField<PrimField<String>>,
    #[doc = ""]
    pub resource_type: PrimField<String>,
}

impl BuildBackupPlanAdvancedBackupSettingEl {
    pub fn build(self) -> BackupPlanAdvancedBackupSettingEl {
        BackupPlanAdvancedBackupSettingEl {
            backup_options: self.backup_options,
            resource_type: self.resource_type,
        }
    }
}

pub struct BackupPlanAdvancedBackupSettingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupPlanAdvancedBackupSettingElRef {
    fn new(shared: StackShared, base: String) -> BackupPlanAdvancedBackupSettingElRef {
        BackupPlanAdvancedBackupSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupPlanAdvancedBackupSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_options` after provisioning.\n"]
    pub fn backup_options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.backup_options", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupPlanRuleElCopyActionElLifecycleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cold_storage_after: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_after: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opt_in_to_archive_for_supported_resources: Option<PrimField<bool>>,
}

impl BackupPlanRuleElCopyActionElLifecycleEl {
    #[doc = "Set the field `cold_storage_after`.\n"]
    pub fn set_cold_storage_after(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cold_storage_after = Some(v.into());
        self
    }

    #[doc = "Set the field `delete_after`.\n"]
    pub fn set_delete_after(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.delete_after = Some(v.into());
        self
    }

    #[doc = "Set the field `opt_in_to_archive_for_supported_resources`.\n"]
    pub fn set_opt_in_to_archive_for_supported_resources(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.opt_in_to_archive_for_supported_resources = Some(v.into());
        self
    }
}

impl ToListMappable for BackupPlanRuleElCopyActionElLifecycleEl {
    type O = BlockAssignable<BackupPlanRuleElCopyActionElLifecycleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupPlanRuleElCopyActionElLifecycleEl {}

impl BuildBackupPlanRuleElCopyActionElLifecycleEl {
    pub fn build(self) -> BackupPlanRuleElCopyActionElLifecycleEl {
        BackupPlanRuleElCopyActionElLifecycleEl {
            cold_storage_after: core::default::Default::default(),
            delete_after: core::default::Default::default(),
            opt_in_to_archive_for_supported_resources: core::default::Default::default(),
        }
    }
}

pub struct BackupPlanRuleElCopyActionElLifecycleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupPlanRuleElCopyActionElLifecycleElRef {
    fn new(shared: StackShared, base: String) -> BackupPlanRuleElCopyActionElLifecycleElRef {
        BackupPlanRuleElCopyActionElLifecycleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupPlanRuleElCopyActionElLifecycleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cold_storage_after` after provisioning.\n"]
    pub fn cold_storage_after(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cold_storage_after", self.base))
    }

    #[doc = "Get a reference to the value of field `delete_after` after provisioning.\n"]
    pub fn delete_after(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_after", self.base))
    }

    #[doc = "Get a reference to the value of field `opt_in_to_archive_for_supported_resources` after provisioning.\n"]
    pub fn opt_in_to_archive_for_supported_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.opt_in_to_archive_for_supported_resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupPlanRuleElCopyActionElDynamic {
    lifecycle: Option<DynamicBlock<BackupPlanRuleElCopyActionElLifecycleEl>>,
}

#[derive(Serialize)]
pub struct BackupPlanRuleElCopyActionEl {
    destination_vault_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle: Option<Vec<BackupPlanRuleElCopyActionElLifecycleEl>>,
    dynamic: BackupPlanRuleElCopyActionElDynamic,
}

impl BackupPlanRuleElCopyActionEl {
    #[doc = "Set the field `lifecycle`.\n"]
    pub fn set_lifecycle(mut self, v: impl Into<BlockAssignable<BackupPlanRuleElCopyActionElLifecycleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lifecycle = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lifecycle = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BackupPlanRuleElCopyActionEl {
    type O = BlockAssignable<BackupPlanRuleElCopyActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupPlanRuleElCopyActionEl {
    #[doc = ""]
    pub destination_vault_arn: PrimField<String>,
}

impl BuildBackupPlanRuleElCopyActionEl {
    pub fn build(self) -> BackupPlanRuleElCopyActionEl {
        BackupPlanRuleElCopyActionEl {
            destination_vault_arn: self.destination_vault_arn,
            lifecycle: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BackupPlanRuleElCopyActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupPlanRuleElCopyActionElRef {
    fn new(shared: StackShared, base: String) -> BackupPlanRuleElCopyActionElRef {
        BackupPlanRuleElCopyActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupPlanRuleElCopyActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_vault_arn` after provisioning.\n"]
    pub fn destination_vault_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_vault_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle` after provisioning.\n"]
    pub fn lifecycle(&self) -> ListRef<BackupPlanRuleElCopyActionElLifecycleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupPlanRuleElLifecycleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cold_storage_after: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_after: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opt_in_to_archive_for_supported_resources: Option<PrimField<bool>>,
}

impl BackupPlanRuleElLifecycleEl {
    #[doc = "Set the field `cold_storage_after`.\n"]
    pub fn set_cold_storage_after(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cold_storage_after = Some(v.into());
        self
    }

    #[doc = "Set the field `delete_after`.\n"]
    pub fn set_delete_after(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.delete_after = Some(v.into());
        self
    }

    #[doc = "Set the field `opt_in_to_archive_for_supported_resources`.\n"]
    pub fn set_opt_in_to_archive_for_supported_resources(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.opt_in_to_archive_for_supported_resources = Some(v.into());
        self
    }
}

impl ToListMappable for BackupPlanRuleElLifecycleEl {
    type O = BlockAssignable<BackupPlanRuleElLifecycleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupPlanRuleElLifecycleEl {}

impl BuildBackupPlanRuleElLifecycleEl {
    pub fn build(self) -> BackupPlanRuleElLifecycleEl {
        BackupPlanRuleElLifecycleEl {
            cold_storage_after: core::default::Default::default(),
            delete_after: core::default::Default::default(),
            opt_in_to_archive_for_supported_resources: core::default::Default::default(),
        }
    }
}

pub struct BackupPlanRuleElLifecycleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupPlanRuleElLifecycleElRef {
    fn new(shared: StackShared, base: String) -> BackupPlanRuleElLifecycleElRef {
        BackupPlanRuleElLifecycleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupPlanRuleElLifecycleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cold_storage_after` after provisioning.\n"]
    pub fn cold_storage_after(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cold_storage_after", self.base))
    }

    #[doc = "Get a reference to the value of field `delete_after` after provisioning.\n"]
    pub fn delete_after(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_after", self.base))
    }

    #[doc = "Get a reference to the value of field `opt_in_to_archive_for_supported_resources` after provisioning.\n"]
    pub fn opt_in_to_archive_for_supported_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.opt_in_to_archive_for_supported_resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupPlanRuleElDynamic {
    copy_action: Option<DynamicBlock<BackupPlanRuleElCopyActionEl>>,
    lifecycle: Option<DynamicBlock<BackupPlanRuleElLifecycleEl>>,
}

#[derive(Serialize)]
pub struct BackupPlanRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    completion_window: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_continuous_backup: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recovery_point_tags: Option<RecField<PrimField<String>>>,
    rule_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_expression_timezone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_window: Option<PrimField<f64>>,
    target_vault_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_action: Option<Vec<BackupPlanRuleElCopyActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle: Option<Vec<BackupPlanRuleElLifecycleEl>>,
    dynamic: BackupPlanRuleElDynamic,
}

impl BackupPlanRuleEl {
    #[doc = "Set the field `completion_window`.\n"]
    pub fn set_completion_window(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.completion_window = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_continuous_backup`.\n"]
    pub fn set_enable_continuous_backup(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_continuous_backup = Some(v.into());
        self
    }

    #[doc = "Set the field `recovery_point_tags`.\n"]
    pub fn set_recovery_point_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.recovery_point_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule = Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_expression_timezone`.\n"]
    pub fn set_schedule_expression_timezone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule_expression_timezone = Some(v.into());
        self
    }

    #[doc = "Set the field `start_window`.\n"]
    pub fn set_start_window(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.start_window = Some(v.into());
        self
    }

    #[doc = "Set the field `copy_action`.\n"]
    pub fn set_copy_action(mut self, v: impl Into<BlockAssignable<BackupPlanRuleElCopyActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.copy_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.copy_action = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lifecycle`.\n"]
    pub fn set_lifecycle(mut self, v: impl Into<BlockAssignable<BackupPlanRuleElLifecycleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lifecycle = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lifecycle = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BackupPlanRuleEl {
    type O = BlockAssignable<BackupPlanRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupPlanRuleEl {
    #[doc = ""]
    pub rule_name: PrimField<String>,
    #[doc = ""]
    pub target_vault_name: PrimField<String>,
}

impl BuildBackupPlanRuleEl {
    pub fn build(self) -> BackupPlanRuleEl {
        BackupPlanRuleEl {
            completion_window: core::default::Default::default(),
            enable_continuous_backup: core::default::Default::default(),
            recovery_point_tags: core::default::Default::default(),
            rule_name: self.rule_name,
            schedule: core::default::Default::default(),
            schedule_expression_timezone: core::default::Default::default(),
            start_window: core::default::Default::default(),
            target_vault_name: self.target_vault_name,
            copy_action: core::default::Default::default(),
            lifecycle: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BackupPlanRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupPlanRuleElRef {
    fn new(shared: StackShared, base: String) -> BackupPlanRuleElRef {
        BackupPlanRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupPlanRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `completion_window` after provisioning.\n"]
    pub fn completion_window(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.completion_window", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_continuous_backup` after provisioning.\n"]
    pub fn enable_continuous_backup(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_continuous_backup", self.base))
    }

    #[doc = "Get a reference to the value of field `recovery_point_tags` after provisioning.\n"]
    pub fn recovery_point_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.recovery_point_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_expression_timezone` after provisioning.\n"]
    pub fn schedule_expression_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression_timezone", self.base))
    }

    #[doc = "Get a reference to the value of field `start_window` after provisioning.\n"]
    pub fn start_window(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_window", self.base))
    }

    #[doc = "Get a reference to the value of field `target_vault_name` after provisioning.\n"]
    pub fn target_vault_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_vault_name", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle` after provisioning.\n"]
    pub fn lifecycle(&self) -> ListRef<BackupPlanRuleElLifecycleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupPlanDynamic {
    advanced_backup_setting: Option<DynamicBlock<BackupPlanAdvancedBackupSettingEl>>,
    rule: Option<DynamicBlock<BackupPlanRuleEl>>,
}
