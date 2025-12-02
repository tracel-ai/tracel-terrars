use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Lexv2modelsSlotTypeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bot_id: PrimField<String>,
    bot_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    locale_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_slot_type_signature: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    composite_slot_type_setting: Option<Vec<Lexv2modelsSlotTypeCompositeSlotTypeSettingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_source_setting: Option<Vec<Lexv2modelsSlotTypeExternalSourceSettingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slot_type_values: Option<Vec<Lexv2modelsSlotTypeSlotTypeValuesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Lexv2modelsSlotTypeTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_selection_setting: Option<Vec<Lexv2modelsSlotTypeValueSelectionSettingEl>>,
    dynamic: Lexv2modelsSlotTypeDynamic,
}
struct Lexv2modelsSlotType_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Lexv2modelsSlotTypeData>,
}
#[derive(Clone)]
pub struct Lexv2modelsSlotType(Rc<Lexv2modelsSlotType_>);
impl Lexv2modelsSlotType {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `parent_slot_type_signature`.\n"]
    pub fn set_parent_slot_type_signature(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent_slot_type_signature = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `composite_slot_type_setting`.\n"]
    pub fn set_composite_slot_type_setting(
        self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotTypeCompositeSlotTypeSettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().composite_slot_type_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.composite_slot_type_setting = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `external_source_setting`.\n"]
    pub fn set_external_source_setting(
        self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotTypeExternalSourceSettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_source_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_source_setting = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `slot_type_values`.\n"]
    pub fn set_slot_type_values(
        self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotTypeSlotTypeValuesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().slot_type_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.slot_type_values = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Lexv2modelsSlotTypeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `value_selection_setting`.\n"]
    pub fn set_value_selection_setting(
        self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotTypeValueSelectionSettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().value_selection_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.value_selection_setting = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `bot_id` after provisioning.\n"]
    pub fn bot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bot_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bot_version` after provisioning.\n"]
    pub fn bot_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bot_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `locale_id` after provisioning.\n"]
    pub fn locale_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.locale_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_slot_type_signature` after provisioning.\n"]
    pub fn parent_slot_type_signature(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_slot_type_signature", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slot_type_id` after provisioning.\n"]
    pub fn slot_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_type_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `composite_slot_type_setting` after provisioning.\n"]
    pub fn composite_slot_type_setting(
        &self,
    ) -> ListRef<Lexv2modelsSlotTypeCompositeSlotTypeSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.composite_slot_type_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_source_setting` after provisioning.\n"]
    pub fn external_source_setting(
        &self,
    ) -> ListRef<Lexv2modelsSlotTypeExternalSourceSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_source_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slot_type_values` after provisioning.\n"]
    pub fn slot_type_values(&self) -> ListRef<Lexv2modelsSlotTypeSlotTypeValuesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.slot_type_values", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Lexv2modelsSlotTypeTimeoutsElRef {
        Lexv2modelsSlotTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `value_selection_setting` after provisioning.\n"]
    pub fn value_selection_setting(
        &self,
    ) -> ListRef<Lexv2modelsSlotTypeValueSelectionSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.value_selection_setting", self.extract_ref()),
        )
    }
}
impl Referable for Lexv2modelsSlotType {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Lexv2modelsSlotType {}
impl ToListMappable for Lexv2modelsSlotType {
    type O = ListRef<Lexv2modelsSlotTypeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Lexv2modelsSlotType_ {
    fn extract_resource_type(&self) -> String {
        "aws_lexv2models_slot_type".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLexv2modelsSlotType {
    pub tf_id: String,
    #[doc = ""]
    pub bot_id: PrimField<String>,
    #[doc = ""]
    pub bot_version: PrimField<String>,
    #[doc = ""]
    pub locale_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildLexv2modelsSlotType {
    pub fn build(self, stack: &mut Stack) -> Lexv2modelsSlotType {
        let out = Lexv2modelsSlotType(Rc::new(Lexv2modelsSlotType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Lexv2modelsSlotTypeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bot_id: self.bot_id,
                bot_version: self.bot_version,
                description: core::default::Default::default(),
                locale_id: self.locale_id,
                name: self.name,
                parent_slot_type_signature: core::default::Default::default(),
                region: core::default::Default::default(),
                composite_slot_type_setting: core::default::Default::default(),
                external_source_setting: core::default::Default::default(),
                slot_type_values: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                value_selection_setting: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Lexv2modelsSlotTypeRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Lexv2modelsSlotTypeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bot_id` after provisioning.\n"]
    pub fn bot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bot_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bot_version` after provisioning.\n"]
    pub fn bot_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bot_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `locale_id` after provisioning.\n"]
    pub fn locale_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.locale_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_slot_type_signature` after provisioning.\n"]
    pub fn parent_slot_type_signature(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_slot_type_signature", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slot_type_id` after provisioning.\n"]
    pub fn slot_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_type_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `composite_slot_type_setting` after provisioning.\n"]
    pub fn composite_slot_type_setting(
        &self,
    ) -> ListRef<Lexv2modelsSlotTypeCompositeSlotTypeSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.composite_slot_type_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_source_setting` after provisioning.\n"]
    pub fn external_source_setting(
        &self,
    ) -> ListRef<Lexv2modelsSlotTypeExternalSourceSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_source_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slot_type_values` after provisioning.\n"]
    pub fn slot_type_values(&self) -> ListRef<Lexv2modelsSlotTypeSlotTypeValuesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.slot_type_values", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Lexv2modelsSlotTypeTimeoutsElRef {
        Lexv2modelsSlotTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `value_selection_setting` after provisioning.\n"]
    pub fn value_selection_setting(
        &self,
    ) -> ListRef<Lexv2modelsSlotTypeValueSelectionSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.value_selection_setting", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsEl {
    name: PrimField<String>,
    slot_type_id: PrimField<String>,
}
impl Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsEl {}
impl ToListMappable for Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsEl {
    type O = BlockAssignable<Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub slot_type_id: PrimField<String>,
}
impl BuildLexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsEl {
    pub fn build(self) -> Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsEl {
        Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsEl {
            name: self.name,
            slot_type_id: self.slot_type_id,
        }
    }
}
pub struct Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsElRef {
        Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `slot_type_id` after provisioning.\n"]
    pub fn slot_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slot_type_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotTypeCompositeSlotTypeSettingElDynamic {
    sub_slots: Option<DynamicBlock<Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsEl>>,
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTypeCompositeSlotTypeSettingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_slots: Option<Vec<Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsEl>>,
    dynamic: Lexv2modelsSlotTypeCompositeSlotTypeSettingElDynamic,
}
impl Lexv2modelsSlotTypeCompositeSlotTypeSettingEl {
    #[doc = "Set the field `sub_slots`.\n"]
    pub fn set_sub_slots(
        mut self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sub_slots = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sub_slots = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Lexv2modelsSlotTypeCompositeSlotTypeSettingEl {
    type O = BlockAssignable<Lexv2modelsSlotTypeCompositeSlotTypeSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTypeCompositeSlotTypeSettingEl {}
impl BuildLexv2modelsSlotTypeCompositeSlotTypeSettingEl {
    pub fn build(self) -> Lexv2modelsSlotTypeCompositeSlotTypeSettingEl {
        Lexv2modelsSlotTypeCompositeSlotTypeSettingEl {
            sub_slots: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotTypeCompositeSlotTypeSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeCompositeSlotTypeSettingElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsSlotTypeCompositeSlotTypeSettingElRef {
        Lexv2modelsSlotTypeCompositeSlotTypeSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTypeCompositeSlotTypeSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sub_slots` after provisioning.\n"]
    pub fn sub_slots(&self) -> ListRef<Lexv2modelsSlotTypeCompositeSlotTypeSettingElSubSlotsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sub_slots", self.base))
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceEl {
    kms_key_arn: PrimField<String>,
    s3_bucket_name: PrimField<String>,
    s3_object_key: PrimField<String>,
}
impl Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceEl {}
impl ToListMappable for Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceEl {
    type O =
        BlockAssignable<Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceEl {
    #[doc = ""]
    pub kms_key_arn: PrimField<String>,
    #[doc = ""]
    pub s3_bucket_name: PrimField<String>,
    #[doc = ""]
    pub s3_object_key: PrimField<String>,
}
impl BuildLexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceEl {
    pub fn build(
        self,
    ) -> Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceEl {
        Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceEl {
            kms_key_arn: self.kms_key_arn,
            s3_bucket_name: self.s3_bucket_name,
            s3_object_key: self.s3_object_key,
        }
    }
}
pub struct Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceElRef {
        Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_object_key` after provisioning.\n"]
    pub fn s3_object_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_object_key", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElDynamic {
    source: Option<
        DynamicBlock<Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceEl>,
    >,
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceEl>>,
    dynamic: Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElDynamic,
}
impl Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingEl {
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl Into<
            BlockAssignable<
                Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingEl {
    type O = BlockAssignable<Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingEl {}
impl BuildLexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingEl {
    pub fn build(self) -> Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingEl {
        Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingEl {
            source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElRef {
        Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> ListRef<Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElSourceElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotTypeExternalSourceSettingElDynamic {
    grammar_slot_type_setting:
        Option<DynamicBlock<Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingEl>>,
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTypeExternalSourceSettingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grammar_slot_type_setting:
        Option<Vec<Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingEl>>,
    dynamic: Lexv2modelsSlotTypeExternalSourceSettingElDynamic,
}
impl Lexv2modelsSlotTypeExternalSourceSettingEl {
    #[doc = "Set the field `grammar_slot_type_setting`.\n"]
    pub fn set_grammar_slot_type_setting(
        mut self,
        v: impl Into<
            BlockAssignable<Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grammar_slot_type_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grammar_slot_type_setting = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Lexv2modelsSlotTypeExternalSourceSettingEl {
    type O = BlockAssignable<Lexv2modelsSlotTypeExternalSourceSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTypeExternalSourceSettingEl {}
impl BuildLexv2modelsSlotTypeExternalSourceSettingEl {
    pub fn build(self) -> Lexv2modelsSlotTypeExternalSourceSettingEl {
        Lexv2modelsSlotTypeExternalSourceSettingEl {
            grammar_slot_type_setting: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotTypeExternalSourceSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeExternalSourceSettingElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsSlotTypeExternalSourceSettingElRef {
        Lexv2modelsSlotTypeExternalSourceSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTypeExternalSourceSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `grammar_slot_type_setting` after provisioning.\n"]
    pub fn grammar_slot_type_setting(
        &self,
    ) -> ListRef<Lexv2modelsSlotTypeExternalSourceSettingElGrammarSlotTypeSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grammar_slot_type_setting", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTypeSlotTypeValuesElSampleValueEl {
    value: PrimField<String>,
}
impl Lexv2modelsSlotTypeSlotTypeValuesElSampleValueEl {}
impl ToListMappable for Lexv2modelsSlotTypeSlotTypeValuesElSampleValueEl {
    type O = BlockAssignable<Lexv2modelsSlotTypeSlotTypeValuesElSampleValueEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTypeSlotTypeValuesElSampleValueEl {
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotTypeSlotTypeValuesElSampleValueEl {
    pub fn build(self) -> Lexv2modelsSlotTypeSlotTypeValuesElSampleValueEl {
        Lexv2modelsSlotTypeSlotTypeValuesElSampleValueEl { value: self.value }
    }
}
pub struct Lexv2modelsSlotTypeSlotTypeValuesElSampleValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeSlotTypeValuesElSampleValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotTypeSlotTypeValuesElSampleValueElRef {
        Lexv2modelsSlotTypeSlotTypeValuesElSampleValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTypeSlotTypeValuesElSampleValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTypeSlotTypeValuesElSynonymsEl {
    value: PrimField<String>,
}
impl Lexv2modelsSlotTypeSlotTypeValuesElSynonymsEl {}
impl ToListMappable for Lexv2modelsSlotTypeSlotTypeValuesElSynonymsEl {
    type O = BlockAssignable<Lexv2modelsSlotTypeSlotTypeValuesElSynonymsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTypeSlotTypeValuesElSynonymsEl {
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotTypeSlotTypeValuesElSynonymsEl {
    pub fn build(self) -> Lexv2modelsSlotTypeSlotTypeValuesElSynonymsEl {
        Lexv2modelsSlotTypeSlotTypeValuesElSynonymsEl { value: self.value }
    }
}
pub struct Lexv2modelsSlotTypeSlotTypeValuesElSynonymsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeSlotTypeValuesElSynonymsElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsSlotTypeSlotTypeValuesElSynonymsElRef {
        Lexv2modelsSlotTypeSlotTypeValuesElSynonymsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTypeSlotTypeValuesElSynonymsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotTypeSlotTypeValuesElDynamic {
    sample_value: Option<DynamicBlock<Lexv2modelsSlotTypeSlotTypeValuesElSampleValueEl>>,
    synonyms: Option<DynamicBlock<Lexv2modelsSlotTypeSlotTypeValuesElSynonymsEl>>,
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTypeSlotTypeValuesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_value: Option<Vec<Lexv2modelsSlotTypeSlotTypeValuesElSampleValueEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    synonyms: Option<Vec<Lexv2modelsSlotTypeSlotTypeValuesElSynonymsEl>>,
    dynamic: Lexv2modelsSlotTypeSlotTypeValuesElDynamic,
}
impl Lexv2modelsSlotTypeSlotTypeValuesEl {
    #[doc = "Set the field `sample_value`.\n"]
    pub fn set_sample_value(
        mut self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotTypeSlotTypeValuesElSampleValueEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sample_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sample_value = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `synonyms`.\n"]
    pub fn set_synonyms(
        mut self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotTypeSlotTypeValuesElSynonymsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.synonyms = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.synonyms = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Lexv2modelsSlotTypeSlotTypeValuesEl {
    type O = BlockAssignable<Lexv2modelsSlotTypeSlotTypeValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTypeSlotTypeValuesEl {}
impl BuildLexv2modelsSlotTypeSlotTypeValuesEl {
    pub fn build(self) -> Lexv2modelsSlotTypeSlotTypeValuesEl {
        Lexv2modelsSlotTypeSlotTypeValuesEl {
            sample_value: core::default::Default::default(),
            synonyms: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotTypeSlotTypeValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeSlotTypeValuesElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsSlotTypeSlotTypeValuesElRef {
        Lexv2modelsSlotTypeSlotTypeValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTypeSlotTypeValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `sample_value` after provisioning.\n"]
    pub fn sample_value(&self) -> ListRef<Lexv2modelsSlotTypeSlotTypeValuesElSampleValueElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sample_value", self.base))
    }
    #[doc = "Get a reference to the value of field `synonyms` after provisioning.\n"]
    pub fn synonyms(&self) -> ListRef<Lexv2modelsSlotTypeSlotTypeValuesElSynonymsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.synonyms", self.base))
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTypeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl Lexv2modelsSlotTypeTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for Lexv2modelsSlotTypeTimeoutsEl {
    type O = BlockAssignable<Lexv2modelsSlotTypeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTypeTimeoutsEl {}
impl BuildLexv2modelsSlotTypeTimeoutsEl {
    pub fn build(self) -> Lexv2modelsSlotTypeTimeoutsEl {
        Lexv2modelsSlotTypeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotTypeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsSlotTypeTimeoutsElRef {
        Lexv2modelsSlotTypeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTypeTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_recognition_strategy: Option<PrimField<String>>,
}
impl Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingEl {
    #[doc = "Set the field `audio_recognition_strategy`.\n"]
    pub fn set_audio_recognition_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audio_recognition_strategy = Some(v.into());
        self
    }
}
impl ToListMappable for Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingEl {
    type O =
        BlockAssignable<Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingEl {}
impl BuildLexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingEl {
    pub fn build(self) -> Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingEl {
        Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingEl {
            audio_recognition_strategy: core::default::Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingElRef {
        Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audio_recognition_strategy` after provisioning.\n"]
    pub fn audio_recognition_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.audio_recognition_strategy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterEl {
    pattern: PrimField<String>,
}
impl Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterEl {}
impl ToListMappable for Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterEl {
    type O = BlockAssignable<Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTypeValueSelectionSettingElRegexFilterEl {
    #[doc = ""]
    pub pattern: PrimField<String>,
}
impl BuildLexv2modelsSlotTypeValueSelectionSettingElRegexFilterEl {
    pub fn build(self) -> Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterEl {
        Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterEl {
            pattern: self.pattern,
        }
    }
}
pub struct Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterElRef {
        Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotTypeValueSelectionSettingElDynamic {
    advanced_recognition_setting: Option<
        DynamicBlock<Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingEl>,
    >,
    regex_filter: Option<DynamicBlock<Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterEl>>,
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTypeValueSelectionSettingEl {
    resolution_strategy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_recognition_setting:
        Option<Vec<Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_filter: Option<Vec<Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterEl>>,
    dynamic: Lexv2modelsSlotTypeValueSelectionSettingElDynamic,
}
impl Lexv2modelsSlotTypeValueSelectionSettingEl {
    #[doc = "Set the field `advanced_recognition_setting`.\n"]
    pub fn set_advanced_recognition_setting(
        mut self,
        v: impl Into<
            BlockAssignable<Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_recognition_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_recognition_setting = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `regex_filter`.\n"]
    pub fn set_regex_filter(
        mut self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.regex_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_filter = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Lexv2modelsSlotTypeValueSelectionSettingEl {
    type O = BlockAssignable<Lexv2modelsSlotTypeValueSelectionSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTypeValueSelectionSettingEl {
    #[doc = ""]
    pub resolution_strategy: PrimField<String>,
}
impl BuildLexv2modelsSlotTypeValueSelectionSettingEl {
    pub fn build(self) -> Lexv2modelsSlotTypeValueSelectionSettingEl {
        Lexv2modelsSlotTypeValueSelectionSettingEl {
            resolution_strategy: self.resolution_strategy,
            advanced_recognition_setting: core::default::Default::default(),
            regex_filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotTypeValueSelectionSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTypeValueSelectionSettingElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsSlotTypeValueSelectionSettingElRef {
        Lexv2modelsSlotTypeValueSelectionSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTypeValueSelectionSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resolution_strategy` after provisioning.\n"]
    pub fn resolution_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resolution_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_recognition_setting` after provisioning.\n"]
    pub fn advanced_recognition_setting(
        &self,
    ) -> ListRef<Lexv2modelsSlotTypeValueSelectionSettingElAdvancedRecognitionSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_recognition_setting", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `regex_filter` after provisioning.\n"]
    pub fn regex_filter(
        &self,
    ) -> ListRef<Lexv2modelsSlotTypeValueSelectionSettingElRegexFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.regex_filter", self.base))
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotTypeDynamic {
    composite_slot_type_setting:
        Option<DynamicBlock<Lexv2modelsSlotTypeCompositeSlotTypeSettingEl>>,
    external_source_setting: Option<DynamicBlock<Lexv2modelsSlotTypeExternalSourceSettingEl>>,
    slot_type_values: Option<DynamicBlock<Lexv2modelsSlotTypeSlotTypeValuesEl>>,
    value_selection_setting: Option<DynamicBlock<Lexv2modelsSlotTypeValueSelectionSettingEl>>,
}
