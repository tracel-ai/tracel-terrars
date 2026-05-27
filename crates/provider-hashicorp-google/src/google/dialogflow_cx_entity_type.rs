use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxEntityTypeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_expansion_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_fuzzy_extraction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    kind: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redact: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<DialogflowCxEntityTypeEntitiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_phrases: Option<Vec<DialogflowCxEntityTypeExcludedPhrasesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxEntityTypeTimeoutsEl>,
    dynamic: DialogflowCxEntityTypeDynamic,
}
struct DialogflowCxEntityType_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxEntityTypeData>,
}
#[derive(Clone)]
pub struct DialogflowCxEntityType(Rc<DialogflowCxEntityType_>);
impl DialogflowCxEntityType {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `auto_expansion_mode`.\nRepresents kinds of entities.\n* AUTO_EXPANSION_MODE_UNSPECIFIED: Auto expansion disabled for the entity.\n* AUTO_EXPANSION_MODE_DEFAULT: Allows an agent to recognize values that have not been explicitly listed in the entity. Possible values: [\"AUTO_EXPANSION_MODE_DEFAULT\", \"AUTO_EXPANSION_MODE_UNSPECIFIED\"]"]
    pub fn set_auto_expansion_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_expansion_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_fuzzy_extraction`.\nEnables fuzzy entity extraction during classification."]
    pub fn set_enable_fuzzy_extraction(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_fuzzy_extraction = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code`.\nThe language of the following fields in entityType:\nEntityType.entities.value\nEntityType.entities.synonyms\nEntityType.excluded_phrases.value\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn set_language_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\nThe agent to create a entity type for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `redact`.\nIndicates whether parameters of the entity type should be redacted in log. If redaction is enabled, page parameters and intent parameters referring to the entity type will be replaced by parameter name when logging."]
    pub fn set_redact(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().redact = Some(v.into());
        self
    }
    #[doc = "Set the field `entities`.\n"]
    pub fn set_entities(
        self,
        v: impl Into<BlockAssignable<DialogflowCxEntityTypeEntitiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entities = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entities = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `excluded_phrases`.\n"]
    pub fn set_excluded_phrases(
        self,
        v: impl Into<BlockAssignable<DialogflowCxEntityTypeExcludedPhrasesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().excluded_phrases = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.excluded_phrases = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxEntityTypeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `auto_expansion_mode` after provisioning.\nRepresents kinds of entities.\n* AUTO_EXPANSION_MODE_UNSPECIFIED: Auto expansion disabled for the entity.\n* AUTO_EXPANSION_MODE_DEFAULT: Allows an agent to recognize values that have not been explicitly listed in the entity. Possible values: [\"AUTO_EXPANSION_MODE_DEFAULT\", \"AUTO_EXPANSION_MODE_UNSPECIFIED\"]"]
    pub fn auto_expansion_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_expansion_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the entity type, unique within the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_fuzzy_extraction` after provisioning.\nEnables fuzzy entity extraction during classification."]
    pub fn enable_fuzzy_extraction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_fuzzy_extraction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nIndicates whether the entity type can be automatically expanded.\n* KIND_MAP: Map entity types allow mapping of a group of synonyms to a canonical value.\n* KIND_LIST: List entity types contain a set of entries that do not map to canonical values. However, list entity types can contain references to other entity types (with or without aliases).\n* KIND_REGEXP: Regexp entity types allow to specify regular expressions in entries values. Possible values: [\"KIND_MAP\", \"KIND_LIST\", \"KIND_REGEXP\"]"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nThe language of the following fields in entityType:\nEntityType.entities.value\nEntityType.entities.synonyms\nEntityType.excluded_phrases.value\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the entity type.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a entity type for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redact` after provisioning.\nIndicates whether parameters of the entity type should be redacted in log. If redaction is enabled, page parameters and intent parameters referring to the entity type will be replaced by parameter name when logging."]
    pub fn redact(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entities` after provisioning.\n"]
    pub fn entities(&self) -> ListRef<DialogflowCxEntityTypeEntitiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_phrases` after provisioning.\n"]
    pub fn excluded_phrases(&self) -> ListRef<DialogflowCxEntityTypeExcludedPhrasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_phrases", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxEntityTypeTimeoutsElRef {
        DialogflowCxEntityTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxEntityType {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxEntityType {}
impl ToListMappable for DialogflowCxEntityType {
    type O = ListRef<DialogflowCxEntityTypeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxEntityType_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_entity_type".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxEntityType {
    pub tf_id: String,
    #[doc = "The human-readable name of the entity type, unique within the agent."]
    pub display_name: PrimField<String>,
    #[doc = "Indicates whether the entity type can be automatically expanded.\n* KIND_MAP: Map entity types allow mapping of a group of synonyms to a canonical value.\n* KIND_LIST: List entity types contain a set of entries that do not map to canonical values. However, list entity types can contain references to other entity types (with or without aliases).\n* KIND_REGEXP: Regexp entity types allow to specify regular expressions in entries values. Possible values: [\"KIND_MAP\", \"KIND_LIST\", \"KIND_REGEXP\"]"]
    pub kind: PrimField<String>,
}
impl BuildDialogflowCxEntityType {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxEntityType {
        let out = DialogflowCxEntityType(Rc::new(DialogflowCxEntityType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxEntityTypeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_expansion_mode: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                enable_fuzzy_extraction: core::default::Default::default(),
                id: core::default::Default::default(),
                kind: self.kind,
                language_code: core::default::Default::default(),
                parent: core::default::Default::default(),
                redact: core::default::Default::default(),
                entities: core::default::Default::default(),
                excluded_phrases: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxEntityTypeRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxEntityTypeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxEntityTypeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_expansion_mode` after provisioning.\nRepresents kinds of entities.\n* AUTO_EXPANSION_MODE_UNSPECIFIED: Auto expansion disabled for the entity.\n* AUTO_EXPANSION_MODE_DEFAULT: Allows an agent to recognize values that have not been explicitly listed in the entity. Possible values: [\"AUTO_EXPANSION_MODE_DEFAULT\", \"AUTO_EXPANSION_MODE_UNSPECIFIED\"]"]
    pub fn auto_expansion_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_expansion_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the entity type, unique within the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_fuzzy_extraction` after provisioning.\nEnables fuzzy entity extraction during classification."]
    pub fn enable_fuzzy_extraction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_fuzzy_extraction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nIndicates whether the entity type can be automatically expanded.\n* KIND_MAP: Map entity types allow mapping of a group of synonyms to a canonical value.\n* KIND_LIST: List entity types contain a set of entries that do not map to canonical values. However, list entity types can contain references to other entity types (with or without aliases).\n* KIND_REGEXP: Regexp entity types allow to specify regular expressions in entries values. Possible values: [\"KIND_MAP\", \"KIND_LIST\", \"KIND_REGEXP\"]"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\nThe language of the following fields in entityType:\nEntityType.entities.value\nEntityType.entities.synonyms\nEntityType.excluded_phrases.value\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the entity type.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a entity type for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redact` after provisioning.\nIndicates whether parameters of the entity type should be redacted in log. If redaction is enabled, page parameters and intent parameters referring to the entity type will be replaced by parameter name when logging."]
    pub fn redact(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entities` after provisioning.\n"]
    pub fn entities(&self) -> ListRef<DialogflowCxEntityTypeEntitiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_phrases` after provisioning.\n"]
    pub fn excluded_phrases(&self) -> ListRef<DialogflowCxEntityTypeExcludedPhrasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.excluded_phrases", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxEntityTypeTimeoutsElRef {
        DialogflowCxEntityTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxEntityTypeEntitiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    synonyms: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DialogflowCxEntityTypeEntitiesEl {
    #[doc = "Set the field `synonyms`.\nA collection of value synonyms. For example, if the entity type is vegetable, and value is scallions, a synonym could be green onions.\nFor KIND_LIST entity types: This collection must contain exactly one synonym equal to value."]
    pub fn set_synonyms(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.synonyms = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe primary value associated with this entity entry. For example, if the entity type is vegetable, the value could be scallions.\nFor KIND_MAP entity types: A canonical value to be used in place of synonyms.\nFor KIND_LIST entity types: A string that can contain references to other entity types (with or without aliases)."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxEntityTypeEntitiesEl {
    type O = BlockAssignable<DialogflowCxEntityTypeEntitiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxEntityTypeEntitiesEl {}
impl BuildDialogflowCxEntityTypeEntitiesEl {
    pub fn build(self) -> DialogflowCxEntityTypeEntitiesEl {
        DialogflowCxEntityTypeEntitiesEl {
            synonyms: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxEntityTypeEntitiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxEntityTypeEntitiesElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxEntityTypeEntitiesElRef {
        DialogflowCxEntityTypeEntitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxEntityTypeEntitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `synonyms` after provisioning.\nA collection of value synonyms. For example, if the entity type is vegetable, and value is scallions, a synonym could be green onions.\nFor KIND_LIST entity types: This collection must contain exactly one synonym equal to value."]
    pub fn synonyms(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.synonyms", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe primary value associated with this entity entry. For example, if the entity type is vegetable, the value could be scallions.\nFor KIND_MAP entity types: A canonical value to be used in place of synonyms.\nFor KIND_LIST entity types: A string that can contain references to other entity types (with or without aliases)."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxEntityTypeExcludedPhrasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DialogflowCxEntityTypeExcludedPhrasesEl {
    #[doc = "Set the field `value`.\nThe word or phrase to be excluded."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxEntityTypeExcludedPhrasesEl {
    type O = BlockAssignable<DialogflowCxEntityTypeExcludedPhrasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxEntityTypeExcludedPhrasesEl {}
impl BuildDialogflowCxEntityTypeExcludedPhrasesEl {
    pub fn build(self) -> DialogflowCxEntityTypeExcludedPhrasesEl {
        DialogflowCxEntityTypeExcludedPhrasesEl {
            value: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxEntityTypeExcludedPhrasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxEntityTypeExcludedPhrasesElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxEntityTypeExcludedPhrasesElRef {
        DialogflowCxEntityTypeExcludedPhrasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxEntityTypeExcludedPhrasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe word or phrase to be excluded."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxEntityTypeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowCxEntityTypeTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxEntityTypeTimeoutsEl {
    type O = BlockAssignable<DialogflowCxEntityTypeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxEntityTypeTimeoutsEl {}
impl BuildDialogflowCxEntityTypeTimeoutsEl {
    pub fn build(self) -> DialogflowCxEntityTypeTimeoutsEl {
        DialogflowCxEntityTypeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxEntityTypeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxEntityTypeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxEntityTypeTimeoutsElRef {
        DialogflowCxEntityTypeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxEntityTypeTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxEntityTypeDynamic {
    entities: Option<DynamicBlock<DialogflowCxEntityTypeEntitiesEl>>,
    excluded_phrases: Option<DynamicBlock<DialogflowCxEntityTypeExcludedPhrasesEl>>,
}
