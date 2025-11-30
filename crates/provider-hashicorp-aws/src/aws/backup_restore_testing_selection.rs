use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct BackupRestoreTestingSelectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    iam_role_arn: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected_resource_arns: Option<SetField<PrimField<String>>>,
    protected_resource_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_metadata_overrides: Option<RecField<PrimField<String>>>,
    restore_testing_plan_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_window_hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected_resource_conditions:
        Option<Vec<BackupRestoreTestingSelectionProtectedResourceConditionsEl>>,
    dynamic: BackupRestoreTestingSelectionDynamic,
}

struct BackupRestoreTestingSelection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BackupRestoreTestingSelectionData>,
}

#[derive(Clone)]
pub struct BackupRestoreTestingSelection(Rc<BackupRestoreTestingSelection_>);

impl BackupRestoreTestingSelection {
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

    #[doc = "Set the field `protected_resource_arns`.\n"]
    pub fn set_protected_resource_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().protected_resource_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `restore_metadata_overrides`.\n"]
    pub fn set_restore_metadata_overrides(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().restore_metadata_overrides = Some(v.into());
        self
    }

    #[doc = "Set the field `validation_window_hours`.\n"]
    pub fn set_validation_window_hours(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().validation_window_hours = Some(v.into());
        self
    }

    #[doc = "Set the field `protected_resource_conditions`.\n"]
    pub fn set_protected_resource_conditions(
        self,
        v: impl Into<BlockAssignable<BackupRestoreTestingSelectionProtectedResourceConditionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().protected_resource_conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .protected_resource_conditions = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protected_resource_arns` after provisioning.\n"]
    pub fn protected_resource_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.protected_resource_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protected_resource_type` after provisioning.\n"]
    pub fn protected_resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protected_resource_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `restore_metadata_overrides` after provisioning.\n"]
    pub fn restore_metadata_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.restore_metadata_overrides", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `restore_testing_plan_name` after provisioning.\n"]
    pub fn restore_testing_plan_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restore_testing_plan_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `validation_window_hours` after provisioning.\n"]
    pub fn validation_window_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.validation_window_hours", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protected_resource_conditions` after provisioning.\n"]
    pub fn protected_resource_conditions(
        &self,
    ) -> ListRef<BackupRestoreTestingSelectionProtectedResourceConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protected_resource_conditions", self.extract_ref()),
        )
    }
}

impl Referable for BackupRestoreTestingSelection {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for BackupRestoreTestingSelection {}

impl ToListMappable for BackupRestoreTestingSelection {
    type O = ListRef<BackupRestoreTestingSelectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BackupRestoreTestingSelection_ {
    fn extract_resource_type(&self) -> String {
        "aws_backup_restore_testing_selection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBackupRestoreTestingSelection {
    pub tf_id: String,
    #[doc = ""]
    pub iam_role_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub protected_resource_type: PrimField<String>,
    #[doc = ""]
    pub restore_testing_plan_name: PrimField<String>,
}

impl BuildBackupRestoreTestingSelection {
    pub fn build(self, stack: &mut Stack) -> BackupRestoreTestingSelection {
        let out = BackupRestoreTestingSelection(Rc::new(BackupRestoreTestingSelection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BackupRestoreTestingSelectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                iam_role_arn: self.iam_role_arn,
                name: self.name,
                protected_resource_arns: core::default::Default::default(),
                protected_resource_type: self.protected_resource_type,
                region: core::default::Default::default(),
                restore_metadata_overrides: core::default::Default::default(),
                restore_testing_plan_name: self.restore_testing_plan_name,
                validation_window_hours: core::default::Default::default(),
                protected_resource_conditions: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BackupRestoreTestingSelectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupRestoreTestingSelectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl BackupRestoreTestingSelectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protected_resource_arns` after provisioning.\n"]
    pub fn protected_resource_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.protected_resource_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protected_resource_type` after provisioning.\n"]
    pub fn protected_resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protected_resource_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `restore_metadata_overrides` after provisioning.\n"]
    pub fn restore_metadata_overrides(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.restore_metadata_overrides", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `restore_testing_plan_name` after provisioning.\n"]
    pub fn restore_testing_plan_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restore_testing_plan_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `validation_window_hours` after provisioning.\n"]
    pub fn validation_window_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.validation_window_hours", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protected_resource_conditions` after provisioning.\n"]
    pub fn protected_resource_conditions(
        &self,
    ) -> ListRef<BackupRestoreTestingSelectionProtectedResourceConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protected_resource_conditions", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsEl {
    key: PrimField<String>,
    value: PrimField<String>,
}

impl BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsEl {}

impl ToListMappable for BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsEl {
    type O =
        BlockAssignable<BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildBackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsEl {
    pub fn build(self) -> BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsEl {
        BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsElRef {
        BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsElRef {
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
pub struct BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsEl {
    key: PrimField<String>,
    value: PrimField<String>,
}

impl BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsEl {}

impl ToListMappable
    for BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsEl
{
    type O = BlockAssignable<
        BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildBackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsEl {
    pub fn build(
        self,
    ) -> BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsEl {
        BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsElRef {
        BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsElRef {
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
struct BackupRestoreTestingSelectionProtectedResourceConditionsElDynamic {
    string_equals: Option<
        DynamicBlock<BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsEl>,
    >,
    string_not_equals: Option<
        DynamicBlock<BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsEl>,
    >,
}

#[derive(Serialize)]
pub struct BackupRestoreTestingSelectionProtectedResourceConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    string_equals:
        Option<Vec<BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_not_equals:
        Option<Vec<BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsEl>>,
    dynamic: BackupRestoreTestingSelectionProtectedResourceConditionsElDynamic,
}

impl BackupRestoreTestingSelectionProtectedResourceConditionsEl {
    #[doc = "Set the field `string_equals`.\n"]
    pub fn set_string_equals(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.string_equals = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.string_equals = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `string_not_equals`.\n"]
    pub fn set_string_not_equals(
        mut self,
        v: impl Into<
            BlockAssignable<
                BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.string_not_equals = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.string_not_equals = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BackupRestoreTestingSelectionProtectedResourceConditionsEl {
    type O = BlockAssignable<BackupRestoreTestingSelectionProtectedResourceConditionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupRestoreTestingSelectionProtectedResourceConditionsEl {}

impl BuildBackupRestoreTestingSelectionProtectedResourceConditionsEl {
    pub fn build(self) -> BackupRestoreTestingSelectionProtectedResourceConditionsEl {
        BackupRestoreTestingSelectionProtectedResourceConditionsEl {
            string_equals: core::default::Default::default(),
            string_not_equals: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BackupRestoreTestingSelectionProtectedResourceConditionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupRestoreTestingSelectionProtectedResourceConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BackupRestoreTestingSelectionProtectedResourceConditionsElRef {
        BackupRestoreTestingSelectionProtectedResourceConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupRestoreTestingSelectionProtectedResourceConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `string_equals` after provisioning.\n"]
    pub fn string_equals(
        &self,
    ) -> ListRef<BackupRestoreTestingSelectionProtectedResourceConditionsElStringEqualsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.string_equals", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `string_not_equals` after provisioning.\n"]
    pub fn string_not_equals(
        &self,
    ) -> ListRef<BackupRestoreTestingSelectionProtectedResourceConditionsElStringNotEqualsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.string_not_equals", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct BackupRestoreTestingSelectionDynamic {
    protected_resource_conditions:
        Option<DynamicBlock<BackupRestoreTestingSelectionProtectedResourceConditionsEl>>,
}
