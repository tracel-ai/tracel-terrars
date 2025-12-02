use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChimesdkvoiceGlobalSettingsData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    voice_connector: Option<Vec<ChimesdkvoiceGlobalSettingsVoiceConnectorEl>>,
    dynamic: ChimesdkvoiceGlobalSettingsDynamic,
}
struct ChimesdkvoiceGlobalSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChimesdkvoiceGlobalSettingsData>,
}
#[derive(Clone)]
pub struct ChimesdkvoiceGlobalSettings(Rc<ChimesdkvoiceGlobalSettings_>);
impl ChimesdkvoiceGlobalSettings {
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
    #[doc = "Set the field `voice_connector`.\n"]
    pub fn set_voice_connector(
        self,
        v: impl Into<BlockAssignable<ChimesdkvoiceGlobalSettingsVoiceConnectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().voice_connector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.voice_connector = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `voice_connector` after provisioning.\n"]
    pub fn voice_connector(&self) -> ListRef<ChimesdkvoiceGlobalSettingsVoiceConnectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.voice_connector", self.extract_ref()),
        )
    }
}
impl Referable for ChimesdkvoiceGlobalSettings {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChimesdkvoiceGlobalSettings {}
impl ToListMappable for ChimesdkvoiceGlobalSettings {
    type O = ListRef<ChimesdkvoiceGlobalSettingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChimesdkvoiceGlobalSettings_ {
    fn extract_resource_type(&self) -> String {
        "aws_chimesdkvoice_global_settings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChimesdkvoiceGlobalSettings {
    pub tf_id: String,
}
impl BuildChimesdkvoiceGlobalSettings {
    pub fn build(self, stack: &mut Stack) -> ChimesdkvoiceGlobalSettings {
        let out = ChimesdkvoiceGlobalSettings(Rc::new(ChimesdkvoiceGlobalSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChimesdkvoiceGlobalSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                voice_connector: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChimesdkvoiceGlobalSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChimesdkvoiceGlobalSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChimesdkvoiceGlobalSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `voice_connector` after provisioning.\n"]
    pub fn voice_connector(&self) -> ListRef<ChimesdkvoiceGlobalSettingsVoiceConnectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.voice_connector", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChimesdkvoiceGlobalSettingsVoiceConnectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cdr_bucket: Option<PrimField<String>>,
}
impl ChimesdkvoiceGlobalSettingsVoiceConnectorEl {
    #[doc = "Set the field `cdr_bucket`.\n"]
    pub fn set_cdr_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cdr_bucket = Some(v.into());
        self
    }
}
impl ToListMappable for ChimesdkvoiceGlobalSettingsVoiceConnectorEl {
    type O = BlockAssignable<ChimesdkvoiceGlobalSettingsVoiceConnectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChimesdkvoiceGlobalSettingsVoiceConnectorEl {}
impl BuildChimesdkvoiceGlobalSettingsVoiceConnectorEl {
    pub fn build(self) -> ChimesdkvoiceGlobalSettingsVoiceConnectorEl {
        ChimesdkvoiceGlobalSettingsVoiceConnectorEl {
            cdr_bucket: core::default::Default::default(),
        }
    }
}
pub struct ChimesdkvoiceGlobalSettingsVoiceConnectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChimesdkvoiceGlobalSettingsVoiceConnectorElRef {
    fn new(shared: StackShared, base: String) -> ChimesdkvoiceGlobalSettingsVoiceConnectorElRef {
        ChimesdkvoiceGlobalSettingsVoiceConnectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChimesdkvoiceGlobalSettingsVoiceConnectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cdr_bucket` after provisioning.\n"]
    pub fn cdr_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdr_bucket", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChimesdkvoiceGlobalSettingsDynamic {
    voice_connector: Option<DynamicBlock<ChimesdkvoiceGlobalSettingsVoiceConnectorEl>>,
}
