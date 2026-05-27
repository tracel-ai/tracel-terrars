use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCatalogTagData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    template: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<DataCatalogTagFieldsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataCatalogTagTimeoutsEl>,
    dynamic: DataCatalogTagDynamic,
}
struct DataCatalogTag_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCatalogTagData>,
}
#[derive(Clone)]
pub struct DataCatalogTag(Rc<DataCatalogTag_>);
impl DataCatalogTag {
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
    #[doc = "Set the field `column`.\nResources like Entry can have schemas associated with them. This scope allows users to attach tags to an\nindividual column based on that schema.\n\nFor attaching a tag to a nested column, use '.' to separate the column names. Example:\n'outer_column.inner_column'"]
    pub fn set_column(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().column = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\nThe name of the parent this tag is attached to. This can be the name of an entry or an entry group. If an entry group, the tag will be attached to\nall entries in that group."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(self, v: impl Into<BlockAssignable<DataCatalogTagFieldsEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<DataCatalogTagTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `column` after provisioning.\nResources like Entry can have schemas associated with them. This scope allows users to attach tags to an\nindividual column based on that schema.\n\nFor attaching a tag to a nested column, use '.' to separate the column names. Example:\n'outer_column.inner_column'"]
    pub fn column(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.column", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the tag in URL format. Example:\nprojects/{project_id}/locations/{location}/entrygroups/{entryGroupId}/entries/{entryId}/tags/{tag_id} or\nprojects/{project_id}/locations/{location}/entrygroups/{entryGroupId}/tags/{tag_id}\nwhere tag_id is a system-generated identifier. Note that this Tag may not actually be stored in the location in this name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe name of the parent this tag is attached to. This can be the name of an entry or an entry group. If an entry group, the tag will be attached to\nall entries in that group."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\nThe resource name of the tag template that this tag uses. Example:\nprojects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}\nThis field cannot be modified after creation."]
    pub fn template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_displayname` after provisioning.\nThe display name of the tag template."]
    pub fn template_displayname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_displayname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataCatalogTagTimeoutsElRef {
        DataCatalogTagTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataCatalogTag {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataCatalogTag {}
impl ToListMappable for DataCatalogTag {
    type O = ListRef<DataCatalogTagRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataCatalogTag_ {
    fn extract_resource_type(&self) -> String {
        "google_data_catalog_tag".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCatalogTag {
    pub tf_id: String,
    #[doc = "The resource name of the tag template that this tag uses. Example:\nprojects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}\nThis field cannot be modified after creation."]
    pub template: PrimField<String>,
}
impl BuildDataCatalogTag {
    pub fn build(self, stack: &mut Stack) -> DataCatalogTag {
        let out = DataCatalogTag(Rc::new(DataCatalogTag_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCatalogTagData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                column: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: core::default::Default::default(),
                template: self.template,
                fields: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataCatalogTagRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogTagRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCatalogTagRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column` after provisioning.\nResources like Entry can have schemas associated with them. This scope allows users to attach tags to an\nindividual column based on that schema.\n\nFor attaching a tag to a nested column, use '.' to separate the column names. Example:\n'outer_column.inner_column'"]
    pub fn column(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.column", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the tag in URL format. Example:\nprojects/{project_id}/locations/{location}/entrygroups/{entryGroupId}/entries/{entryId}/tags/{tag_id} or\nprojects/{project_id}/locations/{location}/entrygroups/{entryGroupId}/tags/{tag_id}\nwhere tag_id is a system-generated identifier. Note that this Tag may not actually be stored in the location in this name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe name of the parent this tag is attached to. This can be the name of an entry or an entry group. If an entry group, the tag will be attached to\nall entries in that group."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\nThe resource name of the tag template that this tag uses. Example:\nprojects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}\nThis field cannot be modified after creation."]
    pub fn template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_displayname` after provisioning.\nThe display name of the tag template."]
    pub fn template_displayname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_displayname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataCatalogTagTimeoutsElRef {
        DataCatalogTagTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCatalogTagFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bool_value: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    double_value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_value: Option<PrimField<String>>,
    field_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_value: Option<PrimField<String>>,
}
impl DataCatalogTagFieldsEl {
    #[doc = "Set the field `bool_value`.\nHolds the value for a tag field with boolean type."]
    pub fn set_bool_value(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bool_value = Some(v.into());
        self
    }
    #[doc = "Set the field `double_value`.\nHolds the value for a tag field with double type."]
    pub fn set_double_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.double_value = Some(v.into());
        self
    }
    #[doc = "Set the field `enum_value`.\nThe display name of the enum value."]
    pub fn set_enum_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enum_value = Some(v.into());
        self
    }
    #[doc = "Set the field `string_value`.\nHolds the value for a tag field with string type."]
    pub fn set_string_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.string_value = Some(v.into());
        self
    }
    #[doc = "Set the field `timestamp_value`.\nHolds the value for a tag field with timestamp type."]
    pub fn set_timestamp_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timestamp_value = Some(v.into());
        self
    }
}
impl ToListMappable for DataCatalogTagFieldsEl {
    type O = BlockAssignable<DataCatalogTagFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogTagFieldsEl {
    #[doc = ""]
    pub field_name: PrimField<String>,
}
impl BuildDataCatalogTagFieldsEl {
    pub fn build(self) -> DataCatalogTagFieldsEl {
        DataCatalogTagFieldsEl {
            bool_value: core::default::Default::default(),
            double_value: core::default::Default::default(),
            enum_value: core::default::Default::default(),
            field_name: self.field_name,
            string_value: core::default::Default::default(),
            timestamp_value: core::default::Default::default(),
        }
    }
}
pub struct DataCatalogTagFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogTagFieldsElRef {
    fn new(shared: StackShared, base: String) -> DataCatalogTagFieldsElRef {
        DataCatalogTagFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogTagFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nHolds the value for a tag field with boolean type."]
    pub fn bool_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of this field"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `double_value` after provisioning.\nHolds the value for a tag field with double type."]
    pub fn double_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.double_value", self.base))
    }
    #[doc = "Get a reference to the value of field `enum_value` after provisioning.\nThe display name of the enum value."]
    pub fn enum_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enum_value", self.base))
    }
    #[doc = "Get a reference to the value of field `field_name` after provisioning.\n"]
    pub fn field_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }
    #[doc = "Get a reference to the value of field `order` after provisioning.\nThe order of this field with respect to other fields in this tag. For example, a higher value can indicate\na more important field. The value can be negative. Multiple fields can have the same order, and field orders\nwithin a tag do not have to be sequential."]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nHolds the value for a tag field with string type."]
    pub fn string_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
    #[doc = "Get a reference to the value of field `timestamp_value` after provisioning.\nHolds the value for a tag field with timestamp type."]
    pub fn timestamp_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timestamp_value", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCatalogTagTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataCatalogTagTimeoutsEl {
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
impl ToListMappable for DataCatalogTagTimeoutsEl {
    type O = BlockAssignable<DataCatalogTagTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogTagTimeoutsEl {}
impl BuildDataCatalogTagTimeoutsEl {
    pub fn build(self) -> DataCatalogTagTimeoutsEl {
        DataCatalogTagTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataCatalogTagTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogTagTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataCatalogTagTimeoutsElRef {
        DataCatalogTagTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogTagTimeoutsElRef {
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
struct DataCatalogTagDynamic {
    fields: Option<DynamicBlock<DataCatalogTagFieldsEl>>,
}
