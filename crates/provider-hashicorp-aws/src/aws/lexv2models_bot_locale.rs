use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Lexv2modelsBotLocaleData {
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
    n_lu_intent_confidence_threshold: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Lexv2modelsBotLocaleTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice_settings: Option<Vec<Lexv2modelsBotLocaleVoiceSettingsEl>>,
    dynamic: Lexv2modelsBotLocaleDynamic,
}
struct Lexv2modelsBotLocale_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Lexv2modelsBotLocaleData>,
}
#[derive(Clone)]
pub struct Lexv2modelsBotLocale(Rc<Lexv2modelsBotLocale_>);
impl Lexv2modelsBotLocale {
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
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Lexv2modelsBotLocaleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `voice_settings`.\n"]
    pub fn set_voice_settings(
        self,
        v: impl Into<BlockAssignable<Lexv2modelsBotLocaleVoiceSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().voice_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.voice_settings = Some(d);
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
    #[doc = "Get a reference to the value of field `n_lu_intent_confidence_threshold` after provisioning.\n"]
    pub fn n_lu_intent_confidence_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.n_lu_intent_confidence_threshold", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Lexv2modelsBotLocaleTimeoutsElRef {
        Lexv2modelsBotLocaleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `voice_settings` after provisioning.\n"]
    pub fn voice_settings(&self) -> ListRef<Lexv2modelsBotLocaleVoiceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.voice_settings", self.extract_ref()),
        )
    }
}
impl Referable for Lexv2modelsBotLocale {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Lexv2modelsBotLocale {}
impl ToListMappable for Lexv2modelsBotLocale {
    type O = ListRef<Lexv2modelsBotLocaleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Lexv2modelsBotLocale_ {
    fn extract_resource_type(&self) -> String {
        "aws_lexv2models_bot_locale".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLexv2modelsBotLocale {
    pub tf_id: String,
    #[doc = ""]
    pub bot_id: PrimField<String>,
    #[doc = ""]
    pub bot_version: PrimField<String>,
    #[doc = ""]
    pub locale_id: PrimField<String>,
    #[doc = ""]
    pub n_lu_intent_confidence_threshold: PrimField<f64>,
}
impl BuildLexv2modelsBotLocale {
    pub fn build(self, stack: &mut Stack) -> Lexv2modelsBotLocale {
        let out = Lexv2modelsBotLocale(Rc::new(Lexv2modelsBotLocale_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Lexv2modelsBotLocaleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bot_id: self.bot_id,
                bot_version: self.bot_version,
                description: core::default::Default::default(),
                locale_id: self.locale_id,
                n_lu_intent_confidence_threshold: self.n_lu_intent_confidence_threshold,
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                voice_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Lexv2modelsBotLocaleRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsBotLocaleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Lexv2modelsBotLocaleRef {
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
    #[doc = "Get a reference to the value of field `n_lu_intent_confidence_threshold` after provisioning.\n"]
    pub fn n_lu_intent_confidence_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.n_lu_intent_confidence_threshold", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Lexv2modelsBotLocaleTimeoutsElRef {
        Lexv2modelsBotLocaleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `voice_settings` after provisioning.\n"]
    pub fn voice_settings(&self) -> ListRef<Lexv2modelsBotLocaleVoiceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.voice_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Lexv2modelsBotLocaleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl Lexv2modelsBotLocaleTimeoutsEl {
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
impl ToListMappable for Lexv2modelsBotLocaleTimeoutsEl {
    type O = BlockAssignable<Lexv2modelsBotLocaleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsBotLocaleTimeoutsEl {}
impl BuildLexv2modelsBotLocaleTimeoutsEl {
    pub fn build(self) -> Lexv2modelsBotLocaleTimeoutsEl {
        Lexv2modelsBotLocaleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct Lexv2modelsBotLocaleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsBotLocaleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsBotLocaleTimeoutsElRef {
        Lexv2modelsBotLocaleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsBotLocaleTimeoutsElRef {
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
pub struct Lexv2modelsBotLocaleVoiceSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<PrimField<String>>,
    voice_id: PrimField<String>,
}
impl Lexv2modelsBotLocaleVoiceSettingsEl {
    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.engine = Some(v.into());
        self
    }
}
impl ToListMappable for Lexv2modelsBotLocaleVoiceSettingsEl {
    type O = BlockAssignable<Lexv2modelsBotLocaleVoiceSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLexv2modelsBotLocaleVoiceSettingsEl {
    #[doc = ""]
    pub voice_id: PrimField<String>,
}
impl BuildLexv2modelsBotLocaleVoiceSettingsEl {
    pub fn build(self) -> Lexv2modelsBotLocaleVoiceSettingsEl {
        Lexv2modelsBotLocaleVoiceSettingsEl {
            engine: core::default::Default::default(),
            voice_id: self.voice_id,
        }
    }
}
pub struct Lexv2modelsBotLocaleVoiceSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Lexv2modelsBotLocaleVoiceSettingsElRef {
    fn new(shared: StackShared, base: String) -> Lexv2modelsBotLocaleVoiceSettingsElRef {
        Lexv2modelsBotLocaleVoiceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Lexv2modelsBotLocaleVoiceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }
    #[doc = "Get a reference to the value of field `voice_id` after provisioning.\n"]
    pub fn voice_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.voice_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct Lexv2modelsBotLocaleDynamic {
    voice_settings: Option<DynamicBlock<Lexv2modelsBotLocaleVoiceSettingsEl>>,
}
