use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Lexv2modelsSlotData {
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
    intent_id: PrimField<String>,
    locale_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slot_type_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiple_values_setting: Option<Vec<Lexv2modelsSlotMultipleValuesSettingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    obfuscation_setting: Option<Vec<Lexv2modelsSlotObfuscationSettingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_slot_setting: Option<Vec<Lexv2modelsSlotSubSlotSettingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Lexv2modelsSlotTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_elicitation_setting: Option<Vec<Lexv2modelsSlotValueElicitationSettingEl>>,
    dynamic: Lexv2modelsSlotDynamic,
}
struct Lexv2modelsSlot_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Lexv2modelsSlotData>,
}
#[derive(Clone)]
pub struct Lexv2modelsSlot(Rc<Lexv2modelsSlot_>);
impl Lexv2modelsSlot {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `slot_type_id`.\n"]
    pub fn set_slot_type_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().slot_type_id = Some(v.into());
        self
    }
    #[doc = "Set the field `multiple_values_setting`.\n"]
    pub fn set_multiple_values_setting(
        self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotMultipleValuesSettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().multiple_values_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.multiple_values_setting = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `obfuscation_setting`.\n"]
    pub fn set_obfuscation_setting(
        self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotObfuscationSettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().obfuscation_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.obfuscation_setting = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sub_slot_setting`.\n"]
    pub fn set_sub_slot_setting(
        self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotSubSlotSettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sub_slot_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sub_slot_setting = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Lexv2modelsSlotTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `value_elicitation_setting`.\n"]
    pub fn set_value_elicitation_setting(
        self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotValueElicitationSettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().value_elicitation_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.value_elicitation_setting = Some(d);
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
    #[doc = "Get a reference to the value of field `intent_id` after provisioning.\n"]
    pub fn intent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.intent_id", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slot_id` after provisioning.\n"]
    pub fn slot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slot_type_id` after provisioning.\n"]
    pub fn slot_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_type_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multiple_values_setting` after provisioning.\n"]
    pub fn multiple_values_setting(&self) -> ListRef<Lexv2modelsSlotMultipleValuesSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiple_values_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `obfuscation_setting` after provisioning.\n"]
    pub fn obfuscation_setting(&self) -> ListRef<Lexv2modelsSlotObfuscationSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.obfuscation_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sub_slot_setting` after provisioning.\n"]
    pub fn sub_slot_setting(&self) -> ListRef<Lexv2modelsSlotSubSlotSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sub_slot_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Lexv2modelsSlotTimeoutsElRef {
        Lexv2modelsSlotTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `value_elicitation_setting` after provisioning.\n"]
    pub fn value_elicitation_setting(
        &self,
    ) -> ListRef<Lexv2modelsSlotValueElicitationSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.value_elicitation_setting", self.extract_ref()),
        )
    }
}
impl Referable for Lexv2modelsSlot {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Lexv2modelsSlot {}
impl ToListMappable for Lexv2modelsSlot {
    type O = ListRef<Lexv2modelsSlotRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Lexv2modelsSlot_ {
    fn extract_resource_type(&self) -> String {
        "aws_lexv2models_slot".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLexv2modelsSlot {
    pub tf_id: String,
    #[doc = ""]
    pub bot_id: PrimField<String>,
    #[doc = ""]
    pub bot_version: PrimField<String>,
    #[doc = ""]
    pub intent_id: PrimField<String>,
    #[doc = ""]
    pub locale_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildLexv2modelsSlot {
    pub fn build(self, stack: &mut Stack) -> Lexv2modelsSlot {
        let out = Lexv2modelsSlot(Rc::new(Lexv2modelsSlot_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Lexv2modelsSlotData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bot_id: self.bot_id,
                bot_version: self.bot_version,
                description: core::default::Default::default(),
                intent_id: self.intent_id,
                locale_id: self.locale_id,
                name: self.name,
                region: core::default::Default::default(),
                slot_type_id: core::default::Default::default(),
                multiple_values_setting: core::default::Default::default(),
                obfuscation_setting: core::default::Default::default(),
                sub_slot_setting: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                value_elicitation_setting: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Lexv2modelsSlotRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Lexv2modelsSlotRef {
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
    #[doc = "Get a reference to the value of field `intent_id` after provisioning.\n"]
    pub fn intent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.intent_id", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slot_id` after provisioning.\n"]
    pub fn slot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slot_type_id` after provisioning.\n"]
    pub fn slot_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_type_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multiple_values_setting` after provisioning.\n"]
    pub fn multiple_values_setting(&self) -> ListRef<Lexv2modelsSlotMultipleValuesSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multiple_values_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `obfuscation_setting` after provisioning.\n"]
    pub fn obfuscation_setting(&self) -> ListRef<Lexv2modelsSlotObfuscationSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.obfuscation_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sub_slot_setting` after provisioning.\n"]
    pub fn sub_slot_setting(&self) -> ListRef<Lexv2modelsSlotSubSlotSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sub_slot_setting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Lexv2modelsSlotTimeoutsElRef {
        Lexv2modelsSlotTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `value_elicitation_setting` after provisioning.\n"]
    pub fn value_elicitation_setting(
        &self,
    ) -> ListRef<Lexv2modelsSlotValueElicitationSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.value_elicitation_setting", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotMultipleValuesSettingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_multiple_values: Option<PrimField<bool>>,
}
impl Lexv2modelsSlotMultipleValuesSettingEl {
    #[doc = "Set the field `allow_multiple_values`.\n"]
    pub fn set_allow_multiple_values(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_multiple_values = Some(v.into());
        self
    }
}
impl ToListMappable for Lexv2modelsSlotMultipleValuesSettingEl {
    type O = BlockAssignable<Lexv2modelsSlotMultipleValuesSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotMultipleValuesSettingEl {}
impl BuildLexv2modelsSlotMultipleValuesSettingEl {
    pub fn build(self) -> Lexv2modelsSlotMultipleValuesSettingEl {
        Lexv2modelsSlotMultipleValuesSettingEl {
            allow_multiple_values: core::default::Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotMultipleValuesSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotMultipleValuesSettingElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsSlotMultipleValuesSettingElRef {
        Lexv2modelsSlotMultipleValuesSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotMultipleValuesSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_multiple_values` after provisioning.\n"]
    pub fn allow_multiple_values(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_multiple_values", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotObfuscationSettingEl {
    obfuscation_setting_type: PrimField<String>,
}
impl Lexv2modelsSlotObfuscationSettingEl {}
impl ToListMappable for Lexv2modelsSlotObfuscationSettingEl {
    type O = BlockAssignable<Lexv2modelsSlotObfuscationSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotObfuscationSettingEl {
    #[doc = ""]
    pub obfuscation_setting_type: PrimField<String>,
}
impl BuildLexv2modelsSlotObfuscationSettingEl {
    pub fn build(self) -> Lexv2modelsSlotObfuscationSettingEl {
        Lexv2modelsSlotObfuscationSettingEl {
            obfuscation_setting_type: self.obfuscation_setting_type,
        }
    }
}
pub struct Lexv2modelsSlotObfuscationSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotObfuscationSettingElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsSlotObfuscationSettingElRef {
        Lexv2modelsSlotObfuscationSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotObfuscationSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `obfuscation_setting_type` after provisioning.\n"]
    pub fn obfuscation_setting_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.obfuscation_setting_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl
{
    default_value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl
{
    #[doc = ""]
    pub default_value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl { default_value : self . default_value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `default_value` after provisioning.\n"] pub fn default_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.default_value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDynamic { default_value_list : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationEl { # [serde (skip_serializing_if = "Option::is_none")] default_value_list : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationEl { # [doc = "Set the field `default_value_list`.\n"] pub fn set_default_value_list (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . default_value_list = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . default_value_list = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationEl { default_value_list : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `default_value_list` after provisioning.\n"] pub fn default_value_list (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElDefaultValueListElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.default_value_list" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElDynamic { message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl >> , variation : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupEl { # [serde (skip_serializing_if = "Option::is_none")] message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] variation : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupEl { # [doc = "Set the field `message`.\n"] pub fn set_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . message = Some (d) ; } } self } # [doc = "Set the field `variation`.\n"] pub fn set_variation (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . variation = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . variation = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupEl { message : core :: default :: Default :: default () , variation : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `message` after provisioning.\n"] pub fn message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.message" , self . base)) } # [doc = "Get a reference to the value of field `variation` after provisioning.\n"] pub fn variation (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.variation" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl
{
    allow_audio_input: PrimField<bool>,
    allow_dtmf_input: PrimField<bool>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl
{
    #[doc = ""]
    pub allow_audio_input: PrimField<bool>,
    #[doc = ""]
    pub allow_dtmf_input: PrimField<bool>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl { allow_audio_input : self . allow_audio_input , allow_dtmf_input : self . allow_dtmf_input , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_audio_input` after provisioning.\n"] pub fn allow_audio_input (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_audio_input" , self . base)) } # [doc = "Get a reference to the value of field `allow_dtmf_input` after provisioning.\n"] pub fn allow_dtmf_input (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_dtmf_input" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl
{
    end_timeout_ms: PrimField<f64>,
    max_length_ms: PrimField<f64>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl
{
    #[doc = ""]
    pub end_timeout_ms: PrimField<f64>,
    #[doc = ""]
    pub max_length_ms: PrimField<f64>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl { end_timeout_ms : self . end_timeout_ms , max_length_ms : self . max_length_ms , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `end_timeout_ms` after provisioning.\n"] pub fn end_timeout_ms (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.end_timeout_ms" , self . base)) } # [doc = "Get a reference to the value of field `max_length_ms` after provisioning.\n"] pub fn max_length_ms (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_length_ms" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl
{
    deletion_character: PrimField<String>,
    end_character: PrimField<String>,
    end_timeout_ms: PrimField<f64>,
    max_length: PrimField<f64>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl
{
    #[doc = ""]
    pub deletion_character: PrimField<String>,
    #[doc = ""]
    pub end_character: PrimField<String>,
    #[doc = ""]
    pub end_timeout_ms: PrimField<f64>,
    #[doc = ""]
    pub max_length: PrimField<f64>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl { deletion_character : self . deletion_character , end_character : self . end_character , end_timeout_ms : self . end_timeout_ms , max_length : self . max_length , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `deletion_character` after provisioning.\n"] pub fn deletion_character (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.deletion_character" , self . base)) } # [doc = "Get a reference to the value of field `end_character` after provisioning.\n"] pub fn end_character (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.end_character" , self . base)) } # [doc = "Get a reference to the value of field `end_timeout_ms` after provisioning.\n"] pub fn end_timeout_ms (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.end_timeout_ms" , self . base)) } # [doc = "Get a reference to the value of field `max_length` after provisioning.\n"] pub fn max_length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_length" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDynamic { audio_specification : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl >> , dtmf_specification : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl { start_timeout_ms : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] audio_specification : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl > > , # [serde (skip_serializing_if = "Option::is_none")] dtmf_specification : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl { # [doc = "Set the field `audio_specification`.\n"] pub fn set_audio_specification (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . audio_specification = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . audio_specification = Some (d) ; } } self } # [doc = "Set the field `dtmf_specification`.\n"] pub fn set_dtmf_specification (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dtmf_specification = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dtmf_specification = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl
{
    #[doc = ""]
    pub start_timeout_ms: PrimField<f64>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl { start_timeout_ms : self . start_timeout_ms , audio_specification : core :: default :: Default :: default () , dtmf_specification : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `start_timeout_ms` after provisioning.\n"] pub fn start_timeout_ms (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.start_timeout_ms" , self . base)) } # [doc = "Get a reference to the value of field `audio_specification` after provisioning.\n"] pub fn audio_specification (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.audio_specification" , self . base)) } # [doc = "Get a reference to the value of field `dtmf_specification` after provisioning.\n"] pub fn dtmf_specification (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dtmf_specification" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl
{
    start_timeout_ms: PrimField<f64>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl
{
    #[doc = ""]
    pub start_timeout_ms: PrimField<f64>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl { start_timeout_ms : self . start_timeout_ms , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `start_timeout_ms` after provisioning.\n"] pub fn start_timeout_ms (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.start_timeout_ms" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElDynamic { allowed_input_types : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl >> , audio_and_dtmf_input_specification : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl >> , text_input_specification : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl { # [serde (skip_serializing_if = "Option::is_none")] allow_interrupt : Option < PrimField < bool > > , map_block_key : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] allowed_input_types : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl > > , # [serde (skip_serializing_if = "Option::is_none")] audio_and_dtmf_input_specification : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl > > , # [serde (skip_serializing_if = "Option::is_none")] text_input_specification : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl { # [doc = "Set the field `allow_interrupt`.\n"] pub fn set_allow_interrupt (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_interrupt = Some (v . into ()) ; self } # [doc = "Set the field `allowed_input_types`.\n"] pub fn set_allowed_input_types (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . allowed_input_types = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . allowed_input_types = Some (d) ; } } self } # [doc = "Set the field `audio_and_dtmf_input_specification`.\n"] pub fn set_audio_and_dtmf_input_specification (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . audio_and_dtmf_input_specification = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . audio_and_dtmf_input_specification = Some (d) ; } } self } # [doc = "Set the field `text_input_specification`.\n"] pub fn set_text_input_specification (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . text_input_specification = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . text_input_specification = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl
{
    #[doc = ""]
    pub map_block_key: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl { allow_interrupt : core :: default :: Default :: default () , map_block_key : self . map_block_key , allowed_input_types : core :: default :: Default :: default () , audio_and_dtmf_input_specification : core :: default :: Default :: default () , text_input_specification : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"] pub fn allow_interrupt (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_interrupt" , self . base)) } # [doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"] pub fn map_block_key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.map_block_key" , self . base)) } # [doc = "Get a reference to the value of field `allowed_input_types` after provisioning.\n"] pub fn allowed_input_types (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.allowed_input_types" , self . base)) } # [doc = "Get a reference to the value of field `audio_and_dtmf_input_specification` after provisioning.\n"] pub fn audio_and_dtmf_input_specification (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.audio_and_dtmf_input_specification" , self . base)) } # [doc = "Get a reference to the value of field `text_input_specification` after provisioning.\n"] pub fn text_input_specification (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.text_input_specification" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElDynamic { message_group : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupEl >> , prompt_attempts_specification : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationEl { # [serde (skip_serializing_if = "Option::is_none")] allow_interrupt : Option < PrimField < bool > > , max_retries : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] message_selection_strategy : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] message_group : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupEl > > , # [serde (skip_serializing_if = "Option::is_none")] prompt_attempts_specification : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElDynamic , }
impl
    Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationEl
{
    #[doc = "Set the field `allow_interrupt`.\n"]
    pub fn set_allow_interrupt(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_interrupt = Some(v.into());
        self
    }
    #[doc = "Set the field `message_selection_strategy`.\n"]
    pub fn set_message_selection_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_selection_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `message_group`.\n"]
    pub fn set_message_group(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message_group = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `prompt_attempts_specification`.\n"]
    pub fn set_prompt_attempts_specification(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prompt_attempts_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt_attempts_specification = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationEl
{
    #[doc = ""]
    pub max_retries: PrimField<f64>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationEl { allow_interrupt : core :: default :: Default :: default () , max_retries : self . max_retries , message_selection_strategy : core :: default :: Default :: default () , message_group : core :: default :: Default :: default () , prompt_attempts_specification : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"] pub fn allow_interrupt (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_interrupt" , self . base)) } # [doc = "Get a reference to the value of field `max_retries` after provisioning.\n"] pub fn max_retries (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_retries" , self . base)) } # [doc = "Get a reference to the value of field `message_selection_strategy` after provisioning.\n"] pub fn message_selection_strategy (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.message_selection_strategy" , self . base)) } # [doc = "Get a reference to the value of field `message_group` after provisioning.\n"] pub fn message_group (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElMessageGroupElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.message_group" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceEl
{
    utterance: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceEl {}
impl ToListMappable
    for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceEl
{
    type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceEl
{
    #[doc = ""]
    pub utterance: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceEl { utterance : self . utterance , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceElRef { shared : shared , base : base . to_string () , } } }
impl
    Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `utterance` after provisioning.\n"]
    pub fn utterance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.utterance", self.base))
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElDynamic { message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl >> , variation : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl { # [serde (skip_serializing_if = "Option::is_none")] message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] variation : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl { # [doc = "Set the field `message`.\n"] pub fn set_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . message = Some (d) ; } } self } # [doc = "Set the field `variation`.\n"] pub fn set_variation (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . variation = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . variation = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl { message : core :: default :: Default :: default () , variation : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `message` after provisioning.\n"] pub fn message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.message" , self . base)) } # [doc = "Get a reference to the value of field `variation` after provisioning.\n"] pub fn variation (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.variation" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElDynamic { message_group : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl { # [serde (skip_serializing_if = "Option::is_none")] allow_interrupt : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] message_group : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl { # [doc = "Set the field `allow_interrupt`.\n"] pub fn set_allow_interrupt (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_interrupt = Some (v . into ()) ; self } # [doc = "Set the field `message_group`.\n"] pub fn set_message_group (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . message_group = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . message_group = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl { allow_interrupt : core :: default :: Default :: default () , message_group : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"] pub fn allow_interrupt (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_interrupt" , self . base)) } # [doc = "Get a reference to the value of field `message_group` after provisioning.\n"] pub fn message_group (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.message_group" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElDynamic { message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl >> , variation : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl { # [serde (skip_serializing_if = "Option::is_none")] message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] variation : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl { # [doc = "Set the field `message`.\n"] pub fn set_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . message = Some (d) ; } } self } # [doc = "Set the field `variation`.\n"] pub fn set_variation (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . variation = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . variation = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl { message : core :: default :: Default :: default () , variation : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `message` after provisioning.\n"] pub fn message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.message" , self . base)) } # [doc = "Get a reference to the value of field `variation` after provisioning.\n"] pub fn variation (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.variation" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElDynamic { message_group : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl { # [serde (skip_serializing_if = "Option::is_none")] allow_interrupt : Option < PrimField < bool > > , frequency_in_seconds : PrimField < f64 > , timeout_in_seconds : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] message_group : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl { # [doc = "Set the field `allow_interrupt`.\n"] pub fn set_allow_interrupt (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_interrupt = Some (v . into ()) ; self } # [doc = "Set the field `message_group`.\n"] pub fn set_message_group (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . message_group = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . message_group = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl
{
    #[doc = ""]
    pub frequency_in_seconds: PrimField<f64>,
    #[doc = ""]
    pub timeout_in_seconds: PrimField<f64>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl { allow_interrupt : core :: default :: Default :: default () , frequency_in_seconds : self . frequency_in_seconds , timeout_in_seconds : self . timeout_in_seconds , message_group : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"] pub fn allow_interrupt (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_interrupt" , self . base)) } # [doc = "Get a reference to the value of field `frequency_in_seconds` after provisioning.\n"] pub fn frequency_in_seconds (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.frequency_in_seconds" , self . base)) } # [doc = "Get a reference to the value of field `timeout_in_seconds` after provisioning.\n"] pub fn timeout_in_seconds (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.timeout_in_seconds" , self . base)) } # [doc = "Get a reference to the value of field `message_group` after provisioning.\n"] pub fn message_group (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.message_group" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElDynamic { message : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl >> , variation : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl { # [serde (skip_serializing_if = "Option::is_none")] message : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] variation : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl { # [doc = "Set the field `message`.\n"] pub fn set_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . message = Some (d) ; } } self } # [doc = "Set the field `variation`.\n"] pub fn set_variation (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . variation = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . variation = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl { message : core :: default :: Default :: default () , variation : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `message` after provisioning.\n"] pub fn message (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.message" , self . base)) } # [doc = "Get a reference to the value of field `variation` after provisioning.\n"] pub fn variation (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.variation" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElDynamic { message_group : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl { # [serde (skip_serializing_if = "Option::is_none")] allow_interrupt : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] message_group : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl { # [doc = "Set the field `allow_interrupt`.\n"] pub fn set_allow_interrupt (mut self , v : impl Into < PrimField < bool > >) -> Self { self . allow_interrupt = Some (v . into ()) ; self } # [doc = "Set the field `message_group`.\n"] pub fn set_message_group (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . message_group = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . message_group = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl { allow_interrupt : core :: default :: Default :: default () , message_group : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"] pub fn allow_interrupt (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_interrupt" , self . base)) } # [doc = "Get a reference to the value of field `message_group` after provisioning.\n"] pub fn message_group (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.message_group" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElDynamic { continue_response : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl >> , still_waiting_response : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl >> , waiting_response : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationEl { # [serde (skip_serializing_if = "Option::is_none")] active : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] continue_response : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl > > , # [serde (skip_serializing_if = "Option::is_none")] still_waiting_response : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl > > , # [serde (skip_serializing_if = "Option::is_none")] waiting_response : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationEl { # [doc = "Set the field `active`.\n"] pub fn set_active (mut self , v : impl Into < PrimField < bool > >) -> Self { self . active = Some (v . into ()) ; self } # [doc = "Set the field `continue_response`.\n"] pub fn set_continue_response (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . continue_response = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . continue_response = Some (d) ; } } self } # [doc = "Set the field `still_waiting_response`.\n"] pub fn set_still_waiting_response (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . still_waiting_response = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . still_waiting_response = Some (d) ; } } self } # [doc = "Set the field `waiting_response`.\n"] pub fn set_waiting_response (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . waiting_response = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . waiting_response = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationEl { type O = BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationEl
{}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationEl { pub fn build (self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationEl { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationEl { active : core :: default :: Default :: default () , continue_response : core :: default :: Default :: default () , still_waiting_response : core :: default :: Default :: default () , waiting_response : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElRef { Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `active` after provisioning.\n"] pub fn active (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.active" , self . base)) } # [doc = "Get a reference to the value of field `continue_response` after provisioning.\n"] pub fn continue_response (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.continue_response" , self . base)) } # [doc = "Get a reference to the value of field `still_waiting_response` after provisioning.\n"] pub fn still_waiting_response (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.still_waiting_response" , self . base)) } # [doc = "Get a reference to the value of field `waiting_response` after provisioning.\n"] pub fn waiting_response (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.waiting_response" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDynamic { default_value_specification : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationEl >> , prompt_specification : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationEl >> , sample_utterance : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceEl >> , wait_and_continue_specification : Option < DynamicBlock < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingEl { # [serde (skip_serializing_if = "Option::is_none")] default_value_specification : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationEl > > , # [serde (skip_serializing_if = "Option::is_none")] prompt_specification : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationEl > > , # [serde (skip_serializing_if = "Option::is_none")] sample_utterance : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceEl > > , # [serde (skip_serializing_if = "Option::is_none")] wait_and_continue_specification : Option < Vec < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationEl > > , dynamic : Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDynamic , }
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingEl {
    #[doc = "Set the field `default_value_specification`.\n"]
    pub fn set_default_value_specification(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_value_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `prompt_specification`.\n"]
    pub fn set_prompt_specification(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prompt_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sample_utterance`.\n"]
    pub fn set_sample_utterance(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sample_utterance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sample_utterance = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `wait_and_continue_specification`.\n"]
    pub fn set_wait_and_continue_specification(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.wait_and_continue_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.wait_and_continue_specification = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingEl
{
    type O = BlockAssignable<
        Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingEl {}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingEl {
    pub fn build(
        self,
    ) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingEl {
        Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingEl {
            default_value_specification: core::default::Default::default(),
            prompt_specification: core::default::Default::default(),
            sample_utterance: core::default::Default::default(),
            wait_and_continue_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElRef {
        Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_value_specification` after provisioning.\n"]    pub fn default_value_specification (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElDefaultValueSpecificationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_value_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prompt_specification` after provisioning.\n"]    pub fn prompt_specification (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElPromptSpecificationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.prompt_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sample_utterance` after provisioning.\n"]    pub fn sample_utterance (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElSampleUtteranceElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.sample_utterance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `wait_and_continue_specification` after provisioning.\n"]    pub fn wait_and_continue_specification (& self) -> ListRef < Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElWaitAndContinueSpecificationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.wait_and_continue_specification", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElDynamic {
    value_elicitation_setting: Option<
        DynamicBlock<Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingEl>,
    >,
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationEl {
    map_block_key: PrimField<String>,
    slot_type_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_elicitation_setting:
        Option<Vec<Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingEl>>,
    dynamic: Lexv2modelsSlotSubSlotSettingElSlotSpecificationElDynamic,
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationEl {
    #[doc = "Set the field `value_elicitation_setting`.\n"]
    pub fn set_value_elicitation_setting(
        mut self,
        v: impl Into<
            BlockAssignable<
                Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value_elicitation_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value_elicitation_setting = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Lexv2modelsSlotSubSlotSettingElSlotSpecificationEl {
    type O = BlockAssignable<Lexv2modelsSlotSubSlotSettingElSlotSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationEl {
    #[doc = ""]
    pub map_block_key: PrimField<String>,
    #[doc = ""]
    pub slot_type_id: PrimField<String>,
}
impl BuildLexv2modelsSlotSubSlotSettingElSlotSpecificationEl {
    pub fn build(self) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationEl {
        Lexv2modelsSlotSubSlotSettingElSlotSpecificationEl {
            map_block_key: self.map_block_key,
            slot_type_id: self.slot_type_id,
            value_elicitation_setting: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotSubSlotSettingElSlotSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElSlotSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotSubSlotSettingElSlotSpecificationElRef {
        Lexv2modelsSlotSubSlotSettingElSlotSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotSubSlotSettingElSlotSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_block_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `slot_type_id` after provisioning.\n"]
    pub fn slot_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slot_type_id", self.base))
    }
    #[doc = "Get a reference to the value of field `value_elicitation_setting` after provisioning.\n"]
    pub fn value_elicitation_setting(
        &self,
    ) -> ListRef<Lexv2modelsSlotSubSlotSettingElSlotSpecificationElValueElicitationSettingElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.value_elicitation_setting", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotSubSlotSettingElDynamic {
    slot_specification: Option<DynamicBlock<Lexv2modelsSlotSubSlotSettingElSlotSpecificationEl>>,
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotSubSlotSettingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slot_specification: Option<Vec<Lexv2modelsSlotSubSlotSettingElSlotSpecificationEl>>,
    dynamic: Lexv2modelsSlotSubSlotSettingElDynamic,
}
impl Lexv2modelsSlotSubSlotSettingEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
    #[doc = "Set the field `slot_specification`.\n"]
    pub fn set_slot_specification(
        mut self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotSubSlotSettingElSlotSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.slot_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.slot_specification = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Lexv2modelsSlotSubSlotSettingEl {
    type O = BlockAssignable<Lexv2modelsSlotSubSlotSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotSubSlotSettingEl {}
impl BuildLexv2modelsSlotSubSlotSettingEl {
    pub fn build(self) -> Lexv2modelsSlotSubSlotSettingEl {
        Lexv2modelsSlotSubSlotSettingEl {
            expression: core::default::Default::default(),
            slot_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotSubSlotSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotSubSlotSettingElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsSlotSubSlotSettingElRef {
        Lexv2modelsSlotSubSlotSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotSubSlotSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl Lexv2modelsSlotTimeoutsEl {
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
impl ToListMappable for Lexv2modelsSlotTimeoutsEl {
    type O = BlockAssignable<Lexv2modelsSlotTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotTimeoutsEl {}
impl BuildLexv2modelsSlotTimeoutsEl {
    pub fn build(self) -> Lexv2modelsSlotTimeoutsEl {
        Lexv2modelsSlotTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsSlotTimeoutsElRef {
        Lexv2modelsSlotTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotTimeoutsElRef {
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
pub struct Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl {
    default_value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl {}
impl ToListMappable
    for Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl
{
    type O = BlockAssignable<
        Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl
{
    #[doc = ""]
    pub default_value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl {
    pub fn build(
        self,
    ) -> Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl {
        Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl {
            default_value: self.default_value,
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListElRef
    {
        Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_value", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDynamic {
    default_value_list: Option<
        DynamicBlock<
            Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value_list: Option<
        Vec<Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl>,
    >,
    dynamic: Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDynamic,
}
impl Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationEl {
    #[doc = "Set the field `default_value_list`.\n"]
    pub fn set_default_value_list(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_value_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value_list = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationEl {
    type O = BlockAssignable<Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationEl {}
impl BuildLexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationEl {
    pub fn build(self) -> Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationEl {
        Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationEl {
            default_value_list: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElRef {
        Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_value_list` after provisioning.\n"]
    pub fn default_value_list(
        &self,
    ) -> ListRef<
        Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElDefaultValueListElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_value_list", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl {
    #[doc = "Set the field `custom_payload`.\n"]
    pub fn set_custom_payload(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_payload = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_payload = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `image_response_card`.\n"]
    pub fn set_image_response_card(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image_response_card = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image_response_card = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `plain_text_message`.\n"]
    pub fn set_plain_text_message(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.plain_text_message = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.plain_text_message = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ssml_message`.\n"]
    pub fn set_ssml_message(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssml_message = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssml_message = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl
{
    type O = BlockAssignable<
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl {
    pub fn build(
        self,
    ) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl {
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl {
            custom_payload: core::default::Default::default(),
            image_response_card: core::default::Default::default(),
            plain_text_message: core::default::Default::default(),
            ssml_message: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElRef
    {
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"]    pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElCustomPayloadElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_payload", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"]    pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElImageResponseCardElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_response_card", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"]    pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElPlainTextMessageElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.plain_text_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"]    pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElSsmlMessageElRef >{
        ListRef::new(self.shared().clone(), format!("{}.ssml_message", self.base))
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl {
    #[doc = "Set the field `custom_payload`.\n"]
    pub fn set_custom_payload(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_payload = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_payload = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `image_response_card`.\n"]
    pub fn set_image_response_card(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image_response_card = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image_response_card = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `plain_text_message`.\n"]
    pub fn set_plain_text_message(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.plain_text_message = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.plain_text_message = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ssml_message`.\n"]
    pub fn set_ssml_message(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssml_message = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssml_message = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl
{
    type O = BlockAssignable<
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl {
    pub fn build(
        self,
    ) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl
    {
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl {
            custom_payload: core::default::Default::default(),
            image_response_card: core::default::Default::default(),
            plain_text_message: core::default::Default::default(),
            ssml_message: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElRef
    {
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"]    pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElCustomPayloadElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_payload", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"]    pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElImageResponseCardElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_response_card", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"]    pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElPlainTextMessageElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.plain_text_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"]    pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElSsmlMessageElRef >{
        ListRef::new(self.shared().clone(), format!("{}.ssml_message", self.base))
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElDynamic {
    message: Option<
        DynamicBlock<
            Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl,
        >,
    >,
    variation: Option<
        DynamicBlock<
            Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<
        Vec<Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    variation: Option<
        Vec<Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl>,
    >,
    dynamic: Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElDynamic,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `variation`.\n"]
    pub fn set_variation(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.variation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.variation = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupEl
{
    type O = BlockAssignable<
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupEl {}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupEl {
    pub fn build(
        self,
    ) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupEl {
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupEl {
            message: core::default::Default::default(),
            variation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElRef {
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(
        &self,
    ) -> ListRef<
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElMessageElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `variation` after provisioning.\n"]
    pub fn variation(
        &self,
    ) -> ListRef<
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElVariationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.variation", self.base))
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl
{
    allow_audio_input: PrimField<bool>,
    allow_dtmf_input: PrimField<bool>,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl
{
    #[doc = ""]
    pub allow_audio_input: PrimField<bool>,
    #[doc = ""]
    pub allow_dtmf_input: PrimField<bool>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl { allow_audio_input : self . allow_audio_input , allow_dtmf_input : self . allow_dtmf_input , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allow_audio_input` after provisioning.\n"] pub fn allow_audio_input (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_audio_input" , self . base)) } # [doc = "Get a reference to the value of field `allow_dtmf_input` after provisioning.\n"] pub fn allow_dtmf_input (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.allow_dtmf_input" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl
{
    end_timeout_ms: PrimField<f64>,
    max_length_ms: PrimField<f64>,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl
{
    #[doc = ""]
    pub end_timeout_ms: PrimField<f64>,
    #[doc = ""]
    pub max_length_ms: PrimField<f64>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl { end_timeout_ms : self . end_timeout_ms , max_length_ms : self . max_length_ms , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `end_timeout_ms` after provisioning.\n"] pub fn end_timeout_ms (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.end_timeout_ms" , self . base)) } # [doc = "Get a reference to the value of field `max_length_ms` after provisioning.\n"] pub fn max_length_ms (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_length_ms" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl
{
    deletion_character: PrimField<String>,
    end_character: PrimField<String>,
    end_timeout_ms: PrimField<f64>,
    max_length: PrimField<f64>,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl
{
    #[doc = ""]
    pub deletion_character: PrimField<String>,
    #[doc = ""]
    pub end_character: PrimField<String>,
    #[doc = ""]
    pub end_timeout_ms: PrimField<f64>,
    #[doc = ""]
    pub max_length: PrimField<f64>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl { deletion_character : self . deletion_character , end_character : self . end_character , end_timeout_ms : self . end_timeout_ms , max_length : self . max_length , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `deletion_character` after provisioning.\n"] pub fn deletion_character (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.deletion_character" , self . base)) } # [doc = "Get a reference to the value of field `end_character` after provisioning.\n"] pub fn end_character (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.end_character" , self . base)) } # [doc = "Get a reference to the value of field `end_timeout_ms` after provisioning.\n"] pub fn end_timeout_ms (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.end_timeout_ms" , self . base)) } # [doc = "Get a reference to the value of field `max_length` after provisioning.\n"] pub fn max_length (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_length" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDynamic { audio_specification : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl >> , dtmf_specification : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl { start_timeout_ms : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] audio_specification : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl > > , # [serde (skip_serializing_if = "Option::is_none")] dtmf_specification : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl { # [doc = "Set the field `audio_specification`.\n"] pub fn set_audio_specification (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . audio_specification = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . audio_specification = Some (d) ; } } self } # [doc = "Set the field `dtmf_specification`.\n"] pub fn set_dtmf_specification (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . dtmf_specification = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . dtmf_specification = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl
{
    #[doc = ""]
    pub start_timeout_ms: PrimField<f64>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl { start_timeout_ms : self . start_timeout_ms , audio_specification : core :: default :: Default :: default () , dtmf_specification : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `start_timeout_ms` after provisioning.\n"] pub fn start_timeout_ms (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.start_timeout_ms" , self . base)) } # [doc = "Get a reference to the value of field `audio_specification` after provisioning.\n"] pub fn audio_specification (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElAudioSpecificationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.audio_specification" , self . base)) } # [doc = "Get a reference to the value of field `dtmf_specification` after provisioning.\n"] pub fn dtmf_specification (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElDtmfSpecificationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.dtmf_specification" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl
{
    start_timeout_ms: PrimField<f64>,
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl
{
    #[doc = ""]
    pub start_timeout_ms: PrimField<f64>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl { start_timeout_ms : self . start_timeout_ms , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `start_timeout_ms` after provisioning.\n"] pub fn start_timeout_ms (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.start_timeout_ms" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElDynamic { allowed_input_types : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl >> , audio_and_dtmf_input_specification : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl >> , text_input_specification : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl { # [serde (skip_serializing_if = "Option::is_none")] allow_interrupt : Option < PrimField < bool > > , map_block_key : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] allowed_input_types : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl > > , # [serde (skip_serializing_if = "Option::is_none")] audio_and_dtmf_input_specification : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl > > , # [serde (skip_serializing_if = "Option::is_none")] text_input_specification : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl {
    #[doc = "Set the field `allow_interrupt`.\n"]
    pub fn set_allow_interrupt(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_interrupt = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_input_types`.\n"]
    pub fn set_allowed_input_types(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allowed_input_types = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_input_types = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `audio_and_dtmf_input_specification`.\n"]
    pub fn set_audio_and_dtmf_input_specification(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audio_and_dtmf_input_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_and_dtmf_input_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text_input_specification`.\n"]
    pub fn set_text_input_specification(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text_input_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text_input_specification = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl
{
    type O = BlockAssignable<
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl
{
    #[doc = ""]
    pub map_block_key: PrimField<String>,
}
impl
    BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl
{
    pub fn build(
        self,
    ) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl
    {
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl {
            allow_interrupt: core::default::Default::default(),
            map_block_key: self.map_block_key,
            allowed_input_types: core::default::Default::default(),
            audio_and_dtmf_input_specification: core::default::Default::default(),
            text_input_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElRef { Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_interrupt", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `map_block_key` after provisioning.\n"]
    pub fn map_block_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_block_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_input_types` after provisioning.\n"]    pub fn allowed_input_types (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAllowedInputTypesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_input_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audio_and_dtmf_input_specification` after provisioning.\n"]    pub fn audio_and_dtmf_input_specification (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElAudioAndDtmfInputSpecificationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_and_dtmf_input_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text_input_specification` after provisioning.\n"]    pub fn text_input_specification (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationElTextInputSpecificationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.text_input_specification", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElDynamic { message_group : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupEl >> , prompt_attempts_specification : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationEl { # [serde (skip_serializing_if = "Option::is_none")] allow_interrupt : Option < PrimField < bool > > , max_retries : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] message_selection_strategy : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] message_group : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupEl > > , # [serde (skip_serializing_if = "Option::is_none")] prompt_attempts_specification : Option < Vec < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationEl {
    #[doc = "Set the field `allow_interrupt`.\n"]
    pub fn set_allow_interrupt(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_interrupt = Some(v.into());
        self
    }
    #[doc = "Set the field `message_selection_strategy`.\n"]
    pub fn set_message_selection_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_selection_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `message_group`.\n"]
    pub fn set_message_group(
        mut self,
        v: impl Into<
            BlockAssignable<
                Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message_group = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `prompt_attempts_specification`.\n"]
    pub fn set_prompt_attempts_specification(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElPromptAttemptsSpecificationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prompt_attempts_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt_attempts_specification = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationEl {
    type O = BlockAssignable<Lexv2modelsSlotValueElicitationSettingElPromptSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationEl {
    #[doc = ""]
    pub max_retries: PrimField<f64>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElPromptSpecificationEl {
    pub fn build(self) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationEl {
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationEl {
            allow_interrupt: core::default::Default::default(),
            max_retries: self.max_retries,
            message_selection_strategy: core::default::Default::default(),
            message_group: core::default::Default::default(),
            prompt_attempts_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElRef {
        Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_interrupt", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }
    #[doc = "Get a reference to the value of field `message_selection_strategy` after provisioning.\n"]
    pub fn message_selection_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_selection_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]
    pub fn message_group(
        &self,
    ) -> ListRef<Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElMessageGroupElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_group", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElSampleUtteranceEl {
    utterance: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElSampleUtteranceEl {}
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElSampleUtteranceEl {
    type O = BlockAssignable<Lexv2modelsSlotValueElicitationSettingElSampleUtteranceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElSampleUtteranceEl {
    #[doc = ""]
    pub utterance: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElSampleUtteranceEl {
    pub fn build(self) -> Lexv2modelsSlotValueElicitationSettingElSampleUtteranceEl {
        Lexv2modelsSlotValueElicitationSettingElSampleUtteranceEl {
            utterance: self.utterance,
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElSampleUtteranceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElSampleUtteranceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotValueElicitationSettingElSampleUtteranceElRef {
        Lexv2modelsSlotValueElicitationSettingElSampleUtteranceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotValueElicitationSettingElSampleUtteranceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `utterance` after provisioning.\n"]
    pub fn utterance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.utterance", self.base))
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingEl {
    slot_resolution_strategy: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingEl {}
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingEl {
    type O = BlockAssignable<Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElSlotResolutionSettingEl {
    #[doc = ""]
    pub slot_resolution_strategy: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElSlotResolutionSettingEl {
    pub fn build(self) -> Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingEl {
        Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingEl {
            slot_resolution_strategy: self.slot_resolution_strategy,
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingElRef {
        Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `slot_resolution_strategy` after provisioning.\n"]
    pub fn slot_resolution_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_resolution_strategy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElDynamic { message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl >> , variation : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl { # [serde (skip_serializing_if = "Option::is_none")] message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] variation : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl { # [doc = "Set the field `message`.\n"] pub fn set_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . message = Some (d) ; } } self } # [doc = "Set the field `variation`.\n"] pub fn set_variation (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . variation = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . variation = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl { message : core :: default :: Default :: default () , variation : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `message` after provisioning.\n"] pub fn message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.message" , self . base)) } # [doc = "Get a reference to the value of field `variation` after provisioning.\n"] pub fn variation (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElVariationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.variation" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElDynamic { message_group : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl { # [serde (skip_serializing_if = "Option::is_none")] allow_interrupt : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] message_group : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl {
    #[doc = "Set the field `allow_interrupt`.\n"]
    pub fn set_allow_interrupt(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_interrupt = Some(v.into());
        self
    }
    #[doc = "Set the field `message_group`.\n"]
    pub fn set_message_group(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message_group = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl
{
    type O = BlockAssignable<
        Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl {
    pub fn build(
        self,
    ) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl
    {
        Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl {
            allow_interrupt: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElRef
    {
        Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElRef { shared : shared , base : base . to_string () , }
    }
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_interrupt", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]    pub fn message_group (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElMessageGroupElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_group", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElDynamic { message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl >> , variation : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl { # [serde (skip_serializing_if = "Option::is_none")] message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] variation : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl { # [doc = "Set the field `message`.\n"] pub fn set_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . message = Some (d) ; } } self } # [doc = "Set the field `variation`.\n"] pub fn set_variation (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . variation = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . variation = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl { message : core :: default :: Default :: default () , variation : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `message` after provisioning.\n"] pub fn message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.message" , self . base)) } # [doc = "Get a reference to the value of field `variation` after provisioning.\n"] pub fn variation (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElVariationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.variation" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElDynamic { message_group : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl { # [serde (skip_serializing_if = "Option::is_none")] allow_interrupt : Option < PrimField < bool > > , frequency_in_seconds : PrimField < f64 > , timeout_in_seconds : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] message_group : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl {
    #[doc = "Set the field `allow_interrupt`.\n"]
    pub fn set_allow_interrupt(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_interrupt = Some(v.into());
        self
    }
    #[doc = "Set the field `message_group`.\n"]
    pub fn set_message_group(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message_group = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl
{
    type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl
{
    #[doc = ""]
    pub frequency_in_seconds: PrimField<f64>,
    #[doc = ""]
    pub timeout_in_seconds: PrimField<f64>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl { allow_interrupt : core :: default :: Default :: default () , frequency_in_seconds : self . frequency_in_seconds , timeout_in_seconds : self . timeout_in_seconds , message_group : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElRef { shared : shared , base : base . to_string () , } } }
impl
    Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_interrupt", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `frequency_in_seconds` after provisioning.\n"]
    pub fn frequency_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.frequency_in_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_in_seconds` after provisioning.\n"]
    pub fn timeout_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_in_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]    pub fn message_group (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElMessageGroupElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_group", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl
{
    text: PrimField<String>,
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl
{
    #[doc = ""]
    pub text: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl { text : self . text , value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElDynamic { button : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl { # [serde (skip_serializing_if = "Option::is_none")] image_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subtitle : Option < PrimField < String > > , title : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] button : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl { # [doc = "Set the field `image_url`.\n"] pub fn set_image_url (mut self , v : impl Into < PrimField < String > >) -> Self { self . image_url = Some (v . into ()) ; self } # [doc = "Set the field `subtitle`.\n"] pub fn set_subtitle (mut self , v : impl Into < PrimField < String > >) -> Self { self . subtitle = Some (v . into ()) ; self } # [doc = "Set the field `button`.\n"] pub fn set_button (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . button = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . button = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl
{
    #[doc = ""]
    pub title: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl { image_url : core :: default :: Default :: default () , subtitle : core :: default :: Default :: default () , title : self . title , button : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `image_url` after provisioning.\n"] pub fn image_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.image_url" , self . base)) } # [doc = "Get a reference to the value of field `subtitle` after provisioning.\n"] pub fn subtitle (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.subtitle" , self . base)) } # [doc = "Get a reference to the value of field `title` after provisioning.\n"] pub fn title (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.title" , self . base)) } # [doc = "Get a reference to the value of field `button` after provisioning.\n"] pub fn button (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElButtonElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.button" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl
{
    value: PrimField<String>,
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl { }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl
{
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl { value : self . value , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElDynamic { custom_payload : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl >> , image_response_card : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl >> , plain_text_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl >> , ssml_message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl { # [serde (skip_serializing_if = "Option::is_none")] custom_payload : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] image_response_card : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl > > , # [serde (skip_serializing_if = "Option::is_none")] plain_text_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] ssml_message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl { # [doc = "Set the field `custom_payload`.\n"] pub fn set_custom_payload (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . custom_payload = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . custom_payload = Some (d) ; } } self } # [doc = "Set the field `image_response_card`.\n"] pub fn set_image_response_card (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . image_response_card = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . image_response_card = Some (d) ; } } self } # [doc = "Set the field `plain_text_message`.\n"] pub fn set_plain_text_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . plain_text_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . plain_text_message = Some (d) ; } } self } # [doc = "Set the field `ssml_message`.\n"] pub fn set_ssml_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . ssml_message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . ssml_message = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl { custom_payload : core :: default :: Default :: default () , image_response_card : core :: default :: Default :: default () , plain_text_message : core :: default :: Default :: default () , ssml_message : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_payload` after provisioning.\n"] pub fn custom_payload (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElCustomPayloadElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.custom_payload" , self . base)) } # [doc = "Get a reference to the value of field `image_response_card` after provisioning.\n"] pub fn image_response_card (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElImageResponseCardElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.image_response_card" , self . base)) } # [doc = "Get a reference to the value of field `plain_text_message` after provisioning.\n"] pub fn plain_text_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElPlainTextMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.plain_text_message" , self . base)) } # [doc = "Get a reference to the value of field `ssml_message` after provisioning.\n"] pub fn ssml_message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElSsmlMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ssml_message" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElDynamic { message : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl >> , variation : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl { # [serde (skip_serializing_if = "Option::is_none")] message : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] variation : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl { # [doc = "Set the field `message`.\n"] pub fn set_message (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . message = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . message = Some (d) ; } } self } # [doc = "Set the field `variation`.\n"] pub fn set_variation (mut self , v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . variation = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . variation = Some (d) ; } } self } }
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl { type O = BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl { pub fn build (self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl { message : core :: default :: Default :: default () , variation : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElRef { fn new (shared : StackShared , base : String) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElRef { Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElRef { shared : shared , base : base . to_string () , } } }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `message` after provisioning.\n"] pub fn message (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElMessageElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.message" , self . base)) } # [doc = "Get a reference to the value of field `variation` after provisioning.\n"] pub fn variation (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElVariationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.variation" , self . base)) } }
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElDynamic { message_group : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl { # [serde (skip_serializing_if = "Option::is_none")] allow_interrupt : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] message_group : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl {
    #[doc = "Set the field `allow_interrupt`.\n"]
    pub fn set_allow_interrupt(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_interrupt = Some(v.into());
        self
    }
    #[doc = "Set the field `message_group`.\n"]
    pub fn set_message_group(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.message_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.message_group = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl
{
    type O = BlockAssignable<
        Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl
{}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl {
    pub fn build(
        self,
    ) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl
    {
        Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl {
            allow_interrupt: core::default::Default::default(),
            message_group: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElRef
    {
        Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_interrupt` after provisioning.\n"]
    pub fn allow_interrupt(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_interrupt", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message_group` after provisioning.\n"]    pub fn message_group (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElMessageGroupElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_group", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElDynamic { continue_response : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl >> , still_waiting_response : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl >> , waiting_response : Option < DynamicBlock < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl >> , }
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationEl { # [serde (skip_serializing_if = "Option::is_none")] active : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] continue_response : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl > > , # [serde (skip_serializing_if = "Option::is_none")] still_waiting_response : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl > > , # [serde (skip_serializing_if = "Option::is_none")] waiting_response : Option < Vec < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl > > , dynamic : Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElDynamic , }
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationEl {
    #[doc = "Set the field `active`.\n"]
    pub fn set_active(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.active = Some(v.into());
        self
    }
    #[doc = "Set the field `continue_response`.\n"]
    pub fn set_continue_response(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.continue_response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.continue_response = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `still_waiting_response`.\n"]
    pub fn set_still_waiting_response(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.still_waiting_response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.still_waiting_response = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `waiting_response`.\n"]
    pub fn set_waiting_response(
        mut self,
        v : impl Into < BlockAssignable < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.waiting_response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.waiting_response = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationEl {
    type O =
        BlockAssignable<Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationEl {}
impl BuildLexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationEl {
    pub fn build(self) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationEl {
        Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationEl {
            active: core::default::Default::default(),
            continue_response: core::default::Default::default(),
            still_waiting_response: core::default::Default::default(),
            waiting_response: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElRef {
        Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }
    #[doc = "Get a reference to the value of field `continue_response` after provisioning.\n"]
    pub fn continue_response(
        &self,
    ) -> ListRef<
        Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElContinueResponseElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.continue_response", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `still_waiting_response` after provisioning.\n"]    pub fn still_waiting_response (& self) -> ListRef < Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElStillWaitingResponseElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.still_waiting_response", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `waiting_response` after provisioning.\n"]
    pub fn waiting_response(
        &self,
    ) -> ListRef<
        Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElWaitingResponseElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.waiting_response", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotValueElicitationSettingElDynamic {
    default_value_specification:
        Option<DynamicBlock<Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationEl>>,
    prompt_specification:
        Option<DynamicBlock<Lexv2modelsSlotValueElicitationSettingElPromptSpecificationEl>>,
    sample_utterance:
        Option<DynamicBlock<Lexv2modelsSlotValueElicitationSettingElSampleUtteranceEl>>,
    slot_resolution_setting:
        Option<DynamicBlock<Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingEl>>,
    wait_and_continue_specification: Option<
        DynamicBlock<Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationEl>,
    >,
}
#[derive(Serialize)]
pub struct Lexv2modelsSlotValueElicitationSettingEl {
    slot_constraint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value_specification:
        Option<Vec<Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_specification:
        Option<Vec<Lexv2modelsSlotValueElicitationSettingElPromptSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_utterance: Option<Vec<Lexv2modelsSlotValueElicitationSettingElSampleUtteranceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slot_resolution_setting:
        Option<Vec<Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_and_continue_specification:
        Option<Vec<Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationEl>>,
    dynamic: Lexv2modelsSlotValueElicitationSettingElDynamic,
}
impl Lexv2modelsSlotValueElicitationSettingEl {
    #[doc = "Set the field `default_value_specification`.\n"]
    pub fn set_default_value_specification(
        mut self,
        v: impl Into<
            BlockAssignable<Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_value_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `prompt_specification`.\n"]
    pub fn set_prompt_specification(
        mut self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotValueElicitationSettingElPromptSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prompt_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prompt_specification = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sample_utterance`.\n"]
    pub fn set_sample_utterance(
        mut self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotValueElicitationSettingElSampleUtteranceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sample_utterance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sample_utterance = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `slot_resolution_setting`.\n"]
    pub fn set_slot_resolution_setting(
        mut self,
        v: impl Into<BlockAssignable<Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.slot_resolution_setting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.slot_resolution_setting = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `wait_and_continue_specification`.\n"]
    pub fn set_wait_and_continue_specification(
        mut self,
        v: impl Into<
            BlockAssignable<Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.wait_and_continue_specification = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.wait_and_continue_specification = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Lexv2modelsSlotValueElicitationSettingEl {
    type O = BlockAssignable<Lexv2modelsSlotValueElicitationSettingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsSlotValueElicitationSettingEl {
    #[doc = ""]
    pub slot_constraint: PrimField<String>,
}
impl BuildLexv2modelsSlotValueElicitationSettingEl {
    pub fn build(self) -> Lexv2modelsSlotValueElicitationSettingEl {
        Lexv2modelsSlotValueElicitationSettingEl {
            slot_constraint: self.slot_constraint,
            default_value_specification: core::default::Default::default(),
            prompt_specification: core::default::Default::default(),
            sample_utterance: core::default::Default::default(),
            slot_resolution_setting: core::default::Default::default(),
            wait_and_continue_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Lexv2modelsSlotValueElicitationSettingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsSlotValueElicitationSettingElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsSlotValueElicitationSettingElRef {
        Lexv2modelsSlotValueElicitationSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsSlotValueElicitationSettingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `slot_constraint` after provisioning.\n"]
    pub fn slot_constraint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_constraint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_value_specification` after provisioning.\n"]
    pub fn default_value_specification(
        &self,
    ) -> ListRef<Lexv2modelsSlotValueElicitationSettingElDefaultValueSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_value_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prompt_specification` after provisioning.\n"]
    pub fn prompt_specification(
        &self,
    ) -> ListRef<Lexv2modelsSlotValueElicitationSettingElPromptSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.prompt_specification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sample_utterance` after provisioning.\n"]
    pub fn sample_utterance(
        &self,
    ) -> ListRef<Lexv2modelsSlotValueElicitationSettingElSampleUtteranceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sample_utterance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `slot_resolution_setting` after provisioning.\n"]
    pub fn slot_resolution_setting(
        &self,
    ) -> ListRef<Lexv2modelsSlotValueElicitationSettingElSlotResolutionSettingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.slot_resolution_setting", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `wait_and_continue_specification` after provisioning.\n"]
    pub fn wait_and_continue_specification(
        &self,
    ) -> ListRef<Lexv2modelsSlotValueElicitationSettingElWaitAndContinueSpecificationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.wait_and_continue_specification", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsSlotDynamic {
    multiple_values_setting: Option<DynamicBlock<Lexv2modelsSlotMultipleValuesSettingEl>>,
    obfuscation_setting: Option<DynamicBlock<Lexv2modelsSlotObfuscationSettingEl>>,
    sub_slot_setting: Option<DynamicBlock<Lexv2modelsSlotSubSlotSettingEl>>,
    value_elicitation_setting: Option<DynamicBlock<Lexv2modelsSlotValueElicitationSettingEl>>,
}
