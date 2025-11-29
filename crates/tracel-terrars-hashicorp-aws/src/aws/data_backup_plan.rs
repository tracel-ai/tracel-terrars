use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBackupPlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    plan_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataBackupPlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBackupPlanData>,
}

#[derive(Clone)]
pub struct DataBackupPlan(Rc<DataBackupPlan_>);

impl DataBackupPlan {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `plan_id` after provisioning.\n"]
    pub fn plan_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plan_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> SetRef<DataBackupPlanRuleElRef> {
        SetRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl Referable for DataBackupPlan {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataBackupPlan { }

impl ToListMappable for DataBackupPlan {
    type O = ListRef<DataBackupPlanRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBackupPlan_ {
    fn extract_datasource_type(&self) -> String {
        "aws_backup_plan".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBackupPlan {
    pub tf_id: String,
    #[doc = ""]
    pub plan_id: PrimField<String>,
}

impl BuildDataBackupPlan {
    pub fn build(self, stack: &mut Stack) -> DataBackupPlan {
        let out = DataBackupPlan(Rc::new(DataBackupPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBackupPlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                plan_id: self.plan_id,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBackupPlanRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBackupPlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataBackupPlanRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `plan_id` after provisioning.\n"]
    pub fn plan_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plan_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> SetRef<DataBackupPlanRuleElRef> {
        SetRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBackupPlanRuleElCopyActionElLifecycleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cold_storage_after: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_after: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opt_in_to_archive_for_supported_resources: Option<PrimField<bool>>,
}

impl DataBackupPlanRuleElCopyActionElLifecycleEl {
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

impl ToListMappable for DataBackupPlanRuleElCopyActionElLifecycleEl {
    type O = BlockAssignable<DataBackupPlanRuleElCopyActionElLifecycleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupPlanRuleElCopyActionElLifecycleEl {}

impl BuildDataBackupPlanRuleElCopyActionElLifecycleEl {
    pub fn build(self) -> DataBackupPlanRuleElCopyActionElLifecycleEl {
        DataBackupPlanRuleElCopyActionElLifecycleEl {
            cold_storage_after: core::default::Default::default(),
            delete_after: core::default::Default::default(),
            opt_in_to_archive_for_supported_resources: core::default::Default::default(),
        }
    }
}

pub struct DataBackupPlanRuleElCopyActionElLifecycleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBackupPlanRuleElCopyActionElLifecycleElRef {
    fn new(shared: StackShared, base: String) -> DataBackupPlanRuleElCopyActionElLifecycleElRef {
        DataBackupPlanRuleElCopyActionElLifecycleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupPlanRuleElCopyActionElLifecycleElRef {
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

#[derive(Serialize)]
pub struct DataBackupPlanRuleElCopyActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_vault_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle: Option<ListField<DataBackupPlanRuleElCopyActionElLifecycleEl>>,
}

impl DataBackupPlanRuleElCopyActionEl {
    #[doc = "Set the field `destination_vault_arn`.\n"]
    pub fn set_destination_vault_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_vault_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle`.\n"]
    pub fn set_lifecycle(mut self, v: impl Into<ListField<DataBackupPlanRuleElCopyActionElLifecycleEl>>) -> Self {
        self.lifecycle = Some(v.into());
        self
    }
}

impl ToListMappable for DataBackupPlanRuleElCopyActionEl {
    type O = BlockAssignable<DataBackupPlanRuleElCopyActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupPlanRuleElCopyActionEl {}

impl BuildDataBackupPlanRuleElCopyActionEl {
    pub fn build(self) -> DataBackupPlanRuleElCopyActionEl {
        DataBackupPlanRuleElCopyActionEl {
            destination_vault_arn: core::default::Default::default(),
            lifecycle: core::default::Default::default(),
        }
    }
}

pub struct DataBackupPlanRuleElCopyActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBackupPlanRuleElCopyActionElRef {
    fn new(shared: StackShared, base: String) -> DataBackupPlanRuleElCopyActionElRef {
        DataBackupPlanRuleElCopyActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupPlanRuleElCopyActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_vault_arn` after provisioning.\n"]
    pub fn destination_vault_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_vault_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle` after provisioning.\n"]
    pub fn lifecycle(&self) -> ListRef<DataBackupPlanRuleElCopyActionElLifecycleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupPlanRuleElLifecycleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cold_storage_after: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_after: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opt_in_to_archive_for_supported_resources: Option<PrimField<bool>>,
}

impl DataBackupPlanRuleElLifecycleEl {
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

impl ToListMappable for DataBackupPlanRuleElLifecycleEl {
    type O = BlockAssignable<DataBackupPlanRuleElLifecycleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupPlanRuleElLifecycleEl {}

impl BuildDataBackupPlanRuleElLifecycleEl {
    pub fn build(self) -> DataBackupPlanRuleElLifecycleEl {
        DataBackupPlanRuleElLifecycleEl {
            cold_storage_after: core::default::Default::default(),
            delete_after: core::default::Default::default(),
            opt_in_to_archive_for_supported_resources: core::default::Default::default(),
        }
    }
}

pub struct DataBackupPlanRuleElLifecycleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBackupPlanRuleElLifecycleElRef {
    fn new(shared: StackShared, base: String) -> DataBackupPlanRuleElLifecycleElRef {
        DataBackupPlanRuleElLifecycleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupPlanRuleElLifecycleElRef {
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

#[derive(Serialize)]
pub struct DataBackupPlanRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    completion_window: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_action: Option<SetField<DataBackupPlanRuleElCopyActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_continuous_backup: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle: Option<ListField<DataBackupPlanRuleElLifecycleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recovery_point_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_expression_timezone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_window: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_vault_name: Option<PrimField<String>>,
}

impl DataBackupPlanRuleEl {
    #[doc = "Set the field `completion_window`.\n"]
    pub fn set_completion_window(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.completion_window = Some(v.into());
        self
    }

    #[doc = "Set the field `copy_action`.\n"]
    pub fn set_copy_action(mut self, v: impl Into<SetField<DataBackupPlanRuleElCopyActionEl>>) -> Self {
        self.copy_action = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_continuous_backup`.\n"]
    pub fn set_enable_continuous_backup(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_continuous_backup = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle`.\n"]
    pub fn set_lifecycle(mut self, v: impl Into<ListField<DataBackupPlanRuleElLifecycleEl>>) -> Self {
        self.lifecycle = Some(v.into());
        self
    }

    #[doc = "Set the field `recovery_point_tags`.\n"]
    pub fn set_recovery_point_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.recovery_point_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_name`.\n"]
    pub fn set_rule_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule_name = Some(v.into());
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

    #[doc = "Set the field `target_vault_name`.\n"]
    pub fn set_target_vault_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_vault_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataBackupPlanRuleEl {
    type O = BlockAssignable<DataBackupPlanRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupPlanRuleEl {}

impl BuildDataBackupPlanRuleEl {
    pub fn build(self) -> DataBackupPlanRuleEl {
        DataBackupPlanRuleEl {
            completion_window: core::default::Default::default(),
            copy_action: core::default::Default::default(),
            enable_continuous_backup: core::default::Default::default(),
            lifecycle: core::default::Default::default(),
            recovery_point_tags: core::default::Default::default(),
            rule_name: core::default::Default::default(),
            schedule: core::default::Default::default(),
            schedule_expression_timezone: core::default::Default::default(),
            start_window: core::default::Default::default(),
            target_vault_name: core::default::Default::default(),
        }
    }
}

pub struct DataBackupPlanRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBackupPlanRuleElRef {
    fn new(shared: StackShared, base: String) -> DataBackupPlanRuleElRef {
        DataBackupPlanRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupPlanRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `completion_window` after provisioning.\n"]
    pub fn completion_window(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.completion_window", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_action` after provisioning.\n"]
    pub fn copy_action(&self) -> SetRef<DataBackupPlanRuleElCopyActionElRef> {
        SetRef::new(self.shared().clone(), format!("{}.copy_action", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_continuous_backup` after provisioning.\n"]
    pub fn enable_continuous_backup(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_continuous_backup", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle` after provisioning.\n"]
    pub fn lifecycle(&self) -> ListRef<DataBackupPlanRuleElLifecycleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle", self.base))
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
}
