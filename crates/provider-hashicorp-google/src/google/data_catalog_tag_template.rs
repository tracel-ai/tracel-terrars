use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCatalogTagTemplateData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    tag_template_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<DataCatalogTagTemplateFieldsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataCatalogTagTemplateTimeoutsEl>,
    dynamic: DataCatalogTagTemplateDynamic,
}
struct DataCatalogTagTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCatalogTagTemplateData>,
}
#[derive(Clone)]
pub struct DataCatalogTagTemplate(Rc<DataCatalogTagTemplate_>);
impl DataCatalogTagTemplate {
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
    #[doc = "Set the field `display_name`.\nThe display name for this template."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `force_delete`.\nThis confirms the deletion of any possible tags using this template. Must be set to true in order to delete the tag template."]
    pub fn set_force_delete(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = Some(v.into());
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
    #[doc = "Set the field `region`.\nTemplate location region."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(self, v: impl Into<BlockAssignable<DataCatalogTagTemplateFieldsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fields = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataCatalogTagTemplateTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for this template."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\nThis confirms the deletion of any possible tags using this template. Must be set to true in order to delete the tag template."]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the tag template in URL format. Example: projects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}"]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nTemplate location region."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tag_template_id` after provisioning.\nThe id of the tag template to create."]
    pub fn tag_template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tag_template_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataCatalogTagTemplateTimeoutsElRef {
        DataCatalogTagTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataCatalogTagTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataCatalogTagTemplate {}
impl ToListMappable for DataCatalogTagTemplate {
    type O = ListRef<DataCatalogTagTemplateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataCatalogTagTemplate_ {
    fn extract_resource_type(&self) -> String {
        "google_data_catalog_tag_template".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCatalogTagTemplate {
    pub tf_id: String,
    #[doc = "The id of the tag template to create."]
    pub tag_template_id: PrimField<String>,
}
impl BuildDataCatalogTagTemplate {
    pub fn build(self, stack: &mut Stack) -> DataCatalogTagTemplate {
        let out = DataCatalogTagTemplate(Rc::new(DataCatalogTagTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCatalogTagTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                tag_template_id: self.tag_template_id,
                fields: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataCatalogTagTemplateRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogTagTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCatalogTagTemplateRef {
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for this template."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\nThis confirms the deletion of any possible tags using this template. Must be set to true in order to delete the tag template."]
    pub fn force_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_delete", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the tag template in URL format. Example: projects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}"]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nTemplate location region."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tag_template_id` after provisioning.\nThe id of the tag template to create."]
    pub fn tag_template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tag_template_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataCatalogTagTemplateTimeoutsElRef {
        DataCatalogTagTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl {
    display_name: PrimField<String>,
}
impl DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl {}
impl ToListMappable for DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl {
    type O = BlockAssignable<DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl {
    #[doc = "The display name of the enum value."]
    pub display_name: PrimField<String>,
}
impl BuildDataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl {
    pub fn build(self) -> DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl {
        DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl {
            display_name: self.display_name,
        }
    }
}
pub struct DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesElRef {
        DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the enum value."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataCatalogTagTemplateFieldsElTypeElEnumTypeElDynamic {
    allowed_values:
        Option<DynamicBlock<DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl>>,
}
#[derive(Serialize)]
pub struct DataCatalogTagTemplateFieldsElTypeElEnumTypeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_values: Option<Vec<DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl>>,
    dynamic: DataCatalogTagTemplateFieldsElTypeElEnumTypeElDynamic,
}
impl DataCatalogTagTemplateFieldsElTypeElEnumTypeEl {
    #[doc = "Set the field `allowed_values`.\n"]
    pub fn set_allowed_values(
        mut self,
        v: impl Into<BlockAssignable<DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allowed_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataCatalogTagTemplateFieldsElTypeElEnumTypeEl {
    type O = BlockAssignable<DataCatalogTagTemplateFieldsElTypeElEnumTypeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogTagTemplateFieldsElTypeElEnumTypeEl {}
impl BuildDataCatalogTagTemplateFieldsElTypeElEnumTypeEl {
    pub fn build(self) -> DataCatalogTagTemplateFieldsElTypeElEnumTypeEl {
        DataCatalogTagTemplateFieldsElTypeElEnumTypeEl {
            allowed_values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataCatalogTagTemplateFieldsElTypeElEnumTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogTagTemplateFieldsElTypeElEnumTypeElRef {
    fn new(shared: StackShared, base: String) -> DataCatalogTagTemplateFieldsElTypeElEnumTypeElRef {
        DataCatalogTagTemplateFieldsElTypeElEnumTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogTagTemplateFieldsElTypeElEnumTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DataCatalogTagTemplateFieldsElTypeElDynamic {
    enum_type: Option<DynamicBlock<DataCatalogTagTemplateFieldsElTypeElEnumTypeEl>>,
}
#[derive(Serialize)]
pub struct DataCatalogTagTemplateFieldsElTypeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primitive_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_type: Option<Vec<DataCatalogTagTemplateFieldsElTypeElEnumTypeEl>>,
    dynamic: DataCatalogTagTemplateFieldsElTypeElDynamic,
}
impl DataCatalogTagTemplateFieldsElTypeEl {
    #[doc = "Set the field `primitive_type`.\nRepresents primitive types - string, bool etc.\n Exactly one of 'primitive_type' or 'enum_type' must be set Possible values: [\"DOUBLE\", \"STRING\", \"BOOL\", \"TIMESTAMP\"]"]
    pub fn set_primitive_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primitive_type = Some(v.into());
        self
    }
    #[doc = "Set the field `enum_type`.\n"]
    pub fn set_enum_type(
        mut self,
        v: impl Into<BlockAssignable<DataCatalogTagTemplateFieldsElTypeElEnumTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.enum_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.enum_type = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataCatalogTagTemplateFieldsElTypeEl {
    type O = BlockAssignable<DataCatalogTagTemplateFieldsElTypeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogTagTemplateFieldsElTypeEl {}
impl BuildDataCatalogTagTemplateFieldsElTypeEl {
    pub fn build(self) -> DataCatalogTagTemplateFieldsElTypeEl {
        DataCatalogTagTemplateFieldsElTypeEl {
            primitive_type: core::default::Default::default(),
            enum_type: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataCatalogTagTemplateFieldsElTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogTagTemplateFieldsElTypeElRef {
    fn new(shared: StackShared, base: String) -> DataCatalogTagTemplateFieldsElTypeElRef {
        DataCatalogTagTemplateFieldsElTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogTagTemplateFieldsElTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `primitive_type` after provisioning.\nRepresents primitive types - string, bool etc.\n Exactly one of 'primitive_type' or 'enum_type' must be set Possible values: [\"DOUBLE\", \"STRING\", \"BOOL\", \"TIMESTAMP\"]"]
    pub fn primitive_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primitive_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enum_type` after provisioning.\n"]
    pub fn enum_type(&self) -> ListRef<DataCatalogTagTemplateFieldsElTypeElEnumTypeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.enum_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataCatalogTagTemplateFieldsElDynamic {
    type_: Option<DynamicBlock<DataCatalogTagTemplateFieldsElTypeEl>>,
}
#[derive(Serialize)]
pub struct DataCatalogTagTemplateFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    field_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<Vec<DataCatalogTagTemplateFieldsElTypeEl>>,
    dynamic: DataCatalogTagTemplateFieldsElDynamic,
}
impl DataCatalogTagTemplateFieldsEl {
    #[doc = "Set the field `description`.\nA description for this field."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name for this field."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `is_required`.\nWhether this is a required field. Defaults to false."]
    pub fn set_is_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_required = Some(v.into());
        self
    }
    #[doc = "Set the field `order`.\nThe order of this field with respect to other fields in this tag template.\nA higher value indicates a more important field. The value can be negative.\nMultiple fields can have the same order, and field orders within a tag do not have to be sequential."]
    pub fn set_order(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.order = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(
        mut self,
        v: impl Into<BlockAssignable<DataCatalogTagTemplateFieldsElTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.type_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.type_ = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataCatalogTagTemplateFieldsEl {
    type O = BlockAssignable<DataCatalogTagTemplateFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogTagTemplateFieldsEl {
    #[doc = ""]
    pub field_id: PrimField<String>,
}
impl BuildDataCatalogTagTemplateFieldsEl {
    pub fn build(self) -> DataCatalogTagTemplateFieldsEl {
        DataCatalogTagTemplateFieldsEl {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            field_id: self.field_id,
            is_required: core::default::Default::default(),
            order: core::default::Default::default(),
            type_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataCatalogTagTemplateFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogTagTemplateFieldsElRef {
    fn new(shared: StackShared, base: String) -> DataCatalogTagTemplateFieldsElRef {
        DataCatalogTagTemplateFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogTagTemplateFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description for this field."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name for this field."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `field_id` after provisioning.\n"]
    pub fn field_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field_id", self.base))
    }
    #[doc = "Get a reference to the value of field `is_required` after provisioning.\nWhether this is a required field. Defaults to false."]
    pub fn is_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_required", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the tag template field in URL format. Example: projects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}/fields/{field}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `order` after provisioning.\nThe order of this field with respect to other fields in this tag template.\nA higher value indicates a more important field. The value can be negative.\nMultiple fields can have the same order, and field orders within a tag do not have to be sequential."]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> ListRef<DataCatalogTagTemplateFieldsElTypeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCatalogTagTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataCatalogTagTemplateTimeoutsEl {
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
impl ToListMappable for DataCatalogTagTemplateTimeoutsEl {
    type O = BlockAssignable<DataCatalogTagTemplateTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogTagTemplateTimeoutsEl {}
impl BuildDataCatalogTagTemplateTimeoutsEl {
    pub fn build(self) -> DataCatalogTagTemplateTimeoutsEl {
        DataCatalogTagTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataCatalogTagTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogTagTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataCatalogTagTemplateTimeoutsElRef {
        DataCatalogTagTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogTagTemplateTimeoutsElRef {
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
struct DataCatalogTagTemplateDynamic {
    fields: Option<DynamicBlock<DataCatalogTagTemplateFieldsEl>>,
}
