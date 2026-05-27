use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DocumentAiWarehouseDocumentSchemaData {
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
    document_is_folder: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    project_number: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    property_definitions: Option<Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DocumentAiWarehouseDocumentSchemaTimeoutsEl>,
    dynamic: DocumentAiWarehouseDocumentSchemaDynamic,
}
struct DocumentAiWarehouseDocumentSchema_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DocumentAiWarehouseDocumentSchemaData>,
}
#[derive(Clone)]
pub struct DocumentAiWarehouseDocumentSchema(Rc<DocumentAiWarehouseDocumentSchema_>);
impl DocumentAiWarehouseDocumentSchema {
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
    #[doc = "Set the field `document_is_folder`.\nTells whether the document is a folder or a typical document."]
    pub fn set_document_is_folder(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().document_is_folder = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `property_definitions`.\n"]
    pub fn set_property_definitions(
        self,
        v: impl Into<BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().property_definitions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.property_definitions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DocumentAiWarehouseDocumentSchemaTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nName of the schema given by the user."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `document_is_folder` after provisioning.\nTells whether the document is a folder or a typical document."]
    pub fn document_is_folder(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_is_folder", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the document schema."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_number` after provisioning.\nThe unique identifier of the project."]
    pub fn project_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `property_definitions` after provisioning.\n"]
    pub fn property_definitions(
        &self,
    ) -> ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.property_definitions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
        DocumentAiWarehouseDocumentSchemaTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DocumentAiWarehouseDocumentSchema {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DocumentAiWarehouseDocumentSchema {}
impl ToListMappable for DocumentAiWarehouseDocumentSchema {
    type O = ListRef<DocumentAiWarehouseDocumentSchemaRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DocumentAiWarehouseDocumentSchema_ {
    fn extract_resource_type(&self) -> String {
        "google_document_ai_warehouse_document_schema".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDocumentAiWarehouseDocumentSchema {
    pub tf_id: String,
    #[doc = "Name of the schema given by the user."]
    pub display_name: PrimField<String>,
    #[doc = "The location of the resource."]
    pub location: PrimField<String>,
    #[doc = "The unique identifier of the project."]
    pub project_number: PrimField<String>,
}
impl BuildDocumentAiWarehouseDocumentSchema {
    pub fn build(self, stack: &mut Stack) -> DocumentAiWarehouseDocumentSchema {
        let out = DocumentAiWarehouseDocumentSchema(Rc::new(DocumentAiWarehouseDocumentSchema_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DocumentAiWarehouseDocumentSchemaData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                document_is_folder: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project_number: self.project_number,
                property_definitions: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DocumentAiWarehouseDocumentSchemaRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DocumentAiWarehouseDocumentSchemaRef {
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nName of the schema given by the user."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `document_is_folder` after provisioning.\nTells whether the document is a folder or a typical document."]
    pub fn document_is_folder(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_is_folder", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the document schema."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_number` after provisioning.\nThe unique identifier of the project."]
    pub fn project_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `property_definitions` after provisioning.\n"]
    pub fn property_definitions(
        &self,
    ) -> ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.property_definitions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
        DocumentAiWarehouseDocumentSchemaTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl {}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl {}
impl ToListMappable
    for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl
{
    type O = BlockAssignable<
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl {}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl {}
    }
}
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
    possible_values: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_check_disabled: Option<PrimField<bool>>,
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
    #[doc = "Set the field `validation_check_disabled`.\nMake sure the enum property value provided in the document is in the possile value list during document creation. The validation check runs by default."]
    pub fn set_validation_check_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.validation_check_disabled = Some(v.into());
        self
    }
}
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
    type O =
        BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
    #[doc = "List of possible enum values."]
    pub possible_values: ListField<PrimField<String>>,
}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
            possible_values: self.possible_values,
            validation_check_disabled: core::default::Default::default(),
        }
    }
}
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `possible_values` after provisioning.\nList of possible enum values."]
    pub fn possible_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.possible_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `validation_check_disabled` after provisioning.\nMake sure the enum property value provided in the document is in the possile value list during document creation. The validation check runs by default."]
    pub fn validation_check_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.validation_check_disabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {}
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {
    type O =
        BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {}
    }
}
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {}
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {
    type O =
        BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {}
    }
}
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {}
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {
    type O =
        BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {}
    }
}
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl
{}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl { }
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl { type O = BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl
{}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl { pub fn build (self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl { } } }
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsElRef { fn new (shared : StackShared , base : String) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsElRef { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsElRef { shared : shared , base : base . to_string () , } } }
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl
{
    possible_values: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_check_disabled: Option<PrimField<bool>>,
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl { # [doc = "Set the field `validation_check_disabled`.\nMake sure the enum property value provided in the document is in the possile value list during document creation. The validation check runs by default."] pub fn set_validation_check_disabled (mut self , v : impl Into < PrimField < bool > >) -> Self { self . validation_check_disabled = Some (v . into ()) ; self } }
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl { type O = BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl
{
    #[doc = "List of possible enum values."]
    pub possible_values: ListField<PrimField<String>>,
}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl { pub fn build (self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl { possible_values : self . possible_values , validation_check_disabled : core :: default :: Default :: default () , } } }
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsElRef { fn new (shared : StackShared , base : String) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsElRef { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsElRef { shared : shared , base : base . to_string () , } } }
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `possible_values` after provisioning.\nList of possible enum values."] pub fn possible_values (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.possible_values" , self . base)) } # [doc = "Get a reference to the value of field `validation_check_disabled` after provisioning.\nMake sure the enum property value provided in the document is in the possile value list during document creation. The validation check runs by default."] pub fn validation_check_disabled (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.validation_check_disabled" , self . base)) } }
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl
{}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl { }
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl { type O = BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl
{}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl { pub fn build (self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl { } } }
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsElRef { fn new (shared : StackShared , base : String) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsElRef { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsElRef { shared : shared , base : base . to_string () , } } }
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl
{}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl { }
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl { type O = BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl
{}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl { pub fn build (self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl { } } }
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsElRef { fn new (shared : StackShared , base : String) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsElRef { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsElRef { shared : shared , base : base . to_string () , } } }
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl
{}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl { }
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl { type O = BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl
{}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl { pub fn build (self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl { } } }
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsElRef { fn new (shared : StackShared , base : String) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsElRef { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsElRef { shared : shared , base : base . to_string () , } } }
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processor_type: Option<PrimField<String>>,
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl { # [doc = "Set the field `name`.\nThe schema name in the source."] pub fn set_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . name = Some (v . into ()) ; self } # [doc = "Set the field `processor_type`.\nThe Doc AI processor type name."] pub fn set_processor_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . processor_type = Some (v . into ()) ; self } }
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl { type O = BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl
{}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl { pub fn build (self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl { name : core :: default :: Default :: default () , processor_type : core :: default :: Default :: default () , } } }
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesElRef { fn new (shared : StackShared , base : String) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesElRef { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesElRef { shared : shared , base : base . to_string () , } } }
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nThe schema name in the source."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `processor_type` after provisioning.\nThe Doc AI processor type name."] pub fn processor_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.processor_type" , self . base)) } }
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl
{}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl { }
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl { type O = BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl
{}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl { pub fn build (self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl { } } }
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsElRef { fn new (shared : StackShared , base : String) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsElRef { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsElRef { shared : shared , base : base . to_string () , } } }
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl
{}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl { }
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl { type O = BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl
{}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl { pub fn build (self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl { } } }
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsElRef { fn new (shared : StackShared , base : String) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsElRef { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsElRef { shared : shared , base : base . to_string () , } } }
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize, Default)]
struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDynamic { date_time_type_options : Option < DynamicBlock < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl >> , enum_type_options : Option < DynamicBlock < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl >> , float_type_options : Option < DynamicBlock < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl >> , integer_type_options : Option < DynamicBlock < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl >> , map_type_options : Option < DynamicBlock < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl >> , schema_sources : Option < DynamicBlock < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl >> , text_type_options : Option < DynamicBlock < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl >> , timestamp_type_options : Option < DynamicBlock < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl >> , }
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl { # [serde (skip_serializing_if = "Option::is_none")] display_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] is_filterable : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] is_metadata : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] is_repeatable : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] is_required : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] is_searchable : Option < PrimField < bool > > , name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] retrieval_importance : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] date_time_type_options : Option < Vec < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl > > , # [serde (skip_serializing_if = "Option::is_none")] enum_type_options : Option < Vec < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl > > , # [serde (skip_serializing_if = "Option::is_none")] float_type_options : Option < Vec < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl > > , # [serde (skip_serializing_if = "Option::is_none")] integer_type_options : Option < Vec < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl > > , # [serde (skip_serializing_if = "Option::is_none")] map_type_options : Option < Vec < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl > > , # [serde (skip_serializing_if = "Option::is_none")] schema_sources : Option < Vec < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl > > , # [serde (skip_serializing_if = "Option::is_none")] text_type_options : Option < Vec < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl > > , # [serde (skip_serializing_if = "Option::is_none")] timestamp_type_options : Option < Vec < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl > > , dynamic : DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDynamic , }
impl
    DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl
{
    #[doc = "Set the field `display_name`.\nThe display-name for the property, used for front-end."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `is_filterable`.\nWhether the property can be filtered. If this is a sub-property, all the parent properties must be marked filterable."]
    pub fn set_is_filterable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_filterable = Some(v.into());
        self
    }
    #[doc = "Set the field `is_metadata`.\nWhether the property is user supplied metadata."]
    pub fn set_is_metadata(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `is_repeatable`.\nWhether the property can have multiple values."]
    pub fn set_is_repeatable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_repeatable = Some(v.into());
        self
    }
    #[doc = "Set the field `is_required`.\nWhether the property is mandatory."]
    pub fn set_is_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_required = Some(v.into());
        self
    }
    #[doc = "Set the field `is_searchable`.\nIndicates that the property should be included in a global search."]
    pub fn set_is_searchable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_searchable = Some(v.into());
        self
    }
    #[doc = "Set the field `retrieval_importance`.\nStores the retrieval importance. Possible values: [\"HIGHEST\", \"HIGHER\", \"HIGH\", \"MEDIUM\", \"LOW\", \"LOWEST\"]"]
    pub fn set_retrieval_importance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retrieval_importance = Some(v.into());
        self
    }
    #[doc = "Set the field `date_time_type_options`.\n"]
    pub fn set_date_time_type_options(
        mut self,
        v : impl Into < BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_time_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_time_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `enum_type_options`.\n"]
    pub fn set_enum_type_options(
        mut self,
        v : impl Into < BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.enum_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.enum_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `float_type_options`.\n"]
    pub fn set_float_type_options(
        mut self,
        v : impl Into < BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.float_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.float_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `integer_type_options`.\n"]
    pub fn set_integer_type_options(
        mut self,
        v : impl Into < BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.integer_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.integer_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `map_type_options`.\n"]
    pub fn set_map_type_options(
        mut self,
        v : impl Into < BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.map_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.map_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schema_sources`.\n"]
    pub fn set_schema_sources(
        mut self,
        v : impl Into < BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema_sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_sources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text_type_options`.\n"]
    pub fn set_text_type_options(
        mut self,
        v : impl Into < BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timestamp_type_options`.\n"]
    pub fn set_timestamp_type_options(
        mut self,
        v : impl Into < BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timestamp_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timestamp_type_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl { type O = BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl
{
    #[doc = "The name of the metadata property."]
    pub name: PrimField<String>,
}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl { pub fn build (self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl { display_name : core :: default :: Default :: default () , is_filterable : core :: default :: Default :: default () , is_metadata : core :: default :: Default :: default () , is_repeatable : core :: default :: Default :: default () , is_required : core :: default :: Default :: default () , is_searchable : core :: default :: Default :: default () , name : self . name , retrieval_importance : core :: default :: Default :: default () , date_time_type_options : core :: default :: Default :: default () , enum_type_options : core :: default :: Default :: default () , float_type_options : core :: default :: Default :: default () , integer_type_options : core :: default :: Default :: default () , map_type_options : core :: default :: Default :: default () , schema_sources : core :: default :: Default :: default () , text_type_options : core :: default :: Default :: default () , timestamp_type_options : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElRef { fn new (shared : StackShared , base : String) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElRef { DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElRef { shared : shared , base : base . to_string () , } } }
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display-name for the property, used for front-end."] pub fn display_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.display_name" , self . base)) } # [doc = "Get a reference to the value of field `is_filterable` after provisioning.\nWhether the property can be filtered. If this is a sub-property, all the parent properties must be marked filterable."] pub fn is_filterable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.is_filterable" , self . base)) } # [doc = "Get a reference to the value of field `is_metadata` after provisioning.\nWhether the property is user supplied metadata."] pub fn is_metadata (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.is_metadata" , self . base)) } # [doc = "Get a reference to the value of field `is_repeatable` after provisioning.\nWhether the property can have multiple values."] pub fn is_repeatable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.is_repeatable" , self . base)) } # [doc = "Get a reference to the value of field `is_required` after provisioning.\nWhether the property is mandatory."] pub fn is_required (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.is_required" , self . base)) } # [doc = "Get a reference to the value of field `is_searchable` after provisioning.\nIndicates that the property should be included in a global search."] pub fn is_searchable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.is_searchable" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the metadata property."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `retrieval_importance` after provisioning.\nStores the retrieval importance. Possible values: [\"HIGHEST\", \"HIGHER\", \"HIGH\", \"MEDIUM\", \"LOW\", \"LOWEST\"]"] pub fn retrieval_importance (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.retrieval_importance" , self . base)) } # [doc = "Get a reference to the value of field `date_time_type_options` after provisioning.\n"] pub fn date_time_type_options (& self) -> ListRef < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.date_time_type_options" , self . base)) } # [doc = "Get a reference to the value of field `enum_type_options` after provisioning.\n"] pub fn enum_type_options (& self) -> ListRef < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.enum_type_options" , self . base)) } # [doc = "Get a reference to the value of field `float_type_options` after provisioning.\n"] pub fn float_type_options (& self) -> ListRef < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.float_type_options" , self . base)) } # [doc = "Get a reference to the value of field `integer_type_options` after provisioning.\n"] pub fn integer_type_options (& self) -> ListRef < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.integer_type_options" , self . base)) } # [doc = "Get a reference to the value of field `map_type_options` after provisioning.\n"] pub fn map_type_options (& self) -> ListRef < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.map_type_options" , self . base)) } # [doc = "Get a reference to the value of field `schema_sources` after provisioning.\n"] pub fn schema_sources (& self) -> ListRef < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.schema_sources" , self . base)) } # [doc = "Get a reference to the value of field `text_type_options` after provisioning.\n"] pub fn text_type_options (& self) -> ListRef < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.text_type_options" , self . base)) } # [doc = "Get a reference to the value of field `timestamp_type_options` after provisioning.\n"] pub fn timestamp_type_options (& self) -> ListRef < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.timestamp_type_options" , self . base)) } }
#[derive(Serialize, Default)]
struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElDynamic { property_definitions : Option < DynamicBlock < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl >> , }
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl { # [serde (skip_serializing_if = "Option::is_none")] property_definitions : Option < Vec < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl > > , dynamic : DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElDynamic , }
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl {
    #[doc = "Set the field `property_definitions`.\n"]
    pub fn set_property_definitions(
        mut self,
        v : impl Into < BlockAssignable < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.property_definitions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.property_definitions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl
{
    type O = BlockAssignable<
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl {}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl {
            property_definitions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `property_definitions` after provisioning.\n"]    pub fn property_definitions (& self) -> ListRef < DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.property_definitions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    processor_type: Option<PrimField<String>>,
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {
    #[doc = "Set the field `name`.\nThe schema name in the source."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `processor_type`.\nThe Doc AI processor type name."]
    pub fn set_processor_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.processor_type = Some(v.into());
        self
    }
}
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {
    type O = BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {
            name: core::default::Default::default(),
            processor_type: core::default::Default::default(),
        }
    }
}
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe schema name in the source."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `processor_type` after provisioning.\nThe Doc AI processor type name."]
    pub fn processor_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.processor_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {}
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {
    type O =
        BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {}
    }
}
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl {}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl {}
impl ToListMappable
    for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl
{
    type O = BlockAssignable<
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl {}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl {}
    }
}
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDynamic {
    date_time_type_options: Option<
        DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl>,
    >,
    enum_type_options: Option<
        DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl>,
    >,
    float_type_options: Option<
        DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl>,
    >,
    integer_type_options: Option<
        DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl>,
    >,
    map_type_options: Option<
        DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl>,
    >,
    property_type_options: Option<
        DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl>,
    >,
    schema_sources:
        Option<DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl>>,
    text_type_options: Option<
        DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl>,
    >,
    timestamp_type_options: Option<
        DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl>,
    >,
}
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_filterable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_metadata: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_repeatable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_searchable: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retrieval_importance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_time_type_options:
        Option<Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_type_options:
        Option<Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    float_type_options:
        Option<Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integer_type_options:
        Option<Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    map_type_options:
        Option<Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    property_type_options:
        Option<Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_sources:
        Option<Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_type_options:
        Option<Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_type_options:
        Option<Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl>>,
    dynamic: DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDynamic,
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
    #[doc = "Set the field `display_name`.\nThe display-name for the property, used for front-end."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `is_filterable`.\nWhether the property can be filtered. If this is a sub-property, all the parent properties must be marked filterable."]
    pub fn set_is_filterable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_filterable = Some(v.into());
        self
    }
    #[doc = "Set the field `is_metadata`.\nWhether the property is user supplied metadata."]
    pub fn set_is_metadata(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `is_repeatable`.\nWhether the property can have multiple values."]
    pub fn set_is_repeatable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_repeatable = Some(v.into());
        self
    }
    #[doc = "Set the field `is_required`.\nWhether the property is mandatory."]
    pub fn set_is_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_required = Some(v.into());
        self
    }
    #[doc = "Set the field `is_searchable`.\nIndicates that the property should be included in a global search."]
    pub fn set_is_searchable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_searchable = Some(v.into());
        self
    }
    #[doc = "Set the field `retrieval_importance`.\nStores the retrieval importance. Possible values: [\"HIGHEST\", \"HIGHER\", \"HIGH\", \"MEDIUM\", \"LOW\", \"LOWEST\"]"]
    pub fn set_retrieval_importance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retrieval_importance = Some(v.into());
        self
    }
    #[doc = "Set the field `date_time_type_options`.\n"]
    pub fn set_date_time_type_options(
        mut self,
        v: impl Into<
            BlockAssignable<
                DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.date_time_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.date_time_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `enum_type_options`.\n"]
    pub fn set_enum_type_options(
        mut self,
        v: impl Into<
            BlockAssignable<
                DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.enum_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.enum_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `float_type_options`.\n"]
    pub fn set_float_type_options(
        mut self,
        v: impl Into<
            BlockAssignable<
                DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.float_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.float_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `integer_type_options`.\n"]
    pub fn set_integer_type_options(
        mut self,
        v: impl Into<
            BlockAssignable<
                DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.integer_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.integer_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `map_type_options`.\n"]
    pub fn set_map_type_options(
        mut self,
        v: impl Into<
            BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.map_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.map_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `property_type_options`.\n"]
    pub fn set_property_type_options(
        mut self,
        v: impl Into<
            BlockAssignable<
                DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.property_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.property_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schema_sources`.\n"]
    pub fn set_schema_sources(
        mut self,
        v: impl Into<
            BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema_sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_sources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `text_type_options`.\n"]
    pub fn set_text_type_options(
        mut self,
        v: impl Into<
            BlockAssignable<
                DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.text_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.text_type_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timestamp_type_options`.\n"]
    pub fn set_timestamp_type_options(
        mut self,
        v: impl Into<
            BlockAssignable<
                DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timestamp_type_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timestamp_type_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
    type O = BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
    #[doc = "The name of the metadata property."]
    pub name: PrimField<String>,
}
impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
            display_name: core::default::Default::default(),
            is_filterable: core::default::Default::default(),
            is_metadata: core::default::Default::default(),
            is_repeatable: core::default::Default::default(),
            is_required: core::default::Default::default(),
            is_searchable: core::default::Default::default(),
            name: self.name,
            retrieval_importance: core::default::Default::default(),
            date_time_type_options: core::default::Default::default(),
            enum_type_options: core::default::Default::default(),
            float_type_options: core::default::Default::default(),
            integer_type_options: core::default::Default::default(),
            map_type_options: core::default::Default::default(),
            property_type_options: core::default::Default::default(),
            schema_sources: core::default::Default::default(),
            text_type_options: core::default::Default::default(),
            timestamp_type_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display-name for the property, used for front-end."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `is_filterable` after provisioning.\nWhether the property can be filtered. If this is a sub-property, all the parent properties must be marked filterable."]
    pub fn is_filterable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_filterable", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_metadata` after provisioning.\nWhether the property is user supplied metadata."]
    pub fn is_metadata(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `is_repeatable` after provisioning.\nWhether the property can have multiple values."]
    pub fn is_repeatable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_repeatable", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_required` after provisioning.\nWhether the property is mandatory."]
    pub fn is_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_required", self.base))
    }
    #[doc = "Get a reference to the value of field `is_searchable` after provisioning.\nIndicates that the property should be included in a global search."]
    pub fn is_searchable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_searchable", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the metadata property."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `retrieval_importance` after provisioning.\nStores the retrieval importance. Possible values: [\"HIGHEST\", \"HIGHER\", \"HIGH\", \"MEDIUM\", \"LOW\", \"LOWEST\"]"]
    pub fn retrieval_importance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retrieval_importance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `date_time_type_options` after provisioning.\n"]
    pub fn date_time_type_options(
        &self,
    ) -> ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.date_time_type_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enum_type_options` after provisioning.\n"]
    pub fn enum_type_options(
        &self,
    ) -> ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enum_type_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `float_type_options` after provisioning.\n"]
    pub fn float_type_options(
        &self,
    ) -> ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.float_type_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `integer_type_options` after provisioning.\n"]
    pub fn integer_type_options(
        &self,
    ) -> ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.integer_type_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `map_type_options` after provisioning.\n"]
    pub fn map_type_options(
        &self,
    ) -> ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.map_type_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `property_type_options` after provisioning.\n"]
    pub fn property_type_options(
        &self,
    ) -> ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.property_type_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schema_sources` after provisioning.\n"]
    pub fn schema_sources(
        &self,
    ) -> ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_sources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text_type_options` after provisioning.\n"]
    pub fn text_type_options(
        &self,
    ) -> ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.text_type_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timestamp_type_options` after provisioning.\n"]
    pub fn timestamp_type_options(
        &self,
    ) -> ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.timestamp_type_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl DocumentAiWarehouseDocumentSchemaTimeoutsEl {
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
}
impl ToListMappable for DocumentAiWarehouseDocumentSchemaTimeoutsEl {
    type O = BlockAssignable<DocumentAiWarehouseDocumentSchemaTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDocumentAiWarehouseDocumentSchemaTimeoutsEl {}
impl BuildDocumentAiWarehouseDocumentSchemaTimeoutsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaTimeoutsEl {
        DocumentAiWarehouseDocumentSchemaTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
        DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct DocumentAiWarehouseDocumentSchemaDynamic {
    property_definitions:
        Option<DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl>>,
}
