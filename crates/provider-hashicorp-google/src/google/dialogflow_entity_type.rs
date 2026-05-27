use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowEntityTypeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_fuzzy_extraction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    kind: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<DialogflowEntityTypeEntitiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowEntityTypeTimeoutsEl>,
    dynamic: DialogflowEntityTypeDynamic,
}
struct DialogflowEntityType_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowEntityTypeData>,
}
#[derive(Clone)]
pub struct DialogflowEntityType(Rc<DialogflowEntityType_>);
impl DialogflowEntityType {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `entities`.\n"]
    pub fn set_entities(
        self,
        v: impl Into<BlockAssignable<DialogflowEntityTypeEntitiesEl>>,
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowEntityTypeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of this entity type to be displayed on the console."]
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
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nIndicates the kind of entity type.\n* KIND_MAP: Map entity types allow mapping of a group of synonyms to a reference value.\n* KIND_LIST: List entity types contain a set of entries that do not map to reference values. However, list entity\ntypes can contain references to other entity types (with or without aliases).\n* KIND_REGEXP: Regexp entity types allow to specify regular expressions in entries values. Possible values: [\"KIND_MAP\", \"KIND_LIST\", \"KIND_REGEXP\"]"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the entity type.\nFormat: projects/<Project ID>/agent/entityTypes/<Entity type ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entities` after provisioning.\n"]
    pub fn entities(&self) -> ListRef<DialogflowEntityTypeEntitiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowEntityTypeTimeoutsElRef {
        DialogflowEntityTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowEntityType {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowEntityType {}
impl ToListMappable for DialogflowEntityType {
    type O = ListRef<DialogflowEntityTypeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowEntityType_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_entity_type".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowEntityType {
    pub tf_id: String,
    #[doc = "The name of this entity type to be displayed on the console."]
    pub display_name: PrimField<String>,
    #[doc = "Indicates the kind of entity type.\n* KIND_MAP: Map entity types allow mapping of a group of synonyms to a reference value.\n* KIND_LIST: List entity types contain a set of entries that do not map to reference values. However, list entity\ntypes can contain references to other entity types (with or without aliases).\n* KIND_REGEXP: Regexp entity types allow to specify regular expressions in entries values. Possible values: [\"KIND_MAP\", \"KIND_LIST\", \"KIND_REGEXP\"]"]
    pub kind: PrimField<String>,
}
impl BuildDialogflowEntityType {
    pub fn build(self, stack: &mut Stack) -> DialogflowEntityType {
        let out = DialogflowEntityType(Rc::new(DialogflowEntityType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowEntityTypeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                enable_fuzzy_extraction: core::default::Default::default(),
                id: core::default::Default::default(),
                kind: self.kind,
                project: core::default::Default::default(),
                entities: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowEntityTypeRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowEntityTypeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowEntityTypeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of this entity type to be displayed on the console."]
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
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nIndicates the kind of entity type.\n* KIND_MAP: Map entity types allow mapping of a group of synonyms to a reference value.\n* KIND_LIST: List entity types contain a set of entries that do not map to reference values. However, list entity\ntypes can contain references to other entity types (with or without aliases).\n* KIND_REGEXP: Regexp entity types allow to specify regular expressions in entries values. Possible values: [\"KIND_MAP\", \"KIND_LIST\", \"KIND_REGEXP\"]"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the entity type.\nFormat: projects/<Project ID>/agent/entityTypes/<Entity type ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entities` after provisioning.\n"]
    pub fn entities(&self) -> ListRef<DialogflowEntityTypeEntitiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowEntityTypeTimeoutsElRef {
        DialogflowEntityTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowEntityTypeEntitiesEl {
    synonyms: ListField<PrimField<String>>,
    value: PrimField<String>,
}
impl DialogflowEntityTypeEntitiesEl {}
impl ToListMappable for DialogflowEntityTypeEntitiesEl {
    type O = BlockAssignable<DialogflowEntityTypeEntitiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowEntityTypeEntitiesEl {
    #[doc = "A collection of value synonyms. For example, if the entity type is vegetable, and value is scallions, a synonym\ncould be green onions.\nFor KIND_LIST entity types:\n* This collection must contain exactly one synonym equal to value."]
    pub synonyms: ListField<PrimField<String>>,
    #[doc = "The primary value associated with this entity entry. For example, if the entity type is vegetable, the value\ncould be scallions.\nFor KIND_MAP entity types:\n* A reference value to be used in place of synonyms.\nFor KIND_LIST entity types:\n* A string that can contain references to other entity types (with or without aliases)."]
    pub value: PrimField<String>,
}
impl BuildDialogflowEntityTypeEntitiesEl {
    pub fn build(self) -> DialogflowEntityTypeEntitiesEl {
        DialogflowEntityTypeEntitiesEl {
            synonyms: self.synonyms,
            value: self.value,
        }
    }
}
pub struct DialogflowEntityTypeEntitiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowEntityTypeEntitiesElRef {
    fn new(shared: StackShared, base: String) -> DialogflowEntityTypeEntitiesElRef {
        DialogflowEntityTypeEntitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowEntityTypeEntitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `synonyms` after provisioning.\nA collection of value synonyms. For example, if the entity type is vegetable, and value is scallions, a synonym\ncould be green onions.\nFor KIND_LIST entity types:\n* This collection must contain exactly one synonym equal to value."]
    pub fn synonyms(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.synonyms", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe primary value associated with this entity entry. For example, if the entity type is vegetable, the value\ncould be scallions.\nFor KIND_MAP entity types:\n* A reference value to be used in place of synonyms.\nFor KIND_LIST entity types:\n* A string that can contain references to other entity types (with or without aliases)."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowEntityTypeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowEntityTypeTimeoutsEl {
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
impl ToListMappable for DialogflowEntityTypeTimeoutsEl {
    type O = BlockAssignable<DialogflowEntityTypeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowEntityTypeTimeoutsEl {}
impl BuildDialogflowEntityTypeTimeoutsEl {
    pub fn build(self) -> DialogflowEntityTypeTimeoutsEl {
        DialogflowEntityTypeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowEntityTypeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowEntityTypeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowEntityTypeTimeoutsElRef {
        DialogflowEntityTypeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowEntityTypeTimeoutsElRef {
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
struct DialogflowEntityTypeDynamic {
    entities: Option<DynamicBlock<DialogflowEntityTypeEntitiesEl>>,
}
