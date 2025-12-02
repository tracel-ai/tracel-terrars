use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct WorkspaceswebDataProtectionSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_encryption_context: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_redaction_configuration:
        Option<Vec<WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationEl>>,
    dynamic: WorkspaceswebDataProtectionSettingsDynamic,
}
struct WorkspaceswebDataProtectionSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WorkspaceswebDataProtectionSettingsData>,
}
#[derive(Clone)]
pub struct WorkspaceswebDataProtectionSettings(Rc<WorkspaceswebDataProtectionSettings_>);
impl WorkspaceswebDataProtectionSettings {
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
    #[doc = "Set the field `additional_encryption_context`.\n"]
    pub fn set_additional_encryption_context(
        self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().additional_encryption_context = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_managed_key`.\n"]
    pub fn set_customer_managed_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_managed_key = Some(v.into());
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
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `inline_redaction_configuration`.\n"]
    pub fn set_inline_redaction_configuration(
        self,
        v: impl Into<BlockAssignable<WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inline_redaction_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .inline_redaction_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.additional_encryption_context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.associated_portal_arns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_managed_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_protection_settings_arn` after provisioning.\n"]
    pub fn data_protection_settings_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_protection_settings_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `inline_redaction_configuration` after provisioning.\n"]
    pub fn inline_redaction_configuration(
        &self,
    ) -> ListRef<WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_redaction_configuration", self.extract_ref()),
        )
    }
}
impl Referable for WorkspaceswebDataProtectionSettings {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for WorkspaceswebDataProtectionSettings {}
impl ToListMappable for WorkspaceswebDataProtectionSettings {
    type O = ListRef<WorkspaceswebDataProtectionSettingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for WorkspaceswebDataProtectionSettings_ {
    fn extract_resource_type(&self) -> String {
        "aws_workspacesweb_data_protection_settings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildWorkspaceswebDataProtectionSettings {
    pub tf_id: String,
    #[doc = ""]
    pub display_name: PrimField<String>,
}
impl BuildWorkspaceswebDataProtectionSettings {
    pub fn build(self, stack: &mut Stack) -> WorkspaceswebDataProtectionSettings {
        let out =
            WorkspaceswebDataProtectionSettings(Rc::new(WorkspaceswebDataProtectionSettings_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(WorkspaceswebDataProtectionSettingsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    additional_encryption_context: core::default::Default::default(),
                    customer_managed_key: core::default::Default::default(),
                    description: core::default::Default::default(),
                    display_name: self.display_name,
                    region: core::default::Default::default(),
                    tags: core::default::Default::default(),
                    inline_redaction_configuration: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct WorkspaceswebDataProtectionSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspaceswebDataProtectionSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl WorkspaceswebDataProtectionSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.additional_encryption_context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.associated_portal_arns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_managed_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_protection_settings_arn` after provisioning.\n"]
    pub fn data_protection_settings_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_protection_settings_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `inline_redaction_configuration` after provisioning.\n"]
    pub fn inline_redaction_configuration(
        &self,
    ) -> ListRef<WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_redaction_configuration", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    keyword_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern_description: Option<PrimField<String>>,
    pattern_name: PrimField<String>,
    pattern_regex: PrimField<String>,
}
impl WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternEl { # [doc = "Set the field `keyword_regex`.\n"] pub fn set_keyword_regex (mut self , v : impl Into < PrimField < String > >) -> Self { self . keyword_regex = Some (v . into ()) ; self } # [doc = "Set the field `pattern_description`.\n"] pub fn set_pattern_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . pattern_description = Some (v . into ()) ; self } }
impl ToListMappable for WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternEl { type O = BlockAssignable < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternEl
{
    #[doc = ""]
    pub pattern_name: PrimField<String>,
    #[doc = ""]
    pub pattern_regex: PrimField<String>,
}
impl BuildWorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternEl { pub fn build (self) -> WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternEl { WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternEl { keyword_regex : core :: default :: Default :: default () , pattern_description : core :: default :: Default :: default () , pattern_name : self . pattern_name , pattern_regex : self . pattern_regex , } } }
pub struct WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternElRef { fn new (shared : StackShared , base : String) -> WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternElRef { WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternElRef { shared : shared , base : base . to_string () , } } }
impl WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `keyword_regex` after provisioning.\n"] pub fn keyword_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.keyword_regex" , self . base)) } # [doc = "Get a reference to the value of field `pattern_description` after provisioning.\n"] pub fn pattern_description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pattern_description" , self . base)) } # [doc = "Get a reference to the value of field `pattern_name` after provisioning.\n"] pub fn pattern_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pattern_name" , self . base)) } # [doc = "Get a reference to the value of field `pattern_regex` after provisioning.\n"] pub fn pattern_regex (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pattern_regex" , self . base)) } }
#[derive(Serialize)]
pub struct WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    redaction_place_holder_text: Option<PrimField<String>>,
    redaction_place_holder_type: PrimField<String>,
}
impl WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderEl { # [doc = "Set the field `redaction_place_holder_text`.\n"] pub fn set_redaction_place_holder_text (mut self , v : impl Into < PrimField < String > >) -> Self { self . redaction_place_holder_text = Some (v . into ()) ; self } }
impl ToListMappable for WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderEl { type O = BlockAssignable < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildWorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderEl
{
    #[doc = ""]
    pub redaction_place_holder_type: PrimField<String>,
}
impl BuildWorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderEl { pub fn build (self) -> WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderEl { WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderEl { redaction_place_holder_text : core :: default :: Default :: default () , redaction_place_holder_type : self . redaction_place_holder_type , } } }
pub struct WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderElRef { fn new (shared : StackShared , base : String) -> WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderElRef { WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderElRef { shared : shared , base : base . to_string () , } } }
impl WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `redaction_place_holder_text` after provisioning.\n"] pub fn redaction_place_holder_text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.redaction_place_holder_text" , self . base)) } # [doc = "Get a reference to the value of field `redaction_place_holder_type` after provisioning.\n"] pub fn redaction_place_holder_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.redaction_place_holder_type" , self . base)) } }
#[derive(Serialize, Default)]
struct WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElDynamic { custom_pattern : Option < DynamicBlock < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternEl >> , redaction_place_holder : Option < DynamicBlock < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderEl >> , }
#[derive(Serialize)]
pub struct WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternEl { # [serde (skip_serializing_if = "Option::is_none")] built_in_pattern_id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] confidence_level : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] enforced_urls : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] exempt_urls : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] custom_pattern : Option < Vec < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternEl > > , # [serde (skip_serializing_if = "Option::is_none")] redaction_place_holder : Option < Vec < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderEl > > , dynamic : WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElDynamic , }
impl WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternEl {
    #[doc = "Set the field `built_in_pattern_id`.\n"]
    pub fn set_built_in_pattern_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.built_in_pattern_id = Some(v.into());
        self
    }
    #[doc = "Set the field `confidence_level`.\n"]
    pub fn set_confidence_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.confidence_level = Some(v.into());
        self
    }
    #[doc = "Set the field `enforced_urls`.\n"]
    pub fn set_enforced_urls(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enforced_urls = Some(v.into());
        self
    }
    #[doc = "Set the field `exempt_urls`.\n"]
    pub fn set_exempt_urls(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exempt_urls = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_pattern`.\n"]
    pub fn set_custom_pattern(
        mut self,
        v : impl Into < BlockAssignable < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_pattern = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_pattern = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redaction_place_holder`.\n"]
    pub fn set_redaction_place_holder(
        mut self,
        v : impl Into < BlockAssignable < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redaction_place_holder = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redaction_place_holder = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternEl
{
    type O = BlockAssignable<
        WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternEl
{}
impl
    BuildWorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternEl
{
    pub fn build(
        self,
    ) -> WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternEl
    {
        WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternEl {
            built_in_pattern_id: core::default::Default::default(),
            confidence_level: core::default::Default::default(),
            enforced_urls: core::default::Default::default(),
            exempt_urls: core::default::Default::default(),
            custom_pattern: core::default::Default::default(),
            redaction_place_holder: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRef
    {
        WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRef { shared : shared , base : base . to_string () , }
    }
}
impl WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `built_in_pattern_id` after provisioning.\n"]
    pub fn built_in_pattern_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.built_in_pattern_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `confidence_level` after provisioning.\n"]
    pub fn confidence_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confidence_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforced_urls` after provisioning.\n"]
    pub fn enforced_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enforced_urls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exempt_urls` after provisioning.\n"]
    pub fn exempt_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exempt_urls", self.base))
    }
    #[doc = "Get a reference to the value of field `custom_pattern` after provisioning.\n"]    pub fn custom_pattern (& self) -> ListRef < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElCustomPatternElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_pattern", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redaction_place_holder` after provisioning.\n"]    pub fn redaction_place_holder (& self) -> ListRef < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRedactionPlaceHolderElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.redaction_place_holder", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElDynamic { inline_redaction_pattern : Option < DynamicBlock < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternEl >> , }
#[derive(Serialize)]
pub struct WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationEl { # [serde (skip_serializing_if = "Option::is_none")] global_confidence_level : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] global_enforced_urls : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] global_exempt_urls : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] inline_redaction_pattern : Option < Vec < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternEl > > , dynamic : WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElDynamic , }
impl WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationEl {
    #[doc = "Set the field `global_confidence_level`.\n"]
    pub fn set_global_confidence_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.global_confidence_level = Some(v.into());
        self
    }
    #[doc = "Set the field `global_enforced_urls`.\n"]
    pub fn set_global_enforced_urls(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.global_enforced_urls = Some(v.into());
        self
    }
    #[doc = "Set the field `global_exempt_urls`.\n"]
    pub fn set_global_exempt_urls(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.global_exempt_urls = Some(v.into());
        self
    }
    #[doc = "Set the field `inline_redaction_pattern`.\n"]
    pub fn set_inline_redaction_pattern(
        mut self,
        v : impl Into < BlockAssignable < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inline_redaction_pattern = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_redaction_pattern = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationEl {
    type O = BlockAssignable<WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkspaceswebDataProtectionSettingsInlineRedactionConfigurationEl {}
impl BuildWorkspaceswebDataProtectionSettingsInlineRedactionConfigurationEl {
    pub fn build(self) -> WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationEl {
        WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationEl {
            global_confidence_level: core::default::Default::default(),
            global_enforced_urls: core::default::Default::default(),
            global_exempt_urls: core::default::Default::default(),
            inline_redaction_pattern: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElRef {
        WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `global_confidence_level` after provisioning.\n"]
    pub fn global_confidence_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_confidence_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `global_enforced_urls` after provisioning.\n"]
    pub fn global_enforced_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.global_enforced_urls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `global_exempt_urls` after provisioning.\n"]
    pub fn global_exempt_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.global_exempt_urls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inline_redaction_pattern` after provisioning.\n"]    pub fn inline_redaction_pattern (& self) -> ListRef < WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationElInlineRedactionPatternElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_redaction_pattern", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct WorkspaceswebDataProtectionSettingsDynamic {
    inline_redaction_configuration:
        Option<DynamicBlock<WorkspaceswebDataProtectionSettingsInlineRedactionConfigurationEl>>,
}
