use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BackupRestoreTestingPlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    schedule_expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_expression_timezone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_window_hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recovery_point_selection: Option<Vec<BackupRestoreTestingPlanRecoveryPointSelectionEl>>,
    dynamic: BackupRestoreTestingPlanDynamic,
}

struct BackupRestoreTestingPlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BackupRestoreTestingPlanData>,
}

#[derive(Clone)]
pub struct BackupRestoreTestingPlan(Rc<BackupRestoreTestingPlan_>);

impl BackupRestoreTestingPlan {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_expression_timezone`.\n"]
    pub fn set_schedule_expression_timezone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().schedule_expression_timezone = Some(v.into());
        self
    }

    #[doc = "Set the field `start_window_hours`.\n"]
    pub fn set_start_window_hours(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().start_window_hours = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `recovery_point_selection`.\n"]
    pub fn set_recovery_point_selection(
        self,
        v: impl Into<BlockAssignable<BackupRestoreTestingPlanRecoveryPointSelectionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recovery_point_selection = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recovery_point_selection = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression_timezone` after provisioning.\n"]
    pub fn schedule_expression_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression_timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_window_hours` after provisioning.\n"]
    pub fn start_window_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_window_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recovery_point_selection` after provisioning.\n"]
    pub fn recovery_point_selection(&self) -> ListRef<BackupRestoreTestingPlanRecoveryPointSelectionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.recovery_point_selection", self.extract_ref()))
    }
}

impl Referable for BackupRestoreTestingPlan {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BackupRestoreTestingPlan { }

impl ToListMappable for BackupRestoreTestingPlan {
    type O = ListRef<BackupRestoreTestingPlanRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BackupRestoreTestingPlan_ {
    fn extract_resource_type(&self) -> String {
        "aws_backup_restore_testing_plan".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBackupRestoreTestingPlan {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub schedule_expression: PrimField<String>,
}

impl BuildBackupRestoreTestingPlan {
    pub fn build(self, stack: &mut Stack) -> BackupRestoreTestingPlan {
        let out = BackupRestoreTestingPlan(Rc::new(BackupRestoreTestingPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BackupRestoreTestingPlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                name: self.name,
                region: core::default::Default::default(),
                schedule_expression: self.schedule_expression,
                schedule_expression_timezone: core::default::Default::default(),
                start_window_hours: core::default::Default::default(),
                tags: core::default::Default::default(),
                recovery_point_selection: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BackupRestoreTestingPlanRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupRestoreTestingPlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl BackupRestoreTestingPlanRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression_timezone` after provisioning.\n"]
    pub fn schedule_expression_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression_timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_window_hours` after provisioning.\n"]
    pub fn start_window_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_window_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recovery_point_selection` after provisioning.\n"]
    pub fn recovery_point_selection(&self) -> ListRef<BackupRestoreTestingPlanRecoveryPointSelectionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.recovery_point_selection", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BackupRestoreTestingPlanRecoveryPointSelectionEl {
    algorithm: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_vaults: Option<SetField<PrimField<String>>>,
    include_vaults: SetField<PrimField<String>>,
    recovery_point_types: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selection_window_days: Option<PrimField<f64>>,
}

impl BackupRestoreTestingPlanRecoveryPointSelectionEl {
    #[doc = "Set the field `exclude_vaults`.\n"]
    pub fn set_exclude_vaults(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exclude_vaults = Some(v.into());
        self
    }

    #[doc = "Set the field `selection_window_days`.\n"]
    pub fn set_selection_window_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.selection_window_days = Some(v.into());
        self
    }
}

impl ToListMappable for BackupRestoreTestingPlanRecoveryPointSelectionEl {
    type O = BlockAssignable<BackupRestoreTestingPlanRecoveryPointSelectionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupRestoreTestingPlanRecoveryPointSelectionEl {
    #[doc = ""]
    pub algorithm: PrimField<String>,
    #[doc = ""]
    pub include_vaults: SetField<PrimField<String>>,
    #[doc = ""]
    pub recovery_point_types: SetField<PrimField<String>>,
}

impl BuildBackupRestoreTestingPlanRecoveryPointSelectionEl {
    pub fn build(self) -> BackupRestoreTestingPlanRecoveryPointSelectionEl {
        BackupRestoreTestingPlanRecoveryPointSelectionEl {
            algorithm: self.algorithm,
            exclude_vaults: core::default::Default::default(),
            include_vaults: self.include_vaults,
            recovery_point_types: self.recovery_point_types,
            selection_window_days: core::default::Default::default(),
        }
    }
}

pub struct BackupRestoreTestingPlanRecoveryPointSelectionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupRestoreTestingPlanRecoveryPointSelectionElRef {
    fn new(shared: StackShared, base: String) -> BackupRestoreTestingPlanRecoveryPointSelectionElRef {
        BackupRestoreTestingPlanRecoveryPointSelectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupRestoreTestingPlanRecoveryPointSelectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_vaults` after provisioning.\n"]
    pub fn exclude_vaults(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exclude_vaults", self.base))
    }

    #[doc = "Get a reference to the value of field `include_vaults` after provisioning.\n"]
    pub fn include_vaults(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.include_vaults", self.base))
    }

    #[doc = "Get a reference to the value of field `recovery_point_types` after provisioning.\n"]
    pub fn recovery_point_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.recovery_point_types", self.base))
    }

    #[doc = "Get a reference to the value of field `selection_window_days` after provisioning.\n"]
    pub fn selection_window_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.selection_window_days", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupRestoreTestingPlanDynamic {
    recovery_point_selection: Option<DynamicBlock<BackupRestoreTestingPlanRecoveryPointSelectionEl>>,
}
