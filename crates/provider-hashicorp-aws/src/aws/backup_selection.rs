use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct BackupSelectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    iam_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_resources: Option<SetField<PrimField<String>>>,
    plan_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<BackupSelectionConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selection_tag: Option<Vec<BackupSelectionSelectionTagEl>>,
    dynamic: BackupSelectionDynamic,
}

struct BackupSelection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BackupSelectionData>,
}

#[derive(Clone)]
pub struct BackupSelection(Rc<BackupSelection_>);

impl BackupSelection {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `not_resources`.\n"]
    pub fn set_not_resources(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().not_resources = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resources = Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(self, v: impl Into<BlockAssignable<BackupSelectionConditionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `selection_tag`.\n"]
    pub fn set_selection_tag(
        self,
        v: impl Into<BlockAssignable<BackupSelectionSelectionTagEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().selection_tag = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.selection_tag = Some(d);
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `not_resources` after provisioning.\n"]
    pub fn not_resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.not_resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `plan_id` after provisioning.\n"]
    pub fn plan_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plan_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }
}

impl Referable for BackupSelection {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for BackupSelection {}

impl ToListMappable for BackupSelection {
    type O = ListRef<BackupSelectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BackupSelection_ {
    fn extract_resource_type(&self) -> String {
        "aws_backup_selection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBackupSelection {
    pub tf_id: String,
    #[doc = ""]
    pub iam_role_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub plan_id: PrimField<String>,
}

impl BuildBackupSelection {
    pub fn build(self, stack: &mut Stack) -> BackupSelection {
        let out = BackupSelection(Rc::new(BackupSelection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BackupSelectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                iam_role_arn: self.iam_role_arn,
                id: core::default::Default::default(),
                name: self.name,
                not_resources: core::default::Default::default(),
                plan_id: self.plan_id,
                region: core::default::Default::default(),
                resources: core::default::Default::default(),
                condition: core::default::Default::default(),
                selection_tag: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BackupSelectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupSelectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl BackupSelectionRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `not_resources` after provisioning.\n"]
    pub fn not_resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.not_resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `plan_id` after provisioning.\n"]
    pub fn plan_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plan_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BackupSelectionConditionElStringEqualsEl {
    key: PrimField<String>,
    value: PrimField<String>,
}

impl BackupSelectionConditionElStringEqualsEl {}

impl ToListMappable for BackupSelectionConditionElStringEqualsEl {
    type O = BlockAssignable<BackupSelectionConditionElStringEqualsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupSelectionConditionElStringEqualsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildBackupSelectionConditionElStringEqualsEl {
    pub fn build(self) -> BackupSelectionConditionElStringEqualsEl {
        BackupSelectionConditionElStringEqualsEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct BackupSelectionConditionElStringEqualsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupSelectionConditionElStringEqualsElRef {
    fn new(shared: StackShared, base: String) -> BackupSelectionConditionElStringEqualsElRef {
        BackupSelectionConditionElStringEqualsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupSelectionConditionElStringEqualsElRef {
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
pub struct BackupSelectionConditionElStringLikeEl {
    key: PrimField<String>,
    value: PrimField<String>,
}

impl BackupSelectionConditionElStringLikeEl {}

impl ToListMappable for BackupSelectionConditionElStringLikeEl {
    type O = BlockAssignable<BackupSelectionConditionElStringLikeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupSelectionConditionElStringLikeEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildBackupSelectionConditionElStringLikeEl {
    pub fn build(self) -> BackupSelectionConditionElStringLikeEl {
        BackupSelectionConditionElStringLikeEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct BackupSelectionConditionElStringLikeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupSelectionConditionElStringLikeElRef {
    fn new(shared: StackShared, base: String) -> BackupSelectionConditionElStringLikeElRef {
        BackupSelectionConditionElStringLikeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupSelectionConditionElStringLikeElRef {
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
pub struct BackupSelectionConditionElStringNotEqualsEl {
    key: PrimField<String>,
    value: PrimField<String>,
}

impl BackupSelectionConditionElStringNotEqualsEl {}

impl ToListMappable for BackupSelectionConditionElStringNotEqualsEl {
    type O = BlockAssignable<BackupSelectionConditionElStringNotEqualsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupSelectionConditionElStringNotEqualsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildBackupSelectionConditionElStringNotEqualsEl {
    pub fn build(self) -> BackupSelectionConditionElStringNotEqualsEl {
        BackupSelectionConditionElStringNotEqualsEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct BackupSelectionConditionElStringNotEqualsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupSelectionConditionElStringNotEqualsElRef {
    fn new(shared: StackShared, base: String) -> BackupSelectionConditionElStringNotEqualsElRef {
        BackupSelectionConditionElStringNotEqualsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupSelectionConditionElStringNotEqualsElRef {
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
pub struct BackupSelectionConditionElStringNotLikeEl {
    key: PrimField<String>,
    value: PrimField<String>,
}

impl BackupSelectionConditionElStringNotLikeEl {}

impl ToListMappable for BackupSelectionConditionElStringNotLikeEl {
    type O = BlockAssignable<BackupSelectionConditionElStringNotLikeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupSelectionConditionElStringNotLikeEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildBackupSelectionConditionElStringNotLikeEl {
    pub fn build(self) -> BackupSelectionConditionElStringNotLikeEl {
        BackupSelectionConditionElStringNotLikeEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct BackupSelectionConditionElStringNotLikeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupSelectionConditionElStringNotLikeElRef {
    fn new(shared: StackShared, base: String) -> BackupSelectionConditionElStringNotLikeElRef {
        BackupSelectionConditionElStringNotLikeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupSelectionConditionElStringNotLikeElRef {
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
struct BackupSelectionConditionElDynamic {
    string_equals: Option<DynamicBlock<BackupSelectionConditionElStringEqualsEl>>,
    string_like: Option<DynamicBlock<BackupSelectionConditionElStringLikeEl>>,
    string_not_equals: Option<DynamicBlock<BackupSelectionConditionElStringNotEqualsEl>>,
    string_not_like: Option<DynamicBlock<BackupSelectionConditionElStringNotLikeEl>>,
}

#[derive(Serialize)]
pub struct BackupSelectionConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    string_equals: Option<Vec<BackupSelectionConditionElStringEqualsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_like: Option<Vec<BackupSelectionConditionElStringLikeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_not_equals: Option<Vec<BackupSelectionConditionElStringNotEqualsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_not_like: Option<Vec<BackupSelectionConditionElStringNotLikeEl>>,
    dynamic: BackupSelectionConditionElDynamic,
}

impl BackupSelectionConditionEl {
    #[doc = "Set the field `string_equals`.\n"]
    pub fn set_string_equals(
        mut self,
        v: impl Into<BlockAssignable<BackupSelectionConditionElStringEqualsEl>>,
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

    #[doc = "Set the field `string_like`.\n"]
    pub fn set_string_like(
        mut self,
        v: impl Into<BlockAssignable<BackupSelectionConditionElStringLikeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.string_like = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.string_like = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `string_not_equals`.\n"]
    pub fn set_string_not_equals(
        mut self,
        v: impl Into<BlockAssignable<BackupSelectionConditionElStringNotEqualsEl>>,
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

    #[doc = "Set the field `string_not_like`.\n"]
    pub fn set_string_not_like(
        mut self,
        v: impl Into<BlockAssignable<BackupSelectionConditionElStringNotLikeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.string_not_like = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.string_not_like = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BackupSelectionConditionEl {
    type O = BlockAssignable<BackupSelectionConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupSelectionConditionEl {}

impl BuildBackupSelectionConditionEl {
    pub fn build(self) -> BackupSelectionConditionEl {
        BackupSelectionConditionEl {
            string_equals: core::default::Default::default(),
            string_like: core::default::Default::default(),
            string_not_equals: core::default::Default::default(),
            string_not_like: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BackupSelectionConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupSelectionConditionElRef {
    fn new(shared: StackShared, base: String) -> BackupSelectionConditionElRef {
        BackupSelectionConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupSelectionConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BackupSelectionSelectionTagEl {
    key: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<String>,
}

impl BackupSelectionSelectionTagEl {}

impl ToListMappable for BackupSelectionSelectionTagEl {
    type O = BlockAssignable<BackupSelectionSelectionTagEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupSelectionSelectionTagEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildBackupSelectionSelectionTagEl {
    pub fn build(self) -> BackupSelectionSelectionTagEl {
        BackupSelectionSelectionTagEl {
            key: self.key,
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct BackupSelectionSelectionTagElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BackupSelectionSelectionTagElRef {
    fn new(shared: StackShared, base: String) -> BackupSelectionSelectionTagElRef {
        BackupSelectionSelectionTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupSelectionSelectionTagElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupSelectionDynamic {
    condition: Option<DynamicBlock<BackupSelectionConditionEl>>,
    selection_tag: Option<DynamicBlock<BackupSelectionSelectionTagEl>>,
}
